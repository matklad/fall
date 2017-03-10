use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

pub const ATOM      : NodeType = NodeType(100);
pub const FILE      : NodeType = NodeType(101);
pub const EMPTY     : NodeType = NodeType(102);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        ATOM.register(NodeTypeInfo { name: "ATOM" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        EMPTY.register(NodeTypeInfo { name: "EMPTY" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: ATOM, re: r"\w+" },
];
