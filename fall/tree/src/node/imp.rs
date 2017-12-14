use std::ops::Index;

use {TextBuf, Text, TextRange, NodeType, Language, Metrics};
use super::Node;
use node::tree_builder::{TreeBuilder, NodeData, NodeId};

pub struct FileImpl {
    pub lang: Language,
    metrics: Metrics,
    text: TextBuf,
    root: NodeId,
    nodes: Vec<NodeData>,
}

impl FileImpl {
    pub fn root<'i, 'f: 'i>(&'i self, file: &'f super::File) -> Node<'f> {
        Node(NodeImpl { id: self.root, file })
    }

    pub fn text(&self) -> Text {
        self.text.as_slice()
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }
}

#[derive(Clone, Copy)]
pub struct NodeImpl<'f> {
    id: NodeId,
    file: &'f super::File,
}

impl<'f> ::std::cmp::PartialEq for NodeImpl<'f> {
    fn eq(&self, other: &NodeImpl<'f>) -> bool {
        self.key() == other.key()
    }
}

impl<'f> ::std::cmp::Eq for NodeImpl<'f> {}

impl<'f> ::std::hash::Hash for NodeImpl<'f> {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

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

    pub fn file(&self) -> &'f super::File { self.file }

    pub fn parent(&self) -> Option<Node<'f>> {
        self.data().parent.map(|id| Node(NodeImpl { id, file: self.file }))
    }

    pub fn children(&self) -> NodeChildren<'f> {
        NodeChildren { file: self.file, inner: self.data().children.iter() }
    }

    pub fn debug(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Node({})", self.file.imp.lang.node_type_info(self.ty()).name)
    }

    fn data(&self) -> &'f NodeData {
        &self.file.imp[self.id]
    }

    fn key(&self) -> (*const super::File, NodeId) {
        (self.file as *const super::File, self.id)
    }
}

#[derive(Clone)]
pub struct NodeChildren<'f> {
    file: &'f super::File,
    inner: ::std::slice::Iter<'f, NodeId>,
}

impl<'f> Iterator for NodeChildren<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        self.inner.next().map(|&id| Node(NodeImpl { id, file: self.file }))
    }
}

impl Index<NodeId> for FileImpl {
    type Output = NodeData;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}


pub fn new_file2(lang: Language, text: TextBuf, metrics: Metrics, builder: TreeBuilder) -> FileImpl {
    FileImpl {
        lang,
        metrics,
        text,
        root: NodeId(0),
        nodes: builder.finish(),
    }
}
