use fall_tree::{AstNode, AstClass, File, Node, TextUnit, TextRange, FileEdit, TextEdit};
use fall_tree::search::{find_leaf_at_offset, LeafAtOffset};
use ::{PIPE, BlockExpr};

pub fn context_actions(file: &File, range: TextRange) -> Vec<&'static str> {
    ACTIONS.iter()
        .filter(|action| action.apply(file, range).is_some())
        .map(|action| action.id())
        .collect()
}

pub fn apply_context_action(file: &File, range: TextRange, action_id: &str) -> TextEdit {
    let action = ACTIONS.iter().find(|action| action.id() == action_id).unwrap();
    action.apply(file, range).unwrap().into_text_edit()
}

const ACTIONS: &[&ContextAction] = &[
    &SwapAlternatives,
];

pub trait ContextAction {
    fn id(&self) -> &'static str;
    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>>;
}

struct SwapAlternatives;

impl ContextAction for SwapAlternatives {
    fn id(&self) -> &'static str { "Swap Alternatives" }

    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>> {
        let (left, right) = match find_swappable_nodes(file, range.start()) {
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
        LeafAtOffset::Single(n) => if n.ty() == PIPE { n } else { return None; },
        LeafAtOffset::Between(n1, n2) => if n1.ty() == PIPE {
            n1
        } else if n2.ty() == PIPE {
            n2
        } else {
            return None;
        },
    };

    let parent = match pipe.parent() {
        None => return None,
        Some(n) => if n.ty() == BlockExpr::NODE_TYPE {
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


#[test]
fn test_actions() {
    let file = ::parse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar | baz }
"####);
    let offset = TextUnit::from_usize(47);
    let actions = context_actions(&file, offset);
    assert_eq!(
        format!("{:?}", actions),
        r#"["Swap Alternatives"]"#
    );
    let edit = apply_context_action(&file, offset, "Swap Alternatives");
    assert_eq!(
        format!("{:?}", edit),
        r#"TextEdit { delete: [43; 52), insert: "baz | bar" }"#
    )
}
