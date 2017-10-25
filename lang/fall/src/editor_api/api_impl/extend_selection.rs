use fall_tree::{TextUnit, TextRange, Node, File};
use fall_tree::search::{ancestors, find_leaf_at_offset};


pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    let node = match find_node_at_range(&file, range) {
        Some(node) => node,
        None => return None,
    };

    match ancestors(node).skip_while(|n| n.range() == range).next() {
        None => None,
        Some(parent) => Some(parent.range()),
    }
}


fn find_node_at_range(file: &File, range: TextRange) -> Option<Node> {
    if range.is_empty() {
        return try_find_non_ws_node_at_offset(file, range.start());
    }

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

fn try_find_non_ws_node_at_offset(file: &File, offset: TextUnit) -> Option<Node> {
    let leaves = find_leaf_at_offset(file.root(), offset);
    if let Some(leaf) = leaves.left_biased() {
        if file.language().node_type_info(leaf.ty()).whitespace_like {
            return leaves.right_biased();
        }
    }

    leaves.left_biased()
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p;
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}

#[test]
fn test_extend_selection() {
    let file = ::parse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"####);
    let offset = TextUnit::from_usize(44);
    let s1 =
        extend_selection(&file, TextRange::from_len(offset, TextUnit::zero()))
            .unwrap();
    let s2 = extend_selection(&file, s1).unwrap();
    let s3 = extend_selection(&file, s2).unwrap();
    assert_eq!(
        format!("{:?}", (s1, s2, s3)),
        "([43; 46), [41; 48), [28; 48))"
    );
}
