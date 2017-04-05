use std::marker::PhantomData;

use {Node, NodeType};
use node::NodeChildren;

pub trait AstNode<'f> {
    fn ty() -> NodeType;
    fn new(node: Node<'f>) -> Self;
    fn node(&self) -> Node<'f>;
}

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
            if node.ty() == A::ty() {
                return Some(A::new(node));
            }
        }
        return None;
    }
}
