use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

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
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: LPAREN, re: r"\(" },
    Rule { ty: RPAREN, re: r"\)" },
    Rule { ty: ATOM, re: r"\w+" },
];

