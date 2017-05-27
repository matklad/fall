use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const LPAREN: NodeType = NodeType(100);
pub const RPAREN: NodeType = NodeType(101);
pub const LBRACE: NodeType = NodeType(102);
pub const RBRACE: NodeType = NodeType(103);
pub const KW_PUB: NodeType = NodeType(104);
pub const STRUCT: NodeType = NodeType(105);
pub const FN: NodeType = NodeType(106);
pub const IDENT: NodeType = NodeType(107);
pub const FILE: NodeType = NodeType(108);
pub const FN_DEF: NodeType = NodeType(109);
pub const STRUCT_DEF: NodeType = NodeType(110);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            LPAREN, RPAREN, LBRACE, RBRACE, KW_PUB, STRUCT, FN, IDENT, FILE, FN_DEF, STRUCT_DEF,
        ];
        let parser_json = r##"[{"ty":10,"body":{"Or":[{"And":[[{"Rep":[{"Or":[{"And":[[{"SkipUntil":[6,8,7]},{"Rule":1}],null]}]},[6,8,7],null]}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rule":2}],null]},{"And":[[{"Rule":3}],null]}]}},{"ty":11,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":6}],null]}]}},{"Token":8},{"Token":9},{"Token":2},{"Token":3},{"Token":4},{"Token":5}],2]}]}},{"ty":12,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":6}],null]}]}},{"Token":7},{"Token":9},{"Token":4},{"Token":5}],2]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, &self.tokenizer, &|&x, y| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(x, y)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    KW_PUB => NodeTypeInfo { name: "KW_PUB" },
                    STRUCT => NodeTypeInfo { name: "STRUCT" },
                    FN => NodeTypeInfo { name: "FN" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF" },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(KW_PUB, "pub", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(IDENT, "\\w+", None),
            ],
            parser: parser,
        })
    };
}


