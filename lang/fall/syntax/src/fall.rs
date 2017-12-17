use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, TextEdit, TreeBuilder};
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


pub fn language() -> &'static Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(EOL_COMMENT, "//.*", None),
            ::fall_parse::LexRule::new(EQ, "=", None),
            ::fall_parse::LexRule::new(PIPE, "\\|", None),
            ::fall_parse::LexRule::new(STAR, "\\*", None),
            ::fall_parse::LexRule::new(QUESTION, "\\?", None),
            ::fall_parse::LexRule::new(DOT, "\\.", None),
            ::fall_parse::LexRule::new(COMMA, ",", None),
            ::fall_parse::LexRule::new(HASH, "\\#", None),
            ::fall_parse::LexRule::new(L_CURLY, "\\{", None),
            ::fall_parse::LexRule::new(R_CURLY, "\\}", None),
            ::fall_parse::LexRule::new(L_SQUARE, "\\[", None),
            ::fall_parse::LexRule::new(R_SQUARE, "\\]", None),
            ::fall_parse::LexRule::new(L_ANGLE, "<", None),
            ::fall_parse::LexRule::new(R_ANGLE, ">", None),
            ::fall_parse::LexRule::new(L_PAREN, "\\(", None),
            ::fall_parse::LexRule::new(R_PAREN, "\\)", None),
            ::fall_parse::LexRule::new(NUMBER, "\\d+", None),
            ::fall_parse::LexRule::new(SIMPLE_STRING, "\'([^\'\\\\]|\\\\.)*\'", None),
            ::fall_parse::LexRule::new(HASH_STRING, "r#*\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(IDENT, "\\w+", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":30,"body":36,"replaceable":false}},{"Or":[38,40,42,44,46,48,50]},{"Or":[51,52,53,54,55]},{"Pub":{"ty":31,"body":76,"replaceable":false}},{"Pub":{"ty":32,"body":82,"replaceable":false}},{"Pub":{"ty":33,"body":85,"replaceable":false}},{"Pratt":{"atoms":[12,7,9],"prefixes":[],"infixes":[{"ty":37,"op":86,"priority":999,"has_rhs":false},{"ty":38,"op":87,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":34,"body":92,"replaceable":false}},{"Pub":{"ty":35,"body":95,"replaceable":false}},{"Pub":{"ty":36,"body":105,"replaceable":false}},{"Pub":{"ty":37,"body":108,"replaceable":false}},{"Pub":{"ty":38,"body":111,"replaceable":false}},{"Pub":{"ty":39,"body":117,"replaceable":false}},{"Pub":{"ty":40,"body":122,"replaceable":false}},{"Pub":{"ty":41,"body":127,"replaceable":false}},{"Pub":{"ty":42,"body":131,"replaceable":false}},{"Pub":{"ty":43,"body":138,"replaceable":false}},{"Pub":{"ty":44,"body":146,"replaceable":false}},{"Pub":{"ty":45,"body":151,"replaceable":false}},{"Pub":{"ty":46,"body":156,"replaceable":false}},{"Or":[166]},{"Or":[171]},{"Or":[175]},{"Or":[179]},{"Or":[183]},{"Or":[198]},{"Pub":{"ty":47,"body":202,"replaceable":false}},{"Pub":{"ty":48,"body":217,"replaceable":false}},{"Pub":{"ty":49,"body":223,"replaceable":false}},{"Pub":{"ty":50,"body":230,"replaceable":false}},{"Pub":{"ty":51,"body":233,"replaceable":false}},{"Pub":{"ty":52,"body":237,"replaceable":false}},{"Or":[241,244,246,248]},{"WithSkip":[1,2]},{"Rep":33},{"And":[[34],null]},{"Or":[35]},{"ContextualToken":[5,"tokenizer"]},{"And":[[37],null]},{"ContextualToken":[9,"pub"]},{"And":[[39],null]},{"ContextualToken":[6,"rule"]},{"And":[[41],null]},{"Token":17},{"And":[[43],null]},{"ContextualToken":[7,"verbatim"]},{"And":[[45],null]},{"ContextualToken":[8,"ast"]},{"And":[[47],null]},{"ContextualToken":[10,"test"]},{"And":[[49],null]},{"And":[[13],null]},{"And":[[3],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[15],null]},{"Opt":16},{"ContextualToken":[9,"pub"]},{"Opt":57},{"ContextualToken":[6,"rule"]},{"Token":29},{"Opt":4},{"Token":18},{"Token":19},{"And":[[62,22,63],null]},{"Token":18},{"Not":1},"Any",{"And":[[66,67],null]},{"Or":[68]},{"Rep":69},{"And":[[65,70],null]},{"Or":[64,71]},{"Layer":[72,9]},{"And":[[56,58,59,60,61,73],3]},{"Or":[74]},{"Cached":75},{"Token":24},{"Call":[20,[[0,5]]]},{"Layer":[23,78]},{"Token":25},{"And":[[77,79,80],null]},{"Or":[81]},{"Token":29},{"And":[[83],null]},{"Or":[84]},{"Token":14},{"Token":13},{"Token":29},{"And":[[88],null]},{"Token":27},{"And":[[90],null]},{"Or":[89,91]},{"Rep":6},{"And":[[93],null]},{"Or":[94]},{"Opt":8},{"Token":12},{"And":[[97,8],null]},{"Or":[98]},{"Rep":99},{"And":[[96,100],null]},{"Or":[101]},{"Call":[21,[[1,102]]]},{"And":[[103],null]},{"Or":[104]},{"Token":14},{"And":[[6,106],null]},{"Or":[107]},{"Token":13},{"And":[[6,109],null]},{"Or":[110]},{"Token":22},{"Token":29},{"Rep":6},{"Token":23},{"And":[[112,113,114,115],null]},{"Or":[116]},{"ContextualToken":[5,"tokenizer"]},{"Rep":14},{"Call":[21,[[1,119]]]},{"And":[[118,120],1]},{"Or":[121]},{"Opt":16},{"Token":29},{"Opt":19},{"And":[[123,124,19,125],2]},{"Or":[126]},{"ContextualToken":[10,"test"]},{"Token":28},{"And":[[128,129],1]},{"Or":[130]},{"Token":17},{"Token":20},{"Call":[20,[[0,17]]]},{"Layer":[24,134]},{"Token":21},{"And":[[132,133,135,136],null]},{"Or":[137]},{"Token":29},{"Token":24},{"Token":25},{"And":[[140,18,141],null]},{"Or":[142]},{"Opt":143},{"And":[[139,144],null]},{"Or":[145]},{"Token":26},{"And":[[147],null]},{"Token":29},{"And":[[149],null]},{"Or":[148,150]},{"Token":27},{"And":[[152],null]},{"Token":28},{"And":[[154],null]},{"Or":[153,155]},{"Var":0},"Eof",{"And":[[158],null]},{"Token":16},{"And":[[160],null]},{"Or":[159,161]},{"And":[[157,162],1]},{"Or":[163]},{"Rep":164},{"And":[[165],null]},{"Token":18},{"Var":1},{"Layer":[22,168]},{"Token":19},{"And":[[167,169,170],1]},{"Token":18},{"Token":19},{"Call":[25,[[2,172],[3,173]]]},{"And":[[174],null]},{"Token":24},{"Token":25},{"Call":[25,[[2,176],[3,177]]]},{"And":[[178],null]},{"Token":20},{"Token":21},{"Call":[25,[[2,180],[3,181]]]},{"And":[[182],null]},{"Var":2},{"Var":2},{"Var":3},{"Call":[25,[[2,185],[3,186]]]},{"Var":3},{"And":[[184,187,188],null]},{"Var":3},{"Not":190},"Any",{"And":[[191,192],null]},{"Or":[193]},{"And":[[194],null]},{"Or":[189,195]},{"Rep":196},{"And":[[197],null]},{"ContextualToken":[7,"verbatim"]},{"Token":28},{"And":[[199,200],1]},{"Or":[201]},{"ContextualToken":[8,"ast"]},{"Token":18},{"ContextualToken":[3,"node"]},{"And":[[205],null]},{"ContextualToken":[4,"class"]},{"And":[[207],null]},{"Or":[206,208]},{"And":[[28],null]},{"And":[[29],null]},{"Or":[210,211]},{"WithSkip":[209,212]},{"Rep":213},{"Token":19},{"And":[[203,204,214,215],1]},{"Or":[216]},{"ContextualToken":[3,"node"]},{"Token":29},{"Rep":30},{"Call":[21,[[1,220]]]},{"And":[[218,219,221],1]},{"Or":[222]},{"ContextualToken":[4,"class"]},{"Token":29},{"Token":29},{"Rep":226},{"Call":[21,[[1,227]]]},{"And":[[224,225,228],1]},{"Or":[229]},{"Token":29},{"And":[[231,31],null]},{"Or":[232]},{"Token":29},{"Opt":32},{"And":[[234,235],null]},{"Or":[236]},{"Token":14},{"Token":15},{"Token":29},{"And":[[238,239,240],null]},{"Token":15},{"Token":29},{"And":[[242,243],null]},{"Token":13},{"And":[[245],null]},{"Token":14},{"And":[[247],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, EOL_COMMENT, NODE, CLASS, TOKENIZER, RULE, VERBATIM, AST, PUB, TEST, EQ, PIPE, STAR, QUESTION, DOT, COMMA, HASH, L_CURLY, R_CURLY, L_SQUARE, R_SQUARE, L_ANGLE, R_ANGLE, L_PAREN, R_PAREN, NUMBER, SIMPLE_STRING, HASH_STRING, IDENT, FALL_FILE, SYN_RULE, PARAMETERS, PARAMETER, REF_EXPR, SEQ_EXPR, BLOCK_EXPR, OPT_EXPR, REP_EXPR, CALL_EXPR, TOKENIZER_DEF, LEX_RULE, TEST_DEF, ATTRIBUTES, ATTRIBUTE, ATTRIBUTE_VALUE, STRING, VERBATIM_DEF, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, METHOD_DEF, AST_SELECTOR,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::{ParserDefinition, parse, reparse};
            use std::any::Any;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &Any,
                    edit: &TextEdit,
                    new_text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
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

            Language::new(Impl {
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
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

use self::fall_tree::{AstNode, AstChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallFile<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for FallFile<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == FALL_FILE {
            Some(FallFile { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for TokenizerDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == TOKENIZER_DEF {
            Some(TokenizerDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for LexRule<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == LEX_RULE {
            Some(LexRule { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for SynRule<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == SYN_RULE {
            Some(SynRule { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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
        AstChildren::new(self.node.children()).next().unwrap()
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

impl<'f> AstNode<'f> for Parameters<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == PARAMETERS {
            Some(Parameters { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for Parameter<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == PARAMETER {
            Some(Parameter { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for Attributes<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTES {
            Some(Attributes { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for Attribute<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTE {
            Some(Attribute { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for AttributeValue<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTE_VALUE {
            Some(AttributeValue { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for VerbatimDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == VERBATIM_DEF {
            Some(VerbatimDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for AstDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == AST_DEF {
            Some(AstDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for AstNodeDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == AST_NODE_DEF {
            Some(AstNodeDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for AstClassDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == AST_CLASS_DEF {
            Some(AstClassDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for MethodDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == METHOD_DEF {
            Some(MethodDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for AstSelector<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == AST_SELECTOR {
            Some(AstSelector { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for TestDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == TEST_DEF {
            Some(TestDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for RefExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == REF_EXPR {
            Some(RefExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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

impl<'f> AstNode<'f> for CallExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == CALL_EXPR {
            Some(CallExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> CallExpr<'f> {
    pub fn fn_name(&self) -> Text<'f> {
        child_of_type_exn(self.node, IDENT).text()
    }
    pub fn args(&self) -> AstChildren<'f, Expr<'f>> {
        AstChildren::new(self.node.children())
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

impl<'f> AstNode<'f> for BlockExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == BLOCK_EXPR {
            Some(BlockExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> BlockExpr<'f> {
    pub fn alts(&self) -> AstChildren<'f, Expr<'f>> {
        AstChildren::new(self.node.children())
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

impl<'f> AstNode<'f> for OptExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == OPT_EXPR {
            Some(OptExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> OptExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
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

impl<'f> AstNode<'f> for RepExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == REP_EXPR {
            Some(RepExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> RepExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        AstChildren::new(self.node.children()).next().unwrap()
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

impl<'f> AstNode<'f> for SeqExpr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == SEQ_EXPR {
            Some(SeqExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> AstChildren<'f, Expr<'f>> {
        AstChildren::new(self.node.children())
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

impl<'f> AstNode<'f> for Expr<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if let Some(n) = RefExpr::wrap(node) {
            return Some(Expr::RefExpr(n))
        }
        if let Some(n) = CallExpr::wrap(node) {
            return Some(Expr::CallExpr(n))
        }
        if let Some(n) = BlockExpr::wrap(node) {
            return Some(Expr::BlockExpr(n))
        }
        if let Some(n) = OptExpr::wrap(node) {
            return Some(Expr::OptExpr(n))
        }
        if let Some(n) = RepExpr::wrap(node) {
            return Some(Expr::RepExpr(n))
        }
        if let Some(n) = SeqExpr::wrap(node) {
            return Some(Expr::SeqExpr(n))
        }
        None
    }

    fn node(self) -> Node<'f> {
        match self {
            Expr::RefExpr(n) => n.node(),
            Expr::CallExpr(n) => n.node(),
            Expr::BlockExpr(n) => n.node(),
            Expr::OptExpr(n) => n.node(),
            Expr::RepExpr(n) => n.node(),
            Expr::SeqExpr(n) => n.node(),
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
        AstNode::node(*self).range().fmt(f)?;
        Ok(())
    }
}