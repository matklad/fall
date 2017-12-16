use super::{MethodKind, ChildKind, Arity};
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use syntax::{AstSelector, FallFile};

impl<'f> db::OnceQExecutor<'f> for super::ResolveMethod<'f> {
    fn execute(self, db: &DB<'f>, _: &mut DiagnosticSink) -> Option<MethodKind<'f>> {
        let method = self.0;
        let child_kind = child_kind(db.file(), method.selector())?;

        let arity = if method.selector().optional().is_some() {
            Arity::Optional
        } else if method.selector().many().is_some() {
            Arity::Many
        } else {
            Arity::Single
        };

        if method.selector().dot().is_some() {
            return match child_kind {
                ChildKind::Token(t) => Some(MethodKind::TextAccessor(t, arity)),
                _ => None
            };
        }

        Some(MethodKind::NodeAccessor(child_kind, arity))
    }
}

fn child_kind<'f>(file: FallFile<'f>, selector: AstSelector<'f>) -> Option<ChildKind<'f>> {
    let ast_def = file.ast_def()?;
    if let Some(ast) = ast_def.ast_nodes().find(|a| a.name() == selector.child()) {
        return Some(ChildKind::AstNode(ast));
    }
    if let Some(class) = ast_def.ast_classes().find(|c| c.name() == selector.child()) {
        return Some(ChildKind::AstClass(class));
    }
    if let Some(lex_rule) = file.tokenizer_def().and_then(|td| td.lex_rules().find(|r| r.node_type() == selector.child())) {
        return Some(ChildKind::Token(lex_rule));
    }
    None
}


#[cfg(test)]
mod tests {}
