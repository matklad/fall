use std::sync::Arc;
use itertools::Itertools;

use fall_tree::{AstNode, AstClass, Text};
use fall_tree::search::{child_of_type, child_of_type_exn};
use fall_tree::visitor::{Visitor, NodeVisitor};

use super::{Analysis, DiagnosticSink};
use ::{Expr, CallExpr, SynRule, IDENT, SIMPLE_STRING, RefKind};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CallKind<'f> {
    Eof,
    Any,
    Commit,

    Not(Expr<'f>),
    Layer(Expr<'f>, Expr<'f>),
    WithSkip(Expr<'f>, Expr<'f>),

    Enter(u32, Expr<'f>),
    Exit(u32, Expr<'f>),
    IsIn(u32),

    RuleCall(SynRule<'f>, Arc<Vec<(u32, Expr<'f>)>>),
    PrevIs(Arc<Vec<usize>>)
}


pub (super) fn resolve<'f>(a: &Analysis<'f>, d: &mut DiagnosticSink, call: CallExpr<'f>) -> Option<CallKind<'f>> {
    let n_args = call.args().count();
    let expect_args = |d: &mut DiagnosticSink, n_expected: usize| {
        if n_expected != n_args {
            d.error(
                call.node(),
                format!("Wrong number of arguments, expected {}, got {}", n_expected, n_args)
            )
        }
    };

    let zero = |d: &mut DiagnosticSink, kind: CallKind<'f>| {
        expect_args(d, 0);
        Some(kind)
    };
    let mut any = |d: &mut DiagnosticSink| zero(d, CallKind::Any);
    let mut commit = |d: &mut DiagnosticSink| zero(d, CallKind::Commit);
    let mut eof = |d: &mut DiagnosticSink| zero(d, CallKind::Eof);

    let one = |d: &mut DiagnosticSink, kind: &mut FnMut(&mut DiagnosticSink, Expr<'f>) -> Option<CallKind<'f>>| {
        expect_args(d, 1);
        call.args().next().and_then(|arg| kind(d, arg))
    };
    let mut not = |d: &mut DiagnosticSink| one(d, &mut |_, arg| Some(CallKind::Not(arg)));
    let mut is_in = |d: &mut DiagnosticSink| one(d, &mut |d, arg| resolve_context(a, d, arg).map(CallKind::IsIn));

    let two = |d: &mut DiagnosticSink, kind: &mut FnMut(&mut DiagnosticSink, Expr<'f>, Expr<'f>) -> Option<CallKind<'f>>| {
        expect_args(d, 2);
        call.args().next_tuple().and_then(|(arg1, arg2)| kind(d, arg1, arg2))
    };
    let mut layer = |d: &mut DiagnosticSink| two(d, &mut |_, arg1, arg2| {
        Some(CallKind::Layer(arg1, arg2))
    });
    let mut with_skip = |d: &mut DiagnosticSink| two(d, &mut |_, arg1, arg2| {
        Some(CallKind::WithSkip(arg1, arg2))
    });
    let mut enter = |d: &mut DiagnosticSink| two(d, &mut |d, arg1, arg2| {
        resolve_context(a, d, arg1).map(|ctx| CallKind::Enter(ctx, arg2))
    });
    let mut exit = |d: &mut DiagnosticSink| two(d, &mut |d, arg1, arg2| {
        resolve_context(a, d, arg1).map(|ctx| CallKind::Exit(ctx, arg2))
    });

    let mut prev_is = |d: &mut DiagnosticSink| {
        let mut args = Vec::new();
        for arg in call.args() {
            let mut ok = false;
            if let Expr::RefExpr(expr) = arg {
                if let Some(RefKind::RuleReference(rule)) = expr.resolve() {
                    if let Some(ty) = rule.resolve_ty() {
                        args.push(ty);
                        ok = true;
                    }
                }
            }
            if !ok {
                d.error(call.node(), "<prev_is> arguments must be public rules")
            }
        }
        Some(CallKind::PrevIs(Arc::new(args)))
    };

    let build_in: Vec<(&str, &mut FnMut(&mut DiagnosticSink) -> Option<CallKind<'f>>)> = vec![
        ("any", &mut any),
        ("commit", &mut commit),
        ("eof", &mut eof),
        ("not", &mut not),
        ("is_in", &mut is_in),
        ("layer", &mut layer),
        ("with_skip", &mut with_skip),
        ("enter", &mut enter),
        ("exit", &mut exit),
        ("prev_is", &mut prev_is)
    ];

    for (name, kind) in build_in.into_iter() {
        if call.fn_name() == name {
            return kind(d);
        }
    }

    if let Some(rule) = a.file.resolve_rule(call.fn_name()) {
        if let Some(parameters) = rule.parameters() {
            let n_expected = parameters.parameters().count();
            if n_expected != n_args {
                d.error(
                    call.node(),
                    format!("Expected {} arguments, got {}", n_expected, n_args)
                )
            }

            let params = parameters.parameters()
                .map(|p| p.idx())
                .zip(call.args())
                .collect();
            return Some(CallKind::RuleCall(rule, Arc::new(params)));
        }
    }

    d.error(
        child_of_type_exn(call.node(), IDENT),
        "Unresolved reference"
    );
    return None;
}

pub fn contexts<'f>(a: &Analysis<'f>) -> Vec<Text<'f>> {
    Visitor(Vec::new())
        .visit::<CallExpr, _>(|contexts, call| {
            if call.fn_name() == "is_in" || call.fn_name() == "enter" || call.fn_name() == "exit" {
                if let Some(ctx) = call.args().next().and_then(context_name) {
                    contexts.push(ctx);
                }
            }
        })
        .walk_recursively_children_first(a.file().node())
}

fn context_name<'f>(ctx: Expr<'f>) -> Option<Text<'f>> {
    child_of_type(ctx.node(), SIMPLE_STRING)
        .map(|n| ::ast_ext::lit_body(n.text()))
}

fn resolve_context(a: &Analysis, d: &mut DiagnosticSink, ctx: Expr) -> Option<u32> {
    if let Some(name) = context_name(ctx) {
        a.contexts().iter()
            .position(|&c| c == name)
            .map(|usize_| usize_ as u32)
    } else {
        d.error(ctx.node(), "Context should be a single quoted string");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fall_tree::search;
    use fall_tree::search::ast;

    use ::test_util::parse_with_caret;

    #[test]
    fn test_resolve_no_args() {
        check_resolved("rule foo { <^eof> }", "Eof");
        check_resolved("rule foo { <^any> }", "Any");
        check_resolved("rule foo { <^commit> }", "Commit");
    }

    #[test]
    fn test_resolve_simple_args() {
        check_resolved("rule foo { <^not a> }", "Not(RefExpr@[16; 17))");
        check_resolved("rule foo { <^with_skip a b> }", "WithSkip(RefExpr@[22; 23), RefExpr@[24; 25))");
        check_resolved("rule foo { <^layer a b> }", "Layer(RefExpr@[18; 19), RefExpr@[20; 21))");
    }

    #[test]
    fn test_resolve_layer() {
        check_resolved("rule foo { <^enter 'ctx' a> }", "Enter(0, RefExpr@[24; 25))");
        check_resolved("rule foo { <^exit  'ctx' a> }", "Exit(0, RefExpr@[24; 25))");

        check_resolved("rule foo { <is_in 'ctx1'> <^enter 'ctx2' a> }", "Enter(1, RefExpr@[40; 41))");
        check_resolved("rule foo { <is_in 'ctx1'> <^exit  'ctx2' a> }", "Exit(1, RefExpr@[40; 41))");

        check_resolved(
            "rule foo { <enter 'ctx1' a> <enter 'ctx2' b> <^is_in 'ctx2'> }",
            "IsIn(1)"
        );

        check(
            r" rule foo { <^is_in foo>}",
            None,
            r#"[(foo, "Context should be a single quoted string")]"#,
        )
    }

    #[test]
    fn test_not() {
        resolve_call("rule foo { <^not bar>}", |c, _| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), "Not(RefExpr@[16; 19))");
        })
    }

    #[test]
    fn test_unresolved_call() {
        check(
            r"rule foo { <^abracadabra>}",
            None,
            r#"[(abracadabra, "Unresolved reference")]"#
        );
    }

    #[test]
    fn wrong_arity() {
        check(
            "rule foo { <^eof foo> }",
            Some(CallKind::Eof),
            r#"[(<eof foo>, "Wrong number of arguments, expected 0, got 1")]"#
        );

        check(
            "rule foo { <^any foo bar> }",
            Some(CallKind::Any),
            r#"[(<any foo bar>, "Wrong number of arguments, expected 0, got 2")]"#
        );

        check(
            "rule foo { <^commit foo bar baz> }",
            Some(CallKind::Commit),
            r#"[(<commit foo bar baz>, "Wrong number of arguments, expected 0, got 3")]"#
        );

        check(
            "rule foo { <^not> }",
            None,
            r#"[(<not>, "Wrong number of arguments, expected 1, got 0")]"#
        );
    }

    #[test]
    fn check_prev_is() {
        check_resolved(
            "pub rule foo {} pub rule bar {} rule baz { <^prev_is foo bar> }",
            "PrevIs([1, 2])"
        )
    }

    #[test]
    fn call_custom_rule() {
        check_resolved(
            "rule foo { <^bar a b>} rule bar(x, y) { }",
            "RuleCall(SynRule@[22; 40), [(0, RefExpr@[16; 17)), (1, RefExpr@[18; 19))])"
        );
        ::analysis::check_diagnostics(
            "rule foo { <bar <eof>>} rule bar(x, y) { }",
            "<bar <eof>>: E Expected 2 arguments, got 1"
        )
    }

    fn check(text: &str, kind: Option<CallKind>, diagnostics: &str) {
        resolve_call(text, |c, a| {
            assert_eq!(c, kind);
            assert_eq!(a.debug_diagnostics(), diagnostics);
        })
    }

    fn check_resolved(text: &str, kind: &str) {
        resolve_call(text, |c, a| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), kind);
            assert_eq!(a.debug_diagnostics(), "[]");
        })
    }

    fn resolve_call<F: FnOnce(Option<CallKind>, Analysis)>(text: &str, f: F) {
        let (file, caret) = parse_with_caret(&text);
        let call = {
            let leaf = search::find_leaf_at_offset(file.root(), caret)
                .left_biased().unwrap();
            ast::ancestor_exn(leaf)
        };
        let a = Analysis::new(::ast(&file));
        let c = a.resolve_call(call);
        f(c, a)
    }
}

