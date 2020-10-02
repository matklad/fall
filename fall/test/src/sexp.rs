use fall_parse::runtime as rt;
pub use self::rt::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const LPAREN: rt::NodeType = rt::NodeType(101);
pub const RPAREN: rt::NodeType = rt::NodeType(102);
pub const ATOM: rt::NodeType = rt::NodeType(103);
pub const FILE: rt::NodeType = rt::NodeType(104);
pub const LIST: rt::NodeType = rt::NodeType(105);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(LPAREN, "\\(", None),
            rt::LexRule::new(RPAREN, "\\)", None),
            rt::LexRule::new(ATOM, "\\w+", None),

        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":5,"body":5,"replaceable":false}},{"Or":[7,8]},{"Pub":{"ty":6,"body":13,"replaceable":false}},{"Rep":1},{"And":[[3],null]},{"Or":[4]},{"Token":4},{"And":[[6],null]},{"And":[[2],null]},{"Token":2},{"Rep":1},{"Token":3},{"And":[[9,10,11],null]},{"Or":[12]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LPAREN, RPAREN, ATOM, FILE, LIST,
            ],
            syntactical_rules: rt::parser_from_str(parser_json),
            
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
                ) -> Option<Box<dyn std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &dyn std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<dyn std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => rt::NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LPAREN => rt::NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                        RPAREN => rt::NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                        ATOM => rt::NodeTypeInfo { name: "ATOM", whitespace_like: false },
                        FILE => rt::NodeTypeInfo { name: "FILE", whitespace_like: false },
                        LIST => rt::NodeTypeInfo { name: "LIST", whitespace_like: false },
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
