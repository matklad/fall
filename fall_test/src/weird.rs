use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const RAW_STRING: NodeType = NodeType(100);
pub const FOO: NodeType = NodeType(101);
pub const BAR: NodeType = NodeType(102);
pub const T1: NodeType = NodeType(103);
pub const T2: NodeType = NodeType(104);
pub const T3: NodeType = NodeType(105);
pub const T4: NodeType = NodeType(106);
pub const LBRACE: NodeType = NodeType(107);
pub const RBRACE: NodeType = NodeType(108);
pub const ATOM: NodeType = NodeType(109);
pub const FILE: NodeType = NodeType(110);
pub const PRIVATE_PARTIAL: NodeType = NodeType(111);
pub const EMPTY: NodeType = NodeType(112);
pub const BLOCK: NodeType = NodeType(113);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            RAW_STRING, FOO, BAR, T1, T2, T3, T4, LBRACE, RBRACE, ATOM, FILE, PRIVATE_PARTIAL, EMPTY, BLOCK,
        ];
        let parser_json = r##"[{"ty":12,"body":{"Or":[{"And":[[{"Token":5},{"Token":2}],null]},{"And":[[{"Token":6},{"Rule":4},{"Token":11},{"Rule":4}],null]},{"And":[[{"Token":7},{"Rule":1}],null]},{"And":[[{"Token":8},{"Rule":6}],null]}]}},{"ty":13,"body":{"Or":[{"And":[[{"Rule":2}],null]},{"And":[[{"Rule":3}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":3},{"Token":4}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":3},{"Token":3}],null]}]}},{"ty":14,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Rule":5}],null]}]}}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[],null]}]}},{"ty":15,"body":{"Or":[{"And":[[{"Token":9},{"Rule":7},{"Token":10}],1]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rep":{"Rule":8}}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":9},{"Rule":7},{"Token":10}],1]},{"And":[[{"Not":[10]}],null]}]}}]"##;
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
                    RAW_STRING => NodeTypeInfo { name: "RAW_STRING" },
                    FOO => NodeTypeInfo { name: "FOO" },
                    BAR => NodeTypeInfo { name: "BAR" },
                    T1 => NodeTypeInfo { name: "T1" },
                    T2 => NodeTypeInfo { name: "T2" },
                    T3 => NodeTypeInfo { name: "T3" },
                    T4 => NodeTypeInfo { name: "T4" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    ATOM => NodeTypeInfo { name: "ATOM" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    PRIVATE_PARTIAL => NodeTypeInfo { name: "PRIVATE_PARTIAL" },
                    EMPTY => NodeTypeInfo { name: "EMPTY" },
                    BLOCK => NodeTypeInfo { name: "BLOCK" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(RAW_STRING, "r#+\"", Some(parse_raw_string)),
                LexRule::new(FOO, "foo", None),
                LexRule::new(BAR, "bar", None),
                LexRule::new(T1, "_1", None),
                LexRule::new(T2, "_2", None),
                LexRule::new(T3, "_3", None),
                LexRule::new(T4, "_4", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(ATOM, "\\w+", None),
            ],
            parser: parser,
        })
    };
}
fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

