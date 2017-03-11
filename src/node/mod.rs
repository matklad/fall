use {TextRange, NodeType};


pub struct File {
    text: String,
    root: imp::NodeId,
    nodes: Vec<imp::RawNode>,
}

impl File {
    pub fn root(&self) -> Node {
        self.node(self.root)
    }

    pub fn node_containing_range(&self, range: TextRange) -> Node {
        imp::node_containing_range(self.root(), range)
    }

    pub fn dump_ws(&self) -> String {
        imp::dump(self.root(), &self.text, true)
    }

    pub fn dump(&self) -> String {
        imp::dump(self.root(), &self.text, false)
    }

    fn node(&self, id: imp::NodeId) -> Node {
        Node { id: id, file: self }
    }
}


#[derive(Clone, Copy)]
pub struct Node<'f> {
    id: imp::NodeId,
    file: &'f File,
}

impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.raw().ty
    }

    pub fn parent(&self) -> Option<Node> {
        self.raw().parent.map(|id| self.file.node(id))
    }

    pub fn children<'n>(&'n self) -> NodeChildren<'n, 'f> {
        NodeChildren {
            file: self.file,
            inner: self.raw().children.iter(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.raw().children.is_empty()
    }

    pub fn range(&self) -> TextRange {
        self.raw().range
    }

    pub fn text(&self) -> &str {
        unimplemented!()
    }

    fn raw(&self) -> &imp::RawNode {
        &self.file[self.id]
    }
}

pub struct NodeChildren<'n, 'f: 'n> {
    file: &'f File,
    inner: ::std::slice::Iter<'n, imp::NodeId>,
}

impl<'n, 'f: 'n> Iterator for NodeChildren<'n, 'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Node<'f>> {
        self.inner.next().map(|&id| self.file.node(id))
    }
}

impl<'n, 'f: 'n> NodeChildren<'n, 'f> {
    pub fn single_of_type(mut self, ty: NodeType) -> Option<Node<'f>> {
        self.find(|n| n.ty() == ty)
    }

    pub fn many_of_type(self, ty: NodeType) -> Box<Iterator<Item=Node<'f>> + 'n> {
        Box::new(self.filter(move |n| n.ty() == ty))
    }
}


pub mod imp;
