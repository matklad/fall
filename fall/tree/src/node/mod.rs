use std::any::Any;

use {TextEdit, TextBuf, Text, TextRange, NodeType, Language, Metrics};

mod imp;
mod immutable;
mod tree_builder;

pub use self::imp::NodeChildren;
pub use self::immutable::INode;


pub struct File {
    imp: imp::FileImpl,
    incremental_data: Option<Box<Any + Sync + Send>>,
}

impl File {
    pub fn new<T: Into<TextBuf>>(
        lang: Language,
        text: T,
        metrics: Metrics,
        node: INode,
        incremental_data: Option<Box<Any + Sync + Send>>,
    ) -> File {
        File {
            imp: imp::new_file(lang, text.into(), metrics, &node),
            incremental_data,
        }
    }

    pub fn language(&self) -> &Language {
        &self.imp.lang
    }

    pub fn root(&self) -> Node {
        self.imp.root(self)
    }

    pub fn text(&self) -> Text {
        self.imp.text()
    }

    pub fn metrics(&self) -> &Metrics {
        self.imp.metrics()
    }

    pub fn edit(&self, edit: TextEdit) -> File {
        self.language().reparse(self, edit)
    }

    pub fn incremental_data(&self) -> Option<&(Any + Send + Sync)> {
        self.incremental_data.as_ref().map(|r| r.as_ref())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node<'f>(imp::NodeImpl<'f>);

impl<'f> ::std::fmt::Debug for Node<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.debug(f)
    }
}

impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.0.ty()
    }

    pub fn range(&self) -> TextRange {
        self.0.range()
    }

    pub fn text(&self) -> Text<'f> {
        self.0.text()
    }

    pub fn file(&self) -> &'f File { self.0.file() }

    pub fn parent(&self) -> Option<Node<'f>> {
        self.0.parent()
    }

    pub fn children(&self) -> NodeChildren<'f> {
        self.0.children()
    }
}
