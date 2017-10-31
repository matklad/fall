use fall_tree::{AstNode};
use fall_tree::search::child_of_type_exn;
use super::Analysis;

use ::{CallExpr, IDENT};

#[derive(Debug, Eq, PartialEq)]
pub enum CallKind {
    Eof,
    Any,
    Commit,
//    Not(Expr<'f>),
}


pub fn resolve<'f>(a: &Analysis<'f>, call: CallExpr<'f>) -> Option<CallKind> {
    let n_args = call.args().count();
    let expect_args = |n_expected: usize| {
        if n_expected != n_args {
            a.diagnostics.error(
                call.node(),
                format!("Wrong number of arguments, expected {}, got {}", n_expected, n_args)
            )
        }
    };

    let simple = vec![
        ("any", CallKind::Any),
        ("commit", CallKind::Commit),
        ("eof", CallKind::Eof)
    ];

    for (name, kind) in simple.into_iter() {
        if call.fn_name() == name {
            expect_args(0);
            return Some(kind);
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
    fn test_call_resolve() {
        check_simple_call("eof", CallKind::Eof);
        check_simple_call("any", CallKind::Any);
        check_simple_call("commit", CallKind::Commit);
    }


    #[test]
    fn test_unresolved_call() {
        check(
            r" rule foo {  <^abracadabra>}",
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
    }

    fn check(text: &str, kind: Option<CallKind>, diagnostics: &str) {
        resolve_call(text, |c, a| {
            assert_eq!(c, kind);
            assert_eq!(a.diagnostics.debug(a.file.node().text()), diagnostics);
        })
    }

    fn check_simple_call(name: &str, kind: CallKind) {
        check(
            &format!("rule foo {{ <^{}> }}", name),
            Some(kind),
            "[]"
        );
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

