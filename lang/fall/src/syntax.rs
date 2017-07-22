use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const EOL_COMMENT: NodeType = NodeType(101);
pub const NODE: NodeType = NodeType(102);
pub const CLASS: NodeType = NodeType(103);
pub const TOKENIZER: NodeType = NodeType(104);
pub const RULE: NodeType = NodeType(105);
pub const VERBATIM: NodeType = NodeType(106);
pub const AST: NodeType = NodeType(107);
pub const PUB: NodeType = NodeType(108);
pub const EXAMPLE: NodeType = NodeType(109);
pub const EQ: NodeType = NodeType(110);
pub const PIPE: NodeType = NodeType(111);
pub const STAR: NodeType = NodeType(112);
pub const QUESTION: NodeType = NodeType(113);
pub const DOT: NodeType = NodeType(114);
pub const COMMA: NodeType = NodeType(115);
pub const HASH: NodeType = NodeType(116);
pub const L_CURLY: NodeType = NodeType(117);
pub const R_CURLY: NodeType = NodeType(118);
pub const L_SQUARE: NodeType = NodeType(119);
pub const R_SQUARE: NodeType = NodeType(120);
pub const L_ANGLE: NodeType = NodeType(121);
pub const R_ANGLE: NodeType = NodeType(122);
pub const L_PAREN: NodeType = NodeType(123);
pub const R_PAREN: NodeType = NodeType(124);
pub const NUMBER: NodeType = NodeType(125);
pub const SIMPLE_STRING: NodeType = NodeType(126);
pub const HASH_STRING: NodeType = NodeType(127);
pub const IDENT: NodeType = NodeType(128);
pub const FALL_FILE: NodeType = NodeType(129);
pub const SYN_RULE: NodeType = NodeType(130);
pub const PARAMETERS: NodeType = NodeType(131);
pub const PARAMETER: NodeType = NodeType(132);
pub const REF_EXPR: NodeType = NodeType(133);
pub const SEQ_EXPR: NodeType = NodeType(134);
pub const BLOCK_EXPR: NodeType = NodeType(135);
pub const CALL_EXPR: NodeType = NodeType(136);
pub const TOKENIZER_DEF: NodeType = NodeType(137);
pub const LEX_RULE: NodeType = NodeType(138);
pub const EXAMPLE_DEF: NodeType = NodeType(139);
pub const ATTRIBUTES: NodeType = NodeType(140);
pub const ATTRIBUTE: NodeType = NodeType(141);
pub const ATTRIBUTE_VALUE: NodeType = NodeType(142);
pub const STRING: NodeType = NodeType(143);
pub const VERBATIM_DEF: NodeType = NodeType(144);
pub const AST_DEF: NodeType = NodeType(145);
pub const AST_NODE_DEF: NodeType = NodeType(146);
pub const AST_CLASS_DEF: NodeType = NodeType(147);
pub const METHOD_DEF: NodeType = NodeType(148);
pub const AST_SELECTOR: NodeType = NodeType(149);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, EOL_COMMENT, NODE, CLASS, TOKENIZER, RULE, VERBATIM, AST, PUB, EXAMPLE, EQ, PIPE, STAR, QUESTION, DOT, COMMA, HASH, L_CURLY, R_CURLY, L_SQUARE, R_SQUARE, L_ANGLE, R_ANGLE, L_PAREN, R_PAREN, NUMBER, SIMPLE_STRING, HASH_STRING, IDENT, FALL_FILE, SYN_RULE, PARAMETERS, PARAMETER, REF_EXPR, SEQ_EXPR, BLOCK_EXPR, CALL_EXPR, TOKENIZER_DEF, LEX_RULE, EXAMPLE_DEF, ATTRIBUTES, ATTRIBUTE, ATTRIBUTE_VALUE, STRING, VERBATIM_DEF, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, METHOD_DEF, AST_SELECTOR,
        ];
        let parser_json = r##"[{"body":{"Pub":[30,{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":1},{"Rule":2}]}}],null]}]}]}},{"body":{"Or":[{"And":[[{"ContextualToken":[5,"tokenizer"]}],null]},{"And":[[{"ContextualToken":[9,"pub"]}],null]},{"And":[[{"ContextualToken":[6,"rule"]}],null]},{"And":[[{"Token":17}],null]},{"And":[[{"ContextualToken":[7,"verbatim"]}],null]},{"And":[[{"ContextualToken":[8,"ast"]}],null]},{"And":[[{"ContextualToken":[10,"example"]}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":3}],null]},{"And":[[{"Rule":24}],null]},{"And":[[{"Rule":25}],null]},{"And":[[{"Rule":13}],null]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Opt":{"Rule":14}},{"Opt":{"ContextualToken":[9,"pub"]}},{"ContextualToken":[6,"rule"]},{"Token":29},{"Opt":{"Rule":4}},{"Rule":9}],3]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Token":24},{"Layer":[{"Rule":21},{"Call":[{"Rule":18},[[0,{"Rule":5}]]]}]},{"Token":25}],null]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":29}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":10}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":9}],null]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Token":29}],null]},{"And":[[{"Token":27}],null]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Rep":{"Rule":6}}],null]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Call":[{"Rule":19},[[1,{"Or":[{"And":[[{"Opt":{"Rule":8}},{"Rep":{"Or":[{"And":[[{"Token":12},{"Rule":8}],null]}]}}],null]}]}]]]}],null]}]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Token":22},{"Token":29},{"Rep":{"Rule":6}},{"Token":23}],null]}]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"ContextualToken":[5,"tokenizer"]},{"Call":[{"Rule":19},[[1,{"Rep":{"Rule":12}}]]]}],1]}]}]}},{"body":{"Pub":[39,{"Or":[{"And":[[{"Opt":{"Rule":14}},{"Token":29},{"Rule":17},{"Opt":{"Rule":17}}],2]}]}]}},{"body":{"Pub":[40,{"Or":[{"And":[[{"ContextualToken":[10,"example"]},{"Token":28}],1]}]}]}},{"body":{"Pub":[41,{"Or":[{"And":[[{"Token":17},{"Token":20},{"Layer":[{"Rule":22},{"Call":[{"Rule":18},[[0,{"Rule":15}]]]}]},{"Token":21}],null]}]}]}},{"body":{"Pub":[42,{"Or":[{"And":[[{"Token":29},{"Opt":{"Or":[{"And":[[{"Token":24},{"Rule":16},{"Token":25}],null]}]}}],null]}]}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Token":26}],null]},{"And":[[{"Token":29}],null]}]}]}},{"body":{"Pub":[44,{"Or":[{"And":[[{"Token":27}],null]},{"And":[[{"Token":28}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":0},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":16}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":18},{"Layer":[{"Rule":20},{"Var":1}]},{"Token":19}],1]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":23},[[2,{"Token":18}],[3,{"Token":19}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":23},[[2,{"Token":24}],[3,{"Token":25}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":23},[[2,{"Token":20}],[3,{"Token":21}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Call":[{"Rule":23},[[2,{"Var":2}],[3,{"Var":3}]]]},{"Var":3}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":3}},"Any"],null]}]}],null]}]}}],null]}]}},{"body":{"Pub":[45,{"Or":[{"And":[[{"ContextualToken":[7,"verbatim"]},{"Token":28}],1]}]}]}},{"body":{"Pub":[46,{"Or":[{"And":[[{"ContextualToken":[8,"ast"]},{"Token":18},{"Rep":{"WithSkip":[{"Or":[{"And":[[{"ContextualToken":[3,"node"]}],null]},{"And":[[{"ContextualToken":[4,"class"]}],null]}]},{"Or":[{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]}]}]}},{"Token":19}],1]}]}]}},{"body":{"Pub":[47,{"Or":[{"And":[[{"ContextualToken":[3,"node"]},{"Token":29},{"Call":[{"Rule":19},[[1,{"Rep":{"Rule":28}}]]]}],1]}]}]}},{"body":{"Pub":[48,{"Or":[{"And":[[{"ContextualToken":[4,"class"]},{"Token":29},{"Call":[{"Rule":19},[[1,{"Rep":{"Token":29}}]]]}],1]}]}]}},{"body":{"Pub":[49,{"Or":[{"And":[[{"Token":29},{"Rule":29}],null]}]}]}},{"body":{"Pub":[50,{"Or":[{"And":[[{"Token":29},{"Opt":{"Rule":30}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":14},{"Token":15},{"Token":29}],null]},{"And":[[{"Token":15},{"Token":29}],null]},{"And":[[{"Token":13}],null]},{"And":[[{"Token":14}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                parse(
                    text,
                    &LANG,
                    &self.tokenizer,
                    &|tokens, stats| Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                )
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                    EOL_COMMENT => NodeTypeInfo { name: "EOL_COMMENT", whitespace_like: true },
                    NODE => NodeTypeInfo { name: "NODE", whitespace_like: false },
                    CLASS => NodeTypeInfo { name: "CLASS", whitespace_like: false },
                    TOKENIZER => NodeTypeInfo { name: "TOKENIZER", whitespace_like: false },
                    RULE => NodeTypeInfo { name: "RULE", whitespace_like: false },
                    VERBATIM => NodeTypeInfo { name: "VERBATIM", whitespace_like: false },
                    AST => NodeTypeInfo { name: "AST", whitespace_like: false },
                    PUB => NodeTypeInfo { name: "PUB", whitespace_like: false },
                    EXAMPLE => NodeTypeInfo { name: "EXAMPLE", whitespace_like: false },
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
                    STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                    QUESTION => NodeTypeInfo { name: "QUESTION", whitespace_like: false },
                    DOT => NodeTypeInfo { name: "DOT", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    HASH => NodeTypeInfo { name: "HASH", whitespace_like: false },
                    L_CURLY => NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                    R_CURLY => NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                    L_SQUARE => NodeTypeInfo { name: "L_SQUARE", whitespace_like: false },
                    R_SQUARE => NodeTypeInfo { name: "R_SQUARE", whitespace_like: false },
                    L_ANGLE => NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                    R_ANGLE => NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                    L_PAREN => NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                    R_PAREN => NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                    NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                    SIMPLE_STRING => NodeTypeInfo { name: "SIMPLE_STRING", whitespace_like: false },
                    HASH_STRING => NodeTypeInfo { name: "HASH_STRING", whitespace_like: false },
                    IDENT => NodeTypeInfo { name: "IDENT", whitespace_like: false },
                    FALL_FILE => NodeTypeInfo { name: "FALL_FILE", whitespace_like: false },
                    SYN_RULE => NodeTypeInfo { name: "SYN_RULE", whitespace_like: false },
                    PARAMETERS => NodeTypeInfo { name: "PARAMETERS", whitespace_like: false },
                    PARAMETER => NodeTypeInfo { name: "PARAMETER", whitespace_like: false },
                    REF_EXPR => NodeTypeInfo { name: "REF_EXPR", whitespace_like: false },
                    SEQ_EXPR => NodeTypeInfo { name: "SEQ_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                    TOKENIZER_DEF => NodeTypeInfo { name: "TOKENIZER_DEF", whitespace_like: false },
                    LEX_RULE => NodeTypeInfo { name: "LEX_RULE", whitespace_like: false },
                    EXAMPLE_DEF => NodeTypeInfo { name: "EXAMPLE_DEF", whitespace_like: false },
                    ATTRIBUTES => NodeTypeInfo { name: "ATTRIBUTES", whitespace_like: false },
                    ATTRIBUTE => NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                    ATTRIBUTE_VALUE => NodeTypeInfo { name: "ATTRIBUTE_VALUE", whitespace_like: false },
                    STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                    VERBATIM_DEF => NodeTypeInfo { name: "VERBATIM_DEF", whitespace_like: false },
                    AST_DEF => NodeTypeInfo { name: "AST_DEF", whitespace_like: false },
                    AST_NODE_DEF => NodeTypeInfo { name: "AST_NODE_DEF", whitespace_like: false },
                    AST_CLASS_DEF => NodeTypeInfo { name: "AST_CLASS_DEF", whitespace_like: false },
                    METHOD_DEF => NodeTypeInfo { name: "METHOD_DEF", whitespace_like: false },
                    AST_SELECTOR => NodeTypeInfo { name: "AST_SELECTOR", whitespace_like: false },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(EOL_COMMENT, "//.*", None),
                LexRule::new(EQ, "=", None),
                LexRule::new(PIPE, "\\|", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(QUESTION, "\\?", None),
                LexRule::new(DOT, "\\.", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(HASH, "\\#", None),
                LexRule::new(L_CURLY, "\\{", None),
                LexRule::new(R_CURLY, "\\}", None),
                LexRule::new(L_SQUARE, "\\[", None),
                LexRule::new(R_SQUARE, "\\]", None),
                LexRule::new(L_ANGLE, "<", None),
                LexRule::new(R_ANGLE, ">", None),
                LexRule::new(L_PAREN, "\\(", None),
                LexRule::new(R_PAREN, "\\)", None),
                LexRule::new(NUMBER, "\\d+", None),
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
    // Who needs more then 25 hashes anyway? :)
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

use self::fall_tree::{Text, AstNode, AstChildren, AstClass, AstClassChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

#[derive(Clone, Copy)]
pub struct FallFile<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for FallFile<'f> {
    fn ty() -> NodeType { FALL_FILE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        FallFile { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> FallFile<'f> {
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
    pub fn examples(&self) -> AstChildren<'f, ExampleDef<'f>> {
        AstChildren::new(self.node.children())
    }
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
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn node_type(&self) -> Text<'f> {
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
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        AstChildren::new(self.node.children()).next()
    }
    pub fn name_ident(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == IDENT)
    }
    pub fn name(&self) -> Option<Text<'f>> {
        child_of_type(self.node, IDENT).map(|n| n.text())
    }
    pub fn body(&self) -> Expr<'f> {
        AstClassChildren::new(self.node.children()).next().unwrap()
    }
    pub fn parameters(&self) -> Option<Parameters<'f>> {
        AstChildren::new(self.node.children()).next()
    }
}
#[derive(Clone, Copy)]
pub struct Parameters<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Parameters<'f> {
    fn ty() -> NodeType { PARAMETERS }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Parameters { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Parameters<'f> {
    pub fn parameters(&self) -> AstChildren<'f, Parameter<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct Parameter<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Parameter<'f> {
    fn ty() -> NodeType { PARAMETER }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Parameter { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Parameter<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
}
#[derive(Clone, Copy)]
pub struct Attributes<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Attributes<'f> {
    fn ty() -> NodeType { ATTRIBUTES }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Attributes { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Attributes<'f> {
    pub fn attributes(&self) -> AstChildren<'f, Attribute<'f>> {
        AstChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct Attribute<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for Attribute<'f> {
    fn ty() -> NodeType { ATTRIBUTE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Attribute { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Attribute<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn value(&self) -> Option<AttributeValue<'f>> {
        AstChildren::new(self.node.children()).next()
    }
}
#[derive(Clone, Copy)]
pub struct AttributeValue<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AttributeValue<'f> {
    fn ty() -> NodeType { ATTRIBUTE_VALUE }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AttributeValue { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AttributeValue<'f> {
    
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
    pub fn literal_string(&self) -> Text<'f> {
        child_of_type_exn(self.node, HASH_STRING).text()
    }
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
    pub fn name_ident(&self) -> Node<'f> {
        self.node().children().find(|n| n.ty() == IDENT).unwrap()
    }
    pub fn name(&self) -> Text<'f> {
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
    pub fn name_ident(&self) -> Node<'f> {
        self.node().children().find(|n| n.ty() == IDENT).unwrap()
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
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn selector(&self) -> AstSelector<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
    }
}
#[derive(Clone, Copy)]
pub struct AstSelector<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for AstSelector<'f> {
    fn ty() -> NodeType { AST_SELECTOR }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        AstSelector { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstSelector<'f> {
    pub fn child(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn optional(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == QUESTION)
    }
    pub fn many(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == STAR)
    }
    pub fn dot(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == DOT)
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
    pub fn fn_name(&self) -> Text<'f> {
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
    pub fn alts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}
#[derive(Clone, Copy)]
pub struct ExampleDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for ExampleDef<'f> {
    fn ty() -> NodeType { EXAMPLE_DEF }
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        ExampleDef { node: node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> ExampleDef<'f> {
    pub fn literal_string(&self) -> Text<'f> {
        child_of_type_exn(self.node, HASH_STRING).text()
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