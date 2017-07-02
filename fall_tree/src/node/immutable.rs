use std::sync::Arc;

use ::{NodeType, TextUnit};

#[derive(Clone, Debug)]
pub struct INode {
    inner: Arc<Inner>
}

#[derive(Clone, Debug)]
struct Inner {
    pub ty: NodeType,
    pub children: Vec<INode>,
    pub len: TextUnit,
}

impl INode {
    pub fn new(ty: NodeType) -> INode {
        INode {
            inner: Arc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: TextUnit::zero(),
            })
        }
    }

    pub fn new_leaf(ty: NodeType, len: TextUnit) -> INode {
        INode {
            inner: Arc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: len,
            })
        }
    }

    pub fn push_child(&mut self, child: INode) {
        if self.children().is_empty() {
            assert!(self.len() == TextUnit::zero());
        }
        let inner = Arc::make_mut(&mut self.inner);
        inner.len += child.len();
        inner.children.push(child);
    }

    pub fn push_token_part(&mut self, len: TextUnit) {
        let inner = Arc::make_mut(&mut self.inner);
        inner.len += len;
    }

    pub fn ty(&self) -> NodeType {
        self.inner.ty
    }

    pub fn len(&self) -> TextUnit {
        self.inner.len
    }

    pub fn children(&self) -> &[INode] {
        &self.inner.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<INode> {
        let inner = Arc::make_mut(&mut self.inner);
        &mut inner.children
    }
}
