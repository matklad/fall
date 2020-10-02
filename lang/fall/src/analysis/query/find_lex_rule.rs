use crate::analysis::db::{self, DB};
use crate::analysis::query;
use crate::syntax::{LexRule};


impl<'f> db::QExecutor<'f> for super::FindLexRule<'f> {
    fn execute(self, db: &DB<'f>) -> Option<LexRule<'f>> {
        let all = db.get(query::AllLexRules);
        all.get(&self.0).map(|&r| r)
    }
}
