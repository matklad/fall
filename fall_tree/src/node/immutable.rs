use std::rc::Rc;

use ::{NodeType, TextUnit};

#[derive(Clone)]
pub struct ImmutableNode {
    inner: Rc<Inner>
}

#[derive(Clone)]
struct Inner {
    pub ty: NodeType,
    pub children: Vec<ImmutableNode>,
    pub len: TextUnit,
}

impl ImmutableNode {
    pub fn new(ty: NodeType) -> ImmutableNode {
        ImmutableNode {
            inner: Rc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: TextUnit::zero(),
            })
        }
    }

    pub fn new_leaf(ty: NodeType, len: TextUnit) -> ImmutableNode {
        ImmutableNode {
            inner: Rc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: len,
            })
        }
    }

    pub fn push_child(&mut self, child: ImmutableNode) {
        if self.children().is_empty() {
            assert!(self.len() == TextUnit::zero());
        }
        let inner = Rc::make_mut(&mut self.inner);
        inner.len += child.len();
        inner.children.push(child);
    }

    pub fn ty(&self) -> NodeType {
        self.inner.ty
    }

    pub fn len(&self) -> TextUnit {
        self.inner.len
    }

    pub fn children(&self) -> &[ImmutableNode] {
        &self.inner.children
    }
}
