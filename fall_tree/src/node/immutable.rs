use std::rc::Rc;

use ::{NodeType, TextUnit};

#[derive(Clone)]
pub struct ImmutableNode {
    inner: Rc<Inner>
}

struct Inner {
    pub ty: NodeType,
    pub children: Vec<ImmutableNode>,
    pub len: TextUnit,
}

impl ImmutableNode {
    pub fn ty(&self) -> NodeType {
        self.inner.ty
    }

    pub fn len(&self) -> TextUnit {
        self.inner.len
    }

    pub fn children(&self) -> &[ImmutableNode] {
        &self.inner.children
    }

    pub fn replace(&self, path: &[usize], new_node: ImmutableNode) -> ImmutableNode {
        if path.is_empty() {
            return new_node;
        }
        let mut builder = ImmutableNodeBuilder {
            ty: self.ty(),
            children: self.inner.children.clone(),
            len: None,
        };
        builder.children[path[0]] = self.children()[path[0]].replace(&path[1..], new_node);
        builder.build()
    }
}

pub struct ImmutableNodeBuilder {
    ty: NodeType,
    children: Vec<ImmutableNode>,
    len: Option<TextUnit>,
}

impl ImmutableNodeBuilder {

    pub fn new(ty: NodeType) -> ImmutableNodeBuilder{
        ImmutableNodeBuilder { ty: ty, children: Vec::new(), len: None }
    }

    pub fn set_len(&mut self, len: TextUnit) {
        self.len = Some(len)
    }

    pub fn push_child(&mut self, child: ImmutableNode) {
        self.children.push(child)
    }

    pub fn build(self) -> ImmutableNode {
        let len = self.children.iter().map(|node| node.len()).sum();
        if len != TextUnit::zero() && self.len.is_some() && self.len.unwrap() != len {
            panic!("BadLen")
        }
        ImmutableNode {
            inner: Rc::new(Inner {
                ty: self.ty,
                children: self.children,
                len: self.len.unwrap_or(len),
            })
        }
    }
}
