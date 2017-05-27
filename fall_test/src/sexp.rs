use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const LPAREN: NodeType = NodeType(100);
pub const RPAREN: NodeType = NodeType(101);
pub const ATOM: NodeType = NodeType(102);
pub const FILE: NodeType = NodeType(103);
pub const LIST: NodeType = NodeType(104);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            LPAREN, RPAREN, ATOM, FILE, LIST,
        ];
        let parser_json = r##"[{"ty":5,"body":{"Or":[{"And":[[{"Rep":[{"Or":[{"And":[[{"Rule":1}],null]}]},null,null]}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":4}],null]},{"And":[[{"Rule":2}],null]}]}},{"ty":6,"body":{"Or":[{"And":[[{"Token":2},{"Rep":[{"Or":[{"And":[[{"Rule":1}],null]}]},null,null]},{"Token":3}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse2(lang, text, &self.tokenizer, &|&x, y| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse2(x, y)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    ATOM => NodeTypeInfo { name: "ATOM" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    LIST => NodeTypeInfo { name: "LIST" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(ATOM, "\\w+", None),
            ],
            parser: parser,
        })
    };
}


