use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LPAREN: NodeType = NodeType(101);
pub const RPAREN: NodeType = NodeType(102);
pub const ATOM: NodeType = NodeType(103);
pub const FILE: NodeType = NodeType(104);
pub const LIST: NodeType = NodeType(105);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, LPAREN, RPAREN, ATOM, FILE, LIST,
        ];
        let parser_json = r##"[{"body":{"Pub":{"ty_idx":5,"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Rule":1}],null]}]}}],null]}]},"replaceable":false,"replaces":false}}},{"body":{"Or":[{"And":[[{"Token":4}],null]},{"And":[[{"Rule":2}],null]}]}},{"body":{"Pub":{"ty_idx":6,"body":{"Or":[{"And":[[{"Token":2},{"Rep":{"Or":[{"And":[[{"Rule":1}],null]}]}},{"Token":3}],null]}]},"replaceable":false,"replaces":false}}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                parse(
                    text,
                    &LANG,
                    &self.tokenizer,
                    &|tokens, stats| Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                )
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
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(ATOM, "\\w+", None),
            ],
            parser: parser,
        })
    };
}


