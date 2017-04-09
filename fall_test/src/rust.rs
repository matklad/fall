use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
use fall_parse::Rule;
use fall_parse::syn;
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

fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(|| {
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        PUB.register(NodeTypeInfo { name: "PUB" });
        STRUCT.register(NodeTypeInfo { name: "STRUCT" });
        FN.register(NodeTypeInfo { name: "FN" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRUCT_DEF.register(NodeTypeInfo { name: "STRUCT_DEF" });
        FN_DEF.register(NodeTypeInfo { name: "FN_DEF" });
    });
}

const PARSER: &'static [syn::Rule] = &[
    syn::Rule {
        ty: Some(FILE),
        alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(1)], commit: None })], commit: None }],
    },
    syn::Rule {
        ty: None,
        alts: &[syn::Alt { parts: &[syn::Part::Rule(2)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(3)], commit: None }],
    },
    syn::Rule {
        ty: Some(FN_DEF),
        alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Token(PUB)], commit: None }), syn::Part::Token(FN), syn::Part::Token(IDENT), syn::Part::Token(LPAREN), syn::Part::Token(RPAREN), syn::Part::Token(LBRACE), syn::Part::Token(RBRACE)], commit: None }],
    },
    syn::Rule {
        ty: Some(STRUCT_DEF),
        alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Token(PUB)], commit: None }), syn::Part::Token(STRUCT), syn::Part::Token(IDENT), syn::Part::Token(LBRACE), syn::Part::Token(RBRACE)], commit: None }],
    },
];

lazy_static! {
    pub static ref LANG: Language = {
        register_node_types();
        struct Impl { tokenizer: Vec<Rule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(text, FILE, &self.tokenizer, &|b| syn::Parser::new(PARSER).parse(b))
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                Rule { ty: LPAREN, re: "\\(", f: None },
                Rule { ty: RPAREN, re: "\\)", f: None },
                Rule { ty: LBRACE, re: "\\{", f: None },
                Rule { ty: RBRACE, re: "\\}", f: None },
                Rule { ty: PUB, re: "pub", f: None },
                Rule { ty: STRUCT, re: "struct", f: None },
                Rule { ty: FN, re: "fn", f: None },
                Rule { ty: WHITESPACE, re: "\\s+", f: None },
                Rule { ty: IDENT, re: "\\w+", f: None },
            ]
        })
    };
}
