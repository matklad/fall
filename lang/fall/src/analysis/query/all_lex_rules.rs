use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use fall_tree::{Text, AstNode};
use analysis::diagnostics::DiagnosticSink;
use analysis::db::{self, DB};
use syntax::LexRule;

impl<'f> db::OnceQExecutor<'f> for super::AllLexRules {
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Arc<HashMap<Text<'f>, LexRule<'f>>> {
        let file = db.file();

        let mut result = HashMap::new();
        let mut tys = HashSet::new();
        if let Some(tokenizer) = file.tokenizer_def() {
            for rule in tokenizer.lex_rules() {
                result.insert(rule.token_name(), rule);
                if !tys.insert(rule.node_type()) {
                    d.error(rule.node(), "Duplicate token");
                }
            }
        }

        Arc::new(result)
    }
}
