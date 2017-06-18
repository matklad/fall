use std::sync::Arc;
use {File, NodeType, NodeTypeInfo, FileStats, INode, TextRange, TextUnit, Edit};

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse(&self, text: String) -> File {
        let (stats, inode) = self.imp.parse(&text);
        File::new(self.clone(), text, stats, inode)
    }

    pub fn reparse(&self, file: &File, edit: &Edit) -> File {
        let before = file.text().slice(TextRange::from_to(TextUnit::zero(), edit.delete.start()));
        let after = file.text().slice(TextRange::from_to(edit.delete.end(), file.text().len()));
        let new_text = before.to_string() + &edit.insert + &after.to_string();
        let (stats, inode) = self.imp.parse(&new_text);
        let result = File::new(self.clone(), new_text, stats, inode);
        return result;
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(&self, text: &str) -> (FileStats, INode);
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}
