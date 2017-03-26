use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const NULL      : NodeType = NodeType(100);
pub const BOOL      : NodeType = NodeType(101);
pub const NUMBER    : NodeType = NodeType(102);
pub const STRING    : NodeType = NodeType(103);
pub const LBRACE    : NodeType = NodeType(104);
pub const RBRACE    : NodeType = NodeType(105);
pub const LBRACK    : NodeType = NodeType(106);
pub const RBRACK    : NodeType = NodeType(107);
pub const COMMA     : NodeType = NodeType(108);
pub const COLON     : NodeType = NodeType(109);
pub const OBJECT    : NodeType = NodeType(110);
pub const ARRAY     : NodeType = NodeType(111);
pub const PRIMITIVE : NodeType = NodeType(112);
pub const FIELD     : NodeType = NodeType(113);
pub const FILE      : NodeType = NodeType(114);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        NULL.register(NodeTypeInfo { name: "NULL" });
        BOOL.register(NodeTypeInfo { name: "BOOL" });
        NUMBER.register(NodeTypeInfo { name: "NUMBER" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LBRACK.register(NodeTypeInfo { name: "LBRACK" });
        RBRACK.register(NodeTypeInfo { name: "RBRACK" });
        COMMA.register(NodeTypeInfo { name: "COMMA" });
        COLON.register(NodeTypeInfo { name: "COLON" });
        OBJECT.register(NodeTypeInfo { name: "OBJECT" });
        ARRAY.register(NodeTypeInfo { name: "ARRAY" });
        PRIMITIVE.register(NodeTypeInfo { name: "PRIMITIVE" });
        FIELD.register(NodeTypeInfo { name: "FIELD" });
        FILE.register(NodeTypeInfo { name: "FILE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: LBRACK, re: r"\[", f: None },
    Rule { ty: RBRACK, re: r"\]", f: None },
    Rule { ty: COLON, re: r":", f: None },
    Rule { ty: COMMA, re: r",", f: None },
    Rule { ty: STRING, re: r#""[^"]*""#, f: None },
    Rule { ty: NUMBER, re: r"\d+", f: None },
];

pub const PARSER: &'static[syn::Rule] = &[
    syn::Rule { name: "file", alts: &[] },
    syn::Rule { name: "object", alts: &[] },
    syn::Rule { name: "field", alts: &[] },
    syn::Rule { name: "array", alts: &[] },
    syn::Rule { name: "value", alts: &[] },
    syn::Rule { name: "primitive", alts: &[] },
];