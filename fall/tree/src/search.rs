use {Node, NodeType, TextUnit, TextRange};
use ::visitor::{Visitor, NodeVisitor};

pub fn child_of_type(node: Node, ty: NodeType) -> Option<Node> {
    node.children().find(|n| n.ty() == ty)
}

pub fn children_of_type<'f>(node: Node<'f>, ty: NodeType) -> Box<Iterator<Item=Node<'f>> + 'f> {
    Box::new(node.children().filter(move |n| n.ty() == ty))
}

pub fn subtree<'f>(node: Node<'f>) -> Box<Iterator<Item=Node<'f>> + 'f> {
    Box::new(node.children().flat_map(subtree).chain(::std::iter::once(node)))
}

pub fn descendants_of_type<'f>(node: Node<'f>, ty: NodeType) -> Vec<Node<'f>> {
    Visitor(Vec::new())
        .visit_nodes(&[ty], |nodes, node| nodes.push(node))
        .walk_recursively_children_first(node)
}

pub fn child_of_type_exn(node: Node, ty: NodeType) -> Node {
    child_of_type(node, ty).unwrap_or_else(|| {
        panic!("No child of type {:?} for {:?}\
                ----\
                {}\
                ----", ty, node.ty(), node.text())
    })
}


pub fn ancestors(node: Node) -> Ancestors {
    Ancestors(Some(node))
}

pub struct Ancestors<'f>(Option<Node<'f>>);

impl<'f> Iterator for Ancestors<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        self.0 = current.and_then(|n| n.parent());
        current
    }
}

pub fn is_leaf(node: Node) -> bool {
    node.children().next().is_none() && !node.range().is_empty()
}

pub fn find_leaf_at_offset(node: Node, offset: TextUnit) -> LeafAtOffset {
    let range = node.range();
    assert!(node.range().contains_offset_nonstrict(offset), "Bad offset: range {:?} offset {:?}", range, offset);
    if range.is_empty() {
        return LeafAtOffset::None;
    }

    if is_leaf(node) {
        return LeafAtOffset::Single(node);
    }

    let mut children = node.children()
        .filter(|child| !child.range().is_empty())
        .filter(|child| child.range().contains_offset_nonstrict(offset));

    let left = children.next().unwrap();
    let right = children.next();
    assert!(children.next().is_none());
    return if let Some(right) = right {
        match (find_leaf_at_offset(left, offset), find_leaf_at_offset(right, offset)) {
            (LeafAtOffset::Single(left), LeafAtOffset::Single(right)) =>
                LeafAtOffset::Between(left, right),
            _ => unreachable!()
        }
    } else {
        find_leaf_at_offset(left, offset)
    };
}

#[derive(Clone, Copy, Debug)]
pub enum LeafAtOffset<'f> {
    None,
    Single(Node<'f>),
    Between(Node<'f>, Node<'f>)
}

impl<'f> LeafAtOffset<'f> {
    pub fn right_biased(self) -> Option<Node<'f>> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(_, right) => Some(right)
        }
    }

    pub fn left_biased(self) -> Option<Node<'f>> {
        match self {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => Some(node),
            LeafAtOffset::Between(left, _) => Some(left)
        }
    }
}

pub fn find_covering_node(root: Node, range: TextRange) -> Node {
    assert!(range.is_subrange_of(root.range()));
    let (left, right) = match (
        find_leaf_at_offset(root, range.start()).right_biased(),
        find_leaf_at_offset(root, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return root
    };

    common_ancestor(left, right)
}

pub fn prev_sibling(node: Node) -> Option<Node> {
    match child_position(node) {
        Some((parent, idx)) if idx > 0 => parent.children().nth(idx - 1),
        _ => None
    }
}

pub fn next_sibling<'f>(node: Node<'f>) -> Option<Node<'f>> {
    match child_position(node) {
        Some((parent, idx)) => parent.children().nth(idx + 1),
        _ => None
    }
}

pub mod ast {
    use {Node, AstNode};
    use visitor::{NodeVisitor, Visitor};
    use super::ancestors;

    pub fn ancestor<'f, T: AstNode<'f>>(node: Node<'f>) -> Option<T> {
        ancestors(node)
            .filter_map(T::wrap)
            .next()
    }

    pub fn ancestor_exn<'f, T: AstNode<'f>>(node: Node<'f>) -> T {
        ancestor(node).unwrap()
    }

    pub fn descendants_of_type<'f, N: AstNode<'f>>(node: Node<'f>) -> Vec<N> {
        Visitor(Vec::new())
            .visit::<N, _>(|acc, node| acc.push(node))
            .walk_recursively_children_first(node)
    }
}

fn child_position(child: Node) -> Option<(Node, usize)> {
    child.parent()
        .map(|parent| {
            (parent, parent.children().position(|n| n == child).unwrap())
        })
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p;
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}

