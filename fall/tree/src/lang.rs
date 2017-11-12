use std::sync::Arc;
use {Text, TextBuf, TextEdit, File, NodeType, NodeTypeInfo, IToken, INode, Metrics};

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
        let tokens: Vec<IToken> = metrics.measure_time("lexing", || {
            self.imp.tokenize(text.as_slice()).collect()
        });
        let inode = self.imp.parse(text.as_slice(), &tokens, &metrics);
        File::new(self.clone(), text, metrics, inode)
    }

    pub fn reparse(&self, file: &File, edit: TextEdit) -> File {
        let new_text = edit.apply(file.text());
        self.parse(new_text)
    }

    pub fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        self.imp.node_type_info(ty)
    }
}

pub trait LanguageImpl: 'static + Send + Sync {
    fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't>;
    fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> INode;
    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo;
}
