use {Node, NodeType, AstNode};

pub fn path_to_leaf_at_offset(node: Node, offset: u32) -> Vec<Node> {
    let mut result = Vec::new();
    let mut node = node;
    loop {
        result.push(node);
        if !has_children(node) {
            break;
        }
        node = node.children().find(|&child| contains(child, offset)).unwrap_or_else(|| {
            let last_child = node.children().last().unwrap();
            assert_eq!(offset, last_child.range().end());
            last_child
        });
    }
    result
}

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

fn has_children(node: Node) -> bool {
    node.children().next().is_some()
}

fn contains(node: Node, offset: u32) -> bool {
    let r = node.range();
    let (start, end) = (r.start(), r.end());
    start <= offset && offset < end
}

