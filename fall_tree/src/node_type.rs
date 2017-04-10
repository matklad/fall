#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeType(pub u32);

pub const ERROR: NodeType = NodeType(0);
pub const WHITESPACE: NodeType = NodeType(1);

#[derive(Clone, Copy)]
pub struct NodeTypeInfo {
    pub name: &'static str
}
