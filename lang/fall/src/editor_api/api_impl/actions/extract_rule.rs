use fall_tree::{AstNode, AstClass, File, TextRange, FileEdit};
use fall_tree::search::{find_covering_node, ancestors};
use fall_tree::search::ast;
use ::{Expr, SynRule};
use super::ContextAction;


pub struct ExtractRule;

impl ContextAction for ExtractRule {
    fn id(&self) -> &'static str {
        "Extract Rule"
    }

    fn apply<'f>(&self, file: &'f File, range: TextRange) -> Option<FileEdit<'f>> {
        let expr = ancestors(find_covering_node(file.root(), range))
            .find(|n| {
                Expr::NODE_TYPES.iter().any(|&ty| ty == n.ty())
            });
        let expr = match expr {
            None => return None,
            Some(expr) => expr,
        };
        let rule = ast::ancestor_exn::<SynRule>(expr).node();

        let new_rule = format!("\n\nrule new_rule {{\n  {}\n}}", expr.text());
        let mut edit = FileEdit::new(file);
        edit.replace_with_text(expr, "new_rule".to_owned());
        edit.insert_text_after(rule, new_rule);

        Some(edit)
    }
}

#[cfg(test)]
mod tests {
    use fall_tree::test_util::report_diff;
    use ::test_util::parse_with_range;
    use ::editor_api::{context_actions, apply_context_action};


    #[test]
    fn test_extract_rules() {
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
}

