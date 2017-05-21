use fall_tree::{AstNode, AstClass};
use lang::{SelectorKind, RefKind};
use util::{scream, camel};
use tera::{Tera, Context};

use lang::{self, Expr};

pub fn generate(file: lang::File) -> String {
    #[derive(Serialize)]
    struct CtxSynRule { is_public: bool, ty: usize, body: String };

    #[derive(Serialize)]
    struct CtxLexRule<'f> { ty: &'f str, re: String, f: Option<&'f str> };

    #[derive(Serialize)]
    struct CtxAstNode<'f> { struct_name: String, node_type_name: String, methods: Vec<CtxMethod<'f>> }

    #[derive(Serialize)]
    struct CtxAstClass { enum_name: String, variants: Vec<(String, String)> }

    #[derive(Serialize)]
    struct CtxMethod<'f> { name: &'f str, ret_type: String, body: String }

    let mut context = Context::new();
    context.add("node_types", &file.nodes_def().expect("no nodes defined").nodes());
    context.add("syn_rules", &file.syn_rules().map(|r| {
        CtxSynRule {
            is_public: r.is_public(),
            ty: file.resolve_ty(r.name()).unwrap_or(0xDEADBEEF),
            body: gen_expr(Expr::BlockExpr(r.block_expr()))
        }
    }).collect::<Vec<_>>());
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

fn gen_expr(expr: Expr) -> String {
    match expr {
        Expr::BlockExpr(block) => {
            format!("Expr::Or(vec![{}])",
                    list(block.alts().map(|it| gen_expr(Expr::SeqExpr(it)))))
        }
        Expr::SeqExpr(seq) => {
            fn is_commit(part: Expr) -> bool {
                part.node().text() == "<commit>"
            }
            let commit = seq.parts().position(is_commit);
            let parts = seq.parts()
                .filter(|&p| !is_commit(p))
                .map(gen_expr);
            format!("Expr::And(vec![{}], {:?})", list(parts), commit)
        }
        Expr::RefExpr(ref_) => match ref_.resolve() {
            Some(RefKind::Token(idx)) => format!("Expr::Token({})", idx),
            Some(RefKind::RuleReference { idx }) => format!("Expr::Rule({:?})", idx),
            None => panic!("Unresolved references: {}", ref_.node().text()),
        },
        Expr::CallExpr(call) => {
            let mut args = call.args();
            let arg = args.next().unwrap();
            match call.fn_name() {
                "rep" => {
                    let skip = match args.next() {
                        None => "None".to_owned(),
                        Some(expr) => {
                            let block = match expr {
                                Expr::BlockExpr(block) => block,
                                _ => panic!("bad rep argument!")
                            };
                            let tokens: String = block.alts()
                                .flat_map(|alt| alt.parts())
                                .map(|part| {
                                    let ref_ = match part {
                                        Expr::RefExpr(ref_) => ref_,
                                        _ => panic!("bad rep argument2")
                                    };
                                    if let RefKind::Token(idx) = ref_.resolve().unwrap() {
                                        format!("{}, ", idx)
                                    } else {
                                        panic!("bad break in rep {:?}", part.node().text())
                                    }
                                })
                                .collect();
                            format!("Some(vec![{}])", tokens)
                        }
                    };
                    format!("Expr::Rep(Box::new({}), {}, None)", gen_expr(arg), skip)
                }
                "opt" => format!("Expr::Opt(Box::new({}))", gen_expr(arg)),
                _ => unimplemented!(),
            }
        }
    }
}

fn list<D: ::std::fmt::Display, I: Iterator<Item=D>>(i: I) -> String {
    let mut result = String::new();
    let mut sep = "";
    for item in i {
        result += sep;
        sep = ", ";
        result += &item.to_string();
    }
    result
}

const TEMPLATE: &'static str = r#####"
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

{% for node_type in node_types %}
pub const {{ node_type | upper }}: NodeType = NodeType({{ 100 + loop.index0 }});
{% endfor %}

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Expr, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            {% for node_type in node_types %}{{ node_type | upper }}, {% endfor %}
        ];

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(b)
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
            parser: vec![
                {% for rule in syn_rules %}
                SynRule {
                    ty: {% if rule.is_public %}Some({{ rule.ty }}){% else %}None{% endif %},
                    body: {{ rule.body }},
                },
                {% endfor %}
            ]
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
