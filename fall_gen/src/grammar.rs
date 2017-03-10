use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

pub const NODES     : NodeType = NodeType(100);
pub const TOKENIZER_KW: NodeType = NodeType(101);
pub const EQ        : NodeType = NodeType(102);
pub const LBRACE    : NodeType = NodeType(103);
pub const RBRACE    : NodeType = NodeType(104);
pub const IDENT     : NodeType = NodeType(105);
pub const STRING    : NodeType = NodeType(106);
pub const FILE      : NodeType = NodeType(107);
pub const NODES_DEF : NodeType = NodeType(108);
pub const TOKENIZER_DEF: NodeType = NodeType(109);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        NODES.register(NodeTypeInfo { name: "NODES" });
        TOKENIZER_KW.register(NodeTypeInfo { name: "TOKENIZER_KW" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        NODES_DEF.register(NodeTypeInfo { name: "NODES_DEF" });
        TOKENIZER_DEF.register(NodeTypeInfo { name: "TOKENIZER_DEF" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: EQ, re: "=" },
    Rule { ty: LBRACE, re: r"\{" },
    Rule { ty: RBRACE, re: r"\}" },
    Rule { ty: STRING, re: r#"r?"[^"]*""# },
    Rule { ty: NODES, re: "nodes" },
    Rule { ty: TOKENIZER_KW, re: "tokenizer_kw" },
    Rule { ty: IDENT, re: r"\w+" },
];
