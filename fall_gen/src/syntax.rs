use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_AST    : NodeType = NodeType(100);
pub const KW_NODES  : NodeType = NodeType(101);
pub const KW_TOKENIZER: NodeType = NodeType(102);
pub const KW_RULE   : NodeType = NodeType(103);
pub const KW_VERBATIM: NodeType = NodeType(104);
pub const EQ        : NodeType = NodeType(105);
pub const PIPE      : NodeType = NodeType(106);
pub const STAR      : NodeType = NodeType(107);
pub const QUESTION  : NodeType = NodeType(108);
pub const DOT       : NodeType = NodeType(109);
pub const LBRACE    : NodeType = NodeType(110);
pub const RBRACE    : NodeType = NodeType(111);
pub const LANGLE    : NodeType = NodeType(112);
pub const RANGLE    : NodeType = NodeType(113);
pub const LPAREN    : NodeType = NodeType(114);
pub const RPAREN    : NodeType = NodeType(115);
pub const IDENT     : NodeType = NodeType(116);
pub const SIMPLE_STRING: NodeType = NodeType(117);
pub const HASH_STRING: NodeType = NodeType(118);
pub const FILE      : NodeType = NodeType(119);
pub const STRING    : NodeType = NodeType(120);
pub const VERBATIM_DEF: NodeType = NodeType(121);
pub const NODES_DEF : NodeType = NodeType(122);
pub const TOKENIZER_DEF: NodeType = NodeType(123);
pub const LEX_RULE  : NodeType = NodeType(124);
pub const SYN_RULE  : NodeType = NodeType(125);
pub const ALT       : NodeType = NodeType(126);
pub const PART      : NodeType = NodeType(127);
pub const AST_DEF   : NodeType = NodeType(128);
pub const AST_NODE_DEF: NodeType = NodeType(129);
pub const METHOD_DEF: NodeType = NodeType(130);
pub const AST_SELECTOR: NodeType = NodeType(131);

fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        KW_AST.register(NodeTypeInfo { name: "KW_AST" });
        KW_NODES.register(NodeTypeInfo { name: "KW_NODES" });
        KW_TOKENIZER.register(NodeTypeInfo { name: "KW_TOKENIZER" });
        KW_RULE.register(NodeTypeInfo { name: "KW_RULE" });
        KW_VERBATIM.register(NodeTypeInfo { name: "KW_VERBATIM" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        PIPE.register(NodeTypeInfo { name: "PIPE" });
        STAR.register(NodeTypeInfo { name: "STAR" });
        QUESTION.register(NodeTypeInfo { name: "QUESTION" });
        DOT.register(NodeTypeInfo { name: "DOT" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LANGLE.register(NodeTypeInfo { name: "LANGLE" });
        RANGLE.register(NodeTypeInfo { name: "RANGLE" });
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        SIMPLE_STRING.register(NodeTypeInfo { name: "SIMPLE_STRING" });
        HASH_STRING.register(NodeTypeInfo { name: "HASH_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        VERBATIM_DEF.register(NodeTypeInfo { name: "VERBATIM_DEF" });
        NODES_DEF.register(NodeTypeInfo { name: "NODES_DEF" });
        TOKENIZER_DEF.register(NodeTypeInfo { name: "TOKENIZER_DEF" });
        LEX_RULE.register(NodeTypeInfo { name: "LEX_RULE" });
        SYN_RULE.register(NodeTypeInfo { name: "SYN_RULE" });
        ALT.register(NodeTypeInfo { name: "ALT" });
        PART.register(NodeTypeInfo { name: "PART" });
        AST_DEF.register(NodeTypeInfo { name: "AST_DEF" });
        AST_NODE_DEF.register(NodeTypeInfo { name: "AST_NODE_DEF" });
        METHOD_DEF.register(NodeTypeInfo { name: "METHOD_DEF" });
        AST_SELECTOR.register(NodeTypeInfo { name: "AST_SELECTOR" });
    });
}

const TOKENIZER: &'static [Rule] = &[
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: PIPE, re: "\\|", f: None },
    Rule { ty: STAR, re: "\\*", f: None },
    Rule { ty: QUESTION, re: "\\?", f: None },
    Rule { ty: DOT, re: "\\.", f: None },
    Rule { ty: LBRACE, re: "\\{", f: None },
    Rule { ty: RBRACE, re: "\\}", f: None },
    Rule { ty: LANGLE, re: "<", f: None },
    Rule { ty: RANGLE, re: ">", f: None },
    Rule { ty: LPAREN, re: "\\(", f: None },
    Rule { ty: RPAREN, re: "\\)", f: None },
    Rule { ty: KW_NODES, re: "nodes", f: None },
    Rule { ty: KW_TOKENIZER, re: "tokenizer", f: None },
    Rule { ty: KW_RULE, re: "rule", f: None },
    Rule { ty: KW_VERBATIM, re: "verbatim", f: None },
    Rule { ty: KW_AST, re: "ast", f: None },
    Rule { ty: WHITESPACE, re: "\\s+", f: None },
    Rule { ty: SIMPLE_STRING, re: "\'([^\'\\\\]|\\\\.)*\'", f: None },
    Rule { ty: HASH_STRING, re: "r#*", f: Some(parse_raw_string) },
    Rule { ty: IDENT, re: "\\w+", f: None },
];

const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Rule(1), syn::Part::Rule(2), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(4)], commit: None }), syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(9)], commit: None }), syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(10)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(NODES_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_NODES), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Token(IDENT)], commit: None }), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: Some(TOKENIZER_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_TOKENIZER), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(3)], commit: None }), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: Some(LEX_RULE), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Rule(8), syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(8)], commit: None })], commit: Some(1) }] },
    syn::Rule { ty: Some(SYN_RULE), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_RULE), syn::Part::Token(IDENT), syn::Part::Token(LBRACE), syn::Part::Rule(5), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(6)], commit: None }), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Token(PIPE), syn::Part::Rule(6)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(ALT), alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(7)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(PART), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT)], commit: None }, syn::Alt { parts: &[syn::Part::Token(SIMPLE_STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Token(LANGLE), syn::Part::Token(IDENT), syn::Part::Rule(5), syn::Part::Token(RANGLE)], commit: None }] },
    syn::Rule { ty: Some(STRING), alts: &[syn::Alt { parts: &[syn::Part::Token(SIMPLE_STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Token(HASH_STRING)], commit: None }] },
    syn::Rule { ty: Some(VERBATIM_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_VERBATIM), syn::Part::Token(HASH_STRING)], commit: None }] },
    syn::Rule { ty: Some(AST_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_AST), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(11)], commit: None }), syn::Part::Token(RBRACE)], commit: None }] },
    syn::Rule { ty: Some(AST_NODE_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(12)], commit: None }), syn::Part::Token(RBRACE)], commit: None }] },
    syn::Rule { ty: Some(METHOD_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Rule(13)], commit: None }] },
    syn::Rule { ty: Some(AST_SELECTOR), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(14)], commit: None })], commit: None }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[syn::Part::Token(STAR)], commit: None }, syn::Alt { parts: &[syn::Part::Token(QUESTION)], commit: None }, syn::Alt { parts: &[syn::Part::Token(DOT), syn::Part::Token(IDENT)], commit: None }] },
];

pub fn parse(text: String) -> ::fall_tree::File {
    register_node_types();
    ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

use fall_tree::{AstNode, AstChildren, Node};
use fall_tree::search::child_of_type_exn;

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
    pub fn ast_def(&self) -> Option<AstDef<'f>> {
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


#[derive(Clone, Copy)]
pub struct AstDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for AstDef<'f> {
    fn ty() -> NodeType { AST_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstDef<'f> {
    pub fn ast_nodes(&self) -> AstChildren<'f, AstNodeDef<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub struct AstNodeDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for AstNodeDef<'f> {
    fn ty() -> NodeType { AST_NODE_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstNodeDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstNodeDef<'f> {
    pub fn name(&self) -> &'f str {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn methods(&self) -> AstChildren<'f, MethodDef<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub struct MethodDef<'f> { node: Node<'f> }
impl<'f> AstNode<'f> for MethodDef<'f> {
    fn ty() -> NodeType { METHOD_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        MethodDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> MethodDef<'f> {
    pub fn name(&self) -> &'f str {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn selector(&self) -> &'f str {
        child_of_type_exn(self.node, AST_SELECTOR).text()
    }
}

