use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_AST: NodeType = NodeType(100);
pub const KW_NODES: NodeType = NodeType(101);
pub const KW_TOKENIZER: NodeType = NodeType(102);
pub const KW_RULE: NodeType = NodeType(103);
pub const KW_VERBATIM: NodeType = NodeType(104);
pub const EQ: NodeType = NodeType(105);
pub const PIPE: NodeType = NodeType(106);
pub const STAR: NodeType = NodeType(107);
pub const QUESTION: NodeType = NodeType(108);
pub const DOT: NodeType = NodeType(109);
pub const LBRACE: NodeType = NodeType(110);
pub const RBRACE: NodeType = NodeType(111);
pub const LANGLE: NodeType = NodeType(112);
pub const RANGLE: NodeType = NodeType(113);
pub const LPAREN: NodeType = NodeType(114);
pub const RPAREN: NodeType = NodeType(115);
pub const IDENT: NodeType = NodeType(116);
pub const SIMPLE_STRING: NodeType = NodeType(117);
pub const HASH_STRING: NodeType = NodeType(118);
pub const FILE: NodeType = NodeType(119);
pub const STRING: NodeType = NodeType(120);
pub const VERBATIM_DEF: NodeType = NodeType(121);
pub const NODES_DEF: NodeType = NodeType(122);
pub const TOKENIZER_DEF: NodeType = NodeType(123);
pub const LEX_RULE: NodeType = NodeType(124);
pub const SYN_RULE: NodeType = NodeType(125);
pub const ALT: NodeType = NodeType(126);
pub const PART: NodeType = NodeType(127);
pub const AST_DEF: NodeType = NodeType(128);
pub const AST_NODE_DEF: NodeType = NodeType(129);
pub const METHOD_DEF: NodeType = NodeType(130);
pub const AST_SELECTOR: NodeType = NodeType(131);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Alt, Part, Parser};

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                alts: &[Alt { parts: &[Part::Rule(1), Part::Rule(2), Part::Rep(Alt { parts: &[Part::Rule(4)], commit: None }, None, None), Part::Opt(Alt { parts: &[Part::Rule(9)], commit: None }), Part::Opt(Alt { parts: &[Part::Rule(10)], commit: None })], commit: None }],
            },
            SynRule {
                ty: Some(NODES_DEF),
                alts: &[Alt { parts: &[Part::Token(KW_NODES), Part::Token(LBRACE), Part::Rep(Alt { parts: &[Part::Token(IDENT)], commit: None }, None, None), Part::Token(RBRACE)], commit: Some(1) }],
            },
            SynRule {
                ty: Some(TOKENIZER_DEF),
                alts: &[Alt { parts: &[Part::Token(KW_TOKENIZER), Part::Token(LBRACE), Part::Rep(Alt { parts: &[Part::Rule(3)], commit: None }, None, None), Part::Token(RBRACE)], commit: Some(1) }],
            },
            SynRule {
                ty: Some(LEX_RULE),
                alts: &[Alt { parts: &[Part::Token(IDENT), Part::Rule(8), Part::Opt(Alt { parts: &[Part::Rule(8)], commit: None })], commit: Some(1) }],
            },
            SynRule {
                ty: Some(SYN_RULE),
                alts: &[Alt { parts: &[Part::Token(KW_RULE), Part::Token(IDENT), Part::Rule(5)], commit: Some(1) }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Token(LBRACE), Part::Opt(Alt { parts: &[Part::Rule(6)], commit: None }), Part::Rep(Alt { parts: &[Part::Token(PIPE), Part::Rule(6)], commit: None }, None, None), Part::Token(RBRACE)], commit: None }],
            },
            SynRule {
                ty: Some(ALT),
                alts: &[Alt { parts: &[Part::Rep(Alt { parts: &[Part::Rule(7)], commit: None }, None, None)], commit: None }],
            },
            SynRule {
                ty: Some(PART),
                alts: &[Alt { parts: &[Part::Token(IDENT)], commit: None }, Alt { parts: &[Part::Token(SIMPLE_STRING)], commit: None }, Alt { parts: &[Part::Token(LANGLE), Part::Token(IDENT), Part::Rep(Alt { parts: &[Part::Rule(5)], commit: None }, None, None), Part::Token(RANGLE)], commit: None }],
            },
            SynRule {
                ty: Some(STRING),
                alts: &[Alt { parts: &[Part::Token(SIMPLE_STRING)], commit: None }, Alt { parts: &[Part::Token(HASH_STRING)], commit: None }],
            },
            SynRule {
                ty: Some(VERBATIM_DEF),
                alts: &[Alt { parts: &[Part::Token(KW_VERBATIM), Part::Token(HASH_STRING)], commit: None }],
            },
            SynRule {
                ty: Some(AST_DEF),
                alts: &[Alt { parts: &[Part::Token(KW_AST), Part::Token(LBRACE), Part::Rep(Alt { parts: &[Part::Rule(11)], commit: None }, None, None), Part::Token(RBRACE)], commit: None }],
            },
            SynRule {
                ty: Some(AST_NODE_DEF),
                alts: &[Alt { parts: &[Part::Token(IDENT), Part::Token(LBRACE), Part::Rep(Alt { parts: &[Part::Rule(12)], commit: None }, None, None), Part::Token(RBRACE)], commit: None }],
            },
            SynRule {
                ty: Some(METHOD_DEF),
                alts: &[Alt { parts: &[Part::Token(IDENT), Part::Rule(13)], commit: None }],
            },
            SynRule {
                ty: Some(AST_SELECTOR),
                alts: &[Alt { parts: &[Part::Token(IDENT), Part::Opt(Alt { parts: &[Part::Rule(14)], commit: None })], commit: None }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Token(STAR)], commit: None }, Alt { parts: &[Part::Token(QUESTION)], commit: None }, Alt { parts: &[Part::Token(DOT), Part::Token(IDENT)], commit: None }],
            },
        ];

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| Parser::new(PARSER).parse(b))
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    KW_AST => NodeTypeInfo { name: "KW_AST" },
                    KW_NODES => NodeTypeInfo { name: "KW_NODES" },
                    KW_TOKENIZER => NodeTypeInfo { name: "KW_TOKENIZER" },
                    KW_RULE => NodeTypeInfo { name: "KW_RULE" },
                    KW_VERBATIM => NodeTypeInfo { name: "KW_VERBATIM" },
                    EQ => NodeTypeInfo { name: "EQ" },
                    PIPE => NodeTypeInfo { name: "PIPE" },
                    STAR => NodeTypeInfo { name: "STAR" },
                    QUESTION => NodeTypeInfo { name: "QUESTION" },
                    DOT => NodeTypeInfo { name: "DOT" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    LANGLE => NodeTypeInfo { name: "LANGLE" },
                    RANGLE => NodeTypeInfo { name: "RANGLE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    SIMPLE_STRING => NodeTypeInfo { name: "SIMPLE_STRING" },
                    HASH_STRING => NodeTypeInfo { name: "HASH_STRING" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    STRING => NodeTypeInfo { name: "STRING" },
                    VERBATIM_DEF => NodeTypeInfo { name: "VERBATIM_DEF" },
                    NODES_DEF => NodeTypeInfo { name: "NODES_DEF" },
                    TOKENIZER_DEF => NodeTypeInfo { name: "TOKENIZER_DEF" },
                    LEX_RULE => NodeTypeInfo { name: "LEX_RULE" },
                    SYN_RULE => NodeTypeInfo { name: "SYN_RULE" },
                    ALT => NodeTypeInfo { name: "ALT" },
                    PART => NodeTypeInfo { name: "PART" },
                    AST_DEF => NodeTypeInfo { name: "AST_DEF" },
                    AST_NODE_DEF => NodeTypeInfo { name: "AST_NODE_DEF" },
                    METHOD_DEF => NodeTypeInfo { name: "METHOD_DEF" },
                    AST_SELECTOR => NodeTypeInfo { name: "AST_SELECTOR" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(EQ, "=", None),
                LexRule::new(PIPE, "\\|", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(QUESTION, "\\?", None),
                LexRule::new(DOT, "\\.", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(LANGLE, "<", None),
                LexRule::new(RANGLE, ">", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(KW_NODES, "nodes", None),
                LexRule::new(KW_TOKENIZER, "tokenizer", None),
                LexRule::new(KW_RULE, "rule", None),
                LexRule::new(KW_VERBATIM, "verbatim", None),
                LexRule::new(KW_AST, "ast", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(SIMPLE_STRING, "\'([^\'\\\\]|\\\\.)*\'", None),
                LexRule::new(HASH_STRING, "r#*", Some(parse_raw_string)),
                LexRule::new(IDENT, "\\w+", None),
            ]
        })
    };
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
    pub fn nodes_def(&self) -> Option<NodesDef<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn tokenizer_def(&self) -> Option<TokenizerDef<'f>> {
        AstChildren::new(self.node.children()).next()
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

impl<'f> NodesDef<'f> {
    
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

impl<'f> Part<'f> {
    
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
}