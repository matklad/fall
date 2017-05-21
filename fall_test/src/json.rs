use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const NULL: NodeType = NodeType(100);
pub const BOOL: NodeType = NodeType(101);
pub const NUMBER: NodeType = NodeType(102);
pub const STRING: NodeType = NodeType(103);
pub const LBRACE: NodeType = NodeType(104);
pub const RBRACE: NodeType = NodeType(105);
pub const LBRACK: NodeType = NodeType(106);
pub const RBRACK: NodeType = NodeType(107);
pub const COMMA: NodeType = NodeType(108);
pub const COLON: NodeType = NodeType(109);
pub const OBJECT: NodeType = NodeType(110);
pub const ARRAY: NodeType = NodeType(111);
pub const PRIMITIVE: NodeType = NodeType(112);
pub const FIELD: NodeType = NodeType(113);
pub const FILE: NodeType = NodeType(114);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            NULL, BOOL, NUMBER, STRING, LBRACE, RBRACE, LBRACK, RBRACK, COMMA, COLON, OBJECT, ARRAY, PRIMITIVE, FIELD, FILE,
        ];
        let parser_json = r##"[{"ty":16,"body":{"Or":[{"And":[[{"Rule":1}],null]},{"And":[[{"Rule":4}],null]}]}},{"ty":12,"body":{"Or":[{"And":[[{"Token":6},{"Rule":2},{"Token":7}],1]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rep":[{"Or":[{"And":[[{"Rule":3},{"Token":10}],null]}]},null,null]}],null]}]}},{"ty":15,"body":{"Or":[{"And":[[{"Token":5},{"Token":11},{"Rule":5}],1]}]}},{"ty":13,"body":{"Or":[{"And":[[{"Token":8},{"Rep":[{"Or":[{"And":[[{"Rule":5},{"Token":10}],null]}]},null,null]},{"Token":9}],1]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rule":6}],null]},{"And":[[{"Rule":1}],null]},{"And":[[{"Rule":4}],null]}]}},{"ty":14,"body":{"Or":[{"And":[[{"Token":2}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":3}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(b)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    NULL => NodeTypeInfo { name: "NULL" },
                    BOOL => NodeTypeInfo { name: "BOOL" },
                    NUMBER => NodeTypeInfo { name: "NUMBER" },
                    STRING => NodeTypeInfo { name: "STRING" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    LBRACK => NodeTypeInfo { name: "LBRACK" },
                    RBRACK => NodeTypeInfo { name: "RBRACK" },
                    COMMA => NodeTypeInfo { name: "COMMA" },
                    COLON => NodeTypeInfo { name: "COLON" },
                    OBJECT => NodeTypeInfo { name: "OBJECT" },
                    ARRAY => NodeTypeInfo { name: "ARRAY" },
                    PRIMITIVE => NodeTypeInfo { name: "PRIMITIVE" },
                    FIELD => NodeTypeInfo { name: "FIELD" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(LBRACK, "\\[", None),
                LexRule::new(RBRACK, "\\]", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(NULL, "null", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(BOOL, "true|false", None),
                LexRule::new(STRING, "\"[^\"]*\"", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


