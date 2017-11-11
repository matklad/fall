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
pub const TEST: NodeType = NodeType(109);
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
pub const OPT_EXPR: NodeType = NodeType(136);
pub const REP_EXPR: NodeType = NodeType(137);
pub const CALL_EXPR: NodeType = NodeType(138);
pub const TOKENIZER_DEF: NodeType = NodeType(139);
pub const LEX_RULE: NodeType = NodeType(140);
pub const TEST_DEF: NodeType = NodeType(141);
pub const ATTRIBUTES: NodeType = NodeType(142);
pub const ATTRIBUTE: NodeType = NodeType(143);
pub const ATTRIBUTE_VALUE: NodeType = NodeType(144);
pub const STRING: NodeType = NodeType(145);
pub const VERBATIM_DEF: NodeType = NodeType(146);
pub const AST_DEF: NodeType = NodeType(147);
pub const AST_NODE_DEF: NodeType = NodeType(148);
pub const AST_CLASS_DEF: NodeType = NodeType(149);
pub const METHOD_DEF: NodeType = NodeType(150);
pub const AST_SELECTOR: NodeType = NodeType(151);


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"[{"body":{"Pub":{"ty_idx":30,"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":1},{"Rule":2}]}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"ContextualToken":[5,"tokenizer"]}],null]},{"And":[[{"ContextualToken":[9,"pub"]}],null]},{"And":[[{"ContextualToken":[6,"rule"]}],null]},{"And":[[{"Token":17}],null]},{"And":[[{"ContextualToken":[7,"verbatim"]}],null]},{"And":[[{"ContextualToken":[8,"ast"]}],null]},{"And":[[{"ContextualToken":[10,"test"]}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":13}],null]},{"And":[[{"Rule":3}],null]},{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]},{"And":[[{"Rule":15}],null]}]}},{"body":{"Pub":{"ty_idx":31,"body":{"Or":[{"And":[[{"Opt":{"Rule":16}},{"Opt":{"ContextualToken":[9,"pub"]}},{"ContextualToken":[6,"rule"]},{"Token":29},{"Opt":{"Rule":4}},{"Layer":[{"Or":[{"And":[[{"Token":18},{"Rule":22},{"Token":19}],null]},{"And":[[{"Token":18},{"Rep":{"Or":[{"And":[[{"Not":{"Rule":1}},"Any"],null]}]}}],null]}]},{"Rule":9}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":32,"body":{"Or":[{"And":[[{"Token":24},{"Layer":[{"Rule":23},{"Call":[{"Rule":20},[[0,{"Rule":5}]]]}]},{"Token":25}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":33,"body":{"Or":[{"And":[[{"Token":29}],null]}]},"replaceable":false}}},{"body":{"Pratt":{"atoms":[{"Pub":{"ty_idx":39,"body":{"Or":[{"And":[[{"Token":22},{"Token":29},{"Rep":{"Rule":6}},{"Token":23}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":34,"body":{"Or":[{"And":[[{"Token":29}],null]},{"And":[[{"Token":27}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":36,"body":{"Or":[{"And":[[{"Call":[{"Rule":21},[[1,{"Or":[{"And":[[{"Opt":{"Rule":8}},{"Rep":{"Or":[{"And":[[{"Token":12},{"Rule":8}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}],"prefixes":[],"infixes":[{"ty":37,"op":{"Token":14},"priority":999,"has_rhs":false},{"ty":38,"op":{"Token":13},"priority":999,"has_rhs":false}]}}},{"body":{"Pub":{"ty_idx":34,"body":{"Or":[{"And":[[{"Token":29}],null]},{"And":[[{"Token":27}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":35,"body":{"Or":[{"And":[[{"Rep":{"Rule":6}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":36,"body":{"Or":[{"And":[[{"Call":[{"Rule":21},[[1,{"Or":[{"And":[[{"Opt":{"Rule":8}},{"Rep":{"Or":[{"And":[[{"Token":12},{"Rule":8}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":37,"body":{"Or":[{"And":[[{"Rule":6},{"Token":14}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":38,"body":{"Or":[{"And":[[{"Rule":6},{"Token":13}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":39,"body":{"Or":[{"And":[[{"Token":22},{"Token":29},{"Rep":{"Rule":6}},{"Token":23}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":40,"body":{"Or":[{"And":[[{"ContextualToken":[5,"tokenizer"]},{"Call":[{"Rule":21},[[1,{"Rep":{"Rule":14}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":41,"body":{"Or":[{"And":[[{"Opt":{"Rule":16}},{"Token":29},{"Rule":19},{"Opt":{"Rule":19}}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":42,"body":{"Or":[{"And":[[{"ContextualToken":[10,"test"]},{"Token":28}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":43,"body":{"Or":[{"And":[[{"Token":17},{"Token":20},{"Layer":[{"Rule":24},{"Call":[{"Rule":20},[[0,{"Rule":17}]]]}]},{"Token":21}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":44,"body":{"Or":[{"And":[[{"Token":29},{"Opt":{"Or":[{"And":[[{"Token":24},{"Rule":18},{"Token":25}],null]}]}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":45,"body":{"Or":[{"And":[[{"Token":26}],null]},{"And":[[{"Token":29}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":46,"body":{"Or":[{"And":[[{"Token":27}],null]},{"And":[[{"Token":28}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":0},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":16}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":18},{"Layer":[{"Rule":22},{"Var":1}]},{"Token":19}],1]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":25},[[2,{"Token":18}],[3,{"Token":19}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":25},[[2,{"Token":24}],[3,{"Token":25}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":25},[[2,{"Token":20}],[3,{"Token":21}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Call":[{"Rule":25},[[2,{"Var":2}],[3,{"Var":3}]]]},{"Var":3}],null]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":3}},"Any"],null]}]}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":47,"body":{"Or":[{"And":[[{"ContextualToken":[7,"verbatim"]},{"Token":28}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":48,"body":{"Or":[{"And":[[{"ContextualToken":[8,"ast"]},{"Token":18},{"Rep":{"WithSkip":[{"Or":[{"And":[[{"ContextualToken":[3,"node"]}],null]},{"And":[[{"ContextualToken":[4,"class"]}],null]}]},{"Or":[{"And":[[{"Rule":28}],null]},{"And":[[{"Rule":29}],null]}]}]}},{"Token":19}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":49,"body":{"Or":[{"And":[[{"ContextualToken":[3,"node"]},{"Token":29},{"Call":[{"Rule":21},[[1,{"Rep":{"Rule":30}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":50,"body":{"Or":[{"And":[[{"ContextualToken":[4,"class"]},{"Token":29},{"Call":[{"Rule":21},[[1,{"Rep":{"Token":29}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":51,"body":{"Or":[{"And":[[{"Token":29},{"Rule":31}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":52,"body":{"Or":[{"And":[[{"Token":29},{"Opt":{"Rule":32}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":14},{"Token":15},{"Token":29}],null]},{"And":[[{"Token":15},{"Token":29}],null]},{"And":[[{"Token":13}],null]},{"And":[[{"Token":14}],null]}]}}]"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            WHITESPACE, EOL_COMMENT, NODE, CLASS, TOKENIZER, RULE, VERBATIM, AST, PUB, TEST, EQ, PIPE, STAR, QUESTION, DOT, COMMA, HASH, L_CURLY, R_CURLY, L_SQUARE, R_SQUARE, L_ANGLE, R_ANGLE, L_PAREN, R_PAREN, NUMBER, SIMPLE_STRING, HASH_STRING, IDENT, FALL_FILE, SYN_RULE, PARAMETERS, PARAMETER, REF_EXPR, SEQ_EXPR, BLOCK_EXPR, OPT_EXPR, REP_EXPR, CALL_EXPR, TOKENIZER_DEF, LEX_RULE, TEST_DEF, ATTRIBUTES, ATTRIBUTE, ATTRIBUTE_VALUE, STRING, VERBATIM_DEF, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, METHOD_DEF, AST_SELECTOR,
        ],
        lexical_rules: vec![
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
        syntactical_rules: serde_json::from_str(parser_json).unwrap(),
        
        .. Default::default()
    }
}

pub fn language() -> &'static Language {
    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition };
            impl LanguageImpl for Impl {
                fn parse(&self, text: &str) -> (FileStats, INode) {
                    self.parser_definition.parse(text, &LANG)
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
                        TEST => NodeTypeInfo { name: "TEST", whitespace_like: false },
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
                        OPT_EXPR => NodeTypeInfo { name: "OPT_EXPR", whitespace_like: false },
                        REP_EXPR => NodeTypeInfo { name: "REP_EXPR", whitespace_like: false },
                        CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                        TOKENIZER_DEF => NodeTypeInfo { name: "TOKENIZER_DEF", whitespace_like: false },
                        LEX_RULE => NodeTypeInfo { name: "LEX_RULE", whitespace_like: false },
                        TEST_DEF => NodeTypeInfo { name: "TEST_DEF", whitespace_like: false },
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

            Language::new(Impl { parser_definition: create_parser_definition() })
        };
    }

    &*LANG
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    // Who needs more then 25 hashes anyway? :)
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

use self::fall_tree::{Text, AstElement, AstNode, AstChildren, AstClass, AstClassChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallFile<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for FallFile<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(FallFile { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for FallFile<'f> {
    const NODE_TYPE: NodeType  = FALL_FILE;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        FallFile { node }
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
    pub fn tests(&self) -> AstChildren<'f, TestDef<'f>> {
        AstChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for FallFile<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("FallFile@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenizerDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for TokenizerDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(TokenizerDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for TokenizerDef<'f> {
    const NODE_TYPE: NodeType  = TOKENIZER_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        TokenizerDef { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> TokenizerDef<'f> {
    pub fn lex_rules(&self) -> AstChildren<'f, LexRule<'f>> {
        AstChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for TokenizerDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TokenizerDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LexRule<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for LexRule<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(LexRule { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for LexRule<'f> {
    const NODE_TYPE: NodeType  = LEX_RULE;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        LexRule { node }
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

impl<'f> ::std::fmt::Debug for LexRule<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("LexRule@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SynRule<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for SynRule<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(SynRule { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for SynRule<'f> {
    const NODE_TYPE: NodeType  = SYN_RULE;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        SynRule { node }
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

impl<'f> ::std::fmt::Debug for SynRule<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("SynRule@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parameters<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for Parameters<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(Parameters { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for Parameters<'f> {
    const NODE_TYPE: NodeType  = PARAMETERS;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        Parameters { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Parameters<'f> {
    pub fn parameters(&self) -> AstChildren<'f, Parameter<'f>> {
        AstChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for Parameters<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Parameters@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parameter<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for Parameter<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(Parameter { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for Parameter<'f> {
    const NODE_TYPE: NodeType  = PARAMETER;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        Parameter { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Parameter<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
}

impl<'f> ::std::fmt::Debug for Parameter<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Parameter@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for Attributes<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(Attributes { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for Attributes<'f> {
    const NODE_TYPE: NodeType  = ATTRIBUTES;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        Attributes { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> Attributes<'f> {
    pub fn attributes(&self) -> AstChildren<'f, Attribute<'f>> {
        AstChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for Attributes<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Attributes@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attribute<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for Attribute<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(Attribute { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for Attribute<'f> {
    const NODE_TYPE: NodeType  = ATTRIBUTE;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        Attribute { node }
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

impl<'f> ::std::fmt::Debug for Attribute<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Attribute@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttributeValue<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for AttributeValue<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(AttributeValue { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for AttributeValue<'f> {
    const NODE_TYPE: NodeType  = ATTRIBUTE_VALUE;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        AttributeValue { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AttributeValue<'f> {
    
}

impl<'f> ::std::fmt::Debug for AttributeValue<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AttributeValue@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerbatimDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for VerbatimDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(VerbatimDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for VerbatimDef<'f> {
    const NODE_TYPE: NodeType  = VERBATIM_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        VerbatimDef { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> VerbatimDef<'f> {
    pub fn literal_string(&self) -> Text<'f> {
        child_of_type_exn(self.node, HASH_STRING).text()
    }
}

impl<'f> ::std::fmt::Debug for VerbatimDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("VerbatimDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for AstDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(AstDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for AstDef<'f> {
    const NODE_TYPE: NodeType  = AST_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        AstDef { node }
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

impl<'f> ::std::fmt::Debug for AstDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AstDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstNodeDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for AstNodeDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(AstNodeDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for AstNodeDef<'f> {
    const NODE_TYPE: NodeType  = AST_NODE_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        AstNodeDef { node }
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

impl<'f> ::std::fmt::Debug for AstNodeDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AstNodeDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstClassDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for AstClassDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(AstClassDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for AstClassDef<'f> {
    const NODE_TYPE: NodeType  = AST_CLASS_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        AstClassDef { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> AstClassDef<'f> {
    pub fn name_ident(&self) -> Node<'f> {
        self.node().children().find(|n| n.ty() == IDENT).unwrap()
    }
}

impl<'f> ::std::fmt::Debug for AstClassDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AstClassDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MethodDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for MethodDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(MethodDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for MethodDef<'f> {
    const NODE_TYPE: NodeType  = METHOD_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        MethodDef { node }
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

impl<'f> ::std::fmt::Debug for MethodDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("MethodDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstSelector<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for AstSelector<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(AstSelector { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for AstSelector<'f> {
    const NODE_TYPE: NodeType  = AST_SELECTOR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        AstSelector { node }
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

impl<'f> ::std::fmt::Debug for AstSelector<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AstSelector@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestDef<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for TestDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(TestDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for TestDef<'f> {
    const NODE_TYPE: NodeType  = TEST_DEF;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        TestDef { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> TestDef<'f> {
    pub fn literal_string(&self) -> Option<Text<'f>> {
        child_of_type(self.node, HASH_STRING).map(|n| n.text())
    }
}

impl<'f> ::std::fmt::Debug for TestDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TestDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for RefExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(RefExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for RefExpr<'f> {
    const NODE_TYPE: NodeType  = REF_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        RefExpr { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> RefExpr<'f> {
    
}

impl<'f> ::std::fmt::Debug for RefExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("RefExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for CallExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(CallExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for CallExpr<'f> {
    const NODE_TYPE: NodeType  = CALL_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        CallExpr { node }
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

impl<'f> ::std::fmt::Debug for CallExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("CallExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for BlockExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(BlockExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for BlockExpr<'f> {
    const NODE_TYPE: NodeType  = BLOCK_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        BlockExpr { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> BlockExpr<'f> {
    pub fn alts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for BlockExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("BlockExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for OptExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(OptExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for OptExpr<'f> {
    const NODE_TYPE: NodeType  = OPT_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        OptExpr { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> OptExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        AstClassChildren::new(self.node.children()).next().unwrap()
    }
}

impl<'f> ::std::fmt::Debug for OptExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("OptExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for RepExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(RepExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for RepExpr<'f> {
    const NODE_TYPE: NodeType  = REP_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        RepExpr { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> RepExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        AstClassChildren::new(self.node.children()).next().unwrap()
    }
}

impl<'f> ::std::fmt::Debug for RepExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("RepExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeqExpr<'f> { node: Node<'f> }

impl<'f> AstElement<'f> for SeqExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == Self::NODE_TYPE {
            Some(SeqExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> AstNode<'f> for SeqExpr<'f> {
    const NODE_TYPE: NodeType  = SEQ_EXPR;
    fn new(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::NODE_TYPE);
        SeqExpr { node }
    }
    fn node(&self) -> Node<'f> { self.node }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node.children())
    }
}

impl<'f> ::std::fmt::Debug for SeqExpr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("SeqExpr@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Expr<'f> {
    RefExpr(RefExpr<'f>),
    CallExpr(CallExpr<'f>),
    BlockExpr(BlockExpr<'f>),
    OptExpr(OptExpr<'f>),
    RepExpr(RepExpr<'f>),
    SeqExpr(SeqExpr<'f>),
}

impl<'f> AstElement<'f> for Expr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        match node.ty() {
            REF_EXPR => Some(Expr::RefExpr(RefExpr::new(node))),
            CALL_EXPR => Some(Expr::CallExpr(CallExpr::new(node))),
            BLOCK_EXPR => Some(Expr::BlockExpr(BlockExpr::new(node))),
            OPT_EXPR => Some(Expr::OptExpr(OptExpr::new(node))),
            REP_EXPR => Some(Expr::RepExpr(RepExpr::new(node))),
            SEQ_EXPR => Some(Expr::SeqExpr(SeqExpr::new(node))),
            _ => None
        }
    }

    fn node(self) -> Node<'f> {
        match self {
            Expr::RefExpr(n) => AstElement::node(n),
            Expr::CallExpr(n) => AstElement::node(n),
            Expr::BlockExpr(n) => AstElement::node(n),
            Expr::OptExpr(n) => AstElement::node(n),
            Expr::RepExpr(n) => AstElement::node(n),
            Expr::SeqExpr(n) => AstElement::node(n),
        }
    }
}

impl<'f> AstClass<'f> for Expr<'f> {
    const NODE_TYPES: &'static [NodeType] = &[
        REF_EXPR,
        CALL_EXPR,
        BLOCK_EXPR,
        OPT_EXPR,
        REP_EXPR,
        SEQ_EXPR,
    ];

    fn new(node: Node<'f>) -> Self {
        match node.ty() {
            REF_EXPR => Expr::RefExpr(RefExpr::new(node)),
            CALL_EXPR => Expr::CallExpr(CallExpr::new(node)),
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr::new(node)),
            OPT_EXPR => Expr::OptExpr(OptExpr::new(node)),
            REP_EXPR => Expr::RepExpr(RepExpr::new(node)),
            SEQ_EXPR => Expr::SeqExpr(SeqExpr::new(node)),
            _ => panic!("Bad ast class")
        }
    }

    fn node(&self) -> Node<'f> {
        match *self {
            Expr::RefExpr(n) => AstElement::node(n),
            Expr::CallExpr(n) => AstElement::node(n),
            Expr::BlockExpr(n) => AstElement::node(n),
            Expr::OptExpr(n) => AstElement::node(n),
            Expr::RepExpr(n) => AstElement::node(n),
            Expr::SeqExpr(n) => AstElement::node(n),
        }
    }
}

impl<'f> ::std::fmt::Debug for Expr<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(match *self {
            Expr::RefExpr(..) => "RefExpr@",
            Expr::CallExpr(..) => "CallExpr@",
            Expr::BlockExpr(..) => "BlockExpr@",
            Expr::OptExpr(..) => "OptExpr@",
            Expr::RepExpr(..) => "RepExpr@",
            Expr::SeqExpr(..) => "SeqExpr@",
        })?;
        AstElement::node(*self).range().fmt(f)?;
        Ok(())
    }
}