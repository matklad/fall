use {TextRange, NodeType};

mod imp;

use self::imp::NodeChildren;

pub struct File(imp::FileImpl);

impl File {
    pub fn root(&self) -> Node {
        self.0.root()
    }

    pub fn text(&self) -> &str {
        self.0.text()
    }
}

#[derive(Clone, Copy)]
pub struct Node<'f>(imp::NodeImpl<'f>);

impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.0.ty()
    }

    pub fn range(&self) -> TextRange {
        self.0.range()
    }

    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn parent(&self) -> Option<Node> {
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
    pub fn new(text: String) -> FileBuilder {
        FileBuilder(imp::FileBuilderImpl::new(text))
    }

    pub fn node(&mut self, parent: Option<NodeBuilder>, ty: NodeType, range: TextRange)
                -> NodeBuilder {
        self.0.node(parent, ty, range)
    }

    pub fn build(self) -> File {
        self.0.build()
    }
}
