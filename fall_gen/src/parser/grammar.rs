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
pub const RULE      : NodeType = NodeType(110);

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
        RULE.register(NodeTypeInfo { name: "RULE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: STRING, re: r#"r?"[^"]*""#, f: None },
    Rule { ty: NODES, re: "nodes", f: None },
    Rule { ty: TOKENIZER_KW, re: "tokenizer", f: None },
    Rule { ty: IDENT, re: r"\w+", f: None },
];
