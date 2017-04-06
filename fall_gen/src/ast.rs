use fall_tree::{AstNode, AstChildren, Node, NodeType};
use fall_tree::search::child_of_type_exn;
use syntax::*;

#[derive(Clone, Copy)]
pub struct File<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for File<'f> {
    fn ty() -> NodeType { FILE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        File { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> File<'f> {
    pub fn nodes_def(&self) -> NodesDef<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
    }
    pub fn tokenizer_def(&self) -> TokenizerDef<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
    }
    pub fn syn_rules(&self) -> AstChildren<'f, SynRule<'f>> {
        AstChildren::new(self.node.children())
    }
    pub fn verbatim_def(&self) -> Option<VerbatimDef<'f>> {
        AstChildren::new(self.node.children()).next()
    }
}

#[derive(Clone, Copy)]
pub struct NodesDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for NodesDef<'f> {
    fn ty() -> NodeType { NODES_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        NodesDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}


#[derive(Clone, Copy)]
pub struct TokenizerDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for TokenizerDef<'f> {
    fn ty() -> NodeType { TOKENIZER_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        TokenizerDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> TokenizerDef<'f> {
    pub fn lex_rules(&self) -> AstChildren<'f, LexRule<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub struct LexRule<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for LexRule<'f> {
    fn ty() -> NodeType { LEX_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        LexRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> LexRule<'f> {
    pub fn node_type(&self) -> &'f str {
        child_of_type_exn(self.node, IDENT).text()
    }
}

#[derive(Clone, Copy)]
pub struct SynRule<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for SynRule<'f> {
    fn ty() -> NodeType { SYN_RULE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        SynRule { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SynRule<'f> {
    pub fn name(&self) -> &'f str {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn alts(&self) -> AstChildren<'f, Alt<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub struct Alt<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for Alt<'f> {
    fn ty() -> NodeType { ALT }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Alt { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Alt<'f> {
    pub fn parts(&self) -> AstChildren<'f, Part<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub struct Part<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for Part<'f> {
    fn ty() -> NodeType { PART }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Part { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}


#[derive(Clone, Copy)]
pub struct VerbatimDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for VerbatimDef<'f> {
    fn ty() -> NodeType { VERBATIM_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        VerbatimDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> VerbatimDef<'f> {
    pub fn verbatim(&self) -> &'f str {
        child_of_type_exn(self.node, HASH_STRING).text()
    }
}

