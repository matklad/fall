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
