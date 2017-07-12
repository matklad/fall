use {Node, NodeType, AstNode, TextUnit};

pub fn child_of_type(node: Node, ty: NodeType) -> Option<Node> {
    node.children().find(|n| n.ty() == ty)
}


pub fn children_of_type<'f>(node: Node<'f>, ty: NodeType) -> Box<Iterator<Item=Node<'f>> + 'f> {
    Box::new(node.children().filter(move |n| n.ty() == ty))
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

pub fn ast_parent<'f, T: AstNode<'f>>(node: Node<'f>) -> Option<T> {
    let mut curr = Some(node);
    while let Some(node) = curr {
        if node.ty() == T::ty() {
            return Some(T::new(node));
        }
        curr = node.parent()
    }
    None
}

pub fn ast_parent_exn<'f, T: AstNode<'f>>(node: Node<'f>) -> T {
    ast_parent(node).unwrap()
}


pub fn is_leaf(node: Node) -> bool {
    node.children().next().is_none() && !node.range().is_empty()
}

#[derive(Clone, Copy)]
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


pub fn next_sibling<'f>(node: Node<'f>) -> Option<Node<'f>> {
    match child_position(node) {
        Some((parent, idx)) => parent.children().nth(idx + 1),
        _ => None
    }
}

pub fn prev_sibling(node: Node) -> Option<Node> {
    match child_position(node) {
        Some((parent, idx)) if idx > 0 => parent.children().nth(idx - 1),
        _ => None
    }
}

fn child_position(child: Node) -> Option<(Node, usize)> {
    if let Some(parent) = child.parent() {
        return Some((parent, parent.children().position(|n| n == child).unwrap()));
    }
    None
}
