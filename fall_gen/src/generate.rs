use serde_json;
use fall_parse;
use fall_tree::{Text, AstNode, AstClass};
use lang_fall::{SelectorKind, RefKind, SynRule, Expr, FallFile, BlockExpr, PratKind};
use util::{scream, camel};
use tera::{Tera, Context};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.msg.fmt(f)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

macro_rules! error {
    ( $($tt:tt)* ) => { Error { msg: format!($($tt)*) } };
}

pub fn generate(file: FallFile) -> Result<String> {
    #[derive(Serialize)]
    struct CtxLexRule<'f> { ty: Text<'f>, re: String, f: Option<Text<'f>> };

    #[derive(Serialize)]
    struct CtxAstNode<'f> { struct_name: String, node_type_name: String, methods: Vec<CtxMethod<'f>> }

    #[derive(Serialize)]
    struct CtxAstClass { enum_name: String, variants: Vec<(String, String)> }

    #[derive(Serialize)]
    struct CtxMethod<'f> { name: Text<'f>, ret_type: String, body: String }

    let mut context = Context::new();
    context.add("node_types", &file.node_types());

    let mut parser = Vec::new();
    for r in file.syn_rules() {
        if let Some(r) = compile_rule(r)? {
            parser.push(r)
        }
    }
    let parser = serde_json::to_string(&parser).unwrap();
    context.add("parser_json", &parser);

    let lex_rules = file.tokenizer_def()
        .ok_or(error!("no tokens defined"))?
        .lex_rules()
        .filter(|r| !r.is_contextual())
        .map(|r| {
            let re = r.token_re().ok_or(error!("Bad token"))?;
            Ok(CtxLexRule { ty: r.node_type(), re: format!("{:?}", re), f: r.extern_fn() })
        }).collect::<Result<Vec<_>>>()?;

    context.add("lex_rules", &lex_rules);
    context.add("verbatim", &file.verbatim_def().map(|v| v.contents()));

    if let Some(ast) = file.ast_def() {
        context.add("ast_nodes", &ast.ast_nodes().map(|node| {
            CtxAstNode {
                struct_name: camel(node.name()),
                node_type_name: scream(node.name()),
                methods: node.methods().map(|method| {
                    let iter_type = if method.is_class() { "AstClassChildren" } else { "AstChildren" };
                    CtxMethod {
                        name: method.name(),
                        ret_type: match method.selector_kind() {
                            SelectorKind::Single(name) => format!("{}<'f>", camel(name)),
                            SelectorKind::Opt(name) => format!("Option<{}<'f>>", camel(name)),
                            SelectorKind::Many(name) => format!("{}<'f, {}<'f>>", iter_type, camel(name)),
                            SelectorKind::Text(_) => "Text<'f>".to_owned(),
                            SelectorKind::OptText(_) => "Option<Text<'f>>".to_owned(),
                        },
                        body: match method.selector_kind() {
                            SelectorKind::Single(_) => format!("{}::new(self.node.children()).next().unwrap()", iter_type),
                            SelectorKind::Opt(_) => format!("{}::new(self.node.children()).next()", iter_type),
                            SelectorKind::Many(_) => format!("{}::new(self.node.children())", iter_type),
                            SelectorKind::Text(name) => format!("child_of_type_exn(self.node, {}).text()", name),
                            SelectorKind::OptText(name) => format!("child_of_type(self.node, {}).map(|n| n.text())", name),
                        }
                    }
                }).collect()
            }
        }).collect::<Vec<_>>());

        context.add("ast_classes", &ast.ast_classes().map(|class| {
            CtxAstClass {
                enum_name: camel(class.name()),
                variants: class.variants().map(|variant| (scream(variant), camel(variant))).collect(),
            }
        }).collect::<Vec<_>>());
    }

    Tera::one_off(TEMPLATE.trim(), &context, false)
        .map_err(|e| error!("Failed to format template:\n{:?}", e))
}

fn compile_rule(ast: SynRule) -> Result<Option<fall_parse::SynRule>> {
    let expr = match (ast.is_pratt(), ast.body()) {
        (true, Expr::BlockExpr(block)) => fall_parse::Expr::Pratt(compile_pratt(block)?),
        (true, _) => unreachable!(),
        (false, body) => compile_expr(body)?
    };

    let expr = if let Some(idx) = ast.resolve_ty() {
        fall_parse::Expr::Pub(idx, Box::new(expr))
    } else {
        expr
    };

    Ok(Some(fall_parse::SynRule { body: expr }))
}

fn compile_pratt(ast: BlockExpr) -> Result<Vec<fall_parse::PrattVariant>> {
    fn alt_to_rule<'f>(alt: Expr<'f>) -> Result<SynRule<'f>> {
        match alt {
            Expr::SeqExpr(expr) => match expr.parts().next() {
                Some(Expr::RefExpr(r)) => match r.resolve() {
                    Some(RefKind::RuleReference(rule)) => Ok(rule),
                    _ => return Err(error!("Bad pratt spec")),
                },
                _ => return Err(error!("Bad pratt spec"))
            },
            _ => return Err(error!("Bad pratt spec"))
        }
    }

    let mut result = Vec::new();
    for alt in ast.alts() {
        let rule = alt_to_rule(alt)?;
        let ty = rule.resolve_ty().ok_or(error!("non public pratt rule"))?;
        let prat_kind = rule.pratt_kind().ok_or(error!("pratt rule without attributes"))?;
        let variant =match prat_kind {
            PratKind::Atom => fall_parse::PrattVariant::Atom {
                body: Box::new(compile_rule(rule)?.unwrap().body),
            },
            PratKind::Postfix => {
                let alt = match rule.body() {
                    Expr::BlockExpr(block) => block.alts().next().ok_or(error!(
                    "bad pratt rule"
                    ))?,
                    _ => return Err(error!("bad pratt rule"))
                };
                let op = match alt {
                    Expr::SeqExpr(seq) => seq.parts().nth(1).unwrap(),
                    _ => return Err(error!("bad pratt rule"))
                };
                fall_parse::PrattVariant::Postfix {
                    ty: ty,
                    op: Box::new(compile_expr(op)?),
                }
            },
            PratKind::Bin(priority) => {
                let alt = match rule.body() {
                    Expr::BlockExpr(block) => block.alts().next().ok_or(error!(
                    "bad pratt rule"
                    ))?,
                    _ => return Err(error!("bad pratt rule"))
                };
                let op = match alt {
                    Expr::SeqExpr(seq) => seq.parts().nth(1).unwrap(),
                    _ => return Err(error!("bad pratt rule"))
                };
                fall_parse::PrattVariant::Binary {
                    ty: ty,
                    op: Box::new(compile_expr(op)?),
                    priority: priority
                }
            }
        };

        result.push(variant)
    }

    Ok(result)
}

fn compile_expr(ast: Expr) -> Result<fall_parse::Expr> {
    let result = match ast {
        Expr::BlockExpr(block) => fall_parse::Expr::Or(block.alts().map(compile_expr).collect::<Result<Vec<_>>>()?),
        Expr::SeqExpr(seq) => {
            fn is_commit(part: Expr) -> bool {
                part.node().text() == "<commit>"
            }
            let commit = seq.parts().position(is_commit);
            let parts = seq.parts()
                .filter(|&p| !is_commit(p))
                .map(compile_expr);
            fall_parse::Expr::And(parts.collect::<Result<Vec<_>>>()?, commit)
        }
        Expr::RefExpr(ref_) => match ref_.resolve() {
            Some(RefKind::Token(rule)) => {
                if rule.is_contextual() {
                    let text = rule.token_re().ok_or(error!("Missing token regex"))?;
                    fall_parse::Expr::ContextualToken(rule.node_type_index(), text)
                } else {
                    fall_parse::Expr::Token(rule.node_type_index())
                }
            }
            Some(RefKind::RuleReference(rule)) => fall_parse::Expr::Rule(rule.index()),
            None => return Err(error!("Unresolved references: {}", ref_.node().text())),
        },
        Expr::CallExpr(call) => {
            let fn_name = call.fn_name().to_cow();
            if fn_name == "eof" {
                return Ok(fall_parse::Expr::Eof)
            }
            let mut args = call.args();
            let first_arg = args.next().ok_or(error!("expected an argument"))?;
            macro_rules! token_set_arg {
                () => {
                    first_arg.token_set().ok_or(
                        error!("Bad token set: `{}`", first_arg.node().text())
                    )?
                }
            }
            match fn_name.as_ref() {
                "not" => fall_parse::Expr::Not(token_set_arg!()),
                "rep" => {
                    if args.next().is_some() {
                        return Err(error!("extra argument to rep"))
                    }
                    fall_parse::Expr::Rep(Box::new(compile_expr(first_arg)?))
                }
                "not_ahead" => {
                    if args.next().is_some() {
                        return Err(error!("extra argument to not_ahead"))
                    }
                    fall_parse::Expr::NotAhead(Box::new(compile_expr(first_arg)?))
                }

                "opt" => fall_parse::Expr::Opt(Box::new(compile_expr(first_arg)?)),
                "layer" => fall_parse::Expr::Layer(
                    Box::new(compile_expr(first_arg)?),
                    Box::new(compile_expr(args.next().ok_or(
                        error!("not enough arguments to layer")
                    )?)?)
                ),
                "with_skip" => fall_parse::Expr::WithSkip(
                    Box::new(compile_expr(first_arg)?),
                    Box::new(compile_expr(args.next().ok_or((
                        error!("not enough arguments to layer")
                    ))?)?)
                ),
                _ => unimplemented!(),
            }
        }
    };

    Ok(result)
}

const TEMPLATE: &'static str = r#####"
use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

{% for node_type in node_types %}
pub const {{ node_type.0 | upper }}: NodeType = NodeType({{ 100 + loop.index0 }});
{% endfor %}

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            {% for node_type in node_types %}{{ node_type.0 | upper }}, {% endfor %}
        ];
        let parser_json = r##"{{ parser_json }}"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                parse(
                    text,
                    &LANG,
                    &self.tokenizer,
                    &|tokens, stats| Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                )
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                    {% for node_type in node_types %}
                    {{ node_type.0 | upper }} => NodeTypeInfo { name: "{{ node_type.0 | upper }}", whitespace_like: {{ node_type.1 }} },
                    {% endfor %}
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                {% for rule in lex_rules %}
                LexRule::new({{ rule.ty | upper }}, {{ rule.re }}, {% if rule.f is string %} Some({{ rule.f }}) {% else %} None {% endif %}),
                {% endfor %}
            ],
            parser: parser,
        })
    };
}
{% if verbatim is string %}
{{ verbatim }}
{% endif %}

{% if ast_nodes is defined %}
use self::fall_tree::{Text, AstNode, AstChildren, AstClass, AstClassChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

{% for node in ast_nodes %}
#[derive(Clone, Copy)]
pub struct {{ node.struct_name }}<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for {{ node.struct_name }}<'f> {
    fn ty() -> NodeType { {{ node.node_type_name }} }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        {{ node.struct_name }} { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> {{ node.struct_name }}<'f> {
    {% for method in node.methods %}
    pub fn {{ method.name }}(&self) -> {{ method.ret_type }} {
        {{ method.body }}
    }
    {% endfor %}
}
{% endfor %}

{% for class in ast_classes %}
#[derive(Clone, Copy)]
pub enum {{ class.enum_name }}<'f> {
    {% for v in class.variants %}
        {{ v.1 }}({{ v.1 }}<'f>),
    {% endfor %}
}

impl<'f> AstClass<'f> for {{ class.enum_name }}<'f> {
    fn tys() -> &'static [NodeType] {
        const TYS: &[NodeType] = &[
            {% for v in class.variants %}
                {{ v.0 }},
            {% endfor %}
        ];
        TYS
    }

    fn new(node: Node<'f>) -> Self {
        match node.ty() {
            {% for v in class.variants %}
                {{ v.0 }} => {{ class.enum_name }}::{{ v.1 }}({{ v.1 }}::new(node)),
            {% endfor %}
            _ => panic!("Bad ast class")
        }
    }

    fn node(&self) -> Node<'f> {
        match *self {
            {% for v in class.variants %}
                {{ class.enum_name }}::{{ v.1 }}(n) => n.node(),
            {% endfor %}
        }
    }
}
{% endfor %}

{% endif %}
"#####;
