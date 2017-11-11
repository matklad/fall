use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use analysis::query;
use fall_tree::search::ast;
use fall_tree::AstNode;

use ::{SynRule, CallExpr, RefKind};


impl<'f> db::OnceQExecutor<'f> for super::ResolveRefExpr<'f> {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Option<RefKind<'f>> {
        let ref_ = self.0;
        let reference_name = ref_.reference_name();

        if let Some(param) = ast::ancestor_exn::<SynRule>(ref_.node())
            .parameters()
            .and_then(|params| params.parameters().find(|p| p.name() == reference_name)) {
            return Some(RefKind::Param(param));
        }

        if let Some(syn_rule) = db.get(query::FindSynRule(reference_name)) {
            return Some(RefKind::RuleReference(syn_rule));
        }

        if let Some(lex_rule) = db.get(query::FindLexRule(reference_name)) {
            return Some(RefKind::Token(lex_rule));
        }
        let parent = ref_.node().parent().unwrap();
        if let Some(call) = CallExpr::wrap(parent) {
            if call.context_name().is_some() && call.args().next().map(|a| a.node()) == Some(ref_.node()) {
                return None;
            }
        }

        d.error(ref_.node(), "Unresolved reference");
        None
    }
}
