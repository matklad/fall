use std::sync::Arc;

use ::{NodeType, TextUnit, TextRange};

#[derive(Clone, Debug)]
pub struct INode {
    inner: Arc<Inner>
}

#[derive(Clone, Copy, Debug)]
pub struct ReparseRegion {
    pub range: TextRange,
    pub parser_id: u32,
}

#[derive(Clone, Debug)]
struct Inner {
    pub ty: NodeType,
    pub children: Vec<INode>,
    pub len: TextUnit,
    pub reparse_region: Option<ReparseRegion>
}

impl INode {
    pub fn new(ty: NodeType) -> INode {
        INode {
            inner: Arc::new(Inner {
                ty: ty,
                children: Vec::new(),
                len: TextUnit::zero(),
                reparse_region: None,
            })
        }
    }

    pub fn new_leaf(ty: NodeType, len: TextUnit) -> INode {
        INode {
            inner: Arc::new(Inner {
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
        let inner = Arc::make_mut(&mut self.inner);
        inner.len += child.len();
        inner.children.push(child);
    }

    pub fn set_reparse_region(&mut self, region: ReparseRegion) {
        let inner = Arc::make_mut(&mut self.inner);
        inner.reparse_region = Some(region);
    }

    pub fn ty(&self) -> NodeType {
        self.inner.ty
    }

    pub fn len(&self) -> TextUnit {
        self.inner.len
    }

    pub fn reparse_region(&self) -> Option<ReparseRegion> {
        self.inner.reparse_region
    }

    pub fn children(&self) -> &[INode] {
        &self.inner.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<INode> {
        let inner = Arc::make_mut(&mut self.inner);
        &mut inner.children
    }
}
