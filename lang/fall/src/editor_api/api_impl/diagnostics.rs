use fall_tree::Node;
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::ast;

use editor_api::{Diagnostic, Severity};
use ::*;

pub fn diagnostics<'f>(analysis: &Analysis<'f>) -> Vec<Diagnostic> {
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
        .walk_recursively_children_first(analysis.file().node())
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


#[test]
fn test_diagnostics() {
    let file = ::editor_api::analyse(r"
       rule foo { <eof 1> }
       rule bar { foo <abracadabra> <prev_is {foo}>}
    ".to_string());

    file.analyse(|a| {
        let d = diagnostics(a);
        assert_eq!(
            format!("{:?}", d),
            "[Diagnostic { range: [51; 64), severity: Error, message: \"unknown rule\" }, \
              Diagnostic { range: [65; 80), severity: Error, message: \"Bad prev_is\" }, \
              Diagnostic { range: [41; 44), severity: Warning, message: \"Unused rule\" }]"
        );
    })
}
