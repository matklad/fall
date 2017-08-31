use serde_json;
use fall_parse;
use fall_tree::{Text, AstNode, AstClass};
use lang_fall::{RefKind, SynRule, Expr, FallFile, BlockExpr, PratKind, CallKind,
                MethodDef, MethodDescription, Arity, ChildKind};
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

    let verbatim = file.verbatim_def().map(|v| v.contents());
    context.add("verbatim", &verbatim);
    context.add("has_whitespace_binder", &verbatim.map(|t| t.contains("whitespace_binder")).unwrap_or(false));

    if let Some(ast) = file.ast_def() {
        context.add("ast_nodes", &ast.ast_nodes().map(|node| {
            Ok(CtxAstNode {
                struct_name: camel(node.name()),
                node_type_name: scream(node.name()),
                methods: node.methods()
                    .map(|method| generate_method(method))
                    .collect::<Result<Vec<CtxMethod>>>()?
            })
        }).collect::<Result<Vec<_>>>()?);

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

#[derive(Serialize)]
struct CtxMethod<'f> { name: Text<'f>, ret_type: String, body: String }

fn generate_method<'f>(method: MethodDef<'f>) -> Result<CtxMethod<'f>> {
    let description = method.resolve().ok_or(error!("Bad method `{}`", method.node().text()))?;
    let (ret_type, body) = match description {
        MethodDescription::TextAccessor(lex_rule, arity) => {
            let node_type = scream(lex_rule.node_type());
            match arity {
                Arity::Single =>
                    ("Text<'f>".to_owned(),
                     format!("child_of_type_exn(self.node, {}).text()", node_type)),

                Arity::Optional =>
                    ("Option<Text<'f>>".to_owned(),
                     format!("child_of_type(self.node, {}).map(|n| n.text())", node_type)),

                Arity::Many => unimplemented!(),
            }
        }
        MethodDescription::NodeAccessor(kind, arity) => {
            match (kind, arity) {
                (ChildKind::AstNode(n), Arity::Single) =>
                    (format!("{}<'f>", camel(n.name())),
                     "AstChildren::new(self.node.children()).next().unwrap()".to_owned()),
                (ChildKind::AstNode(n), Arity::Optional) =>
                    (format!("Option<{}<'f>>", camel(n.name())),
                     "AstChildren::new(self.node.children()).next()".to_owned()),
                (ChildKind::AstNode(n), Arity::Many) =>
                    (format!("AstChildren<'f, {}<'f>>", camel(n.name())),
                     "AstChildren::new(self.node.children())".to_owned()),

                (ChildKind::AstClass(n), Arity::Single) =>
                    (format!("{}<'f>", camel(n.name())),
                     "AstClassChildren::new(self.node.children()).next().unwrap()".to_owned()),
                (ChildKind::AstClass(n), Arity::Optional) =>
                    (format!("Option<{}<'f>>", camel(n.name())),
                     "AstClassChildren::new(self.node.children()).next()".to_owned()),
                (ChildKind::AstClass(n), Arity::Many) =>
                    (format!("AstClassChildren<'f, {}<'f>>", camel(n.name())),
                     "AstClassChildren::new(self.node.children())".to_owned()),

                (ChildKind::Token(lex_rule), arity) => {
                    let node_type = scream(lex_rule.node_type());
                    match arity {
                        Arity::Single =>
                            ("Node<'f>".to_owned(),
                             format!("self.node().children().find(|n| n.ty() == {}).unwrap()", node_type)),
                        Arity::Optional =>
                            ("Option<Node<'f>>".to_owned(),
                             format!("self.node().children().find(|n| n.ty() == {})", node_type)),
                        Arity::Many => unimplemented!(),
                    }
                }
            }
        }
    };

    Ok(CtxMethod { name: method.name(), ret_type: ret_type, body: body })
}

fn compile_rule(ast: SynRule) -> Result<Option<fall_parse::SynRule>> {
    let expr = match (ast.is_pratt(), ast.body()) {
        (true, Expr::BlockExpr(block)) => fall_parse::Expr::Pratt(compile_pratt(block)?),
        (true, _) => unreachable!(),
        (false, body) => compile_expr(body)?
    };

    let expr = if let Some(idx) = ast.resolve_ty() {
        if ast.is_replaces() {
            fall_parse::Expr::PubReplace {
                ty_idx: idx,
                body: Box::new(expr),
            }
        } else {
            fall_parse::Expr::Pub {
                ty_idx: idx,
                body: Box::new(expr),
                replaceable: ast.is_replaceable(),
            }
        }
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
        let variant = match prat_kind {
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
            }
            PratKind::Prefix(priority) => {
                let alt = match rule.body() {
                    Expr::BlockExpr(block) => block.alts().next().ok_or(error!(
                        "bad pratt rule"
                    ))?,
                    _ => return Err(error!("bad pratt rule"))
                };
                let op = match alt {
                    Expr::SeqExpr(seq) => seq.parts().nth(0).unwrap(),
                    _ => return Err(error!("bad pratt rule"))
                };
                fall_parse::PrattVariant::Prefix {
                    ty: ty,
                    op: Box::new(compile_expr(op)?),
                    priority: priority,
                }
            }
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
        Expr::RefExpr(ref_) => match ref_.resolve().ok_or(error!("Unresolved references: {}", ref_.node().text()))? {
            RefKind::Token(rule) => {
                if rule.is_contextual() {
                    let text = rule.token_text().ok_or(error!("Missing contextual token text"))?;
                    fall_parse::Expr::ContextualToken(rule.node_type_index(), text.to_string())
                } else {
                    fall_parse::Expr::Token(rule.node_type_index())
                }
            }
            RefKind::RuleReference(rule) => fall_parse::Expr::Rule(rule.index()),
            RefKind::Param(p) => fall_parse::Expr::Var(p.idx()),
        },
        Expr::CallExpr(call) => {
            let r = match call.kind().map_err(|e| error!("Failed to compile {}: {}", call.node().text(), e))? {
                CallKind::Eof => fall_parse::Expr::Eof,
                CallKind::Any => fall_parse::Expr::Any,
                CallKind::Enter(idx, expr) => fall_parse::Expr::Enter(idx, Box::new(compile_expr(expr)?)),
                CallKind::Exit(idx, expr) => fall_parse::Expr::Exit(idx, Box::new(compile_expr(expr)?)),
                CallKind::IsIn(idx) => fall_parse::Expr::IsIn(idx),
                CallKind::Not(expr) => fall_parse::Expr::Not(Box::new(compile_expr(expr)?)),
                CallKind::Layer(e1, e2) => fall_parse::Expr::Layer(
                    Box::new(compile_expr(e1)?),
                    Box::new(compile_expr(e2)?)
                ),
                CallKind::WithSkip(e1, e2) => fall_parse::Expr::WithSkip(
                    Box::new(compile_expr(e1)?),
                    Box::new(compile_expr(e2)?)
                ),
                CallKind::RuleCall(rule, args) => fall_parse::Expr::Call(
                    Box::new(fall_parse::Expr::Rule(rule.index())),
                    args.into_iter()
                        .map(|(i, e)| Ok((i, compile_expr(e)?)))
                        .collect::<Result<Vec<_>>>()?
                ),
                CallKind::PrevIs(tokens) => fall_parse::Expr::PrevIs(tokens),
                CallKind::Commit => panic!("Should be handled specially"),
            };
            return Ok(r);
        }
        Expr::OptExpr(opt_expr) => fall_parse::Expr::Opt(Box::new(compile_expr(opt_expr.expr())?)),
        Expr::RepExpr(rep_expr) => fall_parse::Expr::Rep(Box::new(compile_expr(rep_expr.expr())?)),
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


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"{{ parser_json }}"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            {% for node_type in node_types %}{{ node_type.0 | upper }}, {% endfor %}
        ],
        lexical_rules: vec![
            {% for rule in lex_rules %}
            LexRule::new({{ rule.ty | upper }}, {{ rule.re }}, {% if rule.f is string %} Some({{ rule.f }}) {% else %} None {% endif %}),
            {% endfor %}
        ],
        syntactical_rules: serde_json::from_str(parser_json).unwrap(),
        {% if has_whitespace_binder %}
            whitespace_binder: whitespace_binder,
        {% endif %}
        .. Default::default()
    }
}

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::ParserDefinition;

        struct Impl { parser_definition: ParserDefinition };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                self.parser_definition.parse(text, &LANG)
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

        Language::new(Impl { parser_definition: create_parser_definition() })
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
    const NODE_TYPE: NodeType  = {{ node.node_type_name }};
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
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
    const NODE_TYPES: &'static [NodeType] = &[
        {% for v in class.variants %}
            {{ v.0 }},
        {% endfor %}
    ];

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
