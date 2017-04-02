use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
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
