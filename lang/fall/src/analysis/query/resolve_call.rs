use std::sync::Arc;

use itertools::Itertools;

use fall_tree::search::child_of_type_exn;
use fall_tree::{AstNode, AstClass};

use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use analysis::query;

use ::{CallExpr, RefKind, CallKind, Expr, IDENT};


impl<'f> db::OnceQExecutor<'f> for super::ResolveCall<'f> {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Option<CallKind<'f>> {
        let call = self.0;
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
        let mut is_in = |d: &mut DiagnosticSink| one(d, &mut |d, _| resolve_context(db, d, call).map(CallKind::IsIn));

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
        let mut enter = |d: &mut DiagnosticSink| two(d, &mut |d, _, arg2| {
            resolve_context(db, d, call).map(|ctx| CallKind::Enter(ctx, arg2))
        });
        let mut exit = |d: &mut DiagnosticSink| two(d, &mut |d, _, arg2| {
            resolve_context(db, d, call).map(|ctx| CallKind::Exit(ctx, arg2))
        });

        let mut prev_is = |d: &mut DiagnosticSink| {
            let mut args = Vec::new();
            for arg in call.args() {
                let mut ok = false;
                if let Expr::RefExpr(expr) = arg {
                    if let Some(RefKind::RuleReference(rule)) = db.get(query::ResolveRefExpr(expr)) {
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

        if let Some(rule) = db.get(query::FindSynRule(call.fn_name())) {
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
        None
    }
}

fn resolve_context(db: &DB, d: &mut DiagnosticSink, call: CallExpr) -> Option<u32> {
    if let Some(name) = call.context_name() {
        db.get(query::AllContexts)
            .iter()
            .position(|&c| c == name)
            .map(|usize_| usize_ as u32)
    } else {
        d.error(call.args().next().unwrap().node(), "Context should be a single quoted string");
        None
    }
}

