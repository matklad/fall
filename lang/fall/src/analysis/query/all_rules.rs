use std::collections::hash_map::{self, HashMap};
use std::sync::Arc;

use fall_tree::Text;
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use ::{SynRule};

impl<'f> db::OnceQExecutor<'f> for super::AllRules {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Arc<HashMap<Text<'f>, SynRule<'f>>> {
        let file = db.file();

        let mut result = HashMap::new();
        for rule in file.syn_rules() {
            if let Some(ident) = rule.name_ident() {
                match result.entry(ident.text()) {
                    hash_map::Entry::Vacant(vacant) => {
                        vacant.insert(rule);
                    }
                    hash_map::Entry::Occupied(..) => {
                        d.error(ident, "Duplicate rule");
                    }
                }
            }
        }

        Arc::new(result)
    }
}
