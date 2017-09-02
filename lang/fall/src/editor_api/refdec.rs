use fall_tree::{Node, File, TextUnit, TextRange};
use fall_tree::search::{ancestors, find_leaf_at_offset};

pub struct Declaration<'f> {
    node: Node<'f>,
    name_ident: Option<Node<'f>>,
}

impl<'f> Declaration<'f> {
    pub fn new(node: Node<'f>) -> Declaration<'f> {
        Declaration { node, name_ident: None }
    }

    pub fn with_name_ident(node: Node<'f>, name_identifier: Option<Node<'f>>) -> Declaration<'f> {
        Declaration { node, name_ident: name_identifier }
    }
}


pub struct Reference<'f> {
    node: Node<'f>,
    resolve: fn(Node<'f>) -> Option<Declaration<'f>>
}

impl<'f> Reference<'f> {
    pub fn new(node: Node<'f>, resolve: fn(Node<'f>) -> Option<Declaration<'f>>) -> Reference<'f> {
        Reference { node, resolve }
    }

    fn resolve(&self) -> Option<Declaration<'f>> {
        (self.resolve)(self.node)
    }
}


pub type ReferenceProvider = fn(Node) -> Option<Reference>;


pub fn resolve_reference(
    file: &File,
    offset: TextUnit,
    provider: ReferenceProvider
) -> Option<TextRange> {
    let node = match try_find_non_ws_node_at_offset(file, offset) {
        None => return None,
        Some(node) => node
    };

    let reference = match ancestors(node).filter_map(|node| provider(node)).next() {
        Some(ref_) => ref_,
        None => return None,
    };

    if let Some(decl) = reference.resolve() {
        Some(decl.name_ident.unwrap_or(decl.node).range())
    } else {
        None
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

