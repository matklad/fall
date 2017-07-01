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
pub const EQ: NodeType = NodeType(116);
pub const SEMI: NodeType = NodeType(117);
pub const COLON: NodeType = NodeType(118);
pub const COLONCOLON: NodeType = NodeType(119);
pub const COMMA: NodeType = NodeType(120);
pub const STAR: NodeType = NodeType(121);
pub const IDENT: NodeType = NodeType(122);
pub const NUMBER: NodeType = NodeType(123);
pub const FILE: NodeType = NodeType(124);
pub const USE_DECL: NodeType = NodeType(125);
pub const USE_SPEC: NodeType = NodeType(126);
pub const USE_SPEC_ENTRY: NodeType = NodeType(127);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(128);
pub const FN_DEF: NodeType = NodeType(129);
pub const VALUE_PARAM: NodeType = NodeType(130);
pub const STRUCT_DEF: NodeType = NodeType(131);
pub const MOD_DEF: NodeType = NodeType(132);
pub const STRUCT_FIELD: NodeType = NodeType(133);
pub const TUPLE_FIELD: NodeType = NodeType(134);
pub const VISIBILITY: NodeType = NodeType(135);
pub const PATH: NodeType = NodeType(136);
pub const TYPE_ARGUMENTS: NodeType = NodeType(137);
pub const ALIAS: NodeType = NodeType(138);
pub const TYPE: NodeType = NodeType(139);
pub const STMT: NodeType = NodeType(140);
pub const PATTERN: NodeType = NodeType(141);
pub const EXPR: NodeType = NodeType(142);
pub const BLOCK_EXPR: NodeType = NodeType(143);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, LPAREN, RPAREN, LBRACE, RBRACE, LANGLE, RANGLE, EQ, SEMI, COLON, COLONCOLON, COMMA, STAR, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, VALUE_PARAM, STRUCT_DEF, MOD_DEF, STRUCT_FIELD, TUPLE_FIELD, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, TYPE, STMT, PATTERN, EXPR, BLOCK_EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[25,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":7}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":10}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":12}],null]}]}},{"body":{"Pub":[26,{"Or":[{"And":[[{"Token":9},{"Or":[{"And":[[{"Rule":16},{"Or":[{"And":[[{"Rule":24}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":20},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":20}},{"Rule":5}],null]}]},{"Token":18}],1]}]}]}},{"body":{"Pub":[27,{"Or":[{"And":[[{"Token":22}],null]},{"And":[[{"Call":[{"Rule":31},[[1,{"Call":[{"Rule":30},[[0,{"Rule":6}]]]}]]]}],null]}]}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Token":23},{"Opt":{"Rule":24}}],1]}]}]}},{"body":{"Pub":[29,{"Or":[{"And":[[{"Token":4},{"Token":3},{"Token":23},{"Opt":{"Rule":24}},{"Token":18}],2]}]}]}},{"body":{"Pub":[30,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":5},{"Token":23},{"Rule":9},{"Rule":29}],2]}]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":32},[[2,{"Call":[{"Rule":30},[[0,{"Rule":10}]]]}]]]}],null]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Rule":27},{"Token":19},{"Rule":25}],null]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":8},{"Token":23},{"Or":[{"And":[[{"Call":[{"Rule":31},[[1,{"Call":[{"Rule":30},[[0,{"Rule":13}]]]}]]]}],null]},{"And":[[{"Token":18}],null]},{"And":[[{"Call":[{"Rule":32},[[2,{"Call":[{"Rule":30},[[0,{"Rule":14}]]]}]]]},{"Token":18}],null]}]}],2]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":10},{"Token":23},{"Or":[{"And":[[{"Token":18}],null]},{"And":[[{"Call":[{"Rule":31},[[1,{"Rule":1}]]]}],null]}]}],2]}]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Token":23},{"Token":19},{"Rule":25}],2]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Opt":{"Rule":15}},{"Rule":25}],null]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Token":7}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":19}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":19}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":19}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[37,{"Or":[{"And":[[{"Opt":{"Token":20}},{"Rule":22}],null]}]}]}}},{"Postfix":{"ty":37,"op":{"Or":[{"And":[[{"Token":20},{"Rule":22}],null]}]}}}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Opt":{"Token":20}},{"Rule":22}],null]}]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Rule":19},{"Or":[{"And":[[{"Token":20},{"Rule":22}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":23},{"Opt":{"Rule":23}}],null]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":20}],null]}]},{"Call":[{"Rule":33},[[3,{"Call":[{"Rule":30},[[0,{"Rule":25}]]]}]]]}],null]}]}]}},{"body":{"Pub":[39,{"Or":[{"And":[[{"Token":2},{"Token":23}],null]}]}]}},{"body":{"Pub":[40,{"Or":[{"And":[[{"Rule":17}],null]}]}]}},{"body":{"Pub":[41,{"Or":[{"And":[[{"Token":6},{"Rule":27},{"Token":17},{"Rule":28},{"Token":18}],1]}]}]}},{"body":{"Pub":[42,{"Or":[{"And":[[{"Token":23}],null]}]}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Token":24}],null]},{"And":[[{"Rule":18}],null]}]}]}},{"body":{"Pub":[44,{"Or":[{"And":[[{"Call":[{"Rule":31},[[1,{"Rep":{"Rule":26}}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":0},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":21}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":34},[[4,{"Token":13}],[5,{"Token":14}],[6,{"Var":1}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":34},[[4,{"Token":11}],[5,{"Token":12}],[6,{"Var":2}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":34},[[4,{"Token":15}],[5,{"Token":16}],[6,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":4},{"Layer":[{"Call":[{"Rule":35},[[7,{"Var":4}],[8,{"Var":5}]]]},{"Var":6}]},{"Var":5}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":7},{"Call":[{"Rule":35},[[7,{"Var":7}],[8,{"Var":8}]]]},{"Var":8}],1]},{"And":[[{"Not":{"Var":8}}],null]}]}}],null]}]}}]"##;
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
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
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
                    STMT => NodeTypeInfo { name: "STMT", whitespace_like: false },
                    PATTERN => NodeTypeInfo { name: "PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
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
                LexRule::new(EQ, "=", None),
                LexRule::new(SEMI, ";", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COLONCOLON, "::", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(STAR, "\\*", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


