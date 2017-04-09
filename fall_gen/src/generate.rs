use fall_tree::AstNode;
use ast_ext::{PartKind, SelectorKind};
use util::{scream, snake};
use tera::{Tera, Context};

use syntax::{File, Alt, Part};

pub fn generate(file: File) -> String {

    #[derive(Serialize)]
    struct CtxSynRule<'f> { is_public: bool, name: &'f str, alts: Vec<String> };

    #[derive(Serialize)]
    struct CtxLexRule<'f> { ty: &'f str, re: String, f: Option<&'f str> };

    #[derive(Serialize)]
    struct CtxAstNode<'f> { struct_name: String, node_type_name: String, methods: Vec<CtxMethod<'f>> }

    #[derive(Serialize)]
    struct CtxMethod<'f> { name: &'f str, ret_type: String, body: String }

    let mut context = Context::new();
    context.add("node_types", &file.nodes_def().nodes());
    context.add("syn_rules", &file.syn_rules().map(|r| {
        CtxSynRule {
            is_public: r.is_public(),
            name: r.name(),
            alts: r.alts().map(generate_alt).collect()
        }
    }).collect::<Vec<_>>());
    context.add("lex_rules", &file.tokenizer_def().lex_rules().map(|r| {
        CtxLexRule { ty: r.node_type(), re: format!("{:?}", r.token_re()), f: r.extern_fn() }
    }).collect::<Vec<_>>());
    context.add("verbatim", &file.verbatim_def().map(|v| v.contents()));

    if let Some(ast) = file.ast_def() {
        context.add("ast_nodes", &ast.ast_nodes().map(|node| {
            CtxAstNode {
                struct_name: snake(node.name()),
                node_type_name: scream(node.name()),
                methods: node.methods().map(|method| {
                    CtxMethod {
                        name: method.name(),
                        ret_type: match method.selector_kind() {
                            SelectorKind::Single(name) => format!("{}<'f>", snake(name)),
                            SelectorKind::Opt(name) => format!("Option<{}<'f>>", snake(name)),
                            SelectorKind::Many(name) => format!("AstChildren<'f, {}<'f>>", snake(name)),
                            SelectorKind::Text(_) => "&'f str".to_owned(),
                        },
                        body: match method.selector_kind() {
                            SelectorKind::Single(_) => format!("AstChildren::new(self.node.children()).next().unwrap()"),
                            SelectorKind::Opt(_) => format!("AstChildren::new(self.node.children()).next()"),
                            SelectorKind::Many(_) => format!("AstChildren::new(self.node.children())"),
                            SelectorKind::Text(name) => format!("child_of_type_exn(self.node, {}).text()", name),
                        }
                    }
                }).collect()
            }
        }).collect::<Vec<_>>());
    }

    Tera::one_off(TEMPLATE.trim(), &context, false).unwrap()
}

fn generate_alt(alt: Alt) -> String {
    fn is_commit(part: Part) -> bool {
        part.node().text() == "<commit>"
    }
    let commit = alt.parts().position(is_commit);

    let parts = alt.parts()
        .filter(|&p| !is_commit(p))
        .map(|p| generate_part(p))
        .collect::<Vec<_>>();
    format!("Alt {{ parts: &[{}], commit: {:?} }}", parts.join(", "), commit)
}

fn generate_part(part: Part) -> String {
    match part.kind() {
        PartKind::Token(t) => format!("Part::Token({})", scream(t)),
        PartKind::RuleReference { idx } => format!("Part::Rule({:?})", idx),
        PartKind::Call { name, mut alts } => {
            let op = match name {
                "rep" => "Rep",
                "opt" => "Opt",
                _ => unimplemented!(),
            };
            format!("Part::{}({})", op, generate_alt(alts.next().unwrap()))
        }
    }
}

const TEMPLATE: &'static str = r#####"
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

{% for node_type in node_types %}
pub const {{ node_type | upper }}: NodeType = NodeType({{ 100 + loop.index0 }});
{% endfor %}

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Alt, Part, Parser};

        {% for node_type in node_types %}
        {{ node_type | upper }}.register(NodeTypeInfo { name: "{{ node_type | upper }}" });
        {% endfor %}

        const PARSER: &'static [SynRule] = &[
            {% for rule in syn_rules %}
            SynRule {
                ty: {% if rule.is_public %}Some({{ rule.name | upper }}){% else %}None{% endif %},
                alts: &[{% for alt in rule.alts %}{% if loop.first %}{{ alt }}{% else %}, {{ alt }}{% endif %}{% endfor %}],
            },
            {% endfor %}
        ];

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(text, FILE, &self.tokenizer, &|b| Parser::new(PARSER).parse(b))
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                {% for rule in lex_rules %}
                LexRule::new({{ rule.ty | upper }}, {{ rule.re }}, {% if rule.f is string %} Some({{ rule.f }}) {% else %} None {% endif %}),
                {% endfor %}
            ]
        })
    };
}
{% if verbatim is string %}
{{ verbatim }}
{% endif %}

{% if ast_nodes is defined %}
use fall_tree::{AstNode, AstChildren, Node};
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
{% endif %}
"#####;
