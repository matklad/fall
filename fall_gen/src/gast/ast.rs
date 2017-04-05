use fall_tree::{AstNode, AstChildren, Node, NodeType};
use fall_tree::search::{child_of_type_exn, children_of_type};
use syntax::*;

struct File<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for File<'f> {
    fn ty() -> NodeType { FILE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        File { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> File<'f> {
    fn nodes_def(&self) -> NodesDef {
        AstChildren::new(self.node.children()).next().unwrap()
    }
    fn tokenizer_def(&self) -> TokenizerDef {
        AstChildren::new(self.node.children()).next().unwrap()
    }
    fn syn_rules(&self) -> AstChildren<SynRule> {
        AstChildren::new(self.node.children())
    }
}

struct NodesDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for NodesDef<'f> {
    fn ty() -> NodeType { NODES_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        NodesDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}


struct TokenizerDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for TokenizerDef<'f> {
    fn ty() -> NodeType { TOKENIZER_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        TokenizerDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> TokenizerDef<'f> {
    fn lex_rules(&self) -> AstChildren<LexRule> {
        AstChildren::new(self.node.children())
    }
}

struct LexRule<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for LexRule<'f> {
    fn ty() -> NodeType { LEX_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        LexRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}


struct SynRule<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for SynRule<'f> {
    fn ty() -> NodeType { SYN_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        SynRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SynRule<'f> {
    fn alts(&self) -> Alt {
        AstChildren::new(self.node.children()).next().unwrap()
    }
}

struct Alt<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for Alt<'f> {
    fn ty() -> NodeType { ALT }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Alt { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Alt<'f> {
    fn parts(&self) -> Part {
        AstChildren::new(self.node.children()).next().unwrap()
    }
}

struct Part<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for Part<'f> {
    fn ty() -> NodeType { PART }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Part { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}


