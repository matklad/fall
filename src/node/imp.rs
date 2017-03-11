use std::ops::Index;
use std::fmt::Write;

use {File, NodeType, TextRange, Node, WHITESPACE};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(u32);

pub struct RawNode {
    pub ty: NodeType,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub range: TextRange,
}

impl Index<NodeId> for File {
    type Output = RawNode;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}

pub struct FileBuilder {
    nodes: Vec<RawNode>,
}

impl FileBuilder {
    pub fn new() -> FileBuilder {
        FileBuilder { nodes: vec![] }
    }

    pub fn node(&mut self, parent: Option<NodeId>, ty: NodeType, range: TextRange) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(RawNode {
            ty: ty,
            parent: parent,
            children: vec![],
            range: range,
        });
        if let Some(parent) = parent {
            self.nodes[parent.0 as usize].children.push(id)
        }

        id
    }

    pub fn build(self, text: String) -> File {
        assert!(!self.nodes.is_empty());
        File {
            text: text,
            root: NodeId(0),
            nodes: self.nodes,
        }
    }
}

pub fn node_containing_range(node: ::Node, range: TextRange) -> ::Node {
    fn go<'f>(node: ::Node<'f>, range: TextRange) -> Option<::Node<'f>> {
        if !range.is_subrange_of(node.range()) {
            return None
        }

        for child in node.children() {
            if child.range() == node.range() {
                break;
            }
            if let Some(n) = go(child, range) {
                return Some(n);
            }
        }
        return Some(node)
    }

    assert!(range.is_subrange_of(node.range()));
    go(node, range).unwrap()
}

pub fn dump(root: Node, text: &str, include_whitespace: bool) -> String {
    let mut buf = String::new();
    go(0, root, text, &mut buf, include_whitespace);
    return buf;

    fn go(level: usize, n: Node, text: &str, buf: &mut String, include_whitespace: bool) {
        if  n.ty() == WHITESPACE && !include_whitespace {
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
            for child in n.children() {
                go(level + 1, child, text, buf, include_whitespace);
            }
        }
    }
}
