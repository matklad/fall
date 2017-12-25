#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct NodeType(pub u32);

pub const ERROR: NodeType = NodeType(0);

#[derive(Clone, Copy)]
pub struct NodeTypeInfo {
    pub name: &'static str,
    pub whitespace_like: bool,
}
