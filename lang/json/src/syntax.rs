use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LBRACE: NodeType = NodeType(101);
pub const RBRACE: NodeType = NodeType(102);
pub const LBRACK: NodeType = NodeType(103);
pub const RBRACK: NodeType = NodeType(104);
pub const COLON: NodeType = NodeType(105);
pub const COMMA: NodeType = NodeType(106);
pub const NULL: NodeType = NodeType(107);
pub const BOOL: NodeType = NodeType(108);
pub const STRING: NodeType = NodeType(109);
pub const NUMBER: NodeType = NodeType(110);
pub const FILE: NodeType = NodeType(111);
pub const OBJECT: NodeType = NodeType(112);
pub const FIELD: NodeType = NodeType(113);
pub const ARRAY: NodeType = NodeType(114);
pub const PRIMITIVE: NodeType = NodeType(115);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, LBRACE, RBRACE, LBRACK, RBRACK, COLON, COMMA, NULL, BOOL, STRING, NUMBER, FILE, OBJECT, FIELD, ARRAY, PRIMITIVE,
        ];
        let parser_json = r##"[{"body":{"Pub":[12,{"Or":[{"And":[[{"Rule":1}],null]},{"And":[[{"Rule":4}],null]}]}]}},{"body":{"Pub":[13,{"Or":[{"And":[[{"Token":2},{"Layer":[{"Rule":8},{"Rule":2}]},{"Token":3}],1]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Token":10},{"Or":[{"And":[[{"Rule":3},{"Or":[{"And":[[{"Token":7},{"NotAhead":"Eof"}],null]},{"And":[["Eof"],null]}]}],1]}]}]}}],null]}]}},{"body":{"Pub":[14,{"Or":[{"And":[[{"Token":10},{"Token":6},{"Rule":6}],1]}]}]}},{"body":{"Pub":[15,{"Or":[{"And":[[{"Token":4},{"Layer":[{"Rule":10},{"Rule":5}]},{"Token":5}],1]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"Token":2}],null]},{"And":[[{"Token":4}],null]}]},{"Or":[{"And":[[{"Rule":6},{"Or":[{"And":[[{"Token":7},{"NotAhead":"Eof"}],null]},{"And":[["Eof"],null]}]}],1]}]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":1}],null]},{"And":[[{"Rule":4}],null]}]}},{"body":{"Pub":[16,{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":9}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":9}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":2},{"Rule":8},{"Token":3}],1]},{"And":[[{"Not":{"Token":3}}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":11}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":4},{"Rule":10},{"Token":5}],1]},{"And":[[{"Not":{"Token":5}}],null]}]}}]"##;
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
                    LBRACE => NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                    RBRACE => NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                    LBRACK => NodeTypeInfo { name: "LBRACK", whitespace_like: false },
                    RBRACK => NodeTypeInfo { name: "RBRACK", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    NULL => NodeTypeInfo { name: "NULL", whitespace_like: false },
                    BOOL => NodeTypeInfo { name: "BOOL", whitespace_like: false },
                    STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                    NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                    FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                    OBJECT => NodeTypeInfo { name: "OBJECT", whitespace_like: false },
                    FIELD => NodeTypeInfo { name: "FIELD", whitespace_like: false },
                    ARRAY => NodeTypeInfo { name: "ARRAY", whitespace_like: false },
                    PRIMITIVE => NodeTypeInfo { name: "PRIMITIVE", whitespace_like: false },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(LBRACK, "\\[", None),
                LexRule::new(RBRACK, "\\]", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(NULL, "null", None),
                LexRule::new(BOOL, "true|false", None),
                LexRule::new(STRING, "\"[^\"]*\"", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


