use std::ops::Index;

use {TextBuf, Text, TextRange, NodeType, TextUnit, Language, tu, Metrics, INode};
use super::Node;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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


pub fn new_file(lang: Language, text: TextBuf, metrics: Metrics, node: &INode) -> FileImpl {
    let mut nodes = Vec::new();

    metrics.measure_time("parent links", || {
        go(tu(0), node, &mut nodes);
    });

    return FileImpl {
        lang,
        metrics,
        text,
        root: NodeId(0),
        nodes,
    };

    fn go(range_start: TextUnit, node: &INode, nodes: &mut Vec<NodeData>) {
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
