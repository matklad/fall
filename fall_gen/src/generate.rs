use serde_json;
use std::iter::FromIterator;
use fall_parse;
use fall_tree::{Text, AstNode, AstClass};
use lang_fall::{SelectorKind, RefKind, SynRule, Expr, FallFile};
use util::{scream, camel};
use tera::{Tera, Context};

pub fn generate(file: FallFile) -> String {
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

    let parser = Vec::from_iter(file.syn_rules().map(compile_rule));
    let parser = serde_json::to_string(&parser).unwrap();
    context.add("parser_json", &parser);
    context.add("lex_rules", &file.tokenizer_def().expect("no tokens defined").lex_rules().map(|r| {
        let re = r.token_re().expect("Bad token");
        CtxLexRule { ty: r.node_type(), re: format!("{:?}", re), f: r.extern_fn() }
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
            let fn_name = call.fn_name().to_cow();
            if fn_name == "eof" {
                return fall_parse::Expr::Eof
            }
            let mut args = call.args();
            let first_arg = args.next().unwrap();
            macro_rules! token_set_arg {
                () => {
                    first_arg.token_set().unwrap_or_else(|| {
                        panic!("Bad token set: `{}`", first_arg.node().text())
                    })
                }
            }
            match fn_name.as_ref() {
                "not" => fall_parse::Expr::Not(token_set_arg!()),
                "skip_until" => fall_parse::Expr::SkipUntil(token_set_arg!()),

                "rep" => {
                    assert!(args.next().is_none());
                    fall_parse::Expr::Rep(Box::new(compile_expr(first_arg)))
                }
                "not_ahead" => {
                    assert!(args.next().is_none());
                    fall_parse::Expr::NotAhead(Box::new(compile_expr(first_arg)))
                },

                "opt" => fall_parse::Expr::Opt(Box::new(compile_expr(first_arg))),
                "layer" => fall_parse::Expr::Layer(
                    Box::new(compile_expr(first_arg)),
                    Box::new(compile_expr(args.next().unwrap()))
                ),
                "with_skip" => fall_parse::Expr::WithSkip(
                    Box::new(compile_expr(first_arg)),
                    Box::new(compile_expr(args.next().unwrap()))
                ),
                _ => unimplemented!(),
            }
        }
    }
}

const TEMPLATE: &'static str = r#####"
use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
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
            fn parse(&self, text: &str) -> (FileStats, INode) {
                ::fall_parse::parse(text, &self.tokenizer, &|tokens, stats| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                })
            }

            fn reparse(&self, text: &str, parser_id: u32) -> Option<(FileStats, Vec<INode>)> {
                ::fall_parse::reparse(text, &self.tokenizer, &|tokens, stats| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).reparse(parser_id, tokens, stats)
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
use fall_tree::{Text, AstNode, AstChildren, AstClass, AstClassChildren, Node};
use fall_tree::search::{child_of_type_exn, child_of_type};

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
