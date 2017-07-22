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
pub const SELF: NodeType = NodeType(115);
pub const SUPER: NodeType = NodeType(116);
pub const TYPE: NodeType = NodeType(117);
pub const L_PAREN: NodeType = NodeType(118);
pub const R_PAREN: NodeType = NodeType(119);
pub const L_CURLY: NodeType = NodeType(120);
pub const R_CURLY: NodeType = NodeType(121);
pub const L_ANGLE: NodeType = NodeType(122);
pub const R_ANGLE: NodeType = NodeType(123);
pub const L_BRACK: NodeType = NodeType(124);
pub const R_BRACK: NodeType = NodeType(125);
pub const SHL: NodeType = NodeType(126);
pub const SHR: NodeType = NodeType(127);
pub const THIN_ARROW: NodeType = NodeType(128);
pub const EQ: NodeType = NodeType(129);
pub const SEMI: NodeType = NodeType(130);
pub const COLON: NodeType = NodeType(131);
pub const COLONCOLON: NodeType = NodeType(132);
pub const COMMA: NodeType = NodeType(133);
pub const DOT: NodeType = NodeType(134);
pub const HASH: NodeType = NodeType(135);
pub const STAR: NodeType = NodeType(136);
pub const SLASH: NodeType = NodeType(137);
pub const PERCENT: NodeType = NodeType(138);
pub const PLUS: NodeType = NodeType(139);
pub const MINUS: NodeType = NodeType(140);
pub const AMPERSAND: NodeType = NodeType(141);
pub const PIPE: NodeType = NodeType(142);
pub const UNDERSCORE: NodeType = NodeType(143);
pub const IDENT: NodeType = NodeType(144);
pub const NUMBER: NodeType = NodeType(145);
pub const FILE: NodeType = NodeType(146);
pub const USE_DECL: NodeType = NodeType(147);
pub const USE_SPEC: NodeType = NodeType(148);
pub const USE_SPEC_ENTRY: NodeType = NodeType(149);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(150);
pub const FN_DEF: NodeType = NodeType(151);
pub const VALUE_PARAM: NodeType = NodeType(152);
pub const SELF_PARAMETER: NodeType = NodeType(153);
pub const STRUCT_DEF: NodeType = NodeType(154);
pub const STRUCT_FIELD: NodeType = NodeType(155);
pub const TUPLE_FIELD: NodeType = NodeType(156);
pub const ENUM_DEF: NodeType = NodeType(157);
pub const ENUM_VARIANT: NodeType = NodeType(158);
pub const MOD_DEF: NodeType = NodeType(159);
pub const IMPL_DEF: NodeType = NodeType(160);
pub const TYPE_DEF: NodeType = NodeType(161);
pub const TYPE_PARAMETERS: NodeType = NodeType(162);
pub const TYPE_PARAMETER: NodeType = NodeType(163);
pub const VISIBILITY: NodeType = NodeType(164);
pub const PATH: NodeType = NodeType(165);
pub const TYPE_ARGUMENTS: NodeType = NodeType(166);
pub const ALIAS: NodeType = NodeType(167);
pub const TYPE_REFERENCE: NodeType = NodeType(168);
pub const PATTERN: NodeType = NodeType(169);
pub const EXPR: NodeType = NodeType(170);
pub const LITERAL: NodeType = NodeType(171);
pub const STRUCT_LITERAL: NodeType = NodeType(172);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(173);
pub const PAREN_EXPR: NodeType = NodeType(174);
pub const PATH_EXPR: NodeType = NodeType(175);
pub const BLOCK_EXPR: NodeType = NodeType(176);
pub const LET_STMT: NodeType = NodeType(177);
pub const EMPTY_STMT: NodeType = NodeType(178);
pub const EXPR_STMT: NodeType = NodeType(179);
pub const IF_EXPR: NodeType = NodeType(180);
pub const CALL_EXPR: NodeType = NodeType(181);
pub const FIELD_EXPR: NodeType = NodeType(182);
pub const VALUE_ARGUMENT: NodeType = NodeType(183);
pub const REFERENCE_EXPR: NodeType = NodeType(184);
pub const DEREFERENCE_EXPR: NodeType = NodeType(185);
pub const NEGATION_EXPR: NodeType = NodeType(186);
pub const PRODUCT_EXPR: NodeType = NodeType(187);
pub const SUM_EXPR: NodeType = NodeType(188);
pub const BIT_SHIFT: NodeType = NodeType(189);
pub const BIT_AND: NodeType = NodeType(190);
pub const BIT_OR: NodeType = NodeType(191);
pub const ATTRIBUTE: NodeType = NodeType(192);
pub const ATTR_VALUE: NodeType = NodeType(193);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Parser};
        const ALL_NODE_TYPES: &[NodeType] = &[
            ERROR,
            WHITESPACE, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHR, THIN_ARROW, EQ, SEMI, COLON, COLONCOLON, COMMA, DOT, HASH, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, IDENT, NUMBER, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TYPE_DEF, TYPE_PARAMETERS, TYPE_PARAMETER, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, TYPE_REFERENCE, PATTERN, EXPR, LITERAL, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, PAREN_EXPR, PATH_EXPR, BLOCK_EXPR, LET_STMT, EMPTY_STMT, EXPR_STMT, IF_EXPR, CALL_EXPR, FIELD_EXPR, VALUE_ARGUMENT, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR, ATTRIBUTE, ATTR_VALUE,
        ];
        let parser_json = r##"[{"body":{"Pub":[47,{"Or":[{"And":[[{"Rule":1}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":5}],null]},{"And":[[{"Token":36}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]},{"And":[[{"Token":14}],null]},{"And":[[{"Token":10}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":15}],null]},{"And":[[{"Token":18}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":12}],null]},{"And":[[{"Rule":15}],null]},{"And":[[{"Rule":17}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":21}],null]}]}},{"body":{"Pub":[48,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Token":10},{"Or":[{"And":[[{"Rule":25},{"Or":[{"And":[[{"Rule":33}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":33},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":33}},{"Rule":5}],null]}]},{"Token":31}],3]}]}]}},{"body":{"Pub":[49,{"Or":[{"And":[[{"Token":37}],null]},{"And":[[{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":6}]]]}]]]}],null]}]}]}},{"body":{"Pub":[50,{"Or":[{"And":[[{"Token":45},{"Opt":{"Rule":33}}],1]}]}]}},{"body":{"Pub":[51,{"Or":[{"And":[[{"Rule":63},{"Token":5},{"Token":4},{"Token":45},{"Opt":{"Rule":33}},{"Token":31}],3]}]}]}},{"body":{"Pub":[52,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Token":6},{"Token":45},{"Opt":{"Rule":22}},{"Rule":9},{"Opt":{"Or":[{"And":[[{"Token":29},{"Rule":34}],null]}]}},{"Rule":44}],3]}]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":67},[[4,{"Or":[{"And":[[{"Opt":{"Rule":11}},{"Call":[{"Rule":65},[[2,{"Rule":10}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":[53,{"Or":[{"And":[[{"Rule":35},{"Token":32},{"Rule":34}],null]}]}]}},{"body":{"Pub":[54,{"Or":[{"And":[[{"Opt":{"Token":42}},{"Token":16},{"Or":[{"And":[[{"Token":34}],null]},{"And":[["Eof"],null]}]}],2]}]}]}},{"body":{"Pub":[55,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Or":[{"And":[[{"Token":9}],null]},{"And":[[{"ContextualToken":[2,"union"]}],null]}]},{"Token":45},{"Opt":{"Rule":22}},{"Or":[{"And":[[{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":13}]]]}]]]}],null]},{"And":[[{"Token":31}],null]},{"And":[[{"Call":[{"Rule":67},[[4,{"Call":[{"Rule":65},[[2,{"Rule":14}]]]}]]]},{"Token":31}],null]}]}],3]}]}]}},{"body":{"Pub":[56,{"Or":[{"And":[[{"Opt":{"Rule":24}},{"Token":45},{"Token":32},{"Rule":34}],2]}]}]}},{"body":{"Pub":[57,{"Or":[{"And":[[{"Opt":{"Rule":24}},{"Rule":34}],null]}]}]}},{"body":{"Pub":[58,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Token":14},{"Token":45},{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":16}]]]}]]]}],3]}]}]}},{"body":{"Pub":[59,{"Or":[{"And":[[{"Token":45},{"Opt":{"Or":[{"And":[[{"Token":30},{"Rule":36}],null]},{"And":[[{"Call":[{"Rule":67},[[4,{"Call":[{"Rule":65},[[2,{"Rule":14}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":13}]]]}]]]}],null]}]}}],1]}]}]}},{"body":{"Pub":[60,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Token":11},{"Token":45},{"Or":[{"And":[[{"Token":31}],null]},{"And":[[{"Call":[{"Rule":66},[[3,{"Rule":1}]]]}],null]}]}],3]}]}]}},{"body":{"Pub":[61,{"Or":[{"And":[[{"Rule":63},{"Token":15},{"Token":45},{"Call":[{"Rule":66},[[3,{"Rep":{"WithSkip":[{"Rule":20},{"Rule":19}]}}]]]}],2]}]}]}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":21}],null]}]}},{"body":{"Or":[{"And":[[{"Token":8}],null]},{"And":[[{"Token":6}],null]},{"And":[[{"Token":18}],null]}]}},{"body":{"Pub":[62,{"Or":[{"And":[[{"Rule":63},{"Opt":{"Rule":24}},{"Token":18},{"Token":45},{"Opt":{"Rule":22}},{"Opt":{"Or":[{"And":[[{"Token":30},{"Rule":34}],null]}]}},{"Token":31}],3]}]}]}},{"body":{"Pub":[63,{"Or":[{"And":[[{"Call":[{"Rule":68},[[5,{"Call":[{"Rule":65},[[2,{"Rule":23}]]]}]]]}],null]}]}]}},{"body":{"Pub":[64,{"Or":[{"And":[[{"Token":45}],null]}]}]}},{"body":{"Pub":[65,{"Or":[{"And":[[{"Token":8}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":28}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":28}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":28}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[66,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Rule":31}],null]}]}]}}},{"Postfix":{"ty":66,"op":{"Or":[{"And":[[{"Token":33},{"Rule":31}],null]}]}}}]}},{"body":{"Pub":[66,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Rule":31}],null]}]}]}},{"body":{"Pub":[66,{"Or":[{"And":[[{"Rule":28},{"Or":[{"And":[[{"Token":33},{"Rule":31}],null]}]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":45}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":17}],null]}]},{"Opt":{"Rule":32}}],null]}]}},{"body":{"Pub":[67,{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":33}],null]}]},{"Call":[{"Rule":68},[[5,{"Call":[{"Rule":65},[[2,{"Rule":34}]]]}]]]}],null]}]}]}},{"body":{"Pub":[68,{"Or":[{"And":[[{"Token":3},{"Token":45}],null]}]}]}},{"body":{"Pub":[69,{"Or":[{"And":[[{"Rule":26}],null]},{"And":[[{"Token":42},{"Rule":34}],null]}]}]}},{"body":{"Pub":[70,{"Or":[{"And":[[{"Token":45}],null]},{"And":[[{"Token":44}],null]}]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":[72,{"Or":[{"And":[[{"Token":46}],null]}]}]}}},{"Atom":{"body":{"Pub":[73,{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":27},{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":41}]]]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[76,{"Or":[{"And":[[{"Rule":27}],null]}]}]}}},{"Atom":{"body":{"Pub":[75,{"Or":[{"And":[[{"Token":19},{"Call":[{"Rule":38},[[0,{"Rule":36}]]]},{"Token":20}],null]}]}]}}},{"Atom":{"body":{"Pub":[77,{"Or":[{"And":[[{"Call":[{"Rule":38},[[0,{"Call":[{"Rule":66},[[3,{"Or":[{"And":[[{"Rep":{"Rule":45}},{"Opt":{"Rule":36}}],null]}]}]]]}]]]}],null]}]}]}}},{"Atom":{"body":{"Pub":[81,{"Or":[{"And":[[{"Token":12},{"Enter":[5,{"Rule":36}]},{"Rule":44},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":44}],null]}]}}],1]}]}]}}},{"Postfix":{"ty":82,"op":{"Call":[{"Rule":38},[[0,{"Call":[{"Rule":67},[[4,{"Call":[{"Rule":65},[[2,{"Rule":52}]]]}]]]}]]]}}},{"Postfix":{"ty":83,"op":{"Or":[{"And":[[{"Token":35},{"Token":45}],null]}]}}},{"Prefix":{"ty":85,"op":{"Token":42}}},{"Prefix":{"ty":86,"op":{"Token":37}}},{"Prefix":{"ty":87,"op":{"Token":41}}},{"Binary":{"ty":88,"op":{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"Token":37}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":39}],null]}]}]]]},"priority":5}},{"Binary":{"ty":89,"op":{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"Token":40}],null]},{"And":[[{"Token":41}],null]}]}]]]},"priority":4}},{"Binary":{"ty":90,"op":{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"ContextualToken":[27,"<<"]}],null]},{"And":[[{"ContextualToken":[28,">>"]}],null]}]}]]]},"priority":3}},{"Binary":{"ty":91,"op":{"Call":[{"Rule":59},[[1,{"Token":42}]]]},"priority":2}},{"Binary":{"ty":92,"op":{"Call":[{"Rule":59},[[1,{"Token":43}]]]},"priority":1}}]}},{"body":{"Or":[{"And":[[{"PrevIs":[81,77]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[5,{"Exit":[4,{"Var":0}]}]}],null]}]}},{"body":{"Pub":[72,{"Or":[{"And":[[{"Token":46}],null]}]}]}},{"body":{"Pub":[73,{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Rule":27},{"Call":[{"Rule":66},[[3,{"Call":[{"Rule":65},[[2,{"Rule":41}]]]}]]]}],null]}]}]}},{"body":{"Pub":[74,{"Or":[{"And":[[{"Token":45},{"Token":32},{"Rule":36}],1]}]}]}},{"body":{"Pub":[75,{"Or":[{"And":[[{"Token":19},{"Call":[{"Rule":38},[[0,{"Rule":36}]]]},{"Token":20}],null]}]}]}},{"body":{"Pub":[76,{"Or":[{"And":[[{"Rule":27}],null]}]}]}},{"body":{"Pub":[77,{"Or":[{"And":[[{"Call":[{"Rule":38},[[0,{"Call":[{"Rule":66},[[3,{"Or":[{"And":[[{"Rep":{"Rule":45}},{"Opt":{"Rule":36}}],null]}]}]]]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rule":46}],null]},{"And":[[{"Rule":48}],null]},{"And":[[{"Rule":47}],null]}]}},{"body":{"Pub":[78,{"Or":[{"And":[[{"Token":7},{"Rule":35},{"Token":30},{"Rule":36},{"Token":31}],1]}]}]}},{"body":{"Pub":[79,{"Or":[{"And":[[{"Token":31}],null]}]}]}},{"body":{"Pub":[80,{"Or":[{"And":[[{"Enter":[4,{"Or":[{"And":[[{"Rule":36},{"Or":[{"And":[[{"Rule":37},{"Not":"Eof"}],null]},{"And":[[{"Token":31}],null]}]}],null]}]}]}],null]}]}]}},{"body":{"Pub":[81,{"Or":[{"And":[[{"Token":12},{"Enter":[5,{"Rule":36}]},{"Rule":44},{"Opt":{"Or":[{"And":[[{"Token":13},{"Rule":44}],null]}]}}],1]}]}]}},{"body":{"Pub":[82,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":38},[[0,{"Call":[{"Rule":67},[[4,{"Call":[{"Rule":65},[[2,{"Rule":52}]]]}]]]}]]]}],null]}]}]}},{"body":{"Pub":[83,{"Or":[{"And":[[{"Rule":36},{"Or":[{"And":[[{"Token":35},{"Token":45}],null]}]}],null]}]}]}},{"body":{"Pub":[84,{"Or":[{"And":[[{"Rule":36}],null]}]}]}},{"body":{"Pub":[85,{"Or":[{"And":[[{"Token":42},{"Rule":36}],null]}]}]}},{"body":{"Pub":[86,{"Or":[{"And":[[{"Token":37},{"Rule":36}],null]}]}]}},{"body":{"Pub":[87,{"Or":[{"And":[[{"Token":41},{"Rule":36}],null]}]}]}},{"body":{"Pub":[88,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"Token":37}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":39}],null]}]}]]]},{"Rule":36}],null]}]}]}},{"body":{"Pub":[89,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"Token":40}],null]},{"And":[[{"Token":41}],null]}]}]]]},{"Rule":36}],null]}]}]}},{"body":{"Pub":[90,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":59},[[1,{"Or":[{"And":[[{"ContextualToken":[27,"<<"]}],null]},{"And":[[{"ContextualToken":[28,">>"]}],null]}]}]]]},{"Rule":36}],null]}]}]}},{"body":{"Or":[{"And":[[{"IsIn":4},{"Not":{"Rule":37}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":4}},{"Var":1}],null]}]}},{"body":{"Pub":[91,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":59},[[1,{"Token":42}]]]},{"Rule":36}],null]}]}]}},{"body":{"Pub":[92,{"Or":[{"And":[[{"Rule":36},{"Call":[{"Rule":59},[[1,{"Token":43}]]]},{"Rule":36}],null]}]}]}},{"body":{"Pub":[93,{"Or":[{"And":[[{"Token":36},{"Call":[{"Rule":69},[[6,{"Call":[{"Rule":65},[[2,{"Rule":64}]]]}]]]}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":62}}],null]}]}},{"body":{"Pub":[94,{"Or":[{"And":[[{"Token":45}],null]}]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":34}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":70},[[7,{"Token":21}],[8,{"Token":22}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":70},[[7,{"Token":19}],[8,{"Token":20}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":70},[[7,{"Token":23}],[8,{"Token":24}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":70},[[7,{"Token":25}],[8,{"Token":26}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":71},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":71},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;
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
                    SELF => NodeTypeInfo { name: "SELF", whitespace_like: false },
                    SUPER => NodeTypeInfo { name: "SUPER", whitespace_like: false },
                    TYPE => NodeTypeInfo { name: "TYPE", whitespace_like: false },
                    L_PAREN => NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                    R_PAREN => NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                    L_CURLY => NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                    R_CURLY => NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                    L_ANGLE => NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                    R_ANGLE => NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                    L_BRACK => NodeTypeInfo { name: "L_BRACK", whitespace_like: false },
                    R_BRACK => NodeTypeInfo { name: "R_BRACK", whitespace_like: false },
                    SHL => NodeTypeInfo { name: "SHL", whitespace_like: false },
                    SHR => NodeTypeInfo { name: "SHR", whitespace_like: false },
                    THIN_ARROW => NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                    EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                    SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                    COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                    COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                    COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                    DOT => NodeTypeInfo { name: "DOT", whitespace_like: false },
                    HASH => NodeTypeInfo { name: "HASH", whitespace_like: false },
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
                    SELF_PARAMETER => NodeTypeInfo { name: "SELF_PARAMETER", whitespace_like: false },
                    STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                    STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                    TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                    ENUM_DEF => NodeTypeInfo { name: "ENUM_DEF", whitespace_like: false },
                    ENUM_VARIANT => NodeTypeInfo { name: "ENUM_VARIANT", whitespace_like: false },
                    MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                    IMPL_DEF => NodeTypeInfo { name: "IMPL_DEF", whitespace_like: false },
                    TYPE_DEF => NodeTypeInfo { name: "TYPE_DEF", whitespace_like: false },
                    TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                    TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                    VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                    PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                    TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                    ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                    TYPE_REFERENCE => NodeTypeInfo { name: "TYPE_REFERENCE", whitespace_like: false },
                    PATTERN => NodeTypeInfo { name: "PATTERN", whitespace_like: false },
                    EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                    LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                    STRUCT_LITERAL => NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                    STRUCT_LITERAL_FIELD => NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                    PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                    PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                    BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                    LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                    EMPTY_STMT => NodeTypeInfo { name: "EMPTY_STMT", whitespace_like: false },
                    EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                    IF_EXPR => NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                    CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                    FIELD_EXPR => NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                    VALUE_ARGUMENT => NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                    REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                    DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                    NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                    PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                    SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                    BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                    BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                    BIT_OR => NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
                    ATTRIBUTE => NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                    ATTR_VALUE => NodeTypeInfo { name: "ATTR_VALUE", whitespace_like: false },
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
                LexRule::new(SELF, "self", None),
                LexRule::new(SUPER, "super", None),
                LexRule::new(TYPE, "type", None),
                LexRule::new(L_PAREN, "\\(", None),
                LexRule::new(R_PAREN, "\\)", None),
                LexRule::new(L_CURLY, "\\{", None),
                LexRule::new(R_CURLY, "\\}", None),
                LexRule::new(L_ANGLE, "<", None),
                LexRule::new(R_ANGLE, ">", None),
                LexRule::new(L_BRACK, "\\[", None),
                LexRule::new(R_BRACK, "\\]", None),
                LexRule::new(THIN_ARROW, "\\->", None),
                LexRule::new(EQ, "=", None),
                LexRule::new(SEMI, ";", None),
                LexRule::new(COLON, ":", None),
                LexRule::new(COLONCOLON, "::", None),
                LexRule::new(COMMA, ",", None),
                LexRule::new(DOT, "\\.", None),
                LexRule::new(HASH, "\\#", None),
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


