use fall_tree::search::ast;
use fall_tree::{AstNode, AstClass};

use ::{SynRule, LexRule, Parameter, RefExpr, CallExpr};
use super::{Analysis, DiagnosticSink};

#[derive(Copy, Clone)]
pub enum RefKind<'f> {
    Token(LexRule<'f>),
    RuleReference(SynRule<'f>),
    Param(Parameter<'f>),
    Context(u32),
}

pub (super) fn resolve<'f>(a: &Analysis<'f>, d: &mut DiagnosticSink, ref_: RefExpr<'f>) -> Option<RefKind<'f>> {
    let reference_name = ref_.reference_name();

    if let Some(param) = ast::ancestor_exn::<SynRule>(ref_.node())
        .parameters()
        .and_then(|params| params.parameters().find(|p| p.name() == reference_name)) {
        return Some(RefKind::Param(param));
    }

    if let Some(syn_rule) = a.file().syn_rules().find(|r| r.name() == Some(reference_name)) {
        return Some(RefKind::RuleReference(syn_rule));
    }

    if let Some(lex_rule) = a.file().tokenizer_def()
        .and_then(|td| td.lex_rules().find(|r| r.token_name() == reference_name)) {
        return Some(RefKind::Token(lex_rule));
    }

    let parent = ref_.node().parent().unwrap();
    if parent.ty() == CallExpr::NODE_TYPE {
        let call = CallExpr::new(parent);
        if call.args().next().map(|a| a.node()) == Some(ref_.node())  {
            if let Some(ctx_name) = call.context_name() {
                let idx = a.contexts().iter().position(|&ctx| ctx == ctx_name).unwrap();
                return Some(RefKind::Context(idx as u32));
            }
        }
    }

    d.error(ref_.node(), "Unresolved reference");
    return None;
}
