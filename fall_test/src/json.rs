use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
use fall_parse::syn;
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

const PARSER: &'static [syn::Rule] = &[
    syn::Rule {
        ty: Some(FILE),
        alts: &[syn::Alt { parts: &[syn::Part::Rule(1)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(4)], commit: None }],
    },
    syn::Rule {
        ty: Some(OBJECT),
        alts: &[syn::Alt { parts: &[syn::Part::Token(LBRACE), syn::Part::Rule(2), syn::Part::Token(RBRACE)], commit: Some(1) }],
    },
    syn::Rule {
        ty: None,
        alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(3), syn::Part::Token(COMMA)], commit: None })], commit: None }],
    },
    syn::Rule {
        ty: Some(FIELD),
        alts: &[syn::Alt { parts: &[syn::Part::Token(STRING), syn::Part::Token(COLON), syn::Part::Rule(5)], commit: Some(1) }],
    },
    syn::Rule {
        ty: Some(ARRAY),
        alts: &[syn::Alt { parts: &[syn::Part::Token(LBRACK), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(5), syn::Part::Token(COMMA)], commit: None }), syn::Part::Token(RBRACK)], commit: Some(1) }],
    },
    syn::Rule {
        ty: None,
        alts: &[syn::Alt { parts: &[syn::Part::Rule(6)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(1)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(4)], commit: None }],
    },
    syn::Rule {
        ty: Some(PRIMITIVE),
        alts: &[syn::Alt { parts: &[syn::Part::Token(NULL)], commit: None }, syn::Alt { parts: &[syn::Part::Token(NUMBER)], commit: None }, syn::Alt { parts: &[syn::Part::Token(STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Token(BOOL)], commit: None }],
    },
];

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::LexRule;

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

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(text, FILE, &self.tokenizer, &|b| syn::Parser::new(PARSER).parse(b))
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


