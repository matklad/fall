use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const UNION: NodeType = NodeType(101);
pub const AS: NodeType = NodeType(102);
pub const CRATE: NodeType = NodeType(103);
pub const EXTERN: NodeType = NodeType(104);
pub const FN: NodeType = NodeType(105);
pub const LET: NodeType = NodeType(106);
pub const PUB: NodeType = NodeType(107);
pub const STRUCT: NodeType = NodeType(108);
pub const USE: NodeType = NodeType(109);
pub const MOD: NodeType = NodeType(110);
pub const IF: NodeType = NodeType(111);
pub const ELSE: NodeType = NodeType(112);
pub const ENUM: NodeType = NodeType(113);
pub const IMPL: NodeType = NodeType(114);
pub const LPAREN: NodeType = NodeType(115);
pub const RPAREN: NodeType = NodeType(116);
pub const LBRACE: NodeType = NodeType(117);
pub const RBRACE: NodeType = NodeType(118);
pub const LANGLE: NodeType = NodeType(119);
pub const RANGLE: NodeType = NodeType(120);
pub const SHL: NodeType = NodeType(121);
pub const SHR: NodeType = NodeType(122);
pub const THIN_ARROW: NodeType = NodeType(123);
pub const EQ: NodeType = NodeType(124);
pub const SEMI: NodeType = NodeType(125);
pub const COLON: NodeType = NodeType(126);
pub const COLONCOLON: NodeType = NodeType(127);
pub const COMMA: NodeType = NodeType(128);
pub const STAR: NodeType = NodeType(129);
pub const SLASH: NodeType = NodeType(130);
pub const PERCENT: NodeType = NodeType(131);
pub const PLUS: NodeType = NodeType(132);
pub const MINUS: NodeType = NodeType(133);
pub const AMPERSAND: NodeType = NodeType(134);
pub const PIPE: NodeType = NodeType(135);
pub const UNDERSCORE: NodeType = NodeType(136);
pub const IDENT: NodeType = NodeType(137);
pub const NUMBER: NodeType = NodeType(138);
pub const FILE: NodeType = NodeType(139);
pub const USE_DECL: NodeType = NodeType(140);
pub const USE_SPEC: NodeType = NodeType(141);
pub const USE_SPEC_ENTRY: NodeType = NodeType(142);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(143);
pub const FN_DEF: NodeType = NodeType(144);
pub const VALUE_PARAM: NodeType = NodeType(145);
pub const STRUCT_DEF: NodeType = NodeType(146);
pub const STRUCT_FIELD: NodeType = NodeType(147);
pub const TUPLE_FIELD: NodeType = NodeType(148);
pub const ENUM_DEF: NodeType = NodeType(149);
pub const ENUM_VARIANT: NodeType = NodeType(150);
pub const MOD_DEF: NodeType = NodeType(151);
pub const IMPL_DEF: NodeType = NodeType(152);
pub const TYPE_PARAMETERS: NodeType = NodeType(153);
pub const TYPE_PARAMETER: NodeType = NodeType(154);
pub const VISIBILITY: NodeType = NodeType(155);
pub const PATH: NodeType = NodeType(156);
pub const TYPE_ARGUMENTS: NodeType = NodeType(157);
pub const ALIAS: NodeType = NodeType(158);
pub const TYPE: NodeType = NodeType(159);
pub const PATTERN: NodeType = NodeType(160);
pub const EXPR: NodeType = NodeType(161);
pub const LITERAL: NodeType = NodeType(162);
pub const STRUCT_LITERAL: NodeType = NodeType(163);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(164);
pub const PAREN_EXPR: NodeType = NodeType(165);
pub const PATH_EXPR: NodeType = NodeType(166);
pub const BLOCK_EXPR: NodeType = NodeType(167);
pub const LET_STMT: NodeType = NodeType(168);
pub const EXPR_STMT: NodeType = NodeType(169);
pub const IF_EXPR: NodeType = NodeType(170);
pub const REFERENCE_EXPR: NodeType = NodeType(171);
pub const DEREFERENCE_EXPR: NodeType = NodeType(172);
pub const NEGATION_EXPR: NodeType = NodeType(173);
pub const PRODUCT_EXPR: NodeType = NodeType(174);
pub const SUM_EXPR: NodeType = NodeType(175);
pub const BIT_SHIFT: NodeType = NodeType(176);
pub const BIT_AND: NodeType = NodeType(177);
pub const BIT_OR: NodeType = NodeType(178);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, LPAREN, RPAREN, LBRACE, RBRACE, LANGLE, RANGLE, SHL, SHR, THIN_ARROW, EQ, SEMI, COLON, COLONCOLON, COMMA, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, VALUE_PARAM, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TYPE_PARAMETERS, TYPE_PARAMETER, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, TYPE, PATTERN, EXPR, LITERAL, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, PAREN_EXPR, PATH_EXPR, BLOCK_EXPR, LET_STMT, EXPR_STMT, IF_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR,
        ];
        let parser_json = r##"[{"body":{"Pub":[40,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]},{"And":[[{"Token":14}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":15}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":11}],null]},{"And":[[{"Rule":14}],null]},{"And":[[{"Rule":16}],null]},{"And":[[{"Rule":17}],null]}]}},{"body":{"Pub":[41,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Token":10},{"Or":[{"And":[[{"Rule":22},{"Or":[{"And":[[{"Rule":30}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":28},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":28}},{"Rule":5}],null]}]},{"Token":26}],2]}]}]}},{"body":{"Pub":[42,{"Or":[{"And":[[{"Token":30}],null]},{"And":[[{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":6}]]]}]]]}],null]}]}]}},{"body":{"Pub":[43,{"Or":[{"And":[[{"Token":38},{"Opt":{"Rule":30}}],1]}]}]}},{"body":{"Pub":[44,{"Or":[{"And":[[{"Token":5},{"Token":4},{"Token":38},{"Opt":{"Rule":30}},{"Token":26}],2]}]}]}},{"body":{"Pub":[45,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Token":6},{"Token":38},{"Opt":{"Rule":19}},{"Rule":9},{"Opt":{"Or":[{"And":[[{"Token":24},{"Rule":31}],null]}]}},{"Rule":39}],2]}]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":54},[[2,{"Call":[{"Rule":52},[[0,{"Rule":10}]]]}]]]}],null]}]}},{"body":{"Pub":[46,{"Or":[{"And":[[{"Rule":32},{"Token":27},{"Rule":31}],null]}]}]}},{"body":{"Pub":[47,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Or":[{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]}]},{"Token":38},{"Opt":{"Rule":19}},{"Or":[{"And":[[{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":12}]]]}]]]}],null]},{"And":[[{"Token":26}],null]},{"And":[[{"Call":[{"Rule":54},[[2,{"Call":[{"Rule":52},[[0,{"Rule":13}]]]}]]]},{"Token":26}],null]}]}],2]}]}]}},{"body":{"Pub":[48,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Token":38},{"Token":27},{"Rule":31}],2]}]}]}},{"body":{"Pub":[49,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Rule":31}],null]}]}]}},{"body":{"Pub":[50,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Token":14},{"Token":38},{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":15}]]]}]]]}],2]}]}]}},{"body":{"Pub":[51,{"Or":[{"And":[[{"Token":38},{"Opt":{"Or":[{"And":[[{"Token":25},{"Rule":33}],null]},{"And":[[{"Call":[{"Rule":54},[[2,{"Call":[{"Rule":52},[[0,{"Rule":13}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":12}]]]}]]]}],null]}]}}],1]}]}]}},{"body":{"Pub":[52,{"Or":[{"And":[[{"Opt":{"Rule":21}},{"Token":11},{"Token":38},{"Or":[{"And":[[{"Token":26}],null]},{"And":[[{"Call":[{"Rule":53},[[1,{"Rule":1}]]]}],null]}]}],2]}]}]}},{"body":{"Pub":[53,{"Or":[{"And":[[{"Token":15},{"Token":38},{"Call":[{"Rule":53},[[1,{"Rep":{"Rule":18}}]]]}],1]}]}]}},{"body":{"Or":[{"And":[[{"Rule":8}],null]}]}},{"body":{"Pub":[54,{"Or":[{"And":[[{"Call":[{"Rule":55},[[3,{"Call":[{"Rule":52},[[0,{"Rule":20}]]]}]]]}],null]}]}]}},{"body":{"Pub":[55,{"Or":[{"And":[[{"Token":38}],null]}]}]}},{"body":{"Pub":[56,{"Or":[{"And":[[{"Token":8}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":25}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":25}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":25}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[57,{"Or":[{"And":[[{"Opt":{"Token":28}},{"Rule":28}],null]}]}]}}},{"Postfix":{"ty":57,"op":{"Or":[{"And":[[{"Token":28},{"Rule":28}],null]}]}}}]}},{"body":{"Pub":[57,{"Or":[{"And":[[{"Opt":{"Token":28}},{"Rule":28}],null]}]}]}},{"body":{"Pub":[57,{"Or":[{"And":[[{"Rule":25},{"Or":[{"And":[[{"Token":28},{"Rule":28}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Token":38},{"Opt":{"Rule":29}}],null]}]}},{"body":{"Pub":[58,{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":28}],null]}]},{"Call":[{"Rule":55},[[3,{"Call":[{"Rule":52},[[0,{"Rule":31}]]]}]]]}],null]}]}]}},{"body":{"Pub":[59,{"Or":[{"And":[[{"Token":3},{"Token":38}],null]}]}]}},{"body":{"Pub":[60,{"Or":[{"And":[[{"Rule":23}],null]}]}]}},{"body":{"Pub":[61,{"Or":[{"And":[[{"Token":38}],null]},{"And":[[{"Token":37}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[63,{"Or":[{"And":[[{"Token":39}],null]}]}]}}},{"Atom":{"body":{"Pub":[64,{"Or":[{"And":[[{"Not":{"IsIn":4}},{"Rule":24},{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":36}]]]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[67,{"Or":[{"And":[[{"Rule":24}],null]}]}]}}},{"Atom":{"body":{"Pub":[66,{"Or":[{"And":[[{"Token":16},{"Exit":[4,{"Rule":33}]},{"Token":17}],null]}]}]}}},{"Atom":{"body":{"Pub":[68,{"Or":[{"And":[[{"Call":[{"Rule":53},[[1,{"Or":[{"And":[[{"Rep":{"Rule":40}},{"Opt":{"Rule":33}}],null]}]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[71,{"Or":[{"And":[[{"Token":12},{"Enter":[4,{"Rule":33}]},{"Rule":39},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":39}],null]}]}}],1]}]}]}}},{"Prefix":{"ty":72,"op":{"Token":35}}},{"Prefix":{"ty":73,"op":{"Token":30}}},{"Prefix":{"ty":74,"op":{"Token":34}}},{"Binary":{"ty":75,"op":{"Or":[{"And":[[{"Token":30}],null]},{"And":[[{"Token":31}],null]},{"And":[[{"Token":32}],null]}]},"priority":5}},{"Binary":{"ty":76,"op":{"Or":[{"And":[[{"Token":33}],null]},{"And":[[{"Token":34}],null]}]},"priority":4}},{"Binary":{"ty":77,"op":{"Or":[{"And":[[{"ContextualToken":[22,"<<"]}],null]},{"And":[[{"ContextualToken":[23,">>"]}],null]}]},"priority":3}},{"Binary":{"ty":78,"op":{"Token":35},"priority":2}},{"Binary":{"ty":79,"op":{"Token":36},"priority":1}}]}},{"body":{"Pub":[63,{"Or":[{"And":[[{"Token":39}],null]}]}]}},{"body":{"Pub":[64,{"Or":[{"And":[[{"Not":{"IsIn":4}},{"Rule":24},{"Call":[{"Rule":53},[[1,{"Call":[{"Rule":52},[[0,{"Rule":36}]]]}]]]}],null]}]}]}},{"body":{"Pub":[65,{"Or":[{"And":[[{"Token":38},{"Token":27},{"Rule":33}],1]}]}]}},{"body":{"Pub":[66,{"Or":[{"And":[[{"Token":16},{"Exit":[4,{"Rule":33}]},{"Token":17}],null]}]}]}},{"body":{"Pub":[67,{"Or":[{"And":[[{"Rule":24}],null]}]}]}},{"body":{"Pub":[68,{"Or":[{"And":[[{"Call":[{"Rule":53},[[1,{"Or":[{"And":[[{"Rep":{"Rule":40}},{"Opt":{"Rule":33}}],null]}]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":41}],null]},{"And":[[{"Rule":42}],null]}]}},{"body":{"Pub":[69,{"Or":[{"And":[[{"Token":7},{"Rule":32},{"Token":25},{"Rule":33},{"Token":26}],1]}]}]}},{"body":{"Pub":[70,{"Or":[{"And":[[{"Rule":33},{"Token":26}],null]}]}]}},{"body":{"Pub":[71,{"Or":[{"And":[[{"Token":12},{"Enter":[4,{"Rule":33}]},{"Rule":39},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":39}],null]}]}}],1]}]}]}},{"body":{"Pub":[72,{"Or":[{"And":[[{"Token":35},{"Rule":33}],null]}]}]}},{"body":{"Pub":[73,{"Or":[{"And":[[{"Token":30},{"Rule":33}],null]}]}]}},{"body":{"Pub":[74,{"Or":[{"And":[[{"Token":34},{"Rule":33}],null]}]}]}},{"body":{"Pub":[75,{"Or":[{"And":[[{"Rule":33},{"Or":[{"And":[[{"Token":30}],null]},{"And":[[{"Token":31}],null]},{"And":[[{"Token":32}],null]}]},{"Rule":33}],null]}]}]}},{"body":{"Pub":[76,{"Or":[{"And":[[{"Rule":33},{"Or":[{"And":[[{"Token":33}],null]},{"And":[[{"Token":34}],null]}]},{"Rule":33}],null]}]}]}},{"body":{"Pub":[77,{"Or":[{"And":[[{"Rule":33},{"Or":[{"And":[[{"ContextualToken":[22,"<<"]}],null]},{"And":[[{"ContextualToken":[23,">>"]}],null]}]},{"Rule":33}],null]}]}]}},{"body":{"Pub":[78,{"Or":[{"And":[[{"Rule":33},{"Token":35},{"Rule":33}],null]}]}]}},{"body":{"Pub":[79,{"Or":[{"And":[[{"Rule":33},{"Token":36},{"Rule":33}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":0},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":29}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":56},[[4,{"Token":18}],[5,{"Token":19}],[6,{"Var":1}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":56},[[4,{"Token":16}],[5,{"Token":17}],[6,{"Var":2}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":56},[[4,{"Token":20}],[5,{"Token":21}],[6,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":4},{"Layer":[{"Call":[{"Rule":57},[[7,{"Var":4}],[8,{"Var":5}]]]},{"Var":6}]},{"Var":5}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":7},{"Call":[{"Rule":57},[[7,{"Var":7}],[8,{"Var":8}]]]},{"Var":8}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":8}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;
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
                    UNION => NodeTypeInfo { name: "UNION", whitespace_like: false },
                    AS => NodeTypeInfo { name: "AS", whitespace_like: false },
                    CRATE => NodeTypeInfo { name: "CRATE", whitespace_like: false },
                    EXTERN => NodeTypeInfo { name: "EXTERN", whitespace_like: false },
                    FN => NodeTypeInfo { name: "FN", whitespace_like: false },
                    LET => NodeTypeInfo { name: "LET", whitespace_like: false },
                    PUB => NodeTypeInfo { name: "PUB", whitespace_like: false },
                    STRUCT => NodeTypeInfo { name: "STRUCT", whitespace_like: false },
                    USE => NodeTypeInfo { name: "USE", whitespace_like: false },
                    MOD => NodeTypeInfo { name: "MOD", whitespace_like: false },
                    IF => NodeTypeInfo { name: "IF", whitespace_like: false },
                    ELSE => NodeTypeInfo { name: "ELSE", whitespace_like: false },
                    ENUM => NodeTypeInfo { name: "ENUM", whitespace_like: false },
                    IMPL => NodeTypeInfo { name: "IMPL", whitespace_like: false },
                    LPAREN => NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                    RPAREN => NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                    LBRACE => NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                    RBRACE => NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                    LANGLE => NodeTypeInfo { name: "LANGLE", whitespace_like: false },
                    RANGLE => NodeTypeInfo { name: "RANGLE", whitespace_like: false },
                    SHL => NodeTypeInfo { name: "SHL", whitespace_like: false },
                    SHR => NodeTypeInfo { name: "SHR", whitespace_like: false },
                    THIN_ARROW => NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                    SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                    PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                    PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                    MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                    AMPERSAND => NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                    PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
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
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                    ENUM_DEF => NodeTypeInfo { name: "ENUM_DEF", whitespace_like: false },
                    ENUM_VARIANT => NodeTypeInfo { name: "ENUM_VARIANT", whitespace_like: false },
                    MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                    IMPL_DEF => NodeTypeInfo { name: "IMPL_DEF", whitespace_like: false },
                    TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                    TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                    TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                    ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                    TYPE => NodeTypeInfo { name: "TYPE", whitespace_like: false },
                    PATTERN => NodeTypeInfo { name: "PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                    STRUCT_LITERAL => NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                    STRUCT_LITERAL_FIELD => NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                    PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                    PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                    EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                    IF_EXPR => NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                    REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                    DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                    NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                    BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                    BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                    BIT_OR => NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
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
                LexRule::new(IF, "if", None),
                LexRule::new(ELSE, "else", None),
                LexRule::new(ENUM, "enum", None),
                LexRule::new(IMPL, "impl", None),
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
                LexRule::new(STAR, "\\*", None),
                LexRule::new(SLASH, "/", None),
                LexRule::new(PERCENT, "%", None),
                LexRule::new(PLUS, "\\+", None),
                LexRule::new(MINUS, "\\-", None),
                LexRule::new(AMPERSAND, "\\&", None),
                LexRule::new(PIPE, "\\|", None),
                LexRule::new(UNDERSCORE, "_", None),
                LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
                LexRule::new(NUMBER, "\\d+", None),
            ],
            parser: parser,
        })
    };
}


