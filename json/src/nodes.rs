use fall::{NodeType, NodeTypeInfo};
pub use fall::{ERROR, WHITESPACE};

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

pub fn register_node_types() {    NULL.register(NodeTypeInfo { name: "NULL" });
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
}

