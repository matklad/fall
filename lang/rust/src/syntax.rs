use serde_json;
use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use fall_tree::{ERROR, WHITESPACE};

pub const PUB: NodeType = NodeType(100);
pub const LET: NodeType = NodeType(101);
pub const STRUCT: NodeType = NodeType(102);
pub const FN: NodeType = NodeType(103);
pub const USE: NodeType = NodeType(104);
pub const LPAREN: NodeType = NodeType(105);
pub const RPAREN: NodeType = NodeType(106);
pub const LBRACE: NodeType = NodeType(107);
pub const RBRACE: NodeType = NodeType(108);
pub const EQ: NodeType = NodeType(109);
pub const SEMI: NodeType = NodeType(110);
pub const COLON: NodeType = NodeType(111);
pub const COLONCOLON: NodeType = NodeType(112);
pub const COMMA: NodeType = NodeType(113);
pub const IDENT: NodeType = NodeType(114);
pub const NUMBER: NodeType = NodeType(115);
pub const FILE: NodeType = NodeType(116);
pub const USE_DECL: NodeType = NodeType(117);
pub const FN_DEF: NodeType = NodeType(118);
pub const STRUCT_DEF: NodeType = NodeType(119);
pub const STRUCT_FIELD: NodeType = NodeType(120);
pub const TUPLE_FIELD: NodeType = NodeType(121);
pub const PATH: NodeType = NodeType(122);
pub const TYPE: NodeType = NodeType(123);
pub const BLOCK_EXPR: NodeType = NodeType(124);
pub const STMT: NodeType = NodeType(125);
pub const PATTERN: NodeType = NodeType(126);
pub const EXPR: NodeType = NodeType(127);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR, WHITESPACE,
            PUB, LET, STRUCT, FN, USE, LPAREN, RPAREN, LBRACE, RBRACE, EQ, SEMI, COLON, COLONCOLON, COMMA, IDENT, NUMBER, FILE, USE_DECL, FN_DEF, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, PATH, TYPE, BLOCK_EXPR, STMT, PATTERN, EXPR,
        ];
        let parser_json = r##"[{"body":{"Pub":[18,{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":1},{"Rule":2}]}}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":2}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":4}],null]},{"And":[[{"Token":6}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":5}],null]},{"And":[[{"Rule":3}],null]}]}},{"body":{"Pub":[19,{"Or":[{"And":[[{"Token":6},{"Rule":8},{"Token":12}],1]}]}]}},{"body":{"Pub":[20,{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":2}],null]}]}},{"Token":5},{"Token":16},{"Token":7},{"Token":8},{"Rule":12}],2]}]}]}},{"body":{"Pub":[21,{"Or":[{"And":[[{"Opt":{"Token":2}},{"Token":4},{"Token":16},{"Or":[{"And":[[{"Token":9},{"Layer":[{"Rule":13},{"Rep":{"Rule":6}}]},{"Token":10}],null]},{"And":[[{"Token":12}],null]},{"And":[[{"Token":7},{"Layer":[{"Rule":15},{"Rep":{"Rule":7}}]},{"Token":8},{"Token":12}],null]}]}],2]}]}]}},{"body":{"Pub":[22,{"Or":[{"And":[[{"Opt":{"Token":2}},{"Token":16},{"Token":13},{"Rule":11},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":15}],null]}]}],2]}]}]}},{"body":{"Pub":[23,{"Or":[{"And":[[{"Opt":{"Token":2}},{"Rule":11},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":15}],null]}]}],2]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[24,{"Or":[{"And":[[{"Opt":{"Token":14}},{"Token":16}],null]}]}]}}},{"Postfix":{"ty":24,"op":{"Or":[{"And":[[{"Token":14},{"Token":16}],null]}]}}}]}},{"body":{"Pub":[24,{"Or":[{"And":[[{"Opt":{"Token":14}},{"Token":16}],null]}]}]}},{"body":{"Pub":[24,{"Or":[{"And":[[{"Rule":8},{"Or":[{"And":[[{"Token":14},{"Token":16}],null]}]}],null]}]}]}},{"body":{"Pub":[25,{"Or":[{"And":[[{"Token":16}],null]}]}]}},{"body":{"Pub":[26,{"Or":[{"And":[[{"Token":9},{"Layer":[{"Rule":13},{"Rep":{"Rule":17}}]},{"Token":10}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":14}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":9},{"Rule":13},{"Token":10}],1]},{"And":[[{"Not":[10]}],null]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":16}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":7},{"Rule":15},{"Token":8}],1]},{"And":[[{"Not":[8]}],null]}]}},{"body":{"Pub":[27,{"Or":[{"And":[[{"Token":3},{"Rule":18},{"Token":11},{"Rule":19},{"Token":12}],1]}]}]}},{"body":{"Pub":[28,{"Or":[{"And":[[{"Token":16}],null]}]}]}},{"body":{"Pub":[29,{"Or":[{"And":[[{"Token":17}],null]}]}]}}]"##;
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
                    FN_DEF => NodeTypeInfo { name: "FN_DEF" },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF" },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD" },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD" },
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


