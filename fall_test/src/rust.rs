use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const LPAREN: NodeType = NodeType(100);
pub const RPAREN: NodeType = NodeType(101);
pub const LBRACE: NodeType = NodeType(102);
pub const RBRACE: NodeType = NodeType(103);
pub const PUB: NodeType = NodeType(104);
pub const STRUCT: NodeType = NodeType(105);
pub const FN: NodeType = NodeType(106);
pub const IDENT: NodeType = NodeType(107);
pub const FILE: NodeType = NodeType(108);
pub const STRUCT_DEF: NodeType = NodeType(109);
pub const FN_DEF: NodeType = NodeType(110);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Expr, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            LPAREN, RPAREN, LBRACE, RBRACE, PUB, STRUCT, FN, IDENT, FILE, STRUCT_DEF, FN_DEF,
        ];

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
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    PUB => NodeTypeInfo { name: "PUB" },
                    STRUCT => NodeTypeInfo { name: "STRUCT" },
                    FN => NodeTypeInfo { name: "FN" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF" },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF" },
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
                LexRule::new(PUB, "pub", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(IDENT, "\\w+", None),
            ],
            parser: vec![
                SynRule {
                    ty: Some(FILE),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Rep(Box::new(Expr::Or(vec![Expr::And(vec![Expr::Rule(1)], None)])), Some(vec![6, 8, 7, ]), None)], None)]),
                },
                SynRule {
                    ty: None,
                    body: Expr::Or(vec![Expr::And(vec![Expr::Rule(2)], None), Expr::And(vec![Expr::Rule(3)], None)]),
                },
                SynRule {
                    ty: Some(FN_DEF),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Opt(Box::new(Expr::Or(vec![Expr::And(vec![Expr::Token(6)], None)]))), Expr::Token(8), Expr::Token(9), Expr::Token(2), Expr::Token(3), Expr::Token(4), Expr::Token(5)], Some(2))]),
                },
                SynRule {
                    ty: Some(STRUCT_DEF),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Opt(Box::new(Expr::Or(vec![Expr::And(vec![Expr::Token(6)], None)]))), Expr::Token(7), Expr::Token(9), Expr::Token(4), Expr::Token(5)], Some(2))]),
                },
            ]
        })
    };
}


