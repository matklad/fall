use fall_tree::search::ast;
use fall_tree::{AstNode, AstClass};

use analysis::query;
use ::{SynRule, LexRule, Parameter, RefExpr, CallExpr};
use super::{Analysis, DiagnosticSink, CallKind};

#[derive(Copy, Clone)]
pub enum RefKind<'f> {
    Token(LexRule<'f>),
    RuleReference(SynRule<'f>),
    Param(Parameter<'f>),
}

pub (super) fn resolve<'f>(a: &Analysis<'f>, d: &mut DiagnosticSink, ref_: RefExpr<'f>) -> Option<RefKind<'f>> {
    let reference_name = ref_.reference_name();

    if let Some(param) = ast::ancestor_exn::<SynRule>(ref_.node())
        .parameters()
        .and_then(|params| params.parameters().find(|p| p.name() == reference_name)) {
        return Some(RefKind::Param(param));
    }

    if let Some(syn_rule) = a.db.get(query::FindSynRule(reference_name)) {
        return Some(RefKind::RuleReference(syn_rule));
    }

    if let Some(lex_rule) = a.db.get(query::FindLexRule(reference_name)) {
        return Some(RefKind::Token(lex_rule));
    }

    let parent = ref_.node().parent().unwrap();
    if parent.ty() == CallExpr::NODE_TYPE {
        let call = CallExpr::new(parent);
        match a.resolve_call(call) {
            Some(CallKind::Enter(..)) | Some(CallKind::Exit(..)) | Some(CallKind::IsIn(..))
            => if call.args().next().map(|a| a.node()) == Some(ref_.node()) {
                return None;
            },
            _ => ()
        }
    }

    d.error(ref_.node(), "Unresolved reference");
    return None;
}
