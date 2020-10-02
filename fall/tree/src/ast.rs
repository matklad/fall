use std::marker::PhantomData;

use crate::{Node};
use crate::node::NodeChildren;

pub trait AstNode<'f>: Copy {
    fn wrap(node: Node<'f>) -> Option<Self> where Self: Sized;

    fn node(self) -> Node<'f>;
}

#[derive(Clone)]
pub struct AstChildren<'f, A: AstNode<'f>> {
    inner: NodeChildren<'f>,
    phantom: PhantomData<*const A>,
}

impl<'f, A: AstNode<'f>> AstChildren<'f, A> {
    pub fn new(children: NodeChildren<'f>) -> Self {
        AstChildren {
            inner: children,
            phantom: PhantomData,
        }
    }
}

impl<'f, A: AstNode<'f>> Iterator for AstChildren<'f, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if let Some(a) = A::wrap(node) {
                return Some(a);
            }
        }
        return None;
    }
}
