use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const ATOM: NodeType = NodeType(100);
pub const LPAREN: NodeType = NodeType(101);
pub const RPAREN: NodeType = NodeType(102);
pub const FILE: NodeType = NodeType(103);
pub const LIST: NodeType = NodeType(104);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Expr, Parser};

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                body: Expr::Or(&[Expr::And(&[Expr::Rep(&Expr::Or(&[Expr::And(&[Expr::Rule(1)], None)]), None, None)], None)]),
            },
            SynRule {
                ty: None,
                body: Expr::Or(&[Expr::And(&[Expr::Token(ATOM)], None), Expr::And(&[Expr::Rule(2)], None)]),
            },
            SynRule {
                ty: Some(LIST),
                body: Expr::Or(&[Expr::And(&[Expr::Token(LPAREN), Expr::Rep(&Expr::Or(&[Expr::And(&[Expr::Rule(1)], None)]), None, None), Expr::Token(RPAREN)], None)]),
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
                    ATOM => NodeTypeInfo { name: "ATOM" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    LIST => NodeTypeInfo { name: "LIST" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(ATOM, "\\w+", None),
            ]
        })
    };
}


