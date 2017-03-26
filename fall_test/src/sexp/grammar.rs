use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
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