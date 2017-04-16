use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const T1: NodeType = NodeType(100);
pub const T2: NodeType = NodeType(101);
pub const T3: NodeType = NodeType(102);
pub const FOO: NodeType = NodeType(103);
pub const BAR: NodeType = NodeType(104);
pub const ATOM: NodeType = NodeType(105);
pub const RAW_STRING: NodeType = NodeType(106);
pub const FILE: NodeType = NodeType(107);
pub const PRIVATE_PARTIAL: NodeType = NodeType(108);
pub const EMPTY: NodeType = NodeType(109);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Alt, Part, Parser};

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                alts: &[Alt { parts: &[Part::Token(T1), Part::Token(RAW_STRING)], commit: None }, Alt { parts: &[Part::Token(T2), Part::Rule(4), Part::Token(ATOM), Part::Rule(4)], commit: None }, Alt { parts: &[Part::Token(T3), Part::Rule(1)], commit: None }],
            },
            SynRule {
                ty: Some(PRIVATE_PARTIAL),
                alts: &[Alt { parts: &[Part::Rule(2)], commit: None }, Alt { parts: &[Part::Rule(3)], commit: None }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Token(FOO), Part::Token(BAR)], commit: None }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Token(FOO), Part::Token(FOO)], commit: None }],
            },
            SynRule {
                ty: Some(EMPTY),
                alts: &[Alt { parts: &[Part::Opt(Alt { parts: &[Part::Rule(5)], commit: None })], commit: None }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[], commit: None }],
            },
        ];

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| Parser::new(PARSER).parse(b))
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    T1 => NodeTypeInfo { name: "T1" },
                    T2 => NodeTypeInfo { name: "T2" },
                    T3 => NodeTypeInfo { name: "T3" },
                    FOO => NodeTypeInfo { name: "FOO" },
                    BAR => NodeTypeInfo { name: "BAR" },
                    ATOM => NodeTypeInfo { name: "ATOM" },
                    RAW_STRING => NodeTypeInfo { name: "RAW_STRING" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    PRIVATE_PARTIAL => NodeTypeInfo { name: "PRIVATE_PARTIAL" },
                    EMPTY => NodeTypeInfo { name: "EMPTY" },
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
                LexRule::new(ATOM, "\\w+", None),
            ]
        })
    };
}
fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

