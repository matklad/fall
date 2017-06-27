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
pub const EQ: NodeType = NodeType(114);
pub const SEMI: NodeType = NodeType(115);
pub const COLON: NodeType = NodeType(116);
pub const COLONCOLON: NodeType = NodeType(117);
pub const COMMA: NodeType = NodeType(118);
pub const STAR: NodeType = NodeType(119);
pub const IDENT: NodeType = NodeType(120);
pub const NUMBER: NodeType = NodeType(121);
pub const FILE: NodeType = NodeType(122);
pub const USE_DECL: NodeType = NodeType(123);
pub const USE_SPEC: NodeType = NodeType(124);
pub const USE_SPEC_ENTRY: NodeType = NodeType(125);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(126);
pub const FN_DEF: NodeType = NodeType(127);
pub const STRUCT_DEF: NodeType = NodeType(128);
pub const MOD_DEF: NodeType = NodeType(129);
pub const STRUCT_FIELD: NodeType = NodeType(130);
pub const TUPLE_FIELD: NodeType = NodeType(131);
pub const VISIBILITY: NodeType = NodeType(132);
pub const PATH: NodeType = NodeType(133);
pub const ALIAS: NodeType = NodeType(134);
pub const TYPE: NodeType = NodeType(135);
pub const STMT: NodeType = NodeType(136);
pub const PATTERN: NodeType = NodeType(137);
pub const EXPR: NodeType = NodeType(138);
pub const BLOCK_EXPR: NodeType = NodeType(139);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, LPAREN, RPAREN, LBRACE, RBRACE, EQ, SEMI, COLON, COLONCOLON, COMMA, STAR, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, STRUCT_DEF, MOD_DEF, STRUCT_FIELD, TUPLE_FIELD, VISIBILITY, PATH, ALIAS, TYPE, STMT, PATTERN, EXPR, BLOCK_EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[23,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":7}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":10}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":9}],null]},{"And":[[{"Rule":10}],null]}]}},{"body":{"Pub":[24,{"Or":[{"And":[[{"Token":9},{"Or":[{"And":[[{"Rule":14},{"Or":[{"And":[[{"Rule":17}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":18},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":18}},{"Rule":5}],null]}]},{"Token":16}],1]}]}]}},{"body":{"Pub":[25,{"Or":[{"And":[[{"Token":20}],null]},{"And":[[{"Token":13},{"Layer":[{"Rule":23},{"Rep":{"Rule":6}}]},{"Token":14}],null]}]}]}},{"body":{"Pub":[26,{"Or":[{"And":[[{"Token":21},{"Opt":{"Rule":17}},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":19}],null]}]}],1]}]}]}},{"body":{"Pub":[27,{"Or":[{"And":[[{"Token":4},{"Token":3},{"Token":21},{"Opt":{"Rule":17}},{"Token":16}],2]}]}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Token":5},{"Token":21},{"Token":11},{"Token":12},{"Rule":22}],2]}]}]}},{"body":{"Pub":[29,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Token":8},{"Token":21},{"Or":[{"And":[[{"Token":13},{"Layer":[{"Rule":23},{"Rep":{"Rule":11}}]},{"Token":14}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":11},{"Layer":[{"Rule":25},{"Rep":{"Rule":12}}]},{"Token":12},{"Token":16}],null]}]}],2]}]}]}},{"body":{"Pub":[30,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Token":10},{"Token":21},{"Or":[{"And":[[{"Token":16}],null]},{"And":[[{"Token":13},{"Layer":[{"Rule":23},{"Rule":1}]},{"Token":14}],null]}]}],2]}]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Token":21},{"Token":17},{"Rule":18},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":19}],null]}]}],2]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Opt":{"Rule":13}},{"Rule":18},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":19}],null]}]}],2]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":7}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[34,{"Or":[{"And":[[{"Opt":{"Token":18}},{"Token":21}],null]}]}]}}},{"Postfix":{"ty":34,"op":{"Or":[{"And":[[{"Token":18},{"Token":21}],null]}]}}}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Opt":{"Token":18}},{"Token":21}],null]}]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Rule":14},{"Or":[{"And":[[{"Token":18},{"Token":21}],null]}]}],null]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Token":2},{"Token":21}],null]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Token":21}],null]}]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Token":6},{"Rule":20},{"Token":15},{"Rule":21},{"Token":16}],1]}]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"Token":21}],null]}]}]}},{"body":{"Pub":[39,{"Or":[{"And":[[{"Token":22}],null]}]}]}},{"body":{"Pub":[40,{"Or":[{"And":[[{"Token":13},{"Layer":[{"Rule":23},{"Rep":{"Rule":19}}]},{"Token":14}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":24}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":13},{"Rule":23},{"Token":14}],1]},{"And":[[{"Not":[14]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":26}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":11},{"Rule":25},{"Token":12}],1]},{"And":[[{"Not":[12]}],null]}]}}]"##;
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
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                    MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
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


