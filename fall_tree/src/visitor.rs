use std::marker::PhantomData;
use {Node, AstNode, NodeType};

pub trait NodeVisitor<'f, C> {
    fn context(&mut self) -> &mut C;
    fn do_visit(&mut self, node: Node<'f>);
    fn into_context(self) -> C;

    fn visit<T, F>(self, f: F) -> AstVisitor<Self, T, F>
        where Self: Sized, T: AstNode<'f>, F: FnMut(&mut C, T) {
        AstVisitor { visitor: self, f, t: PhantomData }
    }

    fn visit_nodes<'n, F>(self, nodes: &'n [NodeType], f: F) -> NodesVisitor<'n, Self, F>
        where Self: Sized, F: FnMut(&mut C, Node<'f>) {
        NodesVisitor { visitor: self, f, nodes }
    }


    fn walk_recursively_children_first(mut self, node: Node<'f>) -> C where Self: Sized {
        go(&mut self, node);
        return self.into_context();

        fn go<'f, C, V: NodeVisitor<'f, C>>(v: &mut V, node: Node<'f>) {
            for child in node.children() {
                go(v, child)
            }
            v.do_visit(node);
        }
    }

    fn walk_single_node(mut self, node: Node<'f>) -> C where Self: Sized {
        self.do_visit(node);
        self.into_context()
    }
}

pub struct Visitor<C>(pub C);

impl<'f, C> NodeVisitor<'f, C> for Visitor<C> {
    fn context(&mut self) -> &mut C {
        &mut self.0
    }

    fn into_context(self) -> C {
        self.0
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

    fn into_context(self) -> C {
        self.visitor.into_context()
    }

    fn do_visit(&mut self, node: Node<'f>) {
        self.visitor.do_visit(node);
        if T::NODE_TYPE == node.ty() {
            let f = &mut self.f;
            let c = self.visitor.context();
            f(c, T::new(node))
        }
    }
}

pub struct NodesVisitor<'n, V, F> {
    visitor: V,
    f: F,
    nodes: &'n [NodeType]
}

impl<'f, 'n, C, V, F> NodeVisitor<'f, C> for NodesVisitor<'n, V, F>
    where V: NodeVisitor<'f, C>, F: FnMut(&mut C, Node<'f>)
{
    fn context(&mut self) -> &mut C {
        self.visitor.context()
    }

    fn into_context(self) -> C {
        self.visitor.into_context()
    }

    fn do_visit(&mut self, node: Node<'f>) {
        self.visitor.do_visit(node);
        if self.nodes.contains(&node.ty()) {
            let f = &mut self.f;
            let c = self.visitor.context();
            f(c, node)
        }
    }
}
