use fall_tree::{Node, File, TextUnit, TextRange, AstNode};
use fall_tree::search::{ancestors, subtree, find_leaf_at_offset};
use ::Analysis;

#[derive(Eq, PartialEq)]
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

    fn navigation_range(&self) -> TextRange {
        self.name_ident.unwrap_or(self.node).range()
    }
}

pub struct Reference<'f> {
    node: Node<'f>,
    resolve: fn(&Analysis<'f>, Node<'f>) -> Option<Declaration<'f>>
}

impl<'f> Reference<'f> {
    pub fn new(node: Node<'f>, resolve: fn(&Analysis<'f>, Node<'f>) -> Option<Declaration<'f>>) -> Reference<'f> {
        Reference { node, resolve }
    }

    fn resolve(&self, analysis: &Analysis<'f>) -> Option<Declaration<'f>> {
        (self.resolve)(analysis, self.node)
    }
}

pub type DeclarationProvider<'f> = fn(Node<'f>) -> Option<Declaration<'f>>;

pub type ReferenceProvider<'p, 'f> = &'p Fn(Node<'f>) -> Option<Reference<'f>>;

pub fn resolve_reference<'p, 'f>(
    analysis: &Analysis<'f>,
    offset: TextUnit,
    provider: ReferenceProvider<'p, 'f>
) -> Option<TextRange> {
    let reference = match try_find_at_offset(analysis.file().node().file(), offset, |node| provider(node)) {
        Some(ref_) => ref_,
        None => return None,
    };

    reference.resolve(analysis).map(|d| d.navigation_range())
}

pub fn find_usages<'p, 'f>(
    analysis: &Analysis<'f>,
    offset: TextUnit,
    reference_provider: ReferenceProvider<'p, 'f>,
    declaration_provider: DeclarationProvider<'f>
) -> Vec<TextRange> {
    let file = analysis.file();
    let declaration = try_find_at_offset(file.node().file(), offset, |node| {
        declaration_provider(node)
            .and_then(|d| {
                if d.navigation_range().contains_offset_nonstrict(offset) { Some(d) } else { None }
            })
            .or_else(|| reference_provider(node).and_then(|ref_| ref_.resolve(analysis)))
    });
    let declaration = match declaration {
        Some(decl) => decl,
        None => return Vec::new(),
    };

    subtree(file.node())
        .filter_map(|node| reference_provider(node))
        .filter(|ref_| ref_.resolve(analysis).as_ref() == Some(&declaration))
        .map(|ref_| ref_.node.range())
        .collect()
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

fn try_find_at_offset<'f, T, F: Fn(Node<'f>) -> Option<T>>(
    file: &'f File,
    offset: TextUnit,
    f: F
) -> Option<T> {
    let node = match try_find_non_ws_node_at_offset(file, offset) {
        None => return None,
        Some(node) => node
    };

    ancestors(node).filter_map(f).next()
}
