use std::ops::Index;
use builder::PreNode;

use {File, NodeType, TextRange};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(u32);

pub struct Node {
    pub ty: NodeType,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub range: TextRange,
}

impl Index<NodeId> for File {
    type Output = Node;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}

pub fn build_file(text: String, ty: NodeType, children: Vec<PreNode>) -> File {
    fn go(parent: NodeId, node: &PreNode, nodes: &mut Vec<Node>) -> NodeId {
        let id = NodeId(nodes.len() as u32);
        nodes.push(Node {
            ty: node.ty,
            parent: Some(parent),
            children: vec![],
            range: node.range,
        });

        let mut children = vec![];
        for child in node.children.iter() {
            children.push(go(id, child, nodes));
        }
        nodes[id.0 as usize].children = children;
        id
    }

    let mut nodes = vec![Node {
        ty: ty,
        parent: None,
        children: vec![],
        range: TextRange::empty(),
    }];

    let mut root_children: Vec<NodeId> = vec![];
    let mut range = TextRange::empty();
    for child in children {
        range = range.glue(child.range);
        let c = go(NodeId(0), &child, &mut nodes);
        root_children.push(c);
    }
    nodes[0].range = range;
    nodes[0].children = root_children;

    File {
        text: text,
        root: NodeId(0),
        nodes: nodes,
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
