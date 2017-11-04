use std::sync::Arc;
use std::collections::hash_map::{self, HashMap};

use fall_tree::{Text, AstNode};
use fall_tree::visitor::{Visitor, NodeVisitor};
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use ::CallExpr;

impl<'f> db::OnceQExecutor<'f> for super::AllContexts<'f> {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Arc<Vec<Text<'f>>> {
        let result = Visitor(HashMap::<Text<'f>, Option<CallExpr<'f>>>::new())
            .visit::<CallExpr, _>(|contexts, call| {
                if let Some(ctx) = call.context_name() {
                    match contexts.entry(ctx) {
                        hash_map::Entry::Occupied(mut occupied) => {
                            occupied.insert(None);
                        },
                        hash_map::Entry::Vacant(vacant) => {
                            vacant.insert(Some(call));
                        },
                    }
                }
            })
            .walk_recursively_children_first(db.file().node());

        for (k, v) in result.iter() {
            if let &Some(call) = v {
                d.warning(call.node(), format!("Context `{}` is used only once", k))
            };
        }

        Arc::new(result.into_iter().map(|(k, _)| k).collect())
    }
}
