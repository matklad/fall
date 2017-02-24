extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fmt::Write;

mod node_type;
pub use node_type::{NodeType, NodeTypeInfo, ERROR, WHITESPACE};

pub mod builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl TextRange {
    fn is_subrange_of(self, other: TextRange) -> bool {
        other.start <= self.start && self.end <= other.end
    }
}

impl ::std::ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start as usize..index.end as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Node<'f> {
    id: imp::NodeId,
    file: &'f File,
}


impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.raw().ty
    }

    pub fn parent(&self) -> Option<Node> {
        self.raw().parent.map(|id| self.file.node(id))
    }

    pub fn children<'n>(&'n self) -> NodeChildren<'n, 'f> {
        NodeChildren {
            file: self.file,
            inner: self.raw().children.iter(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.raw().children.is_empty()
    }

    pub fn range(&self) -> TextRange {
        self.raw().range
    }

    pub fn text(&self) -> &str {
        unimplemented!()
    }

    fn raw(&self) -> &imp::Node {
        &self.file[self.id]
    }
}

pub struct File {
    text: String,
    root: imp::NodeId,
    nodes: Vec<imp::Node>,
}

impl File {
    pub fn root(&self) -> Node {
        self.node(self.root)
    }

    pub fn node_containing_range(&self, range: TextRange) -> Node {
        imp::node_containing_range(self.root(), range)
    }

    pub fn dump(&self) -> String {
        let mut buf = String::new();
        go(0, self.root(), &self.text, &mut buf);
        return buf;

        fn go(level: usize, n: Node, text: &str, buf: &mut String) {
            if n.ty() == WHITESPACE {
                return
            }

            for _ in 0..level {
                buf.push_str("  ")
            }

            if n.is_leaf() {
                write!(buf, "{} {:?}\n", n.ty().name(), &text[n.range()])
                    .unwrap();
            } else {
                write!(buf, "{}\n", n.ty().name())
                    .unwrap();
                for c in n.children() {
                    go(level + 1, c, text, buf);
                }
            }
        }
    }

    fn node(&self, id: imp::NodeId) -> Node {
        Node { id: id, file: self }
    }
}

mod imp;

pub struct NodeChildren<'n, 'f: 'n> {
    file: &'f File,
    inner: std::slice::Iter<'n, imp::NodeId>,
}

impl<'n, 'f: 'n> Iterator for NodeChildren<'n, 'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        self.inner.next().map(|&id| self.file.node(id))
    }
}
