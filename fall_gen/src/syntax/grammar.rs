use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_NODES  : NodeType = NodeType(100);
pub const KW_TOKENIZER: NodeType = NodeType(101);
pub const KW_RULE   : NodeType = NodeType(102);
pub const EQ        : NodeType = NodeType(103);
pub const PIPE      : NodeType = NodeType(104);
pub const STAR      : NodeType = NodeType(105);
pub const LBRACE    : NodeType = NodeType(106);
pub const RBRACE    : NodeType = NodeType(107);
pub const LANGLE    : NodeType = NodeType(108);
pub const RANGLE    : NodeType = NodeType(109);
pub const LPAREN    : NodeType = NodeType(110);
pub const RPAREN    : NodeType = NodeType(111);
pub const IDENT     : NodeType = NodeType(112);
pub const SIMPLE_STRING: NodeType = NodeType(113);
pub const HASH_STRING: NodeType = NodeType(114);
pub const FILE      : NodeType = NodeType(115);
pub const STRING    : NodeType = NodeType(116);
pub const NODES_DEF : NodeType = NodeType(117);
pub const TOKENIZER_DEF: NodeType = NodeType(118);
pub const TOKEN_DEF : NodeType = NodeType(119);
pub const SYN_RULE  : NodeType = NodeType(120);
pub const ALT       : NodeType = NodeType(121);
pub const PART      : NodeType = NodeType(122);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        KW_NODES.register(NodeTypeInfo { name: "KW_NODES" });
        KW_TOKENIZER.register(NodeTypeInfo { name: "KW_TOKENIZER" });
        KW_RULE.register(NodeTypeInfo { name: "KW_RULE" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        PIPE.register(NodeTypeInfo { name: "PIPE" });
        STAR.register(NodeTypeInfo { name: "STAR" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LANGLE.register(NodeTypeInfo { name: "LANGLE" });
        RANGLE.register(NodeTypeInfo { name: "RANGLE" });
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        SIMPLE_STRING.register(NodeTypeInfo { name: "SIMPLE_STRING" });
        HASH_STRING.register(NodeTypeInfo { name: "HASH_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        NODES_DEF.register(NodeTypeInfo { name: "NODES_DEF" });
        TOKENIZER_DEF.register(NodeTypeInfo { name: "TOKENIZER_DEF" });
        TOKEN_DEF.register(NodeTypeInfo { name: "TOKEN_DEF" });
        SYN_RULE.register(NodeTypeInfo { name: "SYN_RULE" });
        ALT.register(NodeTypeInfo { name: "ALT" });
        PART.register(NodeTypeInfo { name: "PART" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: PIPE, re: r"\|", f: None },
    Rule { ty: STAR, re: r"\*", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: LANGLE, re: "<", f: None },
    Rule { ty: RANGLE, re: ">", f: None },
    Rule { ty: LPAREN, re: r"\(", f: None },
    Rule { ty: RPAREN, re: r"\)", f: None },
    Rule { ty: SIMPLE_STRING, re: r#"r?"([^"\\]|\\.)*""#, f: None },
    Rule { ty: HASH_STRING, re: "r#+", f: Some(super::parse_raw_string) },
    Rule { ty: KW_NODES, re: "nodes", f: None },
    Rule { ty: KW_TOKENIZER, re: "tokenizer", f: None },
    Rule { ty: KW_RULE, re: "rule", f: None },
    Rule { ty: IDENT, re: r"\w+", f: None },
];
