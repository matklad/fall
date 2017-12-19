use lang_fall::Analysis;
use tera::Tera;

mod codegen;

pub type Result<T> = ::std::result::Result<T, ::failure::Error>;

pub fn generate(analysis: &Analysis) -> Result<String> {
    let mut cg = codegen::Codegen::new(analysis);
    let context = cg.generate()?;
    Tera::one_off(TEMPLATE.trim(), &context, false)
        .map_err(|e| format_err!("Failed to format template:\n{:?}", e))
}

const TEMPLATE: &'static str = r#####"
use fall_parse::runtime as rt;
use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeTypeInfo, Metrics, TextEdit, TreeBuilder};
pub use self::fall_tree::ERROR;

{% for node_type in node_types %}
pub const {{ node_type.0 | upper }}: rt::NodeType = rt::NodeType({{ 100 + loop.index0 }});
{% endfor %}


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            {% for rule in lex_rules %}
            ::fall_parse::LexRule::new({{ rule.ty | upper }}, {{ rule.re }}, {% if rule.f is string %} Some({{ rule.f }}) {% else %} None {% endif %}),
            {% endfor %}
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"{{ parser_json }}"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                {% for node_type in node_types %}{{ node_type.0 | upper }}, {% endfor %}
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            {% if has_whitespace_binder %}
                whitespace_binder,
            {% endif %}
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: rt::Language = {
            use fall_parse::{ParserDefinition, parse, reparse};
            use std::any::Any;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &Any,
                    edit: &TextEdit,
                    new_text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
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
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
        };
    }

    &*LANG
}

{% if verbatim is string %}
{{ verbatim }}
{% endif %}

{% if ast_nodes is defined %}
{% for node in ast_nodes %}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct {{ node.struct_name }}<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for {{ node.struct_name }}<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == {{ node.node_type_name }} {
            Some({{ node.struct_name }} { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
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

impl<'f> rt::AstNode<'f> for {{ class.enum_name }}<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        {% for v in class.variants %}
        if let Some(n) = {{ v.1 }}::wrap(node) {
            return Some({{ class.enum_name }}::{{ v.1 }}(n))
        }
        {% endfor %}
        None
    }

    fn node(self) -> rt::Node<'f> {
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
        rt::AstNode::node(*self).range().fmt(f)?;
        Ok(())
    }
}
{% endfor %}

{% for trait_ in ast_traits %}
pub trait {{ trait_.trait_name }}<'f>: rt::AstNode<'f> {
    {% for method in trait_.methods %}
    fn {{ method.name }}(&self) -> {{ method.ret_type }};
    {% endfor %}
}
{% endfor %}

{% endif %}
"#####;
