use std::sync::Arc;
use {File, NodeType, NodeTypeInfo};

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

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(&self, this: Language, text: String) -> File;

    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}
