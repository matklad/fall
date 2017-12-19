use fall_parse::runtime as rt;
use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeTypeInfo, Metrics, TextEdit, TreeBuilder};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const LPAREN: rt::NodeType = rt::NodeType(101);
pub const RPAREN: rt::NodeType = rt::NodeType(102);
pub const ATOM: rt::NodeType = rt::NodeType(103);
pub const FILE: rt::NodeType = rt::NodeType(104);
pub const LIST: rt::NodeType = rt::NodeType(105);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(LPAREN, "\\(", None),
            ::fall_parse::LexRule::new(RPAREN, "\\)", None),
            ::fall_parse::LexRule::new(ATOM, "\\w+", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":5,"body":5,"replaceable":false}},{"Or":[7,8]},{"Pub":{"ty":6,"body":13,"replaceable":false}},{"Rep":1},{"And":[[3],null]},{"Or":[4]},{"Token":4},{"And":[[6],null]},{"And":[[2],null]},{"Token":2},{"Rep":1},{"Token":3},{"And":[[9,10,11],null]},{"Or":[12]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LPAREN, RPAREN, ATOM, FILE, LIST,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            
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
                        WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LPAREN => NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                        RPAREN => NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                        ATOM => NodeTypeInfo { name: "ATOM", whitespace_like: false },
                        FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                        LIST => NodeTypeInfo { name: "LIST", whitespace_like: false },
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



