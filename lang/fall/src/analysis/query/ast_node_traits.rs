use std::collections::HashMap;
use std::sync::Arc;

use fall_tree::{Text, AstNode};
use crate::analysis::diagnostics::DiagnosticSink;
use crate::analysis::db::{self, DB};
use crate::syntax::{AstTraitDef, COLON, IDENT};

impl<'f> db::OnceQExecutor<'f> for super::AstNodeTraits<'f> {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Arc<Vec<AstTraitDef<'f>>> {
        let traits: HashMap<Text, AstTraitDef> = db.file().ast_def().map(|ast| {
            ast.ast_traits().map(|trait_| (trait_.name(), trait_)).collect()
        }).unwrap_or_default();

        let refs = self.0.node().children()
            .skip_while(|node| node.ty() != COLON)
            .filter(|node| node.ty() == IDENT);

        let mut result = Vec::new();
        for r in refs {
            match traits.get(&r.text()) {
                Some(&trait_) => result.push(trait_),
                None => d.error(r, "Unresolved trait"),
            }
        }

        Arc::new(result)
    }
}
