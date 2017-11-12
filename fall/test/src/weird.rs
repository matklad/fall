use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const RAW_STRING: NodeType = NodeType(101);
pub const FOO: NodeType = NodeType(102);
pub const BAR: NodeType = NodeType(103);
pub const T1: NodeType = NodeType(104);
pub const T2: NodeType = NodeType(105);
pub const T3: NodeType = NodeType(106);
pub const T4: NodeType = NodeType(107);
pub const LBRACE: NodeType = NodeType(108);
pub const RBRACE: NodeType = NodeType(109);
pub const ATOM: NodeType = NodeType(110);
pub const FILE: NodeType = NodeType(111);
pub const PRIVATE_PARTIAL: NodeType = NodeType(112);
pub const EMPTY: NodeType = NodeType(113);
pub const BLOCK: NodeType = NodeType(114);


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"[{"body":{"Pub":{"ty_idx":12,"body":{"Or":[{"And":[[{"Token":5},{"Token":2}],null]},{"And":[[{"Token":6},{"Rule":4},{"Token":11},{"Rule":4}],null]},{"And":[[{"Token":7},{"Rule":1}],null]},{"And":[[{"Token":8},{"Rule":6}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":13,"body":{"Or":[{"And":[[{"Rule":2}],null]},{"And":[[{"Rule":3}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":3},{"Token":4}],null]}]}},{"body":{"Or":[{"And":[[{"Token":3},{"Token":3}],null]}]}},{"body":{"Pub":{"ty_idx":14,"body":{"Or":[{"And":[[{"Opt":{"Rule":5}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[],null]}]}},{"body":{"Pub":{"ty_idx":15,"body":{"Or":[{"And":[[{"Token":9},{"Rule":7},{"Token":10}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":8}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":9},{"Rule":7},{"Token":10}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Token":10}},"Any"],null]}]}],null]}]}}]"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            WHITESPACE, RAW_STRING, FOO, BAR, T1, T2, T3, T4, LBRACE, RBRACE, ATOM, FILE, PRIVATE_PARTIAL, EMPTY, BLOCK,
        ],
        lexical_rules: vec![
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
                        RAW_STRING => NodeTypeInfo { name: "RAW_STRING", whitespace_like: false },
                        FOO => NodeTypeInfo { name: "FOO", whitespace_like: false },
                        BAR => NodeTypeInfo { name: "BAR", whitespace_like: false },
                        T1 => NodeTypeInfo { name: "T1", whitespace_like: false },
                        T2 => NodeTypeInfo { name: "T2", whitespace_like: false },
                        T3 => NodeTypeInfo { name: "T3", whitespace_like: false },
                        T4 => NodeTypeInfo { name: "T4", whitespace_like: false },
                        LBRACE => NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                        RBRACE => NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                        ATOM => NodeTypeInfo { name: "ATOM", whitespace_like: false },
                        FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                        PRIVATE_PARTIAL => NodeTypeInfo { name: "PRIVATE_PARTIAL", whitespace_like: false },
                        EMPTY => NodeTypeInfo { name: "EMPTY", whitespace_like: false },
                        BLOCK => NodeTypeInfo { name: "BLOCK", whitespace_like: false },
                        _ => panic!("Unknown NodeType: {:?}", ty)
                    }
                }
            }

            Language::new(Impl { parser_definition: create_parser_definition() })
        };
    }

    &*LANG
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

