use std::any::Any;
use std::sync::Arc;
use {Text, TextBuf, TextSuffix, TextEdit, TextEditOp, File, NodeType, NodeTypeInfo, IToken, INode, Metrics, tu};

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse(
        &self,
        text: Text,
        metrics: &Metrics
    ) -> (Option<Box<Any + Sync + Send>>, INode);

    fn reparse(
        &self,
        incremental_data: &Any,
        edit: &TextEdit,
        new_text: Text,
        metrics: &Metrics
    ) -> (Option<Box<Any + Sync + Send>>, INode);

    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}

#[derive(Clone)]
pub struct Language {
    imp: Arc<LanguageImpl>
}

impl Language {
    pub fn new<I: LanguageImpl>(imp: I) -> Language {
        Language { imp: Arc::new(imp) }
    }

    pub fn parse<T: Into<TextBuf>>(&self, text: T) -> File {
        let text: TextBuf = text.into();
        let metrics = Metrics::new();
        let (incremental, inode) = self.imp.parse(text.as_slice(), &metrics);
        File::new(self.clone(), text, metrics, inode, incremental)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());
        let metrics = Metrics::new();
        let (incremental, inode) =  if let Some(incremental) = file.incremental_data() {
            self.imp.reparse(incremental, &edit, new_text.as_slice(), &metrics)
        } else {
            self.imp.parse(new_text.as_slice(), &metrics)
        };
        File::new(self.clone(), new_text, metrics, inode, incremental)
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}
