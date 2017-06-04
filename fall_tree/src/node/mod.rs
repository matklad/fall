use std::time::Duration;
use {Text, TextRange, NodeType, Language};

mod imp;

pub use self::imp::NodeChildren;

pub struct File {
    lang: Language,
    imp: imp::FileImpl,
}

#[derive(Clone, Copy, Debug)]
pub struct FileStats {
    pub lexing_time: Duration,
    pub parsing_time: Duration,
    pub parsing_ticks: u64,
}

impl FileStats {
    pub fn new() -> FileStats  {
        FileStats {
            lexing_time: Default::default(),
            parsing_time: Default::default(),
            parsing_ticks: Default::default(),
        }
    }
}


impl File {
    pub fn language(&self) -> &Language {
        &self.lang
    }

    pub fn root(&self) -> Node {
        self.imp.root()
    }

    pub fn text(&self) -> Text {
        self.imp.text()
    }

    pub fn stats(&self) -> FileStats {
        self.imp.stats()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node<'f>(imp::NodeImpl<'f>);

impl<'f> ::std::fmt::Debug for Node<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.ty().fmt(f)
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

    pub fn parent(&self) -> Option<Node<'f>> {
        self.0.parent()
    }

    pub fn children(&self) -> NodeChildren<'f> {
        self.0.children()
    }
}

pub struct FileBuilder(imp::FileBuilderImpl);

#[derive(Clone, Copy)]
pub struct NodeBuilder(imp::NodeId);

impl FileBuilder {
    pub fn new(lang: Language, text: String, stats: FileStats) -> FileBuilder {
        FileBuilder(imp::FileBuilderImpl::new(lang, text, stats))
    }

    pub fn node(&mut self, parent: Option<NodeBuilder>, ty: NodeType, range: TextRange)
                -> NodeBuilder {
        self.0.node(parent, ty, range)
    }

    pub fn build(self) -> File {
        self.0.build()
    }
}
