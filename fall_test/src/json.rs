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
        use fall_parse::{LexRule, SynRule, Alt, Part, Parser};

        NULL.register(NodeTypeInfo { name: "NULL" });
        BOOL.register(NodeTypeInfo { name: "BOOL" });
        NUMBER.register(NodeTypeInfo { name: "NUMBER" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LBRACK.register(NodeTypeInfo { name: "LBRACK" });
        RBRACK.register(NodeTypeInfo { name: "RBRACK" });
        COMMA.register(NodeTypeInfo { name: "COMMA" });
        COLON.register(NodeTypeInfo { name: "COLON" });
        OBJECT.register(NodeTypeInfo { name: "OBJECT" });
        ARRAY.register(NodeTypeInfo { name: "ARRAY" });
        PRIMITIVE.register(NodeTypeInfo { name: "PRIMITIVE" });
        FIELD.register(NodeTypeInfo { name: "FIELD" });
        FILE.register(NodeTypeInfo { name: "FILE" });

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                alts: &[Alt { parts: &[Part::Rule(1)], commit: None }, Alt { parts: &[Part::Rule(4)], commit: None }],
            },
            SynRule {
                ty: Some(OBJECT),
                alts: &[Alt { parts: &[Part::Token(LBRACE), Part::Rule(2), Part::Token(RBRACE)], commit: Some(1) }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Rep(Alt { parts: &[Part::Rule(3), Part::Token(COMMA)], commit: None })], commit: None }],
            },
            SynRule {
                ty: Some(FIELD),
                alts: &[Alt { parts: &[Part::Token(STRING), Part::Token(COLON), Part::Rule(5)], commit: Some(1) }],
            },
            SynRule {
                ty: Some(ARRAY),
                alts: &[Alt { parts: &[Part::Token(LBRACK), Part::Rep(Alt { parts: &[Part::Rule(5), Part::Token(COMMA)], commit: None }), Part::Token(RBRACK)], commit: Some(1) }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Rule(6)], commit: None }, Alt { parts: &[Part::Rule(1)], commit: None }, Alt { parts: &[Part::Rule(4)], commit: None }],
            },
            SynRule {
                ty: Some(PRIMITIVE),
                alts: &[Alt { parts: &[Part::Token(NULL)], commit: None }, Alt { parts: &[Part::Token(NUMBER)], commit: None }, Alt { parts: &[Part::Token(STRING)], commit: None }, Alt { parts: &[Part::Token(BOOL)], commit: None }],
            },
        ];

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| Parser::new(PARSER).parse(b))
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
            ]
        })
    };
}


