use fall_tree::{Node, NodeType, TextUnit, File, TextEdit, FileEdit, tu};
use fall_tree::search::{next_sibling, prev_sibling};
use ::{PIPE, L_ANGLE, R_ANGLE, WHITESPACE};

pub fn reformat(file: &File) -> TextEdit {
    reformat_file(file, FALL_SPACING, WHITESPACE)
}

const FALL_SPACING: &[Rule] = &[
    Rule::After(PIPE, Spaces::Single),
    Rule::Before(PIPE, Spaces::Single),
    Rule::After(L_ANGLE, Spaces::None),
    Rule::Before(R_ANGLE, Spaces::None),
];


#[derive(Clone, Copy)]
enum Rule {
    After(NodeType, Spaces),
    Before(NodeType, Spaces)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spaces {
    None,
    Single
}


fn reformat_file(file: &File, rules: &[Rule], ws_type: NodeType) -> TextEdit {
    let spacer = Spacer { rules, ws_type };
    let mut edit = FileEdit::new(file);
    reformat_node(file.root(), &mut edit, &spacer);
    edit.into_text_edit()
}


struct Spacer<'r> {
    rules: &'r [Rule],
    ws_type: NodeType,
}

impl<'r> Spacer<'r> {
    fn apply<'f>(&self, node: Node<'f>, edit: &mut FileEdit<'f>) {
        for &rule in self.rules {
            let (spaces, ws_node, before_ws_node) =
                match (rule, next_sibling(node), prev_sibling(node)) {
                    (Rule::After(rty, spaces), Some(next), _) if rty == node.ty() => {
                        (spaces, next, node)
                    }
                    (Rule::Before(rty, spaces), _, Some(prev)) if rty == node.ty() => {
                        (spaces, prev, prev)
                    }
                    _ => continue
                };

            if ws_node.text().contains("\n") {
                continue
            }

            match (spaces, ws_node.ty() == self.ws_type) {
                (Spaces::None, true) => edit.delete(ws_node),
                (Spaces::Single, false) => edit.insert_text_after(before_ws_node, " ".to_owned()),
                (Spaces::Single, true) if ws_node.text().len() != tu(1) => {
                    edit.replace_with_text(ws_node, " ".to_owned())
                }
                _ => continue
            }
        }
    }
}


fn reformat_node<'f>(node: Node<'f>, edit: &mut FileEdit<'f>, spacer: &Spacer) {
    spacer.apply(node, edit);
    for child in node.children() {
        reformat_node(child, edit, spacer);
    }
}


#[cfg(test)]
fn test_reformat(before: &str, after: &str) {
    let file = ::parse(before.trim());
    let edit = reformat_file(&file, FALL_SPACING, WHITESPACE);
    let actual = edit.apply(file.text());
    ::fall_tree::test_util::report_diff(after.trim(), &actual);
}

#[test]
fn test_adds_spaces_after_pipe() {
    test_reformat(r"
           rule foo { x|y }
       ", r"
           rule foo { x | y }
       ")
}

#[test]
fn test_dont_mess_newlines() {
    test_reformat(r"
           rule foo {
             x
           | y
           }
       ", r"
           rule foo {
             x
           | y
           }
       ")
}
