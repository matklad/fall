use std::sync::Arc;
use {TextEdit, File, NodeType, NodeTypeInfo, INode, Metrics};

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse(&self, text: String) -> File {
        let metrics = Metrics::new();
        let inode = self.imp.parse(&text, &metrics);
        File::new(self.clone(), text, metrics, inode)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());
        let metrics = Metrics::new();
        let inode = self.imp.parse(&new_text.as_slice().to_cow(), &metrics);
        let result = File::new(self.clone(), new_text, metrics, inode);
        return result;
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(&self, text: &str, metrics: &Metrics) -> INode;
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}
