use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const AS: NodeType = NodeType(101);
pub const CRATE: NodeType = NodeType(102);
pub const EXTERN: NodeType = NodeType(103);
pub const FN: NodeType = NodeType(104);
pub const LET: NodeType = NodeType(105);
pub const PUB: NodeType = NodeType(106);
pub const STRUCT: NodeType = NodeType(107);
pub const USE: NodeType = NodeType(108);
pub const MOD: NodeType = NodeType(109);
pub const LPAREN: NodeType = NodeType(110);
pub const RPAREN: NodeType = NodeType(111);
pub const LBRACE: NodeType = NodeType(112);
pub const RBRACE: NodeType = NodeType(113);
pub const LANGLE: NodeType = NodeType(114);
pub const RANGLE: NodeType = NodeType(115);
pub const THIN_ARROW: NodeType = NodeType(116);
pub const EQ: NodeType = NodeType(117);
pub const SEMI: NodeType = NodeType(118);
pub const COLON: NodeType = NodeType(119);
pub const COLONCOLON: NodeType = NodeType(120);
pub const COMMA: NodeType = NodeType(121);
pub const PLUS: NodeType = NodeType(122);
pub const MINUS: NodeType = NodeType(123);
pub const STAR: NodeType = NodeType(124);
pub const SLASH: NodeType = NodeType(125);
pub const PERCENT: NodeType = NodeType(126);
pub const UNDERSCORE: NodeType = NodeType(127);
pub const IDENT: NodeType = NodeType(128);
pub const NUMBER: NodeType = NodeType(129);
pub const FILE: NodeType = NodeType(130);
pub const USE_DECL: NodeType = NodeType(131);
pub const USE_SPEC: NodeType = NodeType(132);
pub const USE_SPEC_ENTRY: NodeType = NodeType(133);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(134);
pub const FN_DEF: NodeType = NodeType(135);
pub const VALUE_PARAM: NodeType = NodeType(136);
pub const STRUCT_DEF: NodeType = NodeType(137);
pub const MOD_DEF: NodeType = NodeType(138);
pub const STRUCT_FIELD: NodeType = NodeType(139);
pub const TUPLE_FIELD: NodeType = NodeType(140);
pub const VISIBILITY: NodeType = NodeType(141);
pub const PATH: NodeType = NodeType(142);
pub const TYPE_ARGUMENTS: NodeType = NodeType(143);
pub const ALIAS: NodeType = NodeType(144);
pub const TYPE: NodeType = NodeType(145);
pub const PATTERN: NodeType = NodeType(146);
pub const EXPR: NodeType = NodeType(147);
pub const PRODUCT_EXPR: NodeType = NodeType(148);
pub const SUM_EXPR: NodeType = NodeType(149);
pub const LITERAL: NodeType = NodeType(150);
pub const PATH_EXPR: NodeType = NodeType(151);
pub const BLOCK_EXPR: NodeType = NodeType(152);
pub const LET_STMT: NodeType = NodeType(153);
pub const EXPR_STMT: NodeType = NodeType(154);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, LPAREN, RPAREN, LBRACE, RBRACE, LANGLE, RANGLE, THIN_ARROW, EQ, SEMI, COLON, COLONCOLON, COMMA, PLUS, MINUS, STAR, SLASH, PERCENT, UNDERSCORE, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, VALUE_PARAM, STRUCT_DEF, MOD_DEF, STRUCT_FIELD, TUPLE_FIELD, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, TYPE, PATTERN, EXPR, PRODUCT_EXPR, SUM_EXPR, LITERAL, PATH_EXPR, BLOCK_EXPR, LET_STMT, EXPR_STMT,
        ];
        let parser_json = r##"[{"body":{"Pub":[31,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":7}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":10}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":12}],null]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Token":9},{"Or":[{"And":[[{"Rule":16},{"Or":[{"And":[[{"Rule":24}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":21},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":21}},{"Rule":5}],null]}]},{"Token":19}],1]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":25}],null]},{"And":[[{"Call":[{"Rule":37},[[1,{"Call":[{"Rule":36},[[0,{"Rule":6}]]]}]]]}],null]}]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Token":29},{"Opt":{"Rule":24}}],1]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Token":4},{"Token":3},{"Token":29},{"Opt":{"Rule":24}},{"Token":19}],2]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":5},{"Token":29},{"Rule":9},{"Opt":{"Or":[{"And":[[{"Token":17},{"Rule":25}],null]}]}},{"Rule":32}],2]}]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":38},[[2,{"Call":[{"Rule":36},[[0,{"Rule":10}]]]}]]]}],null]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Rule":26},{"Token":20},{"Rule":25}],null]}]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":8},{"Token":29},{"Or":[{"And":[[{"Call":[{"Rule":37},[[1,{"Call":[{"Rule":36},[[0,{"Rule":13}]]]}]]]}],null]},{"And":[[{"Token":19}],null]},{"And":[[{"Call":[{"Rule":38},[[2,{"Call":[{"Rule":36},[[0,{"Rule":14}]]]}]]]},{"Token":19}],null]}]}],2]}]}]}},{"body":{"Pub":[39,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":10},{"Token":29},{"Or":[{"And":[[{"Token":19}],null]},{"And":[[{"Call":[{"Rule":37},[[1,{"Rule":1}]]]}],null]}]}],2]}]}]}},{"body":{"Pub":[40,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":29},{"Token":20},{"Rule":25}],2]}]}]}},{"body":{"Pub":[41,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Rule":25}],null]}]}]}},{"body":{"Pub":[42,{"Or":[{"And":[[{"Token":7}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":19}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":19}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":19}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[43,{"Or":[{"And":[[{"Opt":{"Token":21}},{"Rule":22}],null]}]}]}}},{"Postfix":{"ty":43,"op":{"Or":[{"And":[[{"Token":21},{"Rule":22}],null]}]}}}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Opt":{"Token":21}},{"Rule":22}],null]}]}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Rule":19},{"Or":[{"And":[[{"Token":21},{"Rule":22}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":29},{"Opt":{"Rule":23}}],null]}]}},{"body":{"Pub":[44,{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":21}],null]}]},{"Call":[{"Rule":39},[[3,{"Call":[{"Rule":36},[[0,{"Rule":25}]]]}]]]}],null]}]}]}},{"body":{"Pub":[45,{"Or":[{"And":[[{"Token":2},{"Token":29}],null]}]}]}},{"body":{"Pub":[46,{"Or":[{"And":[[{"Rule":17}],null]}]}]}},{"body":{"Pub":[47,{"Or":[{"And":[[{"Token":29}],null]},{"And":[[{"Token":28}],null]}]}]}},{"body":{"Pratt":[{"Binary":{"ty":50,"op":{"Or":[{"And":[[{"Token":23}],null]},{"And":[[{"Token":24}],null]}]},"priority":1}},{"Binary":{"ty":49,"op":{"Or":[{"And":[[{"Token":25}],null]},{"And":[[{"Token":26}],null]},{"And":[[{"Token":27}],null]}]},"priority":2}},{"Atom":{"body":{"Pub":[51,{"Or":[{"And":[[{"Token":30}],null]}]}]}}},{"Atom":{"body":{"Pub":[52,{"Or":[{"And":[[{"Rule":18}],null]}]}]}}},{"Atom":{"body":{"Pub":[53,{"Or":[{"And":[[{"Call":[{"Rule":37},[[1,{"Or":[{"And":[[{"Rep":{"Rule":33}},{"Opt":{"Rule":27}}],null]}]}]]]}],null]}]}]}}}]}},{"body":{"Pub":[49,{"Or":[{"And":[[{"Rule":27},{"Or":[{"And":[[{"Token":25}],null]},{"And":[[{"Token":26}],null]},{"And":[[{"Token":27}],null]}]},{"Rule":27}],null]}]}]}},{"body":{"Pub":[50,{"Or":[{"And":[[{"Rule":27},{"Or":[{"And":[[{"Token":23}],null]},{"And":[[{"Token":24}],null]}]},{"Rule":27}],null]}]}]}},{"body":{"Pub":[51,{"Or":[{"And":[[{"Token":30}],null]}]}]}},{"body":{"Pub":[52,{"Or":[{"And":[[{"Rule":18}],null]}]}]}},{"body":{"Pub":[53,{"Or":[{"And":[[{"Call":[{"Rule":37},[[1,{"Or":[{"And":[[{"Rep":{"Rule":33}},{"Opt":{"Rule":27}}],null]}]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":34}],null]},{"And":[[{"Rule":35}],null]}]}},{"body":{"Pub":[54,{"Or":[{"And":[[{"Token":6},{"Rule":26},{"Token":18},{"Rule":27},{"Token":19}],1]}]}]}},{"body":{"Pub":[55,{"Or":[{"And":[[{"Rule":27},{"Token":19}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":0},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":22}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":40},[[4,{"Token":13}],[5,{"Token":14}],[6,{"Var":1}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":40},[[4,{"Token":11}],[5,{"Token":12}],[6,{"Var":2}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":40},[[4,{"Token":15}],[5,{"Token":16}],[6,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":4},{"Layer":[{"Call":[{"Rule":41},[[7,{"Var":4}],[8,{"Var":5}]]]},{"Var":6}]},{"Var":5}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":7},{"Call":[{"Rule":41},[[7,{"Var":7}],[8,{"Var":8}]]]},{"Var":8}],1]},{"And":[[{"Not":{"Var":8}}],null]}]}}],null]}]}}]"##;
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
                    AS => NodeTypeInfo { name: "AS", whitespace_like: false },
                    CRATE => NodeTypeInfo { name: "CRATE", whitespace_like: false },
                    EXTERN => NodeTypeInfo { name: "EXTERN", whitespace_like: false },
                    FN => NodeTypeInfo { name: "FN", whitespace_like: false },
                    LET => NodeTypeInfo { name: "LET", whitespace_like: false },
                    PUB => NodeTypeInfo { name: "PUB", whitespace_like: false },
                    STRUCT => NodeTypeInfo { name: "STRUCT", whitespace_like: false },
                    USE => NodeTypeInfo { name: "USE", whitespace_like: false },
                    MOD => NodeTypeInfo { name: "MOD", whitespace_like: false },
                    LPAREN => NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                    RPAREN => NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                    LBRACE => NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                    RBRACE => NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                    LANGLE => NodeTypeInfo { name: "LANGLE", whitespace_like: false },
                    RANGLE => NodeTypeInfo { name: "RANGLE", whitespace_like: false },
                    THIN_ARROW => NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                    MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                    STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                    SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                    PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                    UNDERSCORE => NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                    IDENT => NodeTypeInfo { name: "IDENT", whitespace_like: false },
                    NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                    FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                    USE_DECL => NodeTypeInfo { name: "USE_DECL", whitespace_like: false },
                    USE_SPEC => NodeTypeInfo { name: "USE_SPEC", whitespace_like: false },
                    USE_SPEC_ENTRY => NodeTypeInfo { name: "USE_SPEC_ENTRY", whitespace_like: false },
                    EXTERN_CRATE_DECL => NodeTypeInfo { name: "EXTERN_CRATE_DECL", whitespace_like: false },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF", whitespace_like: false },
                    VALUE_PARAM => NodeTypeInfo { name: "VALUE_PARAM", whitespace_like: false },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                    MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                    TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                    ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                    TYPE => NodeTypeInfo { name: "TYPE", whitespace_like: false },
                    PATTERN => NodeTypeInfo { name: "PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                    LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                    PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                    EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(AS, "as", None),
                LexRule::new(CRATE, "crate", None),
                LexRule::new(EXTERN, "extern", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(LET, "let", None),
                LexRule::new(PUB, "pub", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(USE, "use", None),
                LexRule::new(MOD, "mod", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(LANGLE, "<", None),
                LexRule::new(RANGLE, ">", None),
                LexRule::new(THIN_ARROW, "\\->", None),
                LexRule::new(EQ, "=", None),
                LexRule::new(SEMI, ";", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COLONCOLON, "::", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(PLUS, "\\+", None),
                LexRule::new(MINUS, "\\-", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(SLASH, "/", None),
                LexRule::new(PERCENT, "%", None),
                LexRule::new(UNDERSCORE, "_", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


