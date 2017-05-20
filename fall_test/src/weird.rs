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
        use fall_parse::{LexRule, SynRule, Expr, Parser};

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                body: Expr::Or(&[Expr::And(&[Expr::Token(T1), Expr::Token(RAW_STRING)], None), Expr::And(&[Expr::Token(T2), Expr::Rule(4), Expr::Token(ATOM), Expr::Rule(4)], None), Expr::And(&[Expr::Token(T3), Expr::Rule(1)], None)]),
            },
            SynRule {
                ty: Some(PRIVATE_PARTIAL),
                body: Expr::Or(&[Expr::And(&[Expr::Rule(2)], None), Expr::And(&[Expr::Rule(3)], None)]),
            },
            SynRule {
                ty: None,
                body: Expr::Or(&[Expr::And(&[Expr::Token(FOO), Expr::Token(BAR)], None)]),
            },
            SynRule {
                ty: None,
                body: Expr::Or(&[Expr::And(&[Expr::Token(FOO), Expr::Token(FOO)], None)]),
            },
            SynRule {
                ty: Some(EMPTY),
                body: Expr::Or(&[Expr::And(&[Expr::Opt(&Expr::And(&[Expr::Rule(5)], None))], None)]),
            },
            SynRule {
                ty: None,
                body: Expr::Or(&[Expr::And(&[], None)]),
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

