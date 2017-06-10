use std::ops::Index;

use {Text, TextRange, NodeType, TextUnit};
use super::{Node, FileStats};
use super::immutable::ImmutableNode;

pub struct FileImpl {
    stats: FileStats,
    text: String,
    root: NodeId,
    nodes: Vec<NodeData>,
}

impl FileImpl {
    pub fn root(&self) -> Node {
        Node(NodeImpl { id: self.root, file: self })
    }

    pub fn text(&self) -> Text {
        Text::from_owned(&self.text)
    }

    pub fn stats(&self) -> FileStats {
        self.stats
    }
}

#[derive(Clone, Copy)]
pub struct NodeImpl<'f> {
    id: NodeId,
    file: &'f FileImpl,
}

impl<'f> ::std::cmp::PartialEq for NodeImpl<'f> {
    fn eq(&self, other: &NodeImpl<'f>) -> bool {
        self.file as *const FileImpl == other.file as *const FileImpl && self.id == other.id
    }
}

impl<'f> ::std::cmp::Eq for NodeImpl<'f> {}

impl<'f> NodeImpl<'f> {
    pub fn ty(&self) -> NodeType {
        self.data().ty
    }

    pub fn range(&self) -> TextRange {
        self.data().range
    }

    pub fn text(&self) -> Text<'f> {
        self.file.text().slice(self.range())
    }

    pub fn parent(&self) -> Option<Node<'f>> {
        self.data().parent.map(|id| Node(NodeImpl { id: id, file: self.file }))
    }

    pub fn children(&self) -> NodeChildren<'f> {
        NodeChildren { file: self.file, inner: self.data().children.iter() }
    }

    fn data(&self) -> &'f NodeData {
        &self.file[self.id]
    }
}

pub struct NodeChildren<'f> {
    file: &'f FileImpl,
    inner: ::std::slice::Iter<'f, NodeId>,
}

impl<'f> Iterator for NodeChildren<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        self.inner.next().map(|&id| Node(NodeImpl { id: id, file: self.file }))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(u32);

impl Index<NodeId> for FileImpl {
    type Output = NodeData;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}

pub struct NodeData {
    ty: NodeType,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    range: TextRange,
}


pub fn new_file(text: String, stats: FileStats, node: &ImmutableNode) -> FileImpl {
    let mut nodes = Vec::new();
    go(TextUnit::zero(), node, &mut nodes);

    return FileImpl {
        stats: stats,
        text: text,
        root: NodeId(0),
        nodes: nodes,
    };

    fn go(range_start: TextUnit, node: &ImmutableNode, nodes: &mut Vec<NodeData>) {
        let my_idx = nodes.len();
        nodes.push(NodeData {
            ty: node.ty(),
            parent: None,
            children: Vec::new(),
            range: TextRange::from_to(range_start, range_start + node.len()),
        });
        let mut range_start = range_start;
        for child in node.children() {
            let child_idx = nodes.len();
            nodes[my_idx].children.push(NodeId(child_idx as u32));
            go(range_start, child, nodes);
            nodes[child_idx].parent = Some(NodeId(my_idx as u32));
            range_start += child.len();
        }
    }
}
