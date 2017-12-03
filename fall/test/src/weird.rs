use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode, Event};
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


pub fn language() -> &'static Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(RAW_STRING, "r#+\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(FOO, "foo", None),
            ::fall_parse::LexRule::new(BAR, "bar", None),
            ::fall_parse::LexRule::new(T1, "_1", None),
            ::fall_parse::LexRule::new(T2, "_2", None),
            ::fall_parse::LexRule::new(T3, "_3", None),
            ::fall_parse::LexRule::new(T4, "_4", None),
            ::fall_parse::LexRule::new(LBRACE, "\\{", None),
            ::fall_parse::LexRule::new(RBRACE, "\\}", None),
            ::fall_parse::LexRule::new(ATOM, "\\w+", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":12,"body":19,"replaceable":false}},{"Pub":{"ty":13,"body":22,"replaceable":false}},{"Or":[25]},{"Or":[28]},{"Pub":{"ty":14,"body":31,"replaceable":false}},{"Or":[32]},{"Pub":{"ty":15,"body":36,"replaceable":false}},{"Or":[38]},{"Or":[41,47]},{"Token":5},{"Token":2},{"And":[[9,10],null]},{"Token":6},{"Token":11},{"And":[[12,4,13,4],null]},{"Token":7},{"And":[[15,1],null]},{"Token":8},{"And":[[17,6],null]},{"Or":[11,14,16,18]},{"And":[[2],null]},{"And":[[3],null]},{"Or":[20,21]},{"Token":3},{"Token":4},{"And":[[23,24],null]},{"Token":3},{"Token":3},{"And":[[26,27],null]},{"Opt":5},{"And":[[29],null]},{"Or":[30]},{"And":[[],null]},{"Token":9},{"Token":10},{"And":[[33,7,34],1]},{"Or":[35]},{"Rep":8},{"And":[[37],null]},{"Token":9},{"Token":10},{"And":[[39,7,40],1]},{"Token":10},{"Not":42},"Any",{"And":[[43,44],null]},{"Or":[45]},{"And":[[46],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, RAW_STRING, FOO, BAR, T1, T2, T3, T4, LBRACE, RBRACE, ATOM, FILE, PRIVATE_PARTIAL, EMPTY, BLOCK,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn lexer(&self) -> &self::fall_tree::Lexer {
                    &self.lexer
                }

                fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> (Vec<Event>, INode) {
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

            Language::new(Impl {
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
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

