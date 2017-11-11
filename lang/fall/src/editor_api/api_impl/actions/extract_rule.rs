use fall_tree::{AstNode, File, Node, TextRange, FileEdit};
use fall_tree::search::{find_covering_node, ancestors};
use fall_tree::search::ast;
use ::{Expr, SynRule, SeqExpr, RefExpr};
use super::ContextAction;


pub struct ExtractRule;

impl ContextAction for ExtractRule {
    fn id(&self) -> &'static str {
        "Extract Rule"
    }

    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>> {
        if range.is_empty() {
            return None;
        }
        let expr = ancestors(find_covering_node(file.root(), range))
            .find(|&n| is_expression(n));
        let expr = match expr {
            None => return None,
            Some(expr) => expr,
        };
        if RefExpr::wrap(expr).is_some() {
            return None;
        }
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
    Expr::wrap(node).is_some()
}

fn range_to_extract(parent: Node, range: TextRange) -> TextRange {
    if SeqExpr::wrap(parent).is_some() {
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
    use super::super::check_context_action;


    #[test]
    fn test_extract_whole_seq() {
        check_context_action(r#"["Extract Rule"]"#, "Extract Rule", r##"
tokenizer { number r"\d+"}
pub rule foo { ^bar baz^ }
"##, r##"
tokenizer { number r"\d+"}
pub rule foo { new_rule }

rule new_rule {
  bar baz
}
"##);
    }

    #[test]
    fn test_extract_sub_seq() {
        check_context_action(r#"["Extract Rule"]"#, "Extract Rule", r##"
tokenizer { number r"\d+"}
pub rule foo { foo ^bar baz^ quux }
"##, r##"
tokenizer { number r"\d+"}
pub rule foo { foo new_rule quux }

rule new_rule {
  bar baz
}
"##);
    }
}

