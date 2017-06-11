use std::time::Duration;
use {Text, TextRange, NodeType, Language, TextUnit};

mod imp;
mod immutable;

pub use self::imp::NodeChildren;
pub use self::immutable::{INode, ReparseRegion};

pub struct File {
    lang: Language,
    imp: imp::FileImpl,
    inode: INode,
}

impl File {
    pub fn new(lang: Language, text: String, stats: FileStats, node: INode) -> File {
        File {
            lang: lang,
            imp: imp::new_file(text, stats, &node),
            inode: node,
        }
    }

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

    pub fn inode(&self) -> INode {
        self.inode.clone()
    }

    pub fn edit(&self, range: TextRange, edit: String) -> File {
        let before = self.text().slice(TextRange::from_to(TextUnit::zero(), range.start()));
        let after = self.text().slice(TextRange::from_to(range.end(), self.text().len()));
        let new_text = before.to_string() + &edit + &after.to_string();
        self.language().parse(new_text)
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
