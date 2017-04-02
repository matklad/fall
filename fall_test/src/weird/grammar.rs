use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const ATOM      : NodeType = NodeType(100);
pub const RAW_STRING: NodeType = NodeType(101);
pub const FILE      : NodeType = NodeType(102);
pub const EMPTY     : NodeType = NodeType(103);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        ATOM.register(NodeTypeInfo { name: "ATOM" });
        RAW_STRING.register(NodeTypeInfo { name: "RAW_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        EMPTY.register(NodeTypeInfo { name: "EMPTY" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: "\\s+", f: None },
    Rule { ty: RAW_STRING, re: "r#+\"", f: Some(super::parse_raw_string) },
    Rule { ty: ATOM, re: "\\w+", f: None },
];

pub const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Token(RAW_STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(1), syn::Part::Token(ATOM), syn::Part::Rule(1)], commit: None }] },
    syn::Rule { ty: Some(EMPTY), alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(2)], commit: None })], commit: None }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[], commit: None }] },
];

pub fn parse(text: String) -> ::fall_tree::File {
    register_node_types();
    ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
}
