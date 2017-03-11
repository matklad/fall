use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

pub const KW_NODES  : NodeType = NodeType(100);
pub const KW_TOKENIZER: NodeType = NodeType(101);
pub const EQ        : NodeType = NodeType(102);
pub const LBRACE    : NodeType = NodeType(103);
pub const RBRACE    : NodeType = NodeType(104);
pub const IDENT     : NodeType = NodeType(105);
pub const SIMPLE_STRING: NodeType = NodeType(106);
pub const HASH_STRING: NodeType = NodeType(107);
pub const FILE      : NodeType = NodeType(108);
pub const STRING    : NodeType = NodeType(109);
pub const DEF_NODES : NodeType = NodeType(110);
pub const DEF_TOKENIZER: NodeType = NodeType(111);
pub const RULE      : NodeType = NodeType(112);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        KW_NODES.register(NodeTypeInfo { name: "KW_NODES" });
        KW_TOKENIZER.register(NodeTypeInfo { name: "KW_TOKENIZER" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        SIMPLE_STRING.register(NodeTypeInfo { name: "SIMPLE_STRING" });
        HASH_STRING.register(NodeTypeInfo { name: "HASH_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        DEF_NODES.register(NodeTypeInfo { name: "DEF_NODES" });
        DEF_TOKENIZER.register(NodeTypeInfo { name: "DEF_TOKENIZER" });
        RULE.register(NodeTypeInfo { name: "RULE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: SIMPLE_STRING, re: r#"r?"([^"\\]|\\.)*""#, f: None },
    Rule { ty: HASH_STRING, re: "r#+", f: Some(super::parse_raw_string) },
    Rule { ty: KW_NODES, re: "nodes", f: None },
    Rule { ty: KW_TOKENIZER, re: "tokenizer", f: None },
    Rule { ty: IDENT, re: r"\w+", f: None },
];
