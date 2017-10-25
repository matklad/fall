use std::collections::HashSet;

use fall_tree::{Node, File};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::ast;
use fall_tree::search::child_of_type;

use editor_api::Diagnostic;
use ::*;

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
