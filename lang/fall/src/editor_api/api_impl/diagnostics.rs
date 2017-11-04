use fall_tree::Node;
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::ast;

use editor_api::{Diagnostic, Severity};
use ::*;

pub fn diagnostics<'f>(analysis: &Analysis<'f>) -> Vec<Diagnostic> {
    let mut result = Visitor(Vec::new())
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
        .visit::<CallExpr, _>(|_, call| { analysis.resolve_call(call); })
        .visit::<SynRule, _>(|acc, rule| {
            if analysis.is_unused(rule) {
                if let Some(rule_name) = rule.name_ident() {
                    acc.push(Diagnostic::warning(rule_name, "Unused rule".to_string()))
                }
            }
        })
        .walk_recursively_children_first(analysis.file().node());

    result.extend(analysis.diagnostics());
    result.sort_by_key(|d| (d.range.start(), d.range.end()));
    result
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
    use fall_tree::test_util::report_diff;
    let file = ::editor_api::analyse(r"
       pub rule foo { <eof x> }
       rule bar { foo <abracadabra>}
       rule baz { <prev_is foo> <prev_is bar> <prev_is {foo}>}
    ".to_string());

    file.analyse(|a| {
        let d = diagnostics(a);
        let expected = d.into_iter().map(|d| {
            let s = match d.severity {
                Severity::Error => 'E',
                Severity::Warning => 'W',
            };
            format!("{}: {} {}", a.file().node().text().slice(d.range), s, d.message)
        }).collect::<Vec<_>>().join("\n");

        report_diff(&expected, "\
<eof x>: E Wrong number of arguments, expected 0, got 1
x: E Unresolved reference
abracadabra: E Unresolved reference
baz: W Unused rule
<prev_is bar>: E <prev_is> arguments must be public rules
<prev_is {foo}>: E <prev_is> arguments must be public rules"
        );
    })
}
