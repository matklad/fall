use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LPAREN: NodeType = NodeType(101);
pub const RPAREN: NodeType = NodeType(102);
pub const ATOM: NodeType = NodeType(103);
pub const FILE: NodeType = NodeType(104);
pub const LIST: NodeType = NodeType(105);


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"[{"body":{"Pub":{"ty_idx":5,"body":{"Or":[{"And":[[{"Rep":{"Rule":1}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":4}],null]},{"And":[[{"Rule":2}],null]}]}},{"body":{"Pub":{"ty_idx":6,"body":{"Or":[{"And":[[{"Token":2},{"Rep":{"Rule":1}},{"Token":3}],null]}]},"replaceable":false}}}]"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            WHITESPACE, LPAREN, RPAREN, ATOM, FILE, LIST,
        ],
        lexical_rules: vec![
            LexRule::new(WHITESPACE, "\\s+", None),
            LexRule::new(LPAREN, "\\(", None),
            LexRule::new(RPAREN, "\\)", None),
            LexRule::new(ATOM, "\\w+", None),
        ],
        syntactical_rules: serde_json::from_str(parser_json).unwrap(),
        
        .. Default::default()
    }
}

pub fn language() -> &'static Language {
    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition };
            impl LanguageImpl for Impl {
                fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't> {
                    self.parser_definition.tokenize(text)
                }

                fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> INode {
                    self.parser_definition.parse(text, tokens, &LANG, metrics)
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

            Language::new(Impl { parser_definition: create_parser_definition() })
        };
    }

    &*LANG
}



