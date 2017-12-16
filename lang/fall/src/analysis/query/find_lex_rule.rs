use analysis::db::{self, DB};
use syntax::{LexRule};


impl<'f> db::QExecutor<'f> for super::FindLexRule<'f> {
    fn execute(self, db: &DB<'f>) -> Option<LexRule<'f>> {
        db.file().tokenizer_def()
            .and_then(|td| td.lex_rules().find(|r| r.token_name() == self.0))
    }
}
