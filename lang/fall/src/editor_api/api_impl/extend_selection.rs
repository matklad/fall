use fall_tree::{TextUnit, TextRange, Node, File};
use fall_tree::search::{ancestors, find_leaf_at_offset, find_covering_node};


pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    if range.is_empty() {
        return try_find_non_ws_node_at_offset(file, range.start()).map(|n| n.range());
    }
    let node = find_covering_node(file.root(), range);

    match ancestors(node).skip_while(|n| n.range() == range).next() {
        None => None,
        Some(parent) => Some(parent.range()),
    }
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
