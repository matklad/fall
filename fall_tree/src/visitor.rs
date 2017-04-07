use std::marker::PhantomData;
use {Node, AstNode};

pub trait NodeVisitor<'f, C> {
    fn context(&mut self) -> &mut C;
    fn do_visit(&mut self, node: Node<'f>);

    fn visit<T: AstNode<'f>, F: FnMut(&mut C, T)>(self, f: F) -> AstVisitor<Self, T, F>
        where Self: Sized
    {
        AstVisitor { visitor: self, f: f, t: PhantomData }
    }

    fn walk_recursively_children_first(&mut self, node: Node<'f>) {
        for child in node.children() {
            self.walk_recursively_children_first(child);
        }
        self.do_visit(node)
    }
}

pub struct Visitor<C>(pub C);

impl<'f, C> NodeVisitor<'f, C> for Visitor<C> {
    fn context(&mut self) -> &mut C {
        &mut self.0
    }

    fn do_visit(&mut self, _node: Node<'f>) {}
}

pub struct AstVisitor<V, T, F> {
    visitor: V,
    f: F,
    t: PhantomData<*const T>
}

impl<'f, C, V, T, F> NodeVisitor<'f, C> for AstVisitor<V, T, F>
    where V: NodeVisitor<'f, C>, T: AstNode<'f>, F: FnMut(&mut C, T)
{
    fn context(&mut self) -> &mut C {
        self.visitor.context()
    }

    fn do_visit(&mut self, node: Node<'f>) {
        self.visitor.do_visit(node);
        if T::ty() == node.ty() {
            let f = &mut self.f;
            let c = self.visitor.context();
            f(c, T::new(node))
        }
    }
}
