use fall_tree::{Node, File};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::ast;

use editor_api::{Diagnostic, Severity};
use ::*;

pub fn diagnostics(file: &File) -> Vec<Diagnostic> {
    let analysis = Analysis::new(FallFile::new(file.root()));
    Visitor(Vec::new())
        .visit::<RefExpr, _>(|acc, ref_| {
            if ref_.resolve().is_none() {
                if let Some(call) = ast::ancestor::<CallExpr>(ref_.node()) {
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
            if analysis.is_unused(rule) {
                if let Some(rule_name) = rule.name_ident() {
                    acc.push(Diagnostic::warning(rule_name, "Unused rule".to_string()))
                }
            }
        })
        .walk_recursively_children_first(file.root())
}

impl Diagnostic {
    fn error(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            message,
        }
    }

    fn warning(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Warning,
            message,
        }
    }
}
