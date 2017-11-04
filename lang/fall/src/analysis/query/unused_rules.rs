use std::collections::HashSet;

use fall_tree::AstNode;
use fall_tree::search::ast;
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use analysis::query;
use ::{SynRule, RefKind, CallExpr, CallKind, RefExpr};

impl<'f> db::OnceQExecutor<'f> for super::UnusedRules {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) {
        let used: HashSet<SynRule> =
            ast::descendants_of_type::<RefExpr>(db.file().node())
                .into_iter()
                .filter_map(|ref_| db.get(query::ResolveRefExpr(ref_)))
                .filter_map(|r| match r {
                    RefKind::RuleReference(rule) => Some(rule),
                    _ => None
                })
                .chain(
                    ast::descendants_of_type::<CallExpr>(db.file().node())
                        .into_iter()
                        .filter_map(|call| db.get(query::ResolveCall(call)))
                        .filter_map(|kind| match kind {
                            CallKind::RuleCall(rule, ..) => Some(rule),
                            _ => None,
                        })
                )
                .chain(db.file().syn_rules().next().into_iter()) // First rule is always used
                .collect();

        for rule in db.file().syn_rules() {
            if !used.contains(&rule) {
                if let Some(ident) = rule.name_ident() {
                    d.warning(ident, "Unused rule")
                }
            };
        }
    }
}
