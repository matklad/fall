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
pub use self::rt::ERROR;

{% for node_type in node_types %}
pub const {{ node_type.0 | upper }}: rt::NodeType = rt::NodeType({{ 100 + loop.index0 }});
{% endfor %}


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            {% for rule in lex_rules %}
            rt::LexRule::new({{ rule.ty | upper }}, {{ rule.re }}, {% if rule.f is string %} Some({{ rule.f }}) {% else %} None {% endif %}),
            {% endfor %}
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"{{ parser_json }}"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                {% for node_type in node_types %}{{ node_type.0 | upper }}, {% endfor %}
            ],
            syntactical_rules: rt::parser_from_str(parser_json),
            {% if has_whitespace_binder %}
                whitespace_binder,
            {% endif %}
            .. Default::default()
        }
    }
    use self::rt::*;
    lazy_static! {
        static ref LANG: rt::Language = {
            struct Impl { parser_definition: rt::ParserDefinition, lexer: rt::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &::std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        {% for node_type in node_types %}
                        {{ node_type.0 | upper }} => rt::NodeTypeInfo { name: "{{ node_type.0 | upper }}", whitespace_like: {{ node_type.1 }} },
                        {% endfor %}
                        _ => panic!("Unknown rt::NodeType: {:?}", ty)
                    }
                }
            }

            rt::Language::new(Impl {
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
#[allow(unused)]
use self::rt::AstNode;

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
    fn {{ method.name }}(&self) -> {{ method.ret_type }} {
        {{ method.body }}
    }
    {% endfor %}
}
{% for node in trait_.impl_for %}
impl<'f> {{ trait_.trait_name }}<'f> for {{ node }}<'f> {}
{% endfor %}
{% endfor %}

{% endif %}
"#####;
