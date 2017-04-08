use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const ATOM      : NodeType = NodeType(100);
pub const RAW_STRING: NodeType = NodeType(101);
pub const FILE      : NodeType = NodeType(102);
pub const EMPTY     : NodeType = NodeType(103);

fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        ATOM.register(NodeTypeInfo { name: "ATOM" });
        RAW_STRING.register(NodeTypeInfo { name: "RAW_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        EMPTY.register(NodeTypeInfo { name: "EMPTY" });
    });
}

const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: "\\s+", f: None },
    Rule { ty: RAW_STRING, re: "r#+\"", f: Some(parse_raw_string) },
    Rule { ty: ATOM, re: "\\w+", f: None },
];

const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Token(RAW_STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Rule(1), syn::Part::Token(ATOM), syn::Part::Rule(1)], commit: None }] },
    syn::Rule { ty: Some(EMPTY), alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(2)], commit: None })], commit: None }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[], commit: None }] },
];

lazy_static! {
    pub static ref LANG: Language = {
        register_node_types();
        struct Impl;
        impl LanguageImpl for Impl {
            fn parse(&self, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
            }
        }

        Language::new(Impl)
    };
}
fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}
