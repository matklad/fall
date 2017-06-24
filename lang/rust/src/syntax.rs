use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use fall_tree::{ERROR, WHITESPACE};

pub const PUB: NodeType = NodeType(100);
pub const LET: NodeType = NodeType(101);
pub const STRUCT: NodeType = NodeType(102);
pub const FN: NodeType = NodeType(103);
pub const USE: NodeType = NodeType(104);
pub const EXTERN: NodeType = NodeType(105);
pub const CRATE: NodeType = NodeType(106);
pub const LPAREN: NodeType = NodeType(107);
pub const RPAREN: NodeType = NodeType(108);
pub const LBRACE: NodeType = NodeType(109);
pub const RBRACE: NodeType = NodeType(110);
pub const EQ: NodeType = NodeType(111);
pub const SEMI: NodeType = NodeType(112);
pub const COLON: NodeType = NodeType(113);
pub const COLONCOLON: NodeType = NodeType(114);
pub const COMMA: NodeType = NodeType(115);
pub const IDENT: NodeType = NodeType(116);
pub const NUMBER: NodeType = NodeType(117);
pub const FILE: NodeType = NodeType(118);
pub const USE_DECL: NodeType = NodeType(119);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(120);
pub const FN_DEF: NodeType = NodeType(121);
pub const STRUCT_DEF: NodeType = NodeType(122);
pub const STRUCT_FIELD: NodeType = NodeType(123);
pub const TUPLE_FIELD: NodeType = NodeType(124);
pub const VISIBILITY: NodeType = NodeType(125);
pub const PATH: NodeType = NodeType(126);
pub const TYPE: NodeType = NodeType(127);
pub const BLOCK_EXPR: NodeType = NodeType(128);
pub const STMT: NodeType = NodeType(129);
pub const PATTERN: NodeType = NodeType(130);
pub const EXPR: NodeType = NodeType(131);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            PUB, LET, STRUCT, FN, USE, EXTERN, CRATE, LPAREN, RPAREN, LBRACE, RBRACE, EQ, SEMI, COLON, COLONCOLON, COMMA, IDENT, NUMBER, FILE, USE_DECL, EXTERN_CRATE_DECL, FN_DEF, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, VISIBILITY, PATH, TYPE, BLOCK_EXPR, STMT, PATTERN, EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[20,{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":1},{"Rule":2}]}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":2}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":7}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":3}],null]},{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":5}],null]},{"And":[[{"Rule":6}],null]}]}},{"body":{"Pub":[21,{"Or":[{"And":[[{"Token":6},{"Rule":10},{"Token":14}],1]}]}]}},{"body":{"Pub":[22,{"Or":[{"And":[[{"Token":7},{"Token":8},{"Token":18},{"Token":14}],2]}]}]}},{"body":{"Pub":[23,{"Or":[{"And":[[{"Opt":{"Rule":9}},{"Token":5},{"Token":18},{"Token":9},{"Token":10},{"Rule":14}],2]}]}]}},{"body":{"Pub":[24,{"Or":[{"And":[[{"Opt":{"Rule":9}},{"Token":4},{"Token":18},{"Or":[{"And":[[{"Token":11},{"Layer":[{"Rule":15},{"Rep":{"Rule":7}}]},{"Token":12}],null]},{"And":[[{"Token":14}],null]},{"And":[[{"Token":9},{"Layer":[{"Rule":17},{"Rep":{"Rule":8}}]},{"Token":10},{"Token":14}],null]}]}],2]}]}]}},{"body":{"Pub":[25,{"Or":[{"And":[[{"Opt":{"Rule":9}},{"Token":18},{"Token":15},{"Rule":13},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":17}],null]}]}],2]}]}]}},{"body":{"Pub":[26,{"Or":[{"And":[[{"Opt":{"Rule":9}},{"Rule":13},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":17}],null]}]}],2]}]}]}},{"body":{"Pub":[27,{"Or":[{"And":[[{"Token":2}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[28,{"Or":[{"And":[[{"Opt":{"Token":16}},{"Token":18}],null]}]}]}}},{"Postfix":{"ty":28,"op":{"Or":[{"And":[[{"Token":16},{"Token":18}],null]}]}}}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Opt":{"Token":16}},{"Token":18}],null]}]}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Rule":10},{"Or":[{"And":[[{"Token":16},{"Token":18}],null]}]}],null]}]}]}},{"body":{"Pub":[29,{"Or":[{"And":[[{"Token":18}],null]}]}]}},{"body":{"Pub":[30,{"Or":[{"And":[[{"Token":11},{"Layer":[{"Rule":15},{"Rep":{"Rule":19}}]},{"Token":12}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":16}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":11},{"Rule":15},{"Token":12}],1]},{"And":[[{"Not":[12]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":18}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":9},{"Rule":17},{"Token":10}],1]},{"And":[[{"Not":[10]}],null]}]}},{"body":{"Pub":[31,{"Or":[{"And":[[{"Token":3},{"Rule":20},{"Token":13},{"Rule":21},{"Token":14}],1]}]}]}},{"body":{"Pub":[32,{"Or":[{"And":[[{"Token":18}],null]}]}]}},{"body":{"Pub":[33,{"Or":[{"And":[[{"Token":19}],null]}]}]}}]"##;
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
                    PUB => NodeTypeInfo { name: "PUB" },
                    LET => NodeTypeInfo { name: "LET" },
                    STRUCT => NodeTypeInfo { name: "STRUCT" },
                    FN => NodeTypeInfo { name: "FN" },
                    USE => NodeTypeInfo { name: "USE" },
                    EXTERN => NodeTypeInfo { name: "EXTERN" },
                    CRATE => NodeTypeInfo { name: "CRATE" },
                    LPAREN => NodeTypeInfo { name: "LPAREN" },
                    RPAREN => NodeTypeInfo { name: "RPAREN" },
                    LBRACE => NodeTypeInfo { name: "LBRACE" },
                    RBRACE => NodeTypeInfo { name: "RBRACE" },
                    EQ => NodeTypeInfo { name: "EQ" },
                    SEMI => NodeTypeInfo { name: "SEMI" },
                    COLON => NodeTypeInfo { name: "COLON" },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON" },
                    COMMA => NodeTypeInfo { name: "COMMA" },
                    IDENT => NodeTypeInfo { name: "IDENT" },
                    NUMBER => NodeTypeInfo { name: "NUMBER" },
                    FILE => NodeTypeInfo { name: "FILE" },
                    USE_DECL => NodeTypeInfo { name: "USE_DECL" },
                    EXTERN_CRATE_DECL => NodeTypeInfo { name: "EXTERN_CRATE_DECL" },
                    FN_DEF => NodeTypeInfo { name: "FN_DEF" },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF" },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD" },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD" },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY" },
                    PATH => NodeTypeInfo { name: "PATH" },
                    TYPE => NodeTypeInfo { name: "TYPE" },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR" },
                    STMT => NodeTypeInfo { name: "STMT" },
                    PATTERN => NodeTypeInfo { name: "PATTERN" },
                    EXPR => NodeTypeInfo { name: "EXPR" },
                    _ => panic!("Unknown NodeType: {:?}", ty)
                }
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(PUB, "pub", None),
                LexRule::new(LET, "let", None),
                LexRule::new(STRUCT, "struct", None),
                LexRule::new(FN, "fn", None),
                LexRule::new(USE, "use", None),
                LexRule::new(EXTERN, "extern", None),
                LexRule::new(CRATE, "crate", None),
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(LBRACE, "\\{", None),
                LexRule::new(RBRACE, "\\}", None),
                LexRule::new(EQ, "=", None),
                LexRule::new(SEMI, ";", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COLONCOLON, "::", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


