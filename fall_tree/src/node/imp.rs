use std::ops::Index;
use elapsed::ElapsedDuration;

use {TextRange, NodeType, Language};
use super::{File, Node, NodeBuilder};

pub struct FileImpl {
    lex_time: ElapsedDuration,
    parse_time: ElapsedDuration,
    text: String,
    root: NodeId,
    nodes: Vec<NodeData>,
}

impl FileImpl {
    pub fn root(&self) -> Node {
        Node(NodeImpl { id: self.root, file: self })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lexing_time(&self) -> ElapsedDuration {
        self.lex_time
    }

    pub fn parsing_time(&self) -> ElapsedDuration {
        self.parse_time
    }
}

#[derive(Clone, Copy)]
pub struct NodeImpl<'f> {
    id: NodeId,
    file: &'f FileImpl,
}

impl<'f> NodeImpl<'f> {
    pub fn ty(&self) -> NodeType {
        self.data().ty
    }

    pub fn range(&self) -> TextRange {
        self.data().range
    }

    pub fn text(&self) -> &'f str {
        &self.file.text[self.range()]
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


pub struct FileBuilderImpl {
    lang: Language,
    lex_time: ElapsedDuration,
    parse_time: ElapsedDuration,
    nodes: Vec<NodeData>,
    text: String,
}

impl FileBuilderImpl {
    pub fn new(lang: Language, text: String, lex_time: ElapsedDuration, parse_time: ElapsedDuration) -> FileBuilderImpl {
        FileBuilderImpl {
            lang: lang,
            lex_time: lex_time,
            parse_time: parse_time,
            nodes: vec![],
            text: text
        }
    }

    pub fn node(&mut self, parent: Option<NodeBuilder>, ty: NodeType, range: TextRange)
                -> NodeBuilder {
        let parent = parent.map(|n| n.0);
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(NodeData {
            ty: ty,
            parent: parent,
            children: vec![],
            range: range,
        });
        if let Some(parent) = parent {
            self.nodes[parent.0 as usize].children.push(id)
        }

        NodeBuilder(id)
    }

    pub fn build(self) -> File {
        assert!(!self.nodes.is_empty());
        let imp = FileImpl {
            lex_time: self.lex_time,
            parse_time: self.parse_time,
            text: self.text,
            root: NodeId(0),
            nodes: self.nodes,
        };
        File { lang: self.lang, imp: imp }
    }
}
