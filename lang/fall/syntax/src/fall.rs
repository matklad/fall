use fall_parse::runtime as rt;
pub use self::rt::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const EOL_COMMENT: rt::NodeType = rt::NodeType(101);
pub const NODE: rt::NodeType = rt::NodeType(102);
pub const CLASS: rt::NodeType = rt::NodeType(103);
pub const TRAIT: rt::NodeType = rt::NodeType(104);
pub const TOKENIZER: rt::NodeType = rt::NodeType(105);
pub const RULE: rt::NodeType = rt::NodeType(106);
pub const VERBATIM: rt::NodeType = rt::NodeType(107);
pub const AST: rt::NodeType = rt::NodeType(108);
pub const PUB: rt::NodeType = rt::NodeType(109);
pub const TEST: rt::NodeType = rt::NodeType(110);
pub const EQ: rt::NodeType = rt::NodeType(111);
pub const PIPE: rt::NodeType = rt::NodeType(112);
pub const STAR: rt::NodeType = rt::NodeType(113);
pub const QUESTION: rt::NodeType = rt::NodeType(114);
pub const DOT: rt::NodeType = rt::NodeType(115);
pub const COMMA: rt::NodeType = rt::NodeType(116);
pub const COLON: rt::NodeType = rt::NodeType(117);
pub const HASH: rt::NodeType = rt::NodeType(118);
pub const L_CURLY: rt::NodeType = rt::NodeType(119);
pub const R_CURLY: rt::NodeType = rt::NodeType(120);
pub const L_SQUARE: rt::NodeType = rt::NodeType(121);
pub const R_SQUARE: rt::NodeType = rt::NodeType(122);
pub const L_ANGLE: rt::NodeType = rt::NodeType(123);
pub const R_ANGLE: rt::NodeType = rt::NodeType(124);
pub const L_PAREN: rt::NodeType = rt::NodeType(125);
pub const R_PAREN: rt::NodeType = rt::NodeType(126);
pub const NUMBER: rt::NodeType = rt::NodeType(127);
pub const SIMPLE_STRING: rt::NodeType = rt::NodeType(128);
pub const HASH_STRING: rt::NodeType = rt::NodeType(129);
pub const IDENT: rt::NodeType = rt::NodeType(130);
pub const FALL_FILE: rt::NodeType = rt::NodeType(131);
pub const SYN_RULE: rt::NodeType = rt::NodeType(132);
pub const PARAMETERS: rt::NodeType = rt::NodeType(133);
pub const PARAMETER: rt::NodeType = rt::NodeType(134);
pub const REF_EXPR: rt::NodeType = rt::NodeType(135);
pub const SEQ_EXPR: rt::NodeType = rt::NodeType(136);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(137);
pub const OPT_EXPR: rt::NodeType = rt::NodeType(138);
pub const REP_EXPR: rt::NodeType = rt::NodeType(139);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(140);
pub const TOKENIZER_DEF: rt::NodeType = rt::NodeType(141);
pub const LEX_RULE: rt::NodeType = rt::NodeType(142);
pub const TEST_DEF: rt::NodeType = rt::NodeType(143);
pub const ATTRIBUTES: rt::NodeType = rt::NodeType(144);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(145);
pub const ATTRIBUTE_VALUE: rt::NodeType = rt::NodeType(146);
pub const STRING: rt::NodeType = rt::NodeType(147);
pub const VERBATIM_DEF: rt::NodeType = rt::NodeType(148);
pub const AST_DEF: rt::NodeType = rt::NodeType(149);
pub const AST_NODE_DEF: rt::NodeType = rt::NodeType(150);
pub const AST_CLASS_DEF: rt::NodeType = rt::NodeType(151);
pub const AST_TRAIT_DEF: rt::NodeType = rt::NodeType(152);
pub const METHOD_DEF: rt::NodeType = rt::NodeType(153);
pub const AST_SELECTOR: rt::NodeType = rt::NodeType(154);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(EOL_COMMENT, "//.*", None),
            rt::LexRule::new(EQ, "=", None),
            rt::LexRule::new(PIPE, "\\|", None),
            rt::LexRule::new(STAR, "\\*", None),
            rt::LexRule::new(QUESTION, "\\?", None),
            rt::LexRule::new(DOT, "\\.", None),
            rt::LexRule::new(COMMA, ",", None),
            rt::LexRule::new(COLON, ":", None),
            rt::LexRule::new(HASH, "\\#", None),
            rt::LexRule::new(L_CURLY, "\\{", None),
            rt::LexRule::new(R_CURLY, "\\}", None),
            rt::LexRule::new(L_SQUARE, "\\[", None),
            rt::LexRule::new(R_SQUARE, "\\]", None),
            rt::LexRule::new(L_ANGLE, "<", None),
            rt::LexRule::new(R_ANGLE, ">", None),
            rt::LexRule::new(L_PAREN, "\\(", None),
            rt::LexRule::new(R_PAREN, "\\)", None),
            rt::LexRule::new(NUMBER, "\\d+", None),
            rt::LexRule::new(SIMPLE_STRING, "\'([^\'\\\\]|\\\\.)*\'", None),
            rt::LexRule::new(HASH_STRING, "r#*\"", Some(parse_raw_string)),
            rt::LexRule::new(IDENT, "\\w+", None),
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":32,"body":37,"replaceable":false}},{"Or":[39,41,43,45,47,49,51]},{"Or":[52,53,54,55,56]},{"Pub":{"ty":33,"body":77,"replaceable":false}},{"Pub":{"ty":34,"body":83,"replaceable":false}},{"Pub":{"ty":35,"body":86,"replaceable":false}},{"Pratt":{"atoms":[12,7,9],"prefixes":[],"infixes":[{"ty":39,"op":87,"priority":999,"has_rhs":false},{"ty":40,"op":88,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":36,"body":93,"replaceable":false}},{"Pub":{"ty":37,"body":96,"replaceable":false}},{"Pub":{"ty":38,"body":106,"replaceable":false}},{"Pub":{"ty":39,"body":109,"replaceable":false}},{"Pub":{"ty":40,"body":112,"replaceable":false}},{"Pub":{"ty":41,"body":118,"replaceable":false}},{"Pub":{"ty":42,"body":123,"replaceable":false}},{"Pub":{"ty":43,"body":128,"replaceable":false}},{"Pub":{"ty":44,"body":132,"replaceable":false}},{"Pub":{"ty":45,"body":139,"replaceable":false}},{"Pub":{"ty":46,"body":147,"replaceable":false}},{"Pub":{"ty":47,"body":152,"replaceable":false}},{"Pub":{"ty":48,"body":157,"replaceable":false}},{"Or":[167]},{"Or":[172]},{"Or":[176]},{"Or":[180]},{"Or":[184]},{"Or":[199]},{"Pub":{"ty":49,"body":203,"replaceable":false}},{"Pub":{"ty":50,"body":221,"replaceable":false}},{"Pub":{"ty":51,"body":242,"replaceable":false}},{"Pub":{"ty":52,"body":249,"replaceable":false}},{"Pub":{"ty":53,"body":255,"replaceable":false}},{"Pub":{"ty":54,"body":258,"replaceable":false}},{"Pub":{"ty":55,"body":262,"replaceable":false}},{"Or":[266,269,271,273]},{"WithSkip":[1,2]},{"Rep":34},{"And":[[35],null]},{"Or":[36]},{"ContextualToken":[6,"tokenizer"]},{"And":[[38],null]},{"ContextualToken":[10,"pub"]},{"And":[[40],null]},{"ContextualToken":[7,"rule"]},{"And":[[42],null]},{"Token":19},{"And":[[44],null]},{"ContextualToken":[8,"verbatim"]},{"And":[[46],null]},{"ContextualToken":[9,"ast"]},{"And":[[48],null]},{"ContextualToken":[11,"test"]},{"And":[[50],null]},{"And":[[13],null]},{"And":[[3],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[15],null]},{"Opt":16},{"ContextualToken":[10,"pub"]},{"Opt":58},{"ContextualToken":[7,"rule"]},{"Token":31},{"Opt":4},{"Token":20},{"Token":21},{"And":[[63,22,64],null]},{"Token":20},{"Not":1},"Any",{"And":[[67,68],null]},{"Or":[69]},{"Rep":70},{"And":[[66,71],null]},{"Or":[65,72]},{"Layer":[73,9]},{"And":[[57,59,60,61,62,74],3]},{"Or":[75]},{"Cached":76},{"Token":26},{"Call":[20,[[0,5]]]},{"Layer":[23,79]},{"Token":27},{"And":[[78,80,81],null]},{"Or":[82]},{"Token":31},{"And":[[84],null]},{"Or":[85]},{"Token":15},{"Token":14},{"Token":31},{"And":[[89],null]},{"Token":29},{"And":[[91],null]},{"Or":[90,92]},{"Rep":6},{"And":[[94],null]},{"Or":[95]},{"Opt":8},{"Token":13},{"And":[[98,8],null]},{"Or":[99]},{"Rep":100},{"And":[[97,101],null]},{"Or":[102]},{"Call":[21,[[1,103]]]},{"And":[[104],null]},{"Or":[105]},{"Token":15},{"And":[[6,107],null]},{"Or":[108]},{"Token":14},{"And":[[6,110],null]},{"Or":[111]},{"Token":24},{"Token":31},{"Rep":6},{"Token":25},{"And":[[113,114,115,116],null]},{"Or":[117]},{"ContextualToken":[6,"tokenizer"]},{"Rep":14},{"Call":[21,[[1,120]]]},{"And":[[119,121],1]},{"Or":[122]},{"Opt":16},{"Token":31},{"Opt":19},{"And":[[124,125,19,126],2]},{"Or":[127]},{"ContextualToken":[11,"test"]},{"Token":30},{"And":[[129,130],1]},{"Or":[131]},{"Token":19},{"Token":22},{"Call":[20,[[0,17]]]},{"Layer":[24,135]},{"Token":23},{"And":[[133,134,136,137],null]},{"Or":[138]},{"Token":31},{"Token":26},{"Token":27},{"And":[[141,18,142],null]},{"Or":[143]},{"Opt":144},{"And":[[140,145],null]},{"Or":[146]},{"Token":28},{"And":[[148],null]},{"Token":31},{"And":[[150],null]},{"Or":[149,151]},{"Token":29},{"And":[[153],null]},{"Token":30},{"And":[[155],null]},{"Or":[154,156]},{"Var":0},"Eof",{"And":[[159],null]},{"Token":17},{"And":[[161],null]},{"Or":[160,162]},{"And":[[158,163],1]},{"Or":[164]},{"Rep":165},{"And":[[166],null]},{"Token":20},{"Var":1},{"Layer":[22,169]},{"Token":21},{"And":[[168,170,171],1]},{"Token":20},{"Token":21},{"Call":[25,[[2,173],[3,174]]]},{"And":[[175],null]},{"Token":26},{"Token":27},{"Call":[25,[[2,177],[3,178]]]},{"And":[[179],null]},{"Token":22},{"Token":23},{"Call":[25,[[2,181],[3,182]]]},{"And":[[183],null]},{"Var":2},{"Var":2},{"Var":3},{"Call":[25,[[2,186],[3,187]]]},{"Var":3},{"And":[[185,188,189],null]},{"Var":3},{"Not":191},"Any",{"And":[[192,193],null]},{"Or":[194]},{"And":[[195],null]},{"Or":[190,196]},{"Rep":197},{"And":[[198],null]},{"ContextualToken":[8,"verbatim"]},{"Token":30},{"And":[[200,201],1]},{"Or":[202]},{"ContextualToken":[9,"ast"]},{"Token":20},{"ContextualToken":[3,"node"]},{"And":[[206],null]},{"ContextualToken":[4,"class"]},{"And":[[208],null]},{"ContextualToken":[5,"trait"]},{"And":[[210],null]},{"Or":[207,209,211]},{"And":[[28],null]},{"And":[[29],null]},{"And":[[30],null]},{"Or":[213,214,215]},{"WithSkip":[212,216]},{"Rep":217},{"Token":21},{"And":[[204,205,218,219],1]},{"Or":[220]},{"ContextualToken":[3,"node"]},{"Token":31},{"Token":18},{"Token":31},{"Token":17},{"And":[[226],null]},{"Token":20},{"Not":228},{"Not":229},{"And":[[230],null]},{"Or":[227,231]},{"And":[[225,232],null]},{"Or":[233]},{"Rep":234},{"And":[[224,235],null]},{"Or":[236]},{"Opt":237},{"Rep":31},{"Call":[21,[[1,239]]]},{"And":[[222,223,238,240],1]},{"Or":[241]},{"ContextualToken":[4,"class"]},{"Token":31},{"Token":31},{"Rep":245},{"Call":[21,[[1,246]]]},{"And":[[243,244,247],1]},{"Or":[248]},{"ContextualToken":[5,"trait"]},{"Token":31},{"Rep":31},{"Call":[21,[[1,252]]]},{"And":[[250,251,253],1]},{"Or":[254]},{"Token":31},{"And":[[256,32],null]},{"Or":[257]},{"Token":31},{"Opt":33},{"And":[[259,260],null]},{"Or":[261]},{"Token":15},{"Token":16},{"Token":31},{"And":[[263,264,265],null]},{"Token":16},{"Token":31},{"And":[[267,268],null]},{"Token":14},{"And":[[270],null]},{"Token":15},{"And":[[272],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, EOL_COMMENT, NODE, CLASS, TRAIT, TOKENIZER, RULE, VERBATIM, AST, PUB, TEST, EQ, PIPE, STAR, QUESTION, DOT, COMMA, COLON, HASH, L_CURLY, R_CURLY, L_SQUARE, R_SQUARE, L_ANGLE, R_ANGLE, L_PAREN, R_PAREN, NUMBER, SIMPLE_STRING, HASH_STRING, IDENT, FALL_FILE, SYN_RULE, PARAMETERS, PARAMETER, REF_EXPR, SEQ_EXPR, BLOCK_EXPR, OPT_EXPR, REP_EXPR, CALL_EXPR, TOKENIZER_DEF, LEX_RULE, TEST_DEF, ATTRIBUTES, ATTRIBUTE, ATTRIBUTE_VALUE, STRING, VERBATIM_DEF, AST_DEF, AST_NODE_DEF, AST_CLASS_DEF, AST_TRAIT_DEF, METHOD_DEF, AST_SELECTOR,
            ],
            syntactical_rules: rt::parser_from_str(parser_json),

            .. Default::default()
        }
    }
    use self::rt::*;
    lazy_static! {
        static ref LANG: rt::Language = {
            struct Impl { parser_definition: rt::ParserDefinition, lexer: rt::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &::std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => rt::NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        EOL_COMMENT => rt::NodeTypeInfo { name: "EOL_COMMENT", whitespace_like: true },
                        NODE => rt::NodeTypeInfo { name: "NODE", whitespace_like: false },
                        CLASS => rt::NodeTypeInfo { name: "CLASS", whitespace_like: false },
                        TRAIT => rt::NodeTypeInfo { name: "TRAIT", whitespace_like: false },
                        TOKENIZER => rt::NodeTypeInfo { name: "TOKENIZER", whitespace_like: false },
                        RULE => rt::NodeTypeInfo { name: "RULE", whitespace_like: false },
                        VERBATIM => rt::NodeTypeInfo { name: "VERBATIM", whitespace_like: false },
                        AST => rt::NodeTypeInfo { name: "AST", whitespace_like: false },
                        PUB => rt::NodeTypeInfo { name: "PUB", whitespace_like: false },
                        TEST => rt::NodeTypeInfo { name: "TEST", whitespace_like: false },
                        EQ => rt::NodeTypeInfo { name: "EQ", whitespace_like: false },
                        PIPE => rt::NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        STAR => rt::NodeTypeInfo { name: "STAR", whitespace_like: false },
                        QUESTION => rt::NodeTypeInfo { name: "QUESTION", whitespace_like: false },
                        DOT => rt::NodeTypeInfo { name: "DOT", whitespace_like: false },
                        COMMA => rt::NodeTypeInfo { name: "COMMA", whitespace_like: false },
                        COLON => rt::NodeTypeInfo { name: "COLON", whitespace_like: false },
                        HASH => rt::NodeTypeInfo { name: "HASH", whitespace_like: false },
                        L_CURLY => rt::NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                        R_CURLY => rt::NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                        L_SQUARE => rt::NodeTypeInfo { name: "L_SQUARE", whitespace_like: false },
                        R_SQUARE => rt::NodeTypeInfo { name: "R_SQUARE", whitespace_like: false },
                        L_ANGLE => rt::NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                        R_ANGLE => rt::NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                        L_PAREN => rt::NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                        R_PAREN => rt::NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                        NUMBER => rt::NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        SIMPLE_STRING => rt::NodeTypeInfo { name: "SIMPLE_STRING", whitespace_like: false },
                        HASH_STRING => rt::NodeTypeInfo { name: "HASH_STRING", whitespace_like: false },
                        IDENT => rt::NodeTypeInfo { name: "IDENT", whitespace_like: false },
                        FALL_FILE => rt::NodeTypeInfo { name: "FALL_FILE", whitespace_like: false },
                        SYN_RULE => rt::NodeTypeInfo { name: "SYN_RULE", whitespace_like: false },
                        PARAMETERS => rt::NodeTypeInfo { name: "PARAMETERS", whitespace_like: false },
                        PARAMETER => rt::NodeTypeInfo { name: "PARAMETER", whitespace_like: false },
                        REF_EXPR => rt::NodeTypeInfo { name: "REF_EXPR", whitespace_like: false },
                        SEQ_EXPR => rt::NodeTypeInfo { name: "SEQ_EXPR", whitespace_like: false },
                        BLOCK_EXPR => rt::NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                        OPT_EXPR => rt::NodeTypeInfo { name: "OPT_EXPR", whitespace_like: false },
                        REP_EXPR => rt::NodeTypeInfo { name: "REP_EXPR", whitespace_like: false },
                        CALL_EXPR => rt::NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                        TOKENIZER_DEF => rt::NodeTypeInfo { name: "TOKENIZER_DEF", whitespace_like: false },
                        LEX_RULE => rt::NodeTypeInfo { name: "LEX_RULE", whitespace_like: false },
                        TEST_DEF => rt::NodeTypeInfo { name: "TEST_DEF", whitespace_like: false },
                        ATTRIBUTES => rt::NodeTypeInfo { name: "ATTRIBUTES", whitespace_like: false },
                        ATTRIBUTE => rt::NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                        ATTRIBUTE_VALUE => rt::NodeTypeInfo { name: "ATTRIBUTE_VALUE", whitespace_like: false },
                        STRING => rt::NodeTypeInfo { name: "STRING", whitespace_like: false },
                        VERBATIM_DEF => rt::NodeTypeInfo { name: "VERBATIM_DEF", whitespace_like: false },
                        AST_DEF => rt::NodeTypeInfo { name: "AST_DEF", whitespace_like: false },
                        AST_NODE_DEF => rt::NodeTypeInfo { name: "AST_NODE_DEF", whitespace_like: false },
                        AST_CLASS_DEF => rt::NodeTypeInfo { name: "AST_CLASS_DEF", whitespace_like: false },
                        AST_TRAIT_DEF => rt::NodeTypeInfo { name: "AST_TRAIT_DEF", whitespace_like: false },
                        METHOD_DEF => rt::NodeTypeInfo { name: "METHOD_DEF", whitespace_like: false },
                        AST_SELECTOR => rt::NodeTypeInfo { name: "AST_SELECTOR", whitespace_like: false },
                        _ => panic!("Unknown rt::NodeType: {:?}", ty)
                    }
                }
            }

            rt::Language::new(Impl {
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

#[allow(unused)]
use self::rt::AstNode;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallFile<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for FallFile<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == FALL_FILE {
            Some(FallFile { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> FallFile<'f> {
    pub fn tokenizer_def(&self) -> Option<TokenizerDef<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn syn_rules(&self) -> rt::AstChildren<'f, SynRule<'f>> {
        rt::AstChildren::new(self.node().children())
    }
    pub fn verbatim_def(&self) -> Option<VerbatimDef<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn ast_def(&self) -> Option<AstDef<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn tests(&self) -> rt::AstChildren<'f, TestDef<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct TokenizerDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TokenizerDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TOKENIZER_DEF {
            Some(TokenizerDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TokenizerDef<'f> {
    pub fn lex_rules(&self) -> rt::AstChildren<'f, LexRule<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct LexRule<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for LexRule<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == LEX_RULE {
            Some(LexRule { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> LexRule<'f> {
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn node_type(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
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
pub struct SynRule<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for SynRule<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == SYN_RULE {
            Some(SynRule { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> SynRule<'f> {
    pub fn attributes(&self) -> Option<Attributes<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn name_ident(&self) -> Option<rt::Node<'f>> {
        self.node().children().find(|n| n.ty() == IDENT)
    }
    pub fn name(&self) -> Option<rt::Text<'f>> {
        rt::child_of_type(self.node(), IDENT).map(|n| n.text())
    }
    pub fn body(&self) -> Expr<'f> {
        rt::AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn parameters(&self) -> Option<Parameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
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
pub struct Parameters<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for Parameters<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == PARAMETERS {
            Some(Parameters { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> Parameters<'f> {
    pub fn parameters(&self) -> rt::AstChildren<'f, Parameter<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct Parameter<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for Parameter<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == PARAMETER {
            Some(Parameter { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> Parameter<'f> {
    pub fn name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
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
pub struct Attributes<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for Attributes<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTES {
            Some(Attributes { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> Attributes<'f> {
    pub fn attributes(&self) -> rt::AstChildren<'f, Attribute<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct Attribute<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for Attribute<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTE {
            Some(Attribute { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> Attribute<'f> {
    pub fn name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn value(&self) -> Option<AttributeValue<'f>> {
        rt::AstChildren::new(self.node().children()).next()
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
pub struct AttributeValue<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AttributeValue<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == ATTRIBUTE_VALUE {
            Some(AttributeValue { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
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
pub struct VerbatimDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for VerbatimDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == VERBATIM_DEF {
            Some(VerbatimDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> VerbatimDef<'f> {
    pub fn literal_string(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), HASH_STRING).text()
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
pub struct AstDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AstDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == AST_DEF {
            Some(AstDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> AstDef<'f> {
    pub fn ast_nodes(&self) -> rt::AstChildren<'f, AstNodeDef<'f>> {
        rt::AstChildren::new(self.node().children())
    }
    pub fn ast_classes(&self) -> rt::AstChildren<'f, AstClassDef<'f>> {
        rt::AstChildren::new(self.node().children())
    }
    pub fn ast_traits(&self) -> rt::AstChildren<'f, AstTraitDef<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct AstNodeDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AstNodeDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == AST_NODE_DEF {
            Some(AstNodeDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> AstNodeDef<'f> {
    pub fn name_ident(&self) -> rt::Node<'f> {
        self.node().children().find(|n| n.ty() == IDENT).unwrap()
    }
    pub fn name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn methods(&self) -> rt::AstChildren<'f, MethodDef<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct AstTraitDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AstTraitDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == AST_TRAIT_DEF {
            Some(AstTraitDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> AstTraitDef<'f> {
    pub fn name_ident(&self) -> rt::Node<'f> {
        self.node().children().find(|n| n.ty() == IDENT).unwrap()
    }
    pub fn name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn methods(&self) -> rt::AstChildren<'f, MethodDef<'f>> {
        rt::AstChildren::new(self.node().children())
    }
}

impl<'f> ::std::fmt::Debug for AstTraitDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("AstTraitDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstClassDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AstClassDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == AST_CLASS_DEF {
            Some(AstClassDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> AstClassDef<'f> {
    pub fn name_ident(&self) -> rt::Node<'f> {
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
pub struct MethodDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for MethodDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == METHOD_DEF {
            Some(MethodDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> MethodDef<'f> {
    pub fn name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn selector(&self) -> AstSelector<'f> {
        rt::AstChildren::new(self.node().children()).next().unwrap()
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
pub struct AstSelector<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for AstSelector<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == AST_SELECTOR {
            Some(AstSelector { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> AstSelector<'f> {
    pub fn child(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn optional(&self) -> Option<rt::Node<'f>> {
        self.node().children().find(|n| n.ty() == QUESTION)
    }
    pub fn many(&self) -> Option<rt::Node<'f>> {
        self.node().children().find(|n| n.ty() == STAR)
    }
    pub fn dot(&self) -> Option<rt::Node<'f>> {
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
pub struct TestDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TestDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TEST_DEF {
            Some(TestDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TestDef<'f> {
    pub fn literal_string(&self) -> Option<rt::Text<'f>> {
        rt::child_of_type(self.node(), HASH_STRING).map(|n| n.text())
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
pub struct RefExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for RefExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == REF_EXPR {
            Some(RefExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
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
pub struct CallExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for CallExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == CALL_EXPR {
            Some(CallExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> CallExpr<'f> {
    pub fn fn_name(&self) -> rt::Text<'f> {
        rt::child_of_type_exn(self.node(), IDENT).text()
    }
    pub fn args(&self) -> rt::AstChildren<'f, Expr<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct BlockExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for BlockExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == BLOCK_EXPR {
            Some(BlockExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> BlockExpr<'f> {
    pub fn alts(&self) -> rt::AstChildren<'f, Expr<'f>> {
        rt::AstChildren::new(self.node().children())
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
pub struct OptExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for OptExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == OPT_EXPR {
            Some(OptExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> OptExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        rt::AstChildren::new(self.node().children()).next().unwrap()
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
pub struct RepExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for RepExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == REP_EXPR {
            Some(RepExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> RepExpr<'f> {
    pub fn expr(&self) -> Expr<'f> {
        rt::AstChildren::new(self.node().children()).next().unwrap()
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
pub struct SeqExpr<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for SeqExpr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == SEQ_EXPR {
            Some(SeqExpr { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> rt::AstChildren<'f, Expr<'f>> {
        rt::AstChildren::new(self.node().children())
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

impl<'f> rt::AstNode<'f> for Expr<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
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

    fn node(self) -> rt::Node<'f> {
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
        rt::AstNode::node(*self).range().fmt(f)?;
        Ok(())
    }
}