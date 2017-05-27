use serde_json;
use std::iter::FromIterator;
use fall_parse;
use fall_tree::{AstNode, AstClass};
use lang::{SelectorKind, RefKind, SynRule};
use util::{scream, camel};
use tera::{Tera, Context};

use lang::{self, Expr};

pub fn generate(file: lang::File) -> String {
    #[derive(Serialize)]
    struct CtxLexRule<'f> { ty: &'f str, re: String, f: Option<&'f str> };

    #[derive(Serialize)]
    struct CtxAstNode<'f> { struct_name: String, node_type_name: String, methods: Vec<CtxMethod<'f>> }

    #[derive(Serialize)]
    struct CtxAstClass { enum_name: String, variants: Vec<(String, String)> }

    #[derive(Serialize)]
    struct CtxMethod<'f> { name: &'f str, ret_type: String, body: String }

    let mut context = Context::new();
    context.add("node_types", &file.node_types());

    let parser = Vec::from_iter(file.syn_rules().map(compile_rule));
    let parser = serde_json::to_string(&parser).unwrap();
    context.add("parser_json", &parser);
    context.add("lex_rules", &file.tokenizer_def().expect("no tokens defined").lex_rules().map(|r| {
        CtxLexRule { ty: r.node_type(), re: format!("{:?}", r.token_re()), f: r.extern_fn() }
    }).collect::<Vec<_>>());
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
                            SelectorKind::Text(_) => "&'f str".to_owned(),
                        },
                        body: match method.selector_kind() {
                            SelectorKind::Single(_) => format!("{}::new(self.node.children()).next().unwrap()", iter_type),
                            SelectorKind::Opt(_) => format!("{}::new(self.node.children()).next()", iter_type),
                            SelectorKind::Many(_) => format!("{}::new(self.node.children())", iter_type),
                            SelectorKind::Text(name) => format!("child_of_type_exn(self.node, {}).text()", name),
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

    Tera::one_off(TEMPLATE.trim(), &context, false).unwrap()
}

fn compile_rule(ast: SynRule) -> fall_parse::SynRule {
    fall_parse::SynRule {
        ty: ast.resolve_ty(),
        body: compile_expr(ast.body()),
    }
}

fn compile_expr(ast: Expr) -> fall_parse::Expr {
    match ast {
        Expr::BlockExpr(block) => {
            fall_parse::Expr::Or(block.alts().map(compile_expr).collect())
        }
        Expr::SeqExpr(seq) => {
            fn is_commit(part: Expr) -> bool {
                part.node().text() == "<commit>"
            }
            let commit = seq.parts().position(is_commit);
            let parts = seq.parts()
                .filter(|&p| !is_commit(p))
                .map(compile_expr);
            fall_parse::Expr::And(parts.collect(), commit)
        }
        Expr::RefExpr(ref_) => match ref_.resolve() {
            Some(RefKind::Token(idx)) => fall_parse::Expr::Token(idx),
            Some(RefKind::RuleReference { idx }) => fall_parse::Expr::Rule(idx),
            None => panic!("Unresolved references: {}", ref_.node().text()),
        },
        Expr::CallExpr(call) => {
            let mut args = call.args();
            let arg = args.next().unwrap();
            match call.fn_name() {
                "rep" => {
                    fn collect_tokens(expr: Expr) -> Option<Vec<usize>> {
                        let ts = expr.token_set().unwrap_or_else(|| {
                            panic!("Bad token set: `{}`", expr.node().text())
                        });
                        if ts.is_empty() { None } else { Some(ts) }
                    }
                    let skip = match args.next() {
                        None => None,
                        Some(expr) => collect_tokens(expr)
                    };
                    let stop = match args.next() {
                        None => None,
                        Some(expr) => collect_tokens(expr)
                    };
                    fall_parse::Expr::Rep(Box::new(compile_expr(arg)), skip, stop)
                }
                "opt" => fall_parse::Expr::Opt(Box::new(compile_expr(arg))),
                "not" => {
                    fall_parse::Expr::Not(arg.token_set().unwrap_or_else(|| {
                        panic!("Bad token set: `{}`", arg.node().text())
                    }))
                }
                "layer" => fall_parse::Expr::Layer(
                    Box::new(compile_expr(arg)),
                    Box::new(compile_expr(args.next().unwrap()))
                ),
                _ => unimplemented!(),
            }
        }
    }
}

const TEMPLATE: &'static str = r#####"
use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

{% for node_type in node_types %}
pub const {{ node_type | upper }}: NodeType = NodeType({{ 100 + loop.index0 }});
{% endfor %}

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            {% for node_type in node_types %}{{ node_type | upper }}, {% endfor %}
        ];
        let parser_json = r##"{{ parser_json }}"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, &self.tokenizer, &|&x, y| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(x, y)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    {% for node_type in node_types %}
                    {{ node_type | upper }} => NodeTypeInfo { name: "{{ node_type | upper }}" },
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
use fall_tree::{AstNode, AstChildren, AstClass, AstClassChildren, Node};
use fall_tree::search::child_of_type_exn;

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
