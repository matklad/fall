use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::{ERROR, WHITESPACE};

pub const NUMBER: NodeType = NodeType(100);
pub const PLUS: NodeType = NodeType(101);
pub const MINUS: NodeType = NodeType(102);
pub const STAR: NodeType = NodeType(103);
pub const SLASH: NodeType = NodeType(104);
pub const BANG: NodeType = NodeType(105);
pub const LPAREN: NodeType = NodeType(106);
pub const RPAREN: NodeType = NodeType(107);
pub const FILE: NodeType = NodeType(108);
pub const PRODUCT_EXPR: NodeType = NodeType(109);
pub const SUM_EXPR: NodeType = NodeType(110);
pub const CONSTANT_EXPR: NodeType = NodeType(111);
pub const PAREN_EXPR: NodeType = NodeType(112);
pub const FACTORIAL_EXPR: NodeType = NodeType(113);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            NUMBER, PLUS, MINUS, STAR, SLASH, BANG, LPAREN, RPAREN, FILE, PRODUCT_EXPR, SUM_EXPR, CONSTANT_EXPR, PAREN_EXPR, FACTORIAL_EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[10,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Pratt":[{"Binary":{"ty":12,"op":{"Or":[{"And":[[{"Token":3}],null]},{"And":[[{"Token":4}],null]}]},"priority":1}},{"Binary":{"ty":11,"op":{"Or":[{"And":[[{"Token":5}],null]},{"And":[[{"Token":6}],null]}]},"priority":2}},{"Postfix":{"ty":15,"op":{"Token":7}}},{"Atom":{"body":{"Pub":[13,{"Or":[{"And":[[{"Token":2}],null]}]}]}}},{"Atom":{"body":{"Pub":[14,{"Or":[{"And":[[{"Token":8},{"Rule":1},{"Token":9}],null]}]}]}}}]}},{"body":{"Pub":[11,{"Or":[{"And":[[{"Rule":1},{"Or":[{"And":[[{"Token":5}],null]},{"And":[[{"Token":6}],null]}]},{"Rule":1}],null]}]}]}},{"body":{"Pub":[12,{"Or":[{"And":[[{"Rule":1},{"Or":[{"And":[[{"Token":3}],null]},{"And":[[{"Token":4}],null]}]},{"Rule":1}],null]}]}]}},{"body":{"Pub":[13,{"Or":[{"And":[[{"Token":2}],null]}]}]}},{"body":{"Pub":[14,{"Or":[{"And":[[{"Token":8},{"Rule":1},{"Token":9}],null]}]}]}},{"body":{"Pub":[15,{"Or":[{"And":[[{"Rule":1},{"Token":7}],null]}]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                ::fall_parse::parse(text, &self.tokenizer, &|tokens, stats| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    NUMBER => NodeTypeInfo { name: "NUMBER" },
                    PLUS => NodeTypeInfo { name: "PLUS" },
                    MINUS => NodeTypeInfo { name: "MINUS" },
                    STAR => NodeTypeInfo { name: "STAR" },
                    SLASH => NodeTypeInfo { name: "SLASH" },
                    BANG => NodeTypeInfo { name: "BANG" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR" },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR" },
                    CONSTANT_EXPR => NodeTypeInfo { name: "CONSTANT_EXPR" },
                    PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR" },
                    FACTORIAL_EXPR => NodeTypeInfo { name: "FACTORIAL_EXPR" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(NUMBER, "\\d+", None),
                LexRule::new(PLUS, "\\+", None),
                LexRule::new(MINUS, "\\-", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(SLASH, "/", None),
                LexRule::new(BANG, "!", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
            ],
            parser: parser,
        })
    };
}


