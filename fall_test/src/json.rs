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
        use fall_parse::{LexRule, SynRule, Expr, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            NULL, BOOL, NUMBER, STRING, LBRACE, RBRACE, LBRACK, RBRACK, COMMA, COLON, OBJECT, ARRAY, PRIMITIVE, FIELD, FILE,
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
            parser: vec![
                SynRule {
                    ty: Some(FILE),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Rule(1)], None), Expr::And(vec![Expr::Rule(4)], None)]),
                },
                SynRule {
                    ty: Some(OBJECT),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Token(6), Expr::Rule(2), Expr::Token(7)], Some(1))]),
                },
                SynRule {
                    ty: None,
                    body: Expr::Or(vec![Expr::And(vec![Expr::Rep(Box::new(Expr::Or(vec![Expr::And(vec![Expr::Rule(3), Expr::Token(10)], None)])), None, None)], None)]),
                },
                SynRule {
                    ty: Some(FIELD),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Token(5), Expr::Token(11), Expr::Rule(5)], Some(1))]),
                },
                SynRule {
                    ty: Some(ARRAY),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Token(8), Expr::Rep(Box::new(Expr::Or(vec![Expr::And(vec![Expr::Rule(5), Expr::Token(10)], None)])), None, None), Expr::Token(9)], Some(1))]),
                },
                SynRule {
                    ty: None,
                    body: Expr::Or(vec![Expr::And(vec![Expr::Rule(6)], None), Expr::And(vec![Expr::Rule(1)], None), Expr::And(vec![Expr::Rule(4)], None)]),
                },
                SynRule {
                    ty: Some(PRIMITIVE),
                    body: Expr::Or(vec![Expr::And(vec![Expr::Token(2)], None), Expr::And(vec![Expr::Token(4)], None), Expr::And(vec![Expr::Token(5)], None), Expr::And(vec![Expr::Token(3)], None)]),
                },
            ]
        })
    };
}


