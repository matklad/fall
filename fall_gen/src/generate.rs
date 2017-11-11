use serde_json;
use fall_parse;
use fall_tree::{Text, AstNode};
use lang_fall::{RefKind, SynRule, Expr, BlockExpr, PratVariant, PrattOp,
                CallKind, MethodDef, MethodDescription, Arity, ChildKind,
                Analysis};
use lang_fall::editor_api::Severity;
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

pub fn generate(analysis: &Analysis) -> Result<String> {
    for d in analysis.collect_all_diagnostics() {
        match d.severity {
            Severity::Error => {
                let span = analysis.ast().node().text().slice(d.range);
                return Err(error!("E: {}\n{}", d.message, span));
            }
            Severity::Warning => ()
        }
    }

    #[derive(Serialize)]
    struct CtxLexRule<'f> { ty: Text<'f>, re: String, f: Option<Text<'f>> };

    #[derive(Serialize)]
    struct CtxAstNode<'f> { struct_name: String, node_type_name: String, methods: Vec<CtxMethod<'f>> }

    #[derive(Serialize)]
    struct CtxAstClass { enum_name: String, variants: Vec<(String, String)> }

    let file = analysis.ast();
    let mut context = Context::new();
    context.add("node_types", &file.node_types());

    let mut parser = Vec::new();
    for r in file.syn_rules() {
        if let Some(r) = compile_rule(analysis, r)? {
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
                     "AstChildren::new(self.node.children()).next().unwrap()".to_owned()),
                (ChildKind::AstClass(n), Arity::Optional) =>
                    (format!("Option<{}<'f>>", camel(n.name())),
                     "AstChildren::new(self.node.children()).next()".to_owned()),
                (ChildKind::AstClass(n), Arity::Many) =>
                    (format!("AstChildren<'f, {}<'f>>", camel(n.name())),
                     "AstChildren::new(self.node.children())".to_owned()),

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

    Ok(CtxMethod { name: method.name(), ret_type, body })
}

fn compile_rule<'f>(analysis: &Analysis<'f>, ast: SynRule<'f>) -> Result<Option<fall_parse::SynRule>> {
    let expr = match (ast.is_pratt(), ast.body()) {
        (true, Expr::BlockExpr(block)) => fall_parse::Expr::Pratt(compile_pratt(analysis, block)?),
        (true, _) => unreachable!(),
        (false, body) => compile_expr(analysis, body)?
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

fn compile_pratt<'f>(analysis: &Analysis<'f>, ast: BlockExpr<'f>) -> Result<fall_parse::PrattTable> {
    fn alt_to_rule<'f>(analysis: &Analysis<'f>, alt: Expr<'f>) -> Result<SynRule<'f>> {
        match alt {
            Expr::SeqExpr(expr) => match expr.parts().next() {
                Some(Expr::RefExpr(ref_)) => match analysis.resolve_reference(ref_) {
                    Some(RefKind::RuleReference(rule)) => Ok(rule),
                    _ => return Err(error!("Bad pratt spec")),
                },
                _ => return Err(error!("Bad pratt spec"))
            },
            _ => return Err(error!("Bad pratt spec"))
        }
    }

    let mut result = fall_parse::PrattTable {
        atoms: Vec::new(),
        prefixes: Vec::new(),
        infixes: Vec::new(),
    };
    for alt in ast.alts() {
        let rule = alt_to_rule(analysis, alt)?;
        let ty = rule.resolve_ty().ok_or(error!("non public pratt rule"))?;
        let prat_kind = analysis.resolve_pratt_variant(rule).ok_or(error!("pratt rule without attributes"))?;
        match prat_kind {
            PratVariant::Atom(_) =>
                result.atoms.push(compile_rule(analysis, rule)?.unwrap().body),
            PratVariant::Postfix(PrattOp { op, priority }) => {
                result.infixes.push(fall_parse::Infix {
                    ty,
                    op: compile_expr(analysis, op)?,
                    priority,
                    has_rhs: false,
                });
            }
            PratVariant::Prefix(PrattOp { op, priority }) => {
                result.prefixes.push(fall_parse::Prefix {
                    ty,
                    op: compile_expr(analysis, op)?,
                    priority,
                })
            }
            PratVariant::Bin(PrattOp { op, priority }) => {
                result.infixes.push(fall_parse::Infix {
                    ty,
                    op: compile_expr(analysis, op)?,
                    priority,
                    has_rhs: true,
                });
            }
        };
    }

    Ok(result)
}

fn compile_expr<'f>(analysis: &Analysis<'f>, ast: Expr<'f>) -> Result<fall_parse::Expr> {
    let result = match ast {
        Expr::BlockExpr(block) => fall_parse::Expr::Or(block.alts().map(|e| compile_expr(analysis, e)).collect::<Result<Vec<_>>>()?),
        Expr::SeqExpr(seq) => {
            fn is_commit(part: Expr) -> bool {
                part.node().text() == "<commit>"
            }
            let commit = seq.parts().position(is_commit);
            let parts = seq.parts()
                .filter(|&p| !is_commit(p))
                .map(|e| compile_expr(analysis, e));
            fall_parse::Expr::And(parts.collect::<Result<Vec<_>>>()?, commit)
        }
        Expr::RefExpr(ref_) => match analysis.resolve_reference(ref_).ok_or(error!("Unresolved references: {}", ref_.node().text()))? {
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
            let r = match analysis.resolve_call(call).ok_or(error!("Failed to compile {}", call.node().text()))? {
                CallKind::Eof => fall_parse::Expr::Eof,
                CallKind::Any => fall_parse::Expr::Any,
                CallKind::Enter(idx, expr) => fall_parse::Expr::Enter(idx, Box::new(compile_expr(analysis, expr)?)),
                CallKind::Exit(idx, expr) => fall_parse::Expr::Exit(idx, Box::new(compile_expr(analysis, expr)?)),
                CallKind::IsIn(idx) => fall_parse::Expr::IsIn(idx),
                CallKind::Not(expr) => fall_parse::Expr::Not(Box::new(compile_expr(analysis, expr)?)),
                CallKind::Layer(e1, e2) => fall_parse::Expr::Layer(
                    Box::new(compile_expr(analysis, e1)?),
                    Box::new(compile_expr(analysis, e2)?)
                ),
                CallKind::WithSkip(e1, e2) => fall_parse::Expr::WithSkip(
                    Box::new(compile_expr(analysis, e1)?),
                    Box::new(compile_expr(analysis, e2)?)
                ),
                CallKind::RuleCall(rule, args) => fall_parse::Expr::Call(
                    Box::new(fall_parse::Expr::Rule(rule.index())),
                    args.iter()
                        .map(|&(i, e)| Ok((i, compile_expr(analysis, e)?)))
                        .collect::<Result<Vec<_>>>()?
                ),
                CallKind::PrevIs(tokens) => fall_parse::Expr::PrevIs((*tokens).clone()),
                CallKind::Commit => panic!("Should be handled specially"),
            };
            return Ok(r);
        }
        Expr::OptExpr(opt_expr) => fall_parse::Expr::Opt(Box::new(compile_expr(analysis, opt_expr.expr())?)),
        Expr::RepExpr(rep_expr) => fall_parse::Expr::Rep(Box::new(compile_expr(analysis, rep_expr.expr())?)),
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
            whitespace_binder,
        {% endif %}
        .. Default::default()
    }
}

pub fn language() -> &'static Language {
    lazy_static! {
        static ref LANG: Language = {
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

    &*LANG
}

{% if verbatim is string %}
{{ verbatim }}
{% endif %}

{% if ast_nodes is defined %}
use self::fall_tree::{Text, AstNode, AstChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

{% for node in ast_nodes %}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct {{ node.struct_name }}<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for {{ node.struct_name }}<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == {{ node.node_type_name }} {
            Some({{ node.struct_name }} { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> {{ node.struct_name }}<'f> {
    {% for method in node.methods %}
    pub fn {{ method.name }}(&self) -> {{ method.ret_type }} {
        {{ method.body }}
    }
    {% endfor %}
}

impl<'f> ::std::fmt::Debug for {{ node.struct_name }}<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("{{ node.struct_name }}@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
{% endfor %}

{% for class in ast_classes %}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum {{ class.enum_name }}<'f> {
    {% for v in class.variants %}
        {{ v.1 }}({{ v.1 }}<'f>),
    {% endfor %}
}

impl<'f> AstNode<'f> for {{ class.enum_name }}<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        {% for v in class.variants %}
        if let Some(n) = {{ v.1 }}::wrap(node) {
            return Some({{ class.enum_name }}::{{ v.1 }}(n))
        }
        {% endfor %}
        None
    }

    fn node(self) -> Node<'f> {
        match self {
            {% for v in class.variants %}
                {{ class.enum_name }}::{{ v.1 }}(n) => n.node(),
            {% endfor %}
        }
    }
}

impl<'f> ::std::fmt::Debug for {{ class.enum_name }}<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(match *self {
            {% for v in class.variants %}
                {{ class.enum_name }}::{{ v.1 }}(..) => "{{ v.1 }}@",
            {% endfor %}
        })?;
        AstNode::node(*self).range().fmt(f)?;
        Ok(())
    }
}
{% endfor %}

{% endif %}
"#####;
