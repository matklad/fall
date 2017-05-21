use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_AST: NodeType = NodeType(100);
pub const KW_NODES: NodeType = NodeType(101);
pub const KW_NODE: NodeType = NodeType(102);
pub const KW_CLASS: NodeType = NodeType(103);
pub const KW_TOKENIZER: NodeType = NodeType(104);
pub const KW_RULE: NodeType = NodeType(105);
pub const KW_VERBATIM: NodeType = NodeType(106);
pub const EQ: NodeType = NodeType(107);
pub const PIPE: NodeType = NodeType(108);
pub const STAR: NodeType = NodeType(109);
pub const QUESTION: NodeType = NodeType(110);
pub const DOT: NodeType = NodeType(111);
pub const LBRACE: NodeType = NodeType(112);
pub const RBRACE: NodeType = NodeType(113);
pub const LANGLE: NodeType = NodeType(114);
pub const RANGLE: NodeType = NodeType(115);
pub const LPAREN: NodeType = NodeType(116);
pub const RPAREN: NodeType = NodeType(117);
pub const IDENT: NodeType = NodeType(118);
pub const SIMPLE_STRING: NodeType = NodeType(119);
pub const HASH_STRING: NodeType = NodeType(120);
pub const FILE: NodeType = NodeType(121);
pub const STRING: NodeType = NodeType(122);
pub const VERBATIM_DEF: NodeType = NodeType(123);
pub const NODES_DEF: NodeType = NodeType(124);
pub const TOKENIZER_DEF: NodeType = NodeType(125);
pub const LEX_RULE: NodeType = NodeType(126);
pub const SYN_RULE: NodeType = NodeType(127);
pub const AST_DEF: NodeType = NodeType(128);
pub const AST_NODE_DEF: NodeType = NodeType(129);
pub const AST_CLASS_DEF: NodeType = NodeType(130);
pub const METHOD_DEF: NodeType = NodeType(131);
pub const AST_SELECTOR: NodeType = NodeType(132);
pub const REF_EXPR: NodeType = NodeType(133);
pub const CALL_EXPR: NodeType = NodeType(134);
pub const BLOCK_EXPR: NodeType = NodeType(135);
pub const SEQ_EXPR: NodeType = NodeType(136);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            KW_AST, KW_NODES, KW_NODE, KW_CLASS, KW_TOKENIZER, KW_RULE, KW_VERBATIM, EQ, PIPE, STAR, QUESTION, DOT, LBRACE, RBRACE, LANGLE, RANGLE, LPAREN, RPAREN, IDENT, SIMPLE_STRING, HASH_STRING, FILE, STRING, VERBATIM_DEF, NODES_DEF, TOKENIZER_DEF, LEX_RULE, SYN_RULE, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, METHOD_DEF, AST_SELECTOR, REF_EXPR, CALL_EXPR, BLOCK_EXPR, SEQ_EXPR,
        ];
        let parser_json = r##"[{"ty":23,"body":{"Or":[{"And":[[{"Rule":1},{"Rule":2},{"Rep":[{"Rule":4},null,null]},{"Opt":{"Rule":9}},{"Opt":{"Rule":10}}],null]}]}},{"ty":26,"body":{"Or":[{"And":[[{"Token":3},{"Token":14},{"Rep":[{"Token":20},null,null]},{"Token":15}],1]}]}},{"ty":27,"body":{"Or":[{"And":[[{"Token":6},{"Token":14},{"Rep":[{"Rule":3},null,null]},{"Token":15}],1]}]}},{"ty":28,"body":{"Or":[{"And":[[{"Token":20},{"Rule":8},{"Opt":{"Rule":8}}],1]}]}},{"ty":29,"body":{"Or":[{"And":[[{"Token":7},{"Token":20},{"Rule":20}],1]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":14},{"Opt":{"Rule":6}},{"Rep":[{"Or":[{"And":[[{"Token":10},{"Rule":6}],null]}]},null,null]},{"Token":15}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rep":[{"Rule":7},null,null]}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]},{"And":[[{"Token":16},{"Token":20},{"Rep":[{"Rule":5},null,null]},{"Token":17}],null]}]}},{"ty":24,"body":{"Or":[{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]}]}},{"ty":25,"body":{"Or":[{"And":[[{"Token":8},{"Token":22}],null]}]}},{"ty":30,"body":{"Or":[{"And":[[{"Token":2},{"Token":14},{"Rep":[{"Or":[{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":12}],null]}]},null,null]},{"Token":15}],null]}]}},{"ty":31,"body":{"Or":[{"And":[[{"Token":4},{"Token":20},{"Token":14},{"Rep":[{"Rule":13},null,null]},{"Token":15}],null]}]}},{"ty":32,"body":{"Or":[{"And":[[{"Token":5},{"Token":20},{"Token":14},{"Rep":[{"Token":20},null,null]},{"Token":15}],null]}]}},{"ty":33,"body":{"Or":[{"And":[[{"Token":20},{"Rule":14}],null]}]}},{"ty":34,"body":{"Or":[{"And":[[{"Token":20},{"Opt":{"Rule":15}}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Token":11}],null]},{"And":[[{"Token":12}],null]},{"And":[[{"Token":13},{"Token":20}],null]}]}},{"ty":35,"body":{"Or":[{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]}]}},{"ty":36,"body":{"Or":[{"And":[[{"Token":16},{"Token":20},{"Rep":[{"Rule":18},null,null]},{"Token":17}],null]}]}},{"ty":null,"body":{"Or":[{"And":[[{"Rule":17}],null]},{"And":[[{"Rule":16}],null]},{"And":[[{"Rule":20}],null]}]}},{"ty":38,"body":{"Or":[{"And":[[{"Rep":[{"Rule":18},null,null]}],null]}]}},{"ty":37,"body":{"Or":[{"And":[[{"Token":14},{"Opt":{"Rule":19}},{"Rep":[{"Or":[{"And":[[{"Token":10},{"Rule":19}],null]}]},null,null]},{"Token":15}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(b)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    KW_AST => NodeTypeInfo { name: "KW_AST" },
                    KW_NODES => NodeTypeInfo { name: "KW_NODES" },
                    KW_NODE => NodeTypeInfo { name: "KW_NODE" },
                    KW_CLASS => NodeTypeInfo { name: "KW_CLASS" },
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
                    AST_DEF => NodeTypeInfo { name: "AST_DEF" },
                    AST_NODE_DEF => NodeTypeInfo { name: "AST_NODE_DEF" },
                    AST_CLASS_DEF => NodeTypeInfo { name: "AST_CLASS_DEF" },
                    METHOD_DEF => NodeTypeInfo { name: "METHOD_DEF" },
                    AST_SELECTOR => NodeTypeInfo { name: "AST_SELECTOR" },
                    REF_EXPR => NodeTypeInfo { name: "REF_EXPR" },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR" },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR" },
                    SEQ_EXPR => NodeTypeInfo { name: "SEQ_EXPR" },
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
                LexRule::new(KW_NODE, "node", None),
                LexRule::new(KW_CLASS, "class", None),
                LexRule::new(KW_TOKENIZER, "tokenizer", None),
                LexRule::new(KW_RULE, "rule", None),
                LexRule::new(KW_VERBATIM, "verbatim", None),
                LexRule::new(KW_AST, "ast", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(SIMPLE_STRING, "\'([^\'\\\\]|\\\\.)*\'", None),
                LexRule::new(HASH_STRING, "r#*", Some(parse_raw_string)),
                LexRule::new(IDENT, "\\w+", None),
            ],
            parser: parser,
        })
    };
}
fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

use fall_tree::{AstNode, AstChildren, AstClass, AstClassChildren, Node};
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
    pub fn block_expr(&self) -> BlockExpr<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
    }
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
    pub fn ast_classes(&self) -> AstChildren<'f, AstClassDef<'f>> {
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
pub struct AstClassDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AstClassDef<'f> {
    fn ty() -> NodeType { AST_CLASS_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstClassDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstClassDef<'f> {
    
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
#[derive(Clone, Copy)]
pub struct RefExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for RefExpr<'f> {
    fn ty() -> NodeType { REF_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        RefExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> RefExpr<'f> {
    
}
#[derive(Clone, Copy)]
pub struct CallExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for CallExpr<'f> {
    fn ty() -> NodeType { CALL_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        CallExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> CallExpr<'f> {
    pub fn fn_name(&self) -> &'f str {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn args(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct SeqExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for SeqExpr<'f> {
    fn ty() -> NodeType { SEQ_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        SeqExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct BlockExpr<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for BlockExpr<'f> {
    fn ty() -> NodeType { BLOCK_EXPR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        BlockExpr { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> BlockExpr<'f> {
    pub fn alts(&self) -> AstChildren<'f, SeqExpr<'f>> {
        AstChildren::new(self.node.children())
    }
}

#[derive(Clone, Copy)]
pub enum Expr<'f> {
    RefExpr(RefExpr<'f>),
    CallExpr(CallExpr<'f>),
    SeqExpr(SeqExpr<'f>),
    BlockExpr(BlockExpr<'f>),
}

impl<'f> AstClass<'f> for Expr<'f> {
    fn tys() -> &'static [NodeType] {
        const TYS: &[NodeType] = &[
            REF_EXPR,
            CALL_EXPR,
            SEQ_EXPR,
            BLOCK_EXPR,
        ];
        TYS
    }

    fn new(node: Node<'f>) -> Self {
        match node.ty() {
            REF_EXPR => Expr::RefExpr(RefExpr::new(node)),
            CALL_EXPR => Expr::CallExpr(CallExpr::new(node)),
            SEQ_EXPR => Expr::SeqExpr(SeqExpr::new(node)),
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr::new(node)),
            _ => panic!("Bad ast class")
        }
    }

    fn node(&self) -> Node<'f> {
        match *self {
            Expr::RefExpr(n) => n.node(),
            Expr::CallExpr(n) => n.node(),
            Expr::SeqExpr(n) => n.node(),
            Expr::BlockExpr(n) => n.node(),
        }
    }
}