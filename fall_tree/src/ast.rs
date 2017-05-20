use std::marker::PhantomData;

use {Node, NodeType};
use node::NodeChildren;

pub trait AstNode<'f>: Copy {
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

pub trait AstClass<'f>: Copy {
    fn tys() -> &'static [NodeType];
    fn new(node: Node<'f>) -> Self;
    fn node(&self) -> Node<'f>;
}

pub struct AstClassChildren<'f, A: AstClass<'f>> {
    inner: NodeChildren<'f>,
    phantom: PhantomData<*const A>,
}

impl<'f, A: AstClass<'f>> AstClassChildren<'f, A> {
    pub fn new(children: NodeChildren<'f>) -> Self {
        AstClassChildren {
            inner: children,
            phantom: PhantomData,
        }
    }
}

impl<'f, A: AstClass<'f>> Iterator for AstClassChildren<'f, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if A::tys().contains(&node.ty()) {
                return Some(A::new(node));
            }
        }
        return None;
    }
}
