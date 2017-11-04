use fall_tree::{AstNode, AstClass, File, Node, TextUnit, TextRange, FileEdit};
use fall_tree::search::{find_leaf_at_offset, LeafAtOffset};
use ::{PIPE, BlockExpr};
use super::ContextAction;

pub struct SwapAlternatives;

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

#[cfg(test)]
mod tests {
    use super::*;
    use fall_tree::tu;
    use ::editor_api::{context_actions, apply_context_action};
    use ::test_util::parse_with_caret;

    #[test]
    fn test_swap_alternatives() {
        let (file, offset) = parse_with_caret(r####"
tokenizer { number r"\d+"}
pub rule foo { bar ^| baz }
"####);

        let position = TextRange::from_len(offset, tu(0));
        eprintln!("position = {:?}", position);
        eprintln!("file.text() = \n{}\n", file.text());
        let actions = context_actions(&file, position);
        assert_eq!(
            format!("{:?}", actions),
            r#"["Swap Alternatives"]"#
        );
        let edit = apply_context_action(&file, position, "Swap Alternatives");
        assert_eq!(
            format!("{:?}", edit),
            r#"TextEdit { delete: [43; 52), insert: "baz | bar" }"#
        )
    }
}




