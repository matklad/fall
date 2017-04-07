use {Node, NodeType, AstNode};

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

pub fn ast_parent_exn<'f, T: AstNode<'f>>(node: Node<'f>) -> T {
    ast_parent(node).unwrap()
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
