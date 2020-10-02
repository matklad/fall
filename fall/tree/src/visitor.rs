use std::marker::PhantomData;
use crate::search::traversal;
use crate::{Node, NodeType, AstNode};

pub fn visitor<'f, C>(ctx: C) -> VisitorBuilder<'f, C, EmptyVisitor<C>> {
    VisitorBuilder::new(ctx, EmptyVisitor(PhantomData))
}

pub fn process_subtree_bottom_up<'f, V, C>(node: Node<'f>, mut visitor: VisitorBuilder<'f, C, V>) -> C
    where V: Visit<'f, Context=C>
{
    traversal::bottom_up(node, |node| visitor.do_visit(node));
    return visitor.ctx;
}

pub fn process_node<'f, V, C>(node: Node<'f>, mut visitor: VisitorBuilder<'f, C, V>) -> C
    where V: Visit<'f, Context=C>
{
    visitor.do_visit(node);
    visitor.ctx
}

pub struct VisitorBuilder<'f, C, V> {
    ctx: C,
    visitor: V,
    n: PhantomData<Node<'f>>,
}

impl<'f, C, V> VisitorBuilder<'f, C, V> {
    pub fn new(ctx: C, visitor: V) -> VisitorBuilder<'f, C, V> {
        VisitorBuilder { ctx, visitor, n: PhantomData}
    }
}

impl<'f, C, V> VisitorBuilder<'f, C, V> {
    pub fn visit<T, F>(self, f: F) -> VisitorBuilder<'f, C, AstVisitor<V, F, T>>
        where V: Visit<'f>,
              F: FnMut(T, &mut V::Context),
    {
        VisitorBuilder {
            ctx: self.ctx,
            visitor: AstVisitor { visitor: self.visitor, f, p: PhantomData },
            n: PhantomData,
        }
    }

    pub fn visit_nodes<'n, F>(self, nodes: &'n [NodeType], f: F) -> VisitorBuilder<'f, C, TyVisitor<'n, V, F>>
        where V: Visit<'f>,
              F: FnMut(Node<'f>, &mut V::Context),

    {
        VisitorBuilder {
            ctx: self.ctx,
            visitor: TyVisitor { visitor: self.visitor, f, nodes },
            n: PhantomData,
        }
    }

    fn do_visit(&mut self, node: Node<'f>) where V: Visit<'f, Context=C> {
        self.visitor.visit(node, &mut self.ctx)
    }
}


pub trait Visit<'f> {
    type Context;

    fn visit(&mut self, node: Node<'f>, ctx: &mut Self::Context);
}


pub struct EmptyVisitor<C>(PhantomData<C>);

impl<'f, C> Visit<'f> for EmptyVisitor<C> {
    type Context = C;

    fn visit(&mut self, _node: Node<'f>, _ctx: &mut C) {}
}


pub struct AstVisitor<V, F, T> {
    visitor: V,
    f: F,
    p: PhantomData<*const T>
}

impl<'f, V, F, T> Visit<'f> for AstVisitor<V, F, T>
    where V: Visit<'f>,
          T: AstNode<'f>,
          F: FnMut(T, &mut V::Context),
{
    type Context = V::Context;

    fn visit(&mut self, node: Node<'f>, ctx: &mut Self::Context) {
        self.visitor.visit(node, ctx);
        if let Some(a) = T::wrap(node) {
            let f = &mut self.f;
            f(a, ctx)
        }
    }
}


pub struct TyVisitor<'n, V, F> {
    visitor: V,
    f: F,
    nodes: &'n [NodeType]
}

impl<'n, 'f, V, F> Visit<'f> for TyVisitor<'n, V, F>
    where V: Visit<'f>,
          F: FnMut(Node<'f>, &mut V::Context),
{
    type Context = V::Context;

    fn visit(&mut self, node: Node<'f>, ctx: &mut Self::Context) {
        self.visitor.visit(node, ctx);
        if self.nodes.contains(&node.ty()) {
            let f = &mut self.f;
            f(node, ctx)
        }
    }
}
