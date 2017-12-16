use std::sync::Arc;
use std::collections::btree_map::{self, BTreeMap};

use fall_tree::{Text, AstNode};
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use syntax::CallExpr;

impl<'f> db::OnceQExecutor<'f> for super::AllContexts {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Arc<Vec<Text<'f>>> {
        let result = process_subtree_bottom_up(
            db.file().node(),
            visitor(BTreeMap::<Text<'f>, Option<CallExpr<'f>>>::new())
                .visit::<CallExpr, _>(|contexts, call| {
                    if let Some(ctx) = call.context_name() {
                        match contexts.entry(ctx) {
                            btree_map::Entry::Occupied(mut occupied) => {
                                occupied.insert(None);
                            }
                            btree_map::Entry::Vacant(vacant) => {
                                vacant.insert(Some(call));
                            }
                        }
                    }
                })
        );

        for (k, v) in result.iter() {
            if let &Some(call) = v {
                d.warning(call.node(), format!("Context `{}` is used only once", k))
            };
        }

        Arc::new(result.into_iter().map(|(k, _)| k).collect())
    }
}
