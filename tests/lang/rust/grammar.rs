use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

pub const LPAREN    : NodeType = NodeType(100);
pub const RPAREN    : NodeType = NodeType(101);
pub const LBRACE    : NodeType = NodeType(102);
pub const RBRACE    : NodeType = NodeType(103);
pub const PUB       : NodeType = NodeType(104);
pub const STRUCT    : NodeType = NodeType(105);
pub const FN        : NodeType = NodeType(106);
pub const IDENT     : NodeType = NodeType(107);
pub const FILE      : NodeType = NodeType(108);
pub const STRUCT_DEF: NodeType = NodeType(109);
pub const FN_DEF    : NodeType = NodeType(110);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        PUB.register(NodeTypeInfo { name: "PUB" });
        STRUCT.register(NodeTypeInfo { name: "STRUCT" });
        FN.register(NodeTypeInfo { name: "FN" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRUCT_DEF.register(NodeTypeInfo { name: "STRUCT_DEF" });
        FN_DEF.register(NodeTypeInfo { name: "FN_DEF" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: LPAREN, re: r"\(" },
    Rule { ty: RPAREN, re: r"\)" },
    Rule { ty: LBRACE, re: r"\{" },
    Rule { ty: RBRACE, re: r"\}" },
    Rule { ty: PUB, re: "pub" },
    Rule { ty: STRUCT, re: "struct" },
    Rule { ty: FN, re: "fn" },
    Rule { ty: IDENT, re: r"\w+" },
];
