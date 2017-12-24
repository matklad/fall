use fall_tree::{TextUnit, TextRange, Node, File, TextSuffix};
use fall_tree::search::{ancestors, find_leaf_at_offset, find_covering_node};


pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    let lang = file.language();
    let is_ws = |node: Node| lang.node_type_info(node.ty()).whitespace_like;
    if range.is_empty() {
        let offset = range.start();
        let mut leaves = find_leaf_at_offset(file.root(), offset);
        if let Some(leaf) = leaves.clone().find(|&node| !is_ws(node)) {
            return Some(leaf.range());
        }
        let ws = leaves.next()?;
        let ws_suffix = file.text().slice(
            TextRange::from_to(offset, ws.range().end())
        );
        if ws.text().contains("\n") && !ws_suffix.contains("\n") {
            if let Some(line_end) = file.text()
                .slice(TextSuffix::from(ws.range().end()))
                .find("\n")
            {
                let range = TextRange::from_len(ws.range().end(), line_end);
                return Some(find_covering_node(file.root(), range).range());
            }
        }
        return Some(ws.range());
    };
    let node = find_covering_node(file.root(), range);

    match ancestors(node).skip_while(|n| n.range() == range).next() {
        None => None,
        Some(parent) => Some(parent.range()),
    }
}
