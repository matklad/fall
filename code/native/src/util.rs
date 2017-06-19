use fall_tree::{TextRange, Node, File};
use fall_tree::search::{find_leaf_at_offset, ancestors};

pub fn find_node_at_range(file: &File, range: TextRange) -> Option<Node> {
    let root = file.root();
    let (left, right) = match (
        find_leaf_at_offset(root, range.start()).right_biased(),
        find_leaf_at_offset(root, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return None
    };

    Some(common_ancestor(left, right))
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}
