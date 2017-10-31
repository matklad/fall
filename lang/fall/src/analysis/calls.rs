use itertools::Itertools;

use fall_tree::AstNode;
use fall_tree::search::child_of_type_exn;

use super::Analysis;
use ::{Expr, CallExpr, IDENT};

#[derive(Debug, Eq, PartialEq)]
pub enum CallKind<'f> {
    Eof,
    Any,
    Commit,
    Not(Expr<'f>),
    Layer(Expr<'f>, Expr<'f>),
    WithSkip(Expr<'f>, Expr<'f>),
    //    Enter(u32, Expr<'f>),
    //    Exit(u32, Expr<'f>),
    //    IsIn(u32),
}


pub fn resolve<'f>(a: &Analysis<'f>, call: CallExpr<'f>) -> Option<CallKind<'f>> {
    let n_args = call.args().count();
    let expect_args = |n_expected: usize| {
        if n_expected != n_args {
            a.diagnostics.error(
                call.node(),
                format!("Wrong number of arguments, expected {}, got {}", n_expected, n_args)
            )
        }
    };

    let zero_arg = vec![
        ("any", CallKind::Any),
        ("commit", CallKind::Commit),
        ("eof", CallKind::Eof)
    ];

    for (name, kind) in zero_arg.into_iter() {
        if call.fn_name() == name {
            expect_args(0);
            return Some(kind);
        }
    }

    if call.fn_name() == "not" {
        expect_args(1);
        return call.args().next().map(CallKind::Not);
    }

    let two_arg = vec![
        ("layer", CallKind::Layer as fn(_, _) -> _),
        ("with_skip", CallKind::WithSkip)
    ];

    for (name, kind) in two_arg.into_iter() {
        if call.fn_name() == name {
            expect_args(2);
            return call.args().next_tuple()
                .map(|(a, b)| kind(a, b))
        }
    }

    a.diagnostics.error(
        child_of_type_exn(call.node(), IDENT),
        "Unresolved reference"
    );
    return None;
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
    fn test_not() {
        resolve_call("rule foo { <^not bar>}", |c, _| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), "Not(RefExpr@[16; 19))");
        })
    }

    #[test]
    fn test_unresolved_call() {
        check(
            r"rule foo {  <^abracadabra>}",
            None,
            r#"[(abracadabra, "Unresolved reference")]"#
        );
    }

    #[test]
    fn wrong_arity() {
        check(
            r" rule foo { <^eof foo> }",
            Some(CallKind::Eof),
            r#"[(<eof foo>, "Wrong number of arguments, expected 0, got 1")]"#
        );

        check(
            r" rule foo { <^any foo bar> }",
            Some(CallKind::Any),
            r#"[(<any foo bar>, "Wrong number of arguments, expected 0, got 2")]"#
        );

        check(
            r" rule foo { <^commit foo bar baz> }",
            Some(CallKind::Commit),
            r#"[(<commit foo bar baz>, "Wrong number of arguments, expected 0, got 3")]"#
        );

        check(
            r" rule foo { <^not> }",
            None,
            r#"[(<not>, "Wrong number of arguments, expected 1, got 0")]"#
        );
    }

    fn check(text: &str, kind: Option<CallKind>, diagnostics: &str) {
        resolve_call(text, |c, a| {
            assert_eq!(c, kind);
            assert_eq!(a.diagnostics.debug(a.file.node().text()), diagnostics);
        })
    }

    fn check_resolved(text: &str, kind: &str) {
        resolve_call(text, |c, a| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), kind);
            assert_eq!(a.diagnostics.debug(a.file.node().text()), "[]");
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

