use std::marker::PhantomData;
use {Node, NodeType, AstNode};

pub fn visitor<'f, C>(ctx: C) -> VisitorBuilder<'f, C, EmptyVisitor<C>> {
    VisitorBuilder { ctx, visitor: EmptyVisitor(PhantomData), n: PhantomData }
}

pub fn process_subtree_bottom_up<'f, V, C>(node: Node<'f>, visitor: VisitorBuilder<'f, C, V>) -> C
    where V: Visit<'f, Context=C>
{
    let VisitorBuilder { mut ctx, mut visitor, .. } = visitor;
    go(&mut ctx, &mut visitor, node);
    return ctx;

    fn go<'f, C, V: Visit<'f, Context=C>>(ctx: &mut C, v: &mut V, node: Node<'f>) {
        for child in node.children() {
            go(ctx, v, child)
        }
        v.visit(ctx, node);
    }
}

pub fn process_node<'f, V, C>(node: Node<'f>, visitor: VisitorBuilder<'f, C, V>) -> C
    where V: Visit<'f, Context=C>
{
    let VisitorBuilder { mut ctx, mut visitor, .. } = visitor;
    visitor.visit(&mut ctx, node);
    ctx
}

pub struct VisitorBuilder<'f, C, V> {
    ctx: C,
    visitor: V,
    n: PhantomData<Node<'f>>,
}

impl<'f, C, V> VisitorBuilder<'f, C, V> {
    pub fn visit<T, F>(self, f: F) -> VisitorBuilder<'f, C, AstVisitor<V, F, T>>
        where V: Visit<'f>,
              F: FnMut(&mut V::Context, T),
    {
        VisitorBuilder {
            ctx: self.ctx,
            visitor: AstVisitor { visitor: self.visitor, f, p: PhantomData },
            n: PhantomData,
        }
    }

    pub fn visit_nodes<'n, F>(self, nodes: &'n [NodeType], f: F) -> VisitorBuilder<'f, C, TyVisitor<'n, V, F>>
        where V: Visit<'f>,
              F: FnMut(&mut V::Context, Node<'f>),

    {
        VisitorBuilder {
            ctx: self.ctx,
            visitor: TyVisitor { visitor: self.visitor, f, nodes },
            n: PhantomData,
        }
    }
}


pub trait Visit<'f> {
    type Context;

    fn visit(&mut self, ctx: &mut Self::Context, node: Node<'f>);
}


pub struct EmptyVisitor<C>(PhantomData<C>);

impl<'f, C> Visit<'f> for EmptyVisitor<C> {
    type Context = C;

    fn visit(&mut self, _ctx: &mut C, _node: Node<'f>) {}
}


pub struct AstVisitor<V, F, T> {
    visitor: V,
    f: F,
    p: PhantomData<*const T>
}

impl<'f, V, F, T> Visit<'f> for AstVisitor<V, F, T>
    where V: Visit<'f>,
          T: AstNode<'f>,
          F: FnMut(&mut V::Context, T),
{
    type Context = V::Context;

    fn visit(&mut self, ctx: &mut Self::Context, node: Node<'f>) {
        self.visitor.visit(ctx, node);
        if let Some(a) = T::wrap(node) {
            let f = &mut self.f;
            f(ctx, a)
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
          F: FnMut(&mut V::Context, Node<'f>),
{
    type Context = V::Context;

    fn visit(&mut self, ctx: &mut Self::Context, node: Node<'f>) {
        self.visitor.visit(ctx, node);
        if self.nodes.contains(&node.ty()) {
            let f = &mut self.f;
            f(ctx, node)
        }
    }
}
