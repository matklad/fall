use analysis::db::{self, DB};
use analysis::query;
use syntax::{SynRule};


impl<'f> db::QExecutor<'f> for super::FindSynRule<'f> {
    fn execute(self, db: &DB<'f>) -> Option<SynRule<'f>> {
        let all = db.get(query::AllSynRules);
        all.get(&self.0).map(|&r| r)
    }
}
