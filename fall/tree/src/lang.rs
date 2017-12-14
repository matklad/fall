use std::any::Any;
use std::sync::Arc;
use {Text, TextBuf, TextEdit, File, NodeType, NodeTypeInfo, Metrics, TreeBuilder};

pub trait LanguageImpl: 'static + Send + Sync {
    fn parse2(
        &self,
        text: Text,
        metrics: &Metrics,
        builder: &mut TreeBuilder,
    ) -> Option<Box<Any + Sync + Send>>;

    fn reparse2(
        &self,
        incremental_data: &Any,
        edit: &TextEdit,
        new_text: Text,
        metrics: &Metrics,
        builder: &mut TreeBuilder,
    ) -> Option<Box<Any + Sync + Send>>;

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
        let mut builder = TreeBuilder::new();
        let incremental = self.imp.parse2(text.as_slice(), &metrics, &mut builder);
        File::new2(self.clone(), text, metrics, incremental, builder)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());
        let metrics = Metrics::new();
        let mut builder = TreeBuilder::new();
        let incremental = if let Some(incremental) = file.incremental_data() {
            self.imp.reparse2(incremental, &edit, new_text.as_slice(), &metrics, &mut builder)
        } else {
            self.imp.parse2(new_text.as_slice(), &metrics, &mut builder)
        };
        File::new2(self.clone(), new_text, metrics, incremental, builder)
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}
