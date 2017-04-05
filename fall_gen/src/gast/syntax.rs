use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const IDENT     : NodeType = NodeType(100);
pub const LBRACE    : NodeType = NodeType(101);
pub const RBRACE    : NodeType = NodeType(102);
pub const FILE      : NodeType = NodeType(103);
pub const NODE      : NodeType = NodeType(104);
pub const METHOD    : NodeType = NodeType(105);

fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        NODE.register(NodeTypeInfo { name: "NODE" });
        METHOD.register(NodeTypeInfo { name: "METHOD" });
    });
}

const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: "\\s+", f: None },
    Rule { ty: IDENT, re: "(\\w|\\*|\\.|_|\\?)+", f: None },
    Rule { ty: LBRACE, re: "\\{", f: None },
    Rule { ty: RBRACE, re: "\\}", f: None },
];

const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(1)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(NODE), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(2)], commit: None }), syn::Part::Token(RBRACE)], commit: None }] },
    syn::Rule { ty: Some(METHOD), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Token(IDENT)], commit: None }] },
];

pub fn parse(text: String) -> ::fall_tree::File {
    register_node_types();
    ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
}
