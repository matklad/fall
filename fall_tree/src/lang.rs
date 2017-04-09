use std::sync::Arc;
use {File};

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse(&self, text: String) -> File {
        self.imp.parse(self.clone(), text)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(&self, this: Language, text: String) -> File;
}
