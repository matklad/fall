use std::rc::Rc;

use ::{NodeType, TextUnit, TextRange};

#[derive(Clone)]
pub struct INode {
    inner: Rc<Inner>
}

#[derive(Clone, Debug)]
pub struct ReparseRegion {
    pub range: TextRange,
    pub parser_id: u32,
}

#[derive(Clone)]
struct Inner {
    pub ty: NodeType,
    pub children: Vec<INode>,
    pub len: TextUnit,
    pub reparse_region: Option<ReparseRegion>
}

impl INode {
    pub fn new(ty: NodeType) -> INode {
        INode {
            inner: Rc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: TextUnit::zero(),
                reparse_region: None,
            })
        }
    }

    pub fn new_leaf(ty: NodeType, len: TextUnit) -> INode {
        INode {
            inner: Rc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: len,
                reparse_region: None
            })
        }
    }

    pub fn push_child(&mut self, child: INode) {
        if self.children().is_empty() {
            assert!(self.len() == TextUnit::zero());
        }
        let inner = Rc::make_mut(&mut self.inner);
        inner.len += child.len();
        inner.children.push(child);
    }

    pub fn set_reparse_region(&mut self, region: ReparseRegion) {
        let inner = Rc::make_mut(&mut self.inner);
        inner.reparse_region = Some(region);
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
}
