use fall_editor::actions::ActionResult;
use fall_tree::{AstNode, File, Node, TextUnit, FileEdit};
use fall_tree::search::{find_leaf_at_offset, LeafAtOffset};
use syntax::{PIPE, BlockExpr};

pub fn swap_alternatives(file: &File, offset: TextUnit, apply: bool) -> Option<ActionResult> {
    let (left, right) = find_swappable_nodes(file, offset)?;
    if !apply {
        return Some(ActionResult::Available);
    }

    let mut edit = FileEdit::new(file);
    edit.replace(left, right);
    edit.replace(right, left);
    Some(ActionResult::Applied(edit.into_text_edit()))
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
    println!("B");

    let parent = pipe.parent().and_then(BlockExpr::wrap)?;
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
    use fall_editor::actions::check_context_action;

    #[test]
    fn test_swap_alternatives() {
        check_context_action::<::FileWithAnalysis>("Swap alternatives", r##"
tokenizer { number r"\d+"}
pub rule foo { bar ^^| baz }
"##, r##"
tokenizer { number r"\d+"}
pub rule foo { baz | bar }
"##);
    }
}