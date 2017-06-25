use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use fall_tree::{ERROR, WHITESPACE};

pub const AS: NodeType = NodeType(100);
pub const CRATE: NodeType = NodeType(101);
pub const EXTERN: NodeType = NodeType(102);
pub const FN: NodeType = NodeType(103);
pub const LET: NodeType = NodeType(104);
pub const PUB: NodeType = NodeType(105);
pub const STRUCT: NodeType = NodeType(106);
pub const USE: NodeType = NodeType(107);
pub const LPAREN: NodeType = NodeType(108);
pub const RPAREN: NodeType = NodeType(109);
pub const LBRACE: NodeType = NodeType(110);
pub const RBRACE: NodeType = NodeType(111);
pub const EQ: NodeType = NodeType(112);
pub const SEMI: NodeType = NodeType(113);
pub const COLON: NodeType = NodeType(114);
pub const COLONCOLON: NodeType = NodeType(115);
pub const COMMA: NodeType = NodeType(116);
pub const STAR: NodeType = NodeType(117);
pub const IDENT: NodeType = NodeType(118);
pub const NUMBER: NodeType = NodeType(119);
pub const FILE: NodeType = NodeType(120);
pub const USE_DECL: NodeType = NodeType(121);
pub const USE_SPEC: NodeType = NodeType(122);
pub const USE_SPEC_ENTRY: NodeType = NodeType(123);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(124);
pub const FN_DEF: NodeType = NodeType(125);
pub const STRUCT_DEF: NodeType = NodeType(126);
pub const STRUCT_FIELD: NodeType = NodeType(127);
pub const TUPLE_FIELD: NodeType = NodeType(128);
pub const VISIBILITY: NodeType = NodeType(129);
pub const PATH: NodeType = NodeType(130);
pub const ALIAS: NodeType = NodeType(131);
pub const TYPE: NodeType = NodeType(132);
pub const STMT: NodeType = NodeType(133);
pub const PATTERN: NodeType = NodeType(134);
pub const EXPR: NodeType = NodeType(135);
pub const BLOCK_EXPR: NodeType = NodeType(136);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, LPAREN, RPAREN, LBRACE, RBRACE, EQ, SEMI, COLON, COLONCOLON, COMMA, STAR, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, VISIBILITY, PATH, ALIAS, TYPE, STMT, PATTERN, EXPR, BLOCK_EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[22,{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":1},{"Rule":2}]}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":7}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"Token":4}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":3}],null]},{"And":[[{"Rule":6}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]}]}},{"body":{"Pub":[23,{"Or":[{"And":[[{"Token":9},{"Or":[{"And":[[{"Rule":12},{"Or":[{"And":[[{"Opt":{"Token":17}}],null]},{"And":[[{"Rule":15}],null]}]},{"Token":15}],null]},{"And":[[{"Token":17},{"Rule":4},{"Token":15}],null]},{"And":[[{"NotAhead":{"Token":19}},{"Rule":4},{"Token":15}],null]}]}],1]}]}]}},{"body":{"Pub":[24,{"Or":[{"And":[[{"Token":19}],null]},{"And":[[{"Token":12},{"Layer":[{"Rule":21},{"Rep":{"Rule":5}}]},{"Token":13}],null]}]}]}},{"body":{"Pub":[25,{"Or":[{"And":[[{"Token":20},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":18}],null]}]}],null]}]}]}},{"body":{"Pub":[26,{"Or":[{"And":[[{"Token":4},{"Token":3},{"Token":20},{"Token":15}],2]}]}]}},{"body":{"Pub":[27,{"Or":[{"And":[[{"Opt":{"Rule":11}},{"Token":5},{"Token":20},{"Token":10},{"Token":11},{"Rule":20}],2]}]}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Opt":{"Rule":11}},{"Token":8},{"Token":20},{"Or":[{"And":[[{"Token":12},{"Layer":[{"Rule":21},{"Rep":{"Rule":9}}]},{"Token":13}],null]},{"And":[[{"Token":15}],null]},{"And":[[{"Token":10},{"Layer":[{"Rule":23},{"Rep":{"Rule":10}}]},{"Token":11},{"Token":15}],null]}]}],2]}]}]}},{"body":{"Pub":[29,{"Or":[{"And":[[{"Opt":{"Rule":11}},{"Token":20},{"Token":16},{"Rule":16},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":18}],null]}]}],2]}]}]}},{"body":{"Pub":[30,{"Or":[{"And":[[{"Opt":{"Rule":11}},{"Rule":16},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":18}],null]}]}],2]}]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Token":7}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[32,{"Or":[{"And":[[{"Opt":{"Token":17}},{"Token":20}],null]}]}]}}},{"Postfix":{"ty":32,"op":{"Or":[{"And":[[{"Token":17},{"Token":20}],null]}]}}}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Opt":{"Token":17}},{"Token":20}],null]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Rule":12},{"Or":[{"And":[[{"Token":17},{"Token":20}],null]}]}],null]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":2},{"Token":20}],null]}]}]}},{"body":{"Pub":[34,{"Or":[{"And":[[{"Token":20}],null]}]}]}},{"body":{"Pub":[35,{"Or":[{"And":[[{"Token":6},{"Rule":18},{"Token":14},{"Rule":19},{"Token":15}],1]}]}]}},{"body":{"Pub":[36,{"Or":[{"And":[[{"Token":20}],null]}]}]}},{"body":{"Pub":[37,{"Or":[{"And":[[{"Token":21}],null]}]}]}},{"body":{"Pub":[38,{"Or":[{"And":[[{"Token":12},{"Layer":[{"Rule":21},{"Rep":{"Rule":17}}]},{"Token":13}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":22}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":12},{"Rule":21},{"Token":13}],1]},{"And":[[{"Not":[13]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":24}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":10},{"Rule":23},{"Token":11}],1]},{"And":[[{"Not":[11]}],null]}]}}]"##;
        let parser: Vec<SynRule> = serde_json::from_str(parser_json).unwrap();

        struct Impl { tokenizer: Vec<LexRule>, parser: Vec<SynRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, text: &str) -> (FileStats, INode) {
                ::fall_parse::parse(text, &self.tokenizer, &|tokens, stats| {
                    Parser::new(ALL_NODE_TYPES, &self.parser).parse(tokens, stats)
                })
            }

            fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                match ty {
                    ERROR => NodeTypeInfo { name: "ERROR" },
                    WHITESPACE => NodeTypeInfo { name: "WHITESPACE" },
                    AS => NodeTypeInfo { name: "AS" },
                    CRATE => NodeTypeInfo { name: "CRATE" },
                    EXTERN => NodeTypeInfo { name: "EXTERN" },
                    FN => NodeTypeInfo { name: "FN" },
                    LET => NodeTypeInfo { name: "LET" },
                    PUB => NodeTypeInfo { name: "PUB" },
                    STRUCT => NodeTypeInfo { name: "STRUCT" },
                    USE => NodeTypeInfo { name: "USE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    EQ => NodeTypeInfo { name: "EQ" },
                    SEMI => NodeTypeInfo { name: "SEMI" },
                    COLON => NodeTypeInfo { name: "COLON" },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON" },
                    COMMA => NodeTypeInfo { name: "COMMA" },
                    STAR => NodeTypeInfo { name: "STAR" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    NUMBER => NodeTypeInfo { name: "NUMBER" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    USE_DECL => NodeTypeInfo { name: "USE_DECL" },
                    USE_SPEC => NodeTypeInfo { name: "USE_SPEC" },
                    USE_SPEC_ENTRY => NodeTypeInfo { name: "USE_SPEC_ENTRY" },
                    EXTERN_CRATE_DECL => NodeTypeInfo { name: "EXTERN_CRATE_DECL" },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF" },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF" },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD" },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD" },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY" },
                    PATH => NodeTypeInfo { name: "PATH" },
                    ALIAS => NodeTypeInfo { name: "ALIAS" },
                    TYPE => NodeTypeInfo { name: "TYPE" },
                    STMT => NodeTypeInfo { name: "STMT" },
                    PATTERN => NodeTypeInfo { name: "PATTERN" },
                    EXPR => NodeTypeInfo { name: "EXPR" },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(AS, "as", None),
                LexRule::new(CRATE, "crate", None),
                LexRule::new(EXTERN, "extern", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(LET, "let", None),
                LexRule::new(PUB, "pub", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(USE, "use", None),
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
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


