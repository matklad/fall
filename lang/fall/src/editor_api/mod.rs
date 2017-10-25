use std::collections::HashSet;

use fall_tree::{File, AstNode, Node, dump_file, TextRange, TextUnit, TextEdit};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type, find_leaf_at_offset};
use fall_tree::search::ast;

use ::*;

mod formatter;
mod api_impl;

pub fn parse(text: String) -> File {
    lang_fall().parse(text)
}

pub fn tree_as_text(file: &File) -> String {
    dump_file(file)
}

pub fn highlight(file: &File) -> Vec<(TextRange, &'static str)> {
    api_impl::highlighting::highlight(file)
}

pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    api_impl::extend_selection::extend_selection(file, range)
}

pub fn context_actions(file: &File, offset: TextUnit) -> Vec<&'static str> {
    api_impl::actions::context_actions(file, offset)
}

pub fn apply_context_action(file: &File, offset: TextUnit, action_id: &str) -> TextEdit {
    api_impl::actions::apply_context_action(file, offset, action_id)
}

#[derive(Serialize, Debug)]
pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

pub fn structure(file: &File) -> Vec<FileStructureNode> {
    api_impl::structure::structure(file)
}

pub enum Severity {
    Error,
    Warning
}

pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub text: String
}

impl Diagnostic {
    fn error(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            text: message,
        }
    }

    fn warning(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Warning,
            text: message,
        }
    }
}

pub fn diagnostics(file: &File) -> Vec<Diagnostic> {
    let used_rules: HashSet<Node> = ast::descendants_of_type::<RefExpr>(file.root())
        .into_iter()
        .filter_map(|node| node.resolve())
        .filter_map(|r| match r {
            RefKind::RuleReference(rule) => Some(rule.node()),
            _ => None
        })
        .chain(
            ast::descendants_of_type::<CallExpr>(file.root())
                .into_iter()
                .filter_map(|call| call.kind().ok())
                .filter_map(|kind| match kind {
                    CallKind::RuleCall(rule, ..) => Some(rule.node()),
                    _ => None,
                })
        )
        .chain(child_of_type(file.root(), SYN_RULE).into_iter())
        .collect();

    Visitor(Vec::new())
        .visit::<RefExpr, _>(|acc, ref_| {
            if ref_.resolve().is_none() {
                if let Some(call) = ast::parent::<CallExpr>(ref_.node()) {
                    if call.resolve_context().is_some() {
                        return;
                    }
                }

                acc.push(Diagnostic::error(ref_.node(), "Unresolved reference".to_string()))
            }
        })
        .visit::<CallExpr, _>(|acc, call| {
            match call.kind() {
                Err(e) => acc.push(Diagnostic::error(call.node(), e.to_string())),
                _ => {}
            }
        })
        .visit::<SynRule, _>(|acc, rule| {
            if !used_rules.contains(&rule.node()) {
                acc.push(Diagnostic::warning(rule.node(), "Unused rule".to_string()))
            }
        })
        .walk_recursively_children_first(file.root())
}

pub fn resolve_reference(file: &File, offset: TextUnit) -> Option<TextRange> {
    return api_impl::references::resolve_reference(file, offset);
}

pub fn find_usages(file: &File, offset: TextUnit) -> Vec<TextRange> {
    return api_impl::references::find_usages(file, offset);
}

pub fn reformat(file: &File) -> TextEdit {
    self::formatter::reformat_file(file, self::formatter::FALL_SPACING, WHITESPACE)
}

pub fn find_test_at_offset(file: &File, offset: TextUnit) -> Option<usize> {
    let test = find_leaf_at_offset(file.root(), offset)
        .right_biased()
        .and_then(|node| ast::parent::<TestDef>(node));

    if let Some(test) = test {
        Some(FallFile::new(file.root()).tests().position(|t| t.node() == test.node()).unwrap())
    } else {
        None
    }
}


#[test]
fn test_find_refs() {
    let file = parse(r#####"
tokenizer {
  #[skip] whitespace r"\s+"

  number r"\d+"
  plus '+'
  minus '-'
  star '*'
  slash '/'
  bang '!'
  lparen '('
  rparen ')'
}

pub rule file { expr }

#[pratt]
rule expr {
  sum_expr | product_expr
  | factorial_expr
  | negate_expr
  | constant_expr | paren_expr
}

#[bin(2)]
pub rule product_expr { expr {'*' | '/'} expr }

#[bin(1)]
pub rule sum_expr { expr {'+' | '-'} expr }

#[atom]
pub rule constant_expr { number }

#[atom]
pub rule paren_expr { '(' expr ')' }

#[postfix]
pub rule factorial_expr { expr '!' }

#[prefix]
pub rule negate_expr { '-' expr }

test r"
  1 + --1! - -2!
"
"#####);
    let usages = editor_api::find_usages(
        &file,
        TextUnit::from_usize(309)
    );

    assert_eq!(usages, vec![TextRange::from_len(
        TextUnit::from_usize(202),
        TextUnit::from_usize(12)
    )]);
}
