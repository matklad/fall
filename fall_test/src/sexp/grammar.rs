use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const ATOM      : NodeType = NodeType(100);
pub const LPAREN    : NodeType = NodeType(101);
pub const RPAREN    : NodeType = NodeType(102);
pub const FILE      : NodeType = NodeType(103);
pub const LIST      : NodeType = NodeType(104);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        ATOM.register(NodeTypeInfo { name: "ATOM" });
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        LIST.register(NodeTypeInfo { name: "LIST" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: LPAREN, re: r"\(", f: None },
    Rule { ty: RPAREN, re: r"\)", f: None },
    Rule { ty: ATOM, re: r"\w+", f: None },
];

pub const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(1)], commit: None })], commit: None }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[syn::Part::Token(ATOM)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(2)], commit: None }] },
    syn::Rule { ty: Some(LIST), alts: &[syn::Alt { parts: &[syn::Part::Token(LPAREN), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(1)], commit: None }), syn::Part::Token(RPAREN)], commit: None }] },
];

pub fn parse(text: String) -> ::fall_tree::File {
    register_node_types();
    ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
}
