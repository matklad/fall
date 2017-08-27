use fall_tree::{AstNode, AstClass, File, Node, TextUnit};
use fall_tree::FileEdit;
use fall_tree::search::{find_leaf_at_offset, LeafAtOffset};
use ::{PIPE, BlockExpr};


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ContextActionId(pub &'static str);

pub const ACTIONS: &[&ContextAction] = &[
    &SwapAlternatives,
];

pub trait ContextAction {
    fn id(&self) -> ContextActionId;
    fn apply<'f>(&self, file: &'f File, offset: TextUnit) -> Option<FileEdit<'f>>;
}


struct SwapAlternatives;

impl ContextAction for SwapAlternatives {
    fn id(&self) -> ContextActionId {
        ContextActionId("Swap Alternatives")
    }

    fn apply<'f>(&self, file: &'f File, offset: TextUnit) -> Option<FileEdit<'f>> {
        let (left, right) = match find_swappable_nodes(file, offset) {
            Some((left, right)) => (left, right),
            None => return None
        };
        let mut edit = FileEdit::new(file);
        edit.replace(left, right);
        edit.replace(right, left);
        Some(edit)
    }
}

fn find_swappable_nodes<'f>(file: &'f File, offset: TextUnit) -> Option<(Node<'f>, Node<'f>)> {
    let pipe = match find_leaf_at_offset(file.root(), offset) {
        LeafAtOffset::None => return None,
        LeafAtOffset::Single(n) => if n.ty() == PIPE { n } else { return None },
        LeafAtOffset::Between(n1, n2) => if n1.ty() == PIPE {
            n1
        } else if n2.ty() == PIPE {
            n2
        } else {
            return None
        },
    };

    let parent = match pipe.parent() {
        None => return None,
        Some(n) => if n.ty() == BlockExpr::ty() {
            BlockExpr::new(n)
        } else {
            return None;
        }
    };

    for (alt1, alt2) in parent.alts().zip(parent.alts().skip(1)) {
        let n1 = alt1.node();
        let n2 = alt2.node();
        if n1.range() < pipe.range() && pipe.range() < n2.range() {
            return Some((n1, n2));
        }
    };

    None
}

