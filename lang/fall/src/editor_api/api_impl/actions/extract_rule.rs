use fall_tree::{AstNode, AstClass, File, Node, TextRange, FileEdit};
use fall_tree::search::{find_covering_node, ancestors};
use fall_tree::search::ast;
use ::{Expr, SynRule, SeqExpr};
use super::ContextAction;


pub struct ExtractRule;

impl ContextAction for ExtractRule {
    fn id(&self) -> &'static str {
        "Extract Rule"
    }

    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>> {
        let expr = ancestors(find_covering_node(file.root(), range))
            .find(|&n| is_expression(n));
        let expr = match expr {
            None => return None,
            Some(expr) => expr,
        };
        let rule = ast::ancestor_exn::<SynRule>(expr).node();
        let range = range_to_extract(expr, range);

        let new_rule = format!("\n\nrule new_rule {{\n  {}\n}}", file.text().slice(range));

        let mut edit = FileEdit::new(file);
        edit.replace_substring(expr, range, "new_rule".to_owned());
        edit.insert_text_after(rule, new_rule);

        Some(edit)
    }
}

fn is_expression(node: Node) -> bool {
    Expr::NODE_TYPES.iter().any(|&ty| ty == node.ty())
}

fn range_to_extract(parent: Node, range: TextRange) -> TextRange {
    if parent.ty() == SeqExpr::NODE_TYPE {
        let mut children =
            parent.children().filter(|n| n.range().intersects(range));
        let first = children.next();
        let last = children.last();
        if let (Some(first), Some(last)) = (first, last) {
            if is_expression(first) && is_expression(last) {
                return TextRange::from_to(first.range().start(), last.range().end());
            }
        }
    }

    parent.range()
}

#[cfg(test)]
mod tests {
    use fall_tree::test_util::report_diff;
    use ::test_util::parse_with_range;
    use ::editor_api::{context_actions, apply_context_action};


    #[test]
    fn test_extract_whole_seq() {
        let (file, range) = parse_with_range(r####"
tokenizer { number r"\d+"}
pub rule foo { ^bar baz^ }
"####);
        let actions = context_actions(&file, range);
        assert_eq!(
            format!("{:?}", actions),
            r#"["Extract Rule"]"#
        );
        let edit = apply_context_action(&file, range, "Extract Rule");
        let expected = r##"
tokenizer { number r"\d+"}
pub rule foo { new_rule }

rule new_rule {
  bar baz
}
"##;
        let actual = edit.apply(file.text());
        report_diff(expected.trim(), actual.trim())
    }

    #[test]
    fn test_extract_sub_seq() {
        let (file, range) = parse_with_range(r####"
tokenizer { number r"\d+"}
pub rule foo { foo ^bar baz^ quux }
"####);
        let actions = context_actions(&file, range);
        assert_eq!(
            format!("{:?}", actions),
            r#"["Extract Rule"]"#
        );
        let edit = apply_context_action(&file, range, "Extract Rule");
        let expected = r##"
tokenizer { number r"\d+"}
pub rule foo { foo new_rule quux }

rule new_rule {
  bar baz
}
"##;
        let actual = edit.apply(file.text());
        report_diff(expected.trim(), actual.trim())
    }
}
