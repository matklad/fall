use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LINE_COMMENT: NodeType = NodeType(101);
pub const BLOCK_COMMENT: NodeType = NodeType(102);
pub const UNION: NodeType = NodeType(103);
pub const AS: NodeType = NodeType(104);
pub const CRATE: NodeType = NodeType(105);
pub const EXTERN: NodeType = NodeType(106);
pub const FN: NodeType = NodeType(107);
pub const LET: NodeType = NodeType(108);
pub const PUB: NodeType = NodeType(109);
pub const STRUCT: NodeType = NodeType(110);
pub const USE: NodeType = NodeType(111);
pub const MOD: NodeType = NodeType(112);
pub const IF: NodeType = NodeType(113);
pub const ELSE: NodeType = NodeType(114);
pub const ENUM: NodeType = NodeType(115);
pub const IMPL: NodeType = NodeType(116);
pub const SELF: NodeType = NodeType(117);
pub const SUPER: NodeType = NodeType(118);
pub const TYPE: NodeType = NodeType(119);
pub const CONST: NodeType = NodeType(120);
pub const STATIC: NodeType = NodeType(121);
pub const FOR: NodeType = NodeType(122);
pub const LOOP: NodeType = NodeType(123);
pub const WHILE: NodeType = NodeType(124);
pub const MOVE: NodeType = NodeType(125);
pub const MUT: NodeType = NodeType(126);
pub const REF: NodeType = NodeType(127);
pub const TRAIT: NodeType = NodeType(128);
pub const MATCH: NodeType = NodeType(129);
pub const RETURN: NodeType = NodeType(130);
pub const IN: NodeType = NodeType(131);
pub const UNSAFE: NodeType = NodeType(132);
pub const WHERE: NodeType = NodeType(133);
pub const L_PAREN: NodeType = NodeType(134);
pub const R_PAREN: NodeType = NodeType(135);
pub const L_CURLY: NodeType = NodeType(136);
pub const R_CURLY: NodeType = NodeType(137);
pub const L_ANGLE: NodeType = NodeType(138);
pub const R_ANGLE: NodeType = NodeType(139);
pub const L_BRACK: NodeType = NodeType(140);
pub const R_BRACK: NodeType = NodeType(141);
pub const SHL: NodeType = NodeType(142);
pub const SHL_EQ: NodeType = NodeType(143);
pub const SHR: NodeType = NodeType(144);
pub const SHR_EQ: NodeType = NodeType(145);
pub const AND: NodeType = NodeType(146);
pub const OR: NodeType = NodeType(147);
pub const THIN_ARROW: NodeType = NodeType(148);
pub const FAT_ARROW: NodeType = NodeType(149);
pub const EQ: NodeType = NodeType(150);
pub const EQEQ: NodeType = NodeType(151);
pub const BANGEQ: NodeType = NodeType(152);
pub const GTET: NodeType = NodeType(153);
pub const LTEQ: NodeType = NodeType(154);
pub const SEMI: NodeType = NodeType(155);
pub const COLON: NodeType = NodeType(156);
pub const COLONCOLON: NodeType = NodeType(157);
pub const COMMA: NodeType = NodeType(158);
pub const DOT: NodeType = NodeType(159);
pub const DOTDOT: NodeType = NodeType(160);
pub const DOTDOTDOT: NodeType = NodeType(161);
pub const HASH: NodeType = NodeType(162);
pub const DOLLAR: NodeType = NodeType(163);
pub const STAR: NodeType = NodeType(164);
pub const STAR_EQ: NodeType = NodeType(165);
pub const SLASH: NodeType = NodeType(166);
pub const SLASH_EQ: NodeType = NodeType(167);
pub const PERCENT: NodeType = NodeType(168);
pub const PERCENT_EQ: NodeType = NodeType(169);
pub const PLUS: NodeType = NodeType(170);
pub const PLUS_EQ: NodeType = NodeType(171);
pub const MINUS: NodeType = NodeType(172);
pub const MINUS_EQ: NodeType = NodeType(173);
pub const AMPERSAND: NodeType = NodeType(174);
pub const AMPERSAND_EQ: NodeType = NodeType(175);
pub const PIPE: NodeType = NodeType(176);
pub const PIPE_EQ: NodeType = NodeType(177);
pub const UNDERSCORE: NodeType = NodeType(178);
pub const BANG: NodeType = NodeType(179);
pub const QUESTION: NodeType = NodeType(180);
pub const CARET: NodeType = NodeType(181);
pub const CARET_EQ: NodeType = NodeType(182);
pub const CHAR: NodeType = NodeType(183);
pub const LIFETIME: NodeType = NodeType(184);
pub const BOOL: NodeType = NodeType(185);
pub const NUMBER: NodeType = NodeType(186);
pub const STRING: NodeType = NodeType(187);
pub const RAW_STRING: NodeType = NodeType(188);
pub const IDENT: NodeType = NodeType(189);
pub const FILE: NodeType = NodeType(190);
pub const USE_DECL: NodeType = NodeType(191);
pub const USE_SPEC: NodeType = NodeType(192);
pub const USE_SPEC_ENTRY: NodeType = NodeType(193);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(194);
pub const FN_DEF: NodeType = NodeType(195);
pub const LINKAGE: NodeType = NodeType(196);
pub const VALUE_PARAM: NodeType = NodeType(197);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(198);
pub const SELF_PARAMETER: NodeType = NodeType(199);
pub const STRUCT_DEF: NodeType = NodeType(200);
pub const STRUCT_FIELD: NodeType = NodeType(201);
pub const TUPLE_FIELD: NodeType = NodeType(202);
pub const ENUM_DEF: NodeType = NodeType(203);
pub const ENUM_VARIANT: NodeType = NodeType(204);
pub const MOD_DEF: NodeType = NodeType(205);
pub const IMPL_DEF: NodeType = NodeType(206);
pub const TRAIT_DEF: NodeType = NodeType(207);
pub const MEMBERS: NodeType = NodeType(208);
pub const TYPE_DEF: NodeType = NodeType(209);
pub const CONST_DEF: NodeType = NodeType(210);
pub const MACRO_ITEM: NodeType = NodeType(211);
pub const EXTERN_BLOCK: NodeType = NodeType(212);
pub const TYPE_PARAMETERS: NodeType = NodeType(213);
pub const TYPE_PARAMETER: NodeType = NodeType(214);
pub const TYPE_BOUND: NodeType = NodeType(215);
pub const LIFETIME_PARAMETER: NodeType = NodeType(216);
pub const VISIBILITY: NodeType = NodeType(217);
pub const WHERE_CLAUSE: NodeType = NodeType(218);
pub const PATH: NodeType = NodeType(219);
pub const TYPE_ARGUMENTS: NodeType = NodeType(220);
pub const FN_TRAIT_SUGAR: NodeType = NodeType(221);
pub const ALIAS: NodeType = NodeType(222);
pub const PATH_TYPE: NodeType = NodeType(223);
pub const REFERENCE_TYPE: NodeType = NodeType(224);
pub const PLACEHOLDER_TYPE: NodeType = NodeType(225);
pub const UNIT_TYPE: NodeType = NodeType(226);
pub const PAREN_TYPE: NodeType = NodeType(227);
pub const TUPLE_TYPE: NodeType = NodeType(228);
pub const NEVER_TYPE: NodeType = NodeType(229);
pub const ARRAY_TYPE: NodeType = NodeType(230);
pub const FN_POINTER_TYPE: NodeType = NodeType(231);
pub const WILDCARD_PATTERN: NodeType = NodeType(232);
pub const PATH_PATTERN: NodeType = NodeType(233);
pub const TUPE_STRUCT_PATTERN: NodeType = NodeType(234);
pub const STRUCT_PATTERN: NodeType = NodeType(235);
pub const STRUCT_PATTERN_FIELD: NodeType = NodeType(236);
pub const BINDING_PATTERN: NodeType = NodeType(237);
pub const LITERAL_PATTERN: NodeType = NodeType(238);
pub const UNIT_PATTERN: NodeType = NodeType(239);
pub const PAREN_PATTERN: NodeType = NodeType(240);
pub const TUPLE_PATTERN: NodeType = NodeType(241);
pub const REFERENCE_PATTERN: NodeType = NodeType(242);
pub const EXPR: NodeType = NodeType(243);
pub const LITERAL: NodeType = NodeType(244);
pub const PATH_EXPR: NodeType = NodeType(245);
pub const STRUCT_LITERAL: NodeType = NodeType(246);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(247);
pub const UNIT_EXPR: NodeType = NodeType(248);
pub const PAREN_EXPR: NodeType = NodeType(249);
pub const TUPLE_EXPR: NodeType = NodeType(250);
pub const ARRAY_LITERAL: NodeType = NodeType(251);
pub const LAMBDA_EXPR: NodeType = NodeType(252);
pub const RETURN_EXPR: NodeType = NodeType(253);
pub const BLOCK_EXPR: NodeType = NodeType(254);
pub const LET_STMT: NodeType = NodeType(255);
pub const TYPE_ASCRIPTION: NodeType = NodeType(256);
pub const INITIALIZER: NodeType = NodeType(257);
pub const EMPTY_STMT: NodeType = NodeType(258);
pub const EXPR_STMT: NodeType = NodeType(259);
pub const IF_EXPR: NodeType = NodeType(260);
pub const WHILE_EXPR: NodeType = NodeType(261);
pub const LOOP_EXPR: NodeType = NodeType(262);
pub const FOR_EXPR: NodeType = NodeType(263);
pub const MATCH_EXPR: NodeType = NodeType(264);
pub const MATCH_ARM: NodeType = NodeType(265);
pub const GUARD: NodeType = NodeType(266);
pub const BLOCK_MACRO_EXPR: NodeType = NodeType(267);
pub const LINE_MACRO_EXPR: NodeType = NodeType(268);
pub const METHOD_CALL_EXPR: NodeType = NodeType(269);
pub const CALL_EXPR: NodeType = NodeType(270);
pub const VALUE_ARGUMENT: NodeType = NodeType(271);
pub const FIELD_EXPR: NodeType = NodeType(272);
pub const INDEX_EXPR: NodeType = NodeType(273);
pub const TRY_EXPR: NodeType = NodeType(274);
pub const CAST_EXPR: NodeType = NodeType(275);
pub const REFERENCE_EXPR: NodeType = NodeType(276);
pub const DEREFERENCE_EXPR: NodeType = NodeType(277);
pub const NEGATION_EXPR: NodeType = NodeType(278);
pub const NOT_EXPR: NodeType = NodeType(279);
pub const PRODUCT_EXPR: NodeType = NodeType(280);
pub const SUM_EXPR: NodeType = NodeType(281);
pub const BIT_SHIFT: NodeType = NodeType(282);
pub const BIT_AND: NodeType = NodeType(283);
pub const BIT_XOR: NodeType = NodeType(284);
pub const BIT_OR: NodeType = NodeType(285);
pub const COMPARISON: NodeType = NodeType(286);
pub const LOGICAL_AND: NodeType = NodeType(287);
pub const LOGICAL_OR: NodeType = NodeType(288);
pub const RANGE_EXPR: NodeType = NodeType(289);
pub const ASSIGNMENT_EXPR: NodeType = NodeType(290);
pub const ATTRIBUTE: NodeType = NodeType(291);
pub const ATTR_VALUE: NodeType = NodeType(292);
pub const BLOCK_MACRO: NodeType = NodeType(293);
pub const LINE_MACRO: NodeType = NodeType(294);
pub const TT: NodeType = NodeType(295);


pub fn language() -> &'static Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(LINE_COMMENT, "//.*\\n?", None),
            ::fall_parse::LexRule::new(BLOCK_COMMENT, "/\\*", Some(parse_block_comment)),
            ::fall_parse::LexRule::new(AS, "as", None),
            ::fall_parse::LexRule::new(CRATE, "crate", None),
            ::fall_parse::LexRule::new(EXTERN, "extern", None),
            ::fall_parse::LexRule::new(FN, "fn", None),
            ::fall_parse::LexRule::new(LET, "let", None),
            ::fall_parse::LexRule::new(PUB, "pub", None),
            ::fall_parse::LexRule::new(STRUCT, "struct", None),
            ::fall_parse::LexRule::new(USE, "use", None),
            ::fall_parse::LexRule::new(MOD, "mod", None),
            ::fall_parse::LexRule::new(IF, "if", None),
            ::fall_parse::LexRule::new(ELSE, "else", None),
            ::fall_parse::LexRule::new(ENUM, "enum", None),
            ::fall_parse::LexRule::new(IMPL, "impl", None),
            ::fall_parse::LexRule::new(SELF, "self", None),
            ::fall_parse::LexRule::new(SUPER, "super", None),
            ::fall_parse::LexRule::new(TYPE, "type", None),
            ::fall_parse::LexRule::new(CONST, "const", None),
            ::fall_parse::LexRule::new(STATIC, "static", None),
            ::fall_parse::LexRule::new(FOR, "for", None),
            ::fall_parse::LexRule::new(LOOP, "loop", None),
            ::fall_parse::LexRule::new(WHILE, "while", None),
            ::fall_parse::LexRule::new(MOVE, "move", None),
            ::fall_parse::LexRule::new(MUT, "mut", None),
            ::fall_parse::LexRule::new(REF, "ref", None),
            ::fall_parse::LexRule::new(TRAIT, "trait", None),
            ::fall_parse::LexRule::new(MATCH, "match", None),
            ::fall_parse::LexRule::new(RETURN, "return", None),
            ::fall_parse::LexRule::new(IN, "in", None),
            ::fall_parse::LexRule::new(UNSAFE, "unsafe", None),
            ::fall_parse::LexRule::new(WHERE, "where", None),
            ::fall_parse::LexRule::new(L_PAREN, "\\(", None),
            ::fall_parse::LexRule::new(R_PAREN, "\\)", None),
            ::fall_parse::LexRule::new(L_CURLY, "\\{", None),
            ::fall_parse::LexRule::new(R_CURLY, "\\}", None),
            ::fall_parse::LexRule::new(L_ANGLE, "<", None),
            ::fall_parse::LexRule::new(R_ANGLE, ">", None),
            ::fall_parse::LexRule::new(L_BRACK, "\\[", None),
            ::fall_parse::LexRule::new(R_BRACK, "\\]", None),
            ::fall_parse::LexRule::new(THIN_ARROW, "\\->", None),
            ::fall_parse::LexRule::new(FAT_ARROW, "=>", None),
            ::fall_parse::LexRule::new(EQ, "=", None),
            ::fall_parse::LexRule::new(EQEQ, "==", None),
            ::fall_parse::LexRule::new(BANGEQ, "!=", None),
            ::fall_parse::LexRule::new(GTET, ">=", None),
            ::fall_parse::LexRule::new(LTEQ, "<=", None),
            ::fall_parse::LexRule::new(SEMI, ";", None),
            ::fall_parse::LexRule::new(COLON, ":", None),
            ::fall_parse::LexRule::new(COLONCOLON, "::", None),
            ::fall_parse::LexRule::new(COMMA, ",", None),
            ::fall_parse::LexRule::new(DOT, "\\.", None),
            ::fall_parse::LexRule::new(DOTDOT, "\\.\\.", None),
            ::fall_parse::LexRule::new(DOTDOTDOT, "\\.\\.\\.", None),
            ::fall_parse::LexRule::new(HASH, "\\#", None),
            ::fall_parse::LexRule::new(DOLLAR, "\\$", None),
            ::fall_parse::LexRule::new(STAR, "\\*", None),
            ::fall_parse::LexRule::new(STAR_EQ, "\\*=", None),
            ::fall_parse::LexRule::new(SLASH, "/", None),
            ::fall_parse::LexRule::new(SLASH_EQ, "/=", None),
            ::fall_parse::LexRule::new(PERCENT, "%", None),
            ::fall_parse::LexRule::new(PERCENT_EQ, "%=", None),
            ::fall_parse::LexRule::new(PLUS, "\\+", None),
            ::fall_parse::LexRule::new(PLUS_EQ, "\\+=", None),
            ::fall_parse::LexRule::new(MINUS, "\\-", None),
            ::fall_parse::LexRule::new(MINUS_EQ, "\\-=", None),
            ::fall_parse::LexRule::new(AMPERSAND, "\\&", None),
            ::fall_parse::LexRule::new(AMPERSAND_EQ, "\\&=", None),
            ::fall_parse::LexRule::new(PIPE, "\\|", None),
            ::fall_parse::LexRule::new(PIPE_EQ, "\\|=", None),
            ::fall_parse::LexRule::new(UNDERSCORE, "_", None),
            ::fall_parse::LexRule::new(BANG, "!", None),
            ::fall_parse::LexRule::new(QUESTION, "\\?", None),
            ::fall_parse::LexRule::new(CARET, "\\^", None),
            ::fall_parse::LexRule::new(CARET_EQ, "\\^=", None),
            ::fall_parse::LexRule::new(CHAR, "\'(\\s*|\\S*)\'", None),
            ::fall_parse::LexRule::new(LIFETIME, "\'\\p{XID_Continue}*", None),
            ::fall_parse::LexRule::new(BOOL, "true|false", None),
            ::fall_parse::LexRule::new(NUMBER, "\\d+", None),
            ::fall_parse::LexRule::new(STRING, "\"([^\"]|\\\\\")*\"", None),
            ::fall_parse::LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(IDENT, "(\\p{XID_Start}|_)\\p{XID_Continue}*", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"body":{"Pub":{"ty_idx":91,"body":{"Or":[{"And":[[{"Rule":1}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":11}],null]},{"And":[[{"ContextualToken":[4,"union"]}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":12}],null]},{"And":[[{"Token":13}],null]},{"And":[[{"Token":17}],null]},{"And":[[{"Token":29}],null]},{"And":[[{"Rule":25}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":24}],null]},{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":15}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":20}],null]},{"And":[[{"Rule":21}],null]},{"And":[[{"Rule":22}],null]},{"And":[[{"Rule":29}],null]}]}},{"body":{"Pub":{"ty_idx":92,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":12},{"Or":[{"And":[[{"Rule":38},{"Or":[{"And":[[{"Rule":47}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":58},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":58}},{"Rule":5}],null]}]},{"Token":56}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":93,"body":{"Or":[{"And":[[{"Token":65}],null]},{"And":[[{"Call":[{"Rule":141},[[3,{"Call":[{"Rule":140},[[2,{"Rule":6}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":94,"body":{"Or":[{"And":[[{"Token":18}],null]},{"And":[[{"Token":90},{"Opt":{"Rule":47}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":95,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":7},{"Token":6},{"Token":90},{"Opt":{"Rule":47}},{"Token":56}],4]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":96,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Opt":{"Rule":10}},{"Token":8},{"Token":90},{"Opt":{"Rule":31}},{"Rule":11},{"Opt":{"Rule":9}},{"Opt":{"Rule":37}},{"Or":[{"And":[[{"Rule":85}],null]},{"And":[[{"Token":56}],null]}]}],4]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":49},{"Rule":48}],null]}]}},{"body":{"Pub":{"ty_idx":97,"body":{"Or":[{"And":[[{"Token":7},{"Opt":{"Token":88}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Opt":{"Rule":14}},{"Call":[{"Rule":140},[[2,{"Rule":12}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":98,"body":{"Or":[{"And":[[{"Rule":59},{"Token":57},{"Rule":48}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":99,"body":{"Or":[{"And":[[{"Rule":59},{"Opt":{"Or":[{"And":[[{"Token":57},{"Rule":48}],null]}]}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":100,"body":{"Or":[{"And":[[{"Opt":{"Token":75}},{"Opt":{"Token":27}},{"Token":18},{"Opt":{"Or":[{"And":[[{"Token":57},{"Rule":48}],null]}]}},{"Or":[{"And":[[{"Token":59}],null]},{"And":[["Eof"],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":101,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Or":[{"And":[[{"Token":11}],null]},{"And":[[{"ContextualToken":[4,"union"]}],null]}]},{"Token":90},{"Opt":{"Rule":31}},{"Or":[{"And":[[{"Call":[{"Rule":141},[[3,{"Call":[{"Rule":140},[[2,{"Rule":16}]]]}]]]}],null]},{"And":[[{"Token":56}],null]},{"And":[[{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":17}]]]}]]]},{"Token":56}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":102,"body":{"Or":[{"And":[[{"Opt":{"Rule":36}},{"Token":90},{"Token":57},{"Rule":48}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":103,"body":{"Or":[{"And":[[{"Opt":{"Rule":36}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":104,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":16},{"Token":90},{"Call":[{"Rule":141},[[3,{"Call":[{"Rule":140},[[2,{"Rule":19}]]]}]]]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":105,"body":{"Or":[{"And":[[{"Token":90},{"Opt":{"Or":[{"And":[[{"Token":51},{"Rule":71}],null]},{"And":[[{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":17}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":141},[[3,{"Call":[{"Rule":140},[[2,{"Rule":16}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":106,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":13},{"Token":90},{"Or":[{"And":[[{"Token":56}],null]},{"And":[[{"Call":[{"Rule":141},[[3,{"Rule":1}]]]}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":107,"body":{"Or":[{"And":[[{"Rule":135},{"Token":17},{"Opt":{"Rule":31}},{"Or":[{"And":[[{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":23},{"Rule":48}],null]}]}}],null]}]},{"Rule":23}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":108,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":29},{"Token":90},{"Opt":{"Rule":31}},{"Rule":23}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":109,"body":{"Or":[{"And":[[{"Call":[{"Rule":141},[[3,{"Rep":{"WithSkip":[{"Rule":25},{"Rule":24}]}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]},{"And":[[{"Rule":28}],null]}]}},{"body":{"Or":[{"And":[[{"Token":10}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]},{"And":[[{"Token":63}],null]},{"And":[[{"Token":7}],null]},{"And":[[{"Or":[{"And":[[{"Token":90},{"Token":80}],null]}]}],null]}]}},{"body":{"Pub":{"ty_idx":110,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Token":20},{"Token":90},{"Opt":{"Rule":31}},{"Opt":{"Or":[{"And":[[{"Token":51},{"Rule":48}],null]}]}},{"Token":56}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":111,"body":{"Or":[{"And":[[{"Rule":135},{"Opt":{"Rule":36}},{"Or":[{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]}]},{"Token":90},{"Token":57},{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":51},{"Rule":71}],null]}]}},{"Token":56}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":112,"body":{"Or":[{"And":[[{"Rule":137}],null]},{"And":[[{"Rule":138},{"Token":56}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":113,"body":{"Or":[{"And":[[{"Rule":10},{"Call":[{"Rule":141},[[3,{"Rep":{"Rule":30}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":27}],null]}]}},{"body":{"Pub":{"ty_idx":114,"body":{"Or":[{"And":[[{"Call":[{"Rule":143},[[5,{"Or":[{"And":[[{"Call":[{"Rule":140},[[2,{"Rule":35}]]]},{"Call":[{"Rule":140},[[2,{"Rule":32}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":115,"body":{"Or":[{"And":[[{"Token":90},{"Opt":{"Rule":33}}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":57},{"Rep":{"Or":[{"And":[[{"Rule":34},{"Or":[{"And":[[{"Token":71}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":59}}}],null]},{"And":[[{"Not":{"Not":{"Token":37}}}],null]}]}],1]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":116,"body":{"Or":[{"And":[[{"Token":85}],null]},{"And":[[{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":117,"body":{"Or":[{"And":[[{"Token":85},{"Opt":{"Or":[{"And":[[{"Token":57},{"Rep":{"Or":[{"And":[[{"Token":85},{"Or":[{"And":[[{"Token":71}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":59}}}],null]}]}],1]}]}}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":118,"body":{"Or":[{"And":[[{"Token":10}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":119,"body":{"Or":[{"And":[[{"Token":34},{"Rep":{"Or":[{"And":[[{"Rule":48},{"Rule":33},{"Or":[{"And":[[{"Token":59}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":37}}}],null]}]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":41}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[3,{"Rule":41}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":41}]}],null]}]}},{"body":{"Pratt":{"atoms":[{"Pub":{"ty_idx":120,"body":{"Or":[{"And":[[{"Opt":{"Token":58}},{"Rule":44}],null]}]},"replaceable":false}}],"prefixes":[],"infixes":[{"ty":120,"op":{"Or":[{"And":[[{"Token":58},{"Rule":44}],null]}]},"priority":999,"has_rhs":false}]}}},{"body":{"Pub":{"ty_idx":120,"body":{"Or":[{"And":[[{"Opt":{"Token":58}},{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":120,"body":{"Or":[{"And":[[{"Rule":41},{"Or":[{"And":[[{"Token":58},{"Rule":44}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":90}],null]},{"And":[[{"Token":18}],null]},{"And":[[{"Token":19}],null]}]},{"Opt":{"Or":[{"And":[[{"Rule":45}],null]},{"And":[[{"IsIn":3},{"Rule":46}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":121,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":3}],null]},{"And":[[{"IsIn":1},{"Token":58}],null]}]},{"Call":[{"Rule":143},[[5,{"Or":[{"And":[[{"Call":[{"Rule":140},[[2,{"Token":85}]]]},{"Call":[{"Rule":140},[[2,{"Rule":48}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":122,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":48}]]]}]]]},{"Opt":{"Rule":9}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":123,"body":{"Or":[{"And":[[{"Token":5},{"Token":90}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":49}],null]},{"And":[[{"Rule":50}],null]},{"And":[[{"Rule":51}],null]},{"And":[[{"Rule":52}],null]},{"And":[[{"Rule":53}],null]},{"And":[[{"Rule":55}],null]},{"And":[[{"Rule":56}],null]},{"And":[[{"Rule":57}],null]}]}},{"body":{"Pub":{"ty_idx":124,"body":{"Or":[{"And":[[{"Rule":39}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":125,"body":{"Or":[{"And":[[{"Token":75},{"Opt":{"Token":85}},{"Opt":{"Token":27}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":126,"body":{"Or":[{"And":[[{"Token":79}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":127,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":128,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Rule":48},{"Opt":{"Rule":54}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":129,"body":{"Or":[{"And":[[{"Token":59},{"Call":[{"Rule":140},[[2,{"Rule":48}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":130,"body":{"Or":[{"And":[[{"Token":80}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Call":[{"Rule":144},[[6,{"Or":[{"And":[[{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":56},{"Rule":71}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Token":8},{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":58}]]]}]]]},{"Rule":9}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":98,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Rule":59},{"Token":57}],null]}]}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":60}],null]},{"And":[[{"Rule":61}],null]},{"And":[[{"Rule":65}],null]},{"And":[[{"Rule":66}],null]},{"And":[[{"Rule":67}],null]},{"And":[[{"Rule":68}],null]},{"And":[[{"Rule":70}],null]}]}},{"body":{"Pub":{"ty_idx":133,"body":{"Or":[{"And":[[{"Token":79}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":134,"body":{"Or":[{"And":[[{"Rule":40},{"Opt":{"Or":[{"And":[[{"Rule":62}],null]},{"And":[[{"Rule":63}],null]}]}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":135,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Call":[{"Rule":140},[[2,{"Rule":59}]]]},{"Opt":{"Or":[{"And":[[{"Token":61},{"Opt":{"Token":59}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"PubReplace":{"ty_idx":136,"body":{"Or":[{"And":[[{"Call":[{"Rule":141},[[3,{"Or":[{"And":[[{"Call":[{"Rule":140},[[2,{"Rule":64}]]]},{"Opt":{"Or":[{"And":[[{"Token":61},{"Opt":{"Token":59}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":137,"body":{"Or":[{"And":[[{"Rule":65},{"Not":{"Token":57}}],null]},{"And":[[{"Token":90},{"Token":57},{"Rule":59}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":138,"body":{"Or":[{"And":[[{"Opt":{"Token":28}},{"Opt":{"Token":27}},{"Token":90}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":139,"body":{"Or":[{"And":[[{"Rule":74}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":140,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":141,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Rule":59},{"Opt":{"Rule":69}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":142,"body":{"Or":[{"And":[[{"Token":59},{"Call":[{"Rule":140},[[2,{"Rule":59}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":143,"body":{"Or":[{"And":[[{"Token":75},{"Opt":{"Token":27}},{"Rule":59}],null]}]},"replaceable":false}}},{"body":{"Pratt":{"atoms":[{"Pub":{"ty_idx":145,"body":{"Or":[{"And":[[{"Token":87}],null]},{"And":[[{"Token":88}],null]},{"And":[[{"Token":89}],null]},{"And":[[{"Token":84}],null]},{"And":[[{"Token":86}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":146,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":90},{"Token":80}],null]}]}},{"Rule":40},{"Opt":{"Rule":76}}],null]}]},"replaceable":true}},{"Pub":{"ty_idx":149,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":150,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Call":[{"Rule":73},[[0,{"Rule":71}]]]},{"Opt":{"Rule":80}}],null]}]}]]]}],null]}]},"replaceable":true}},{"Pub":{"ty_idx":152,"body":{"Or":[{"And":[[{"Call":[{"Rule":144},[[6,{"Call":[{"Rule":73},[[0,{"Call":[{"Rule":140},[[2,{"Rule":71}]]]}]]]}]]]}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":153,"body":{"Or":[{"And":[[{"Opt":{"Token":26}},{"Token":77},{"Rep":{"Rule":83}},{"Token":77},{"Or":[{"And":[[{"Token":49},{"Rule":48},{"Rule":85}],null]},{"And":[[{"Call":[{"Rule":73},[[0,{"Rule":71}]]]}],null]}]}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":154,"body":{"Or":[{"And":[[{"Token":31},{"Opt":{"Rule":71}}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":155,"body":{"Or":[{"And":[[{"Call":[{"Rule":73},[[0,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Call":[{"Rule":141},[[3,{"Or":[{"And":[[{"Rep":{"Rule":86}},{"Opt":{"Rule":71}}],null]}]}]]]}],null]}]}]]]}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":161,"body":{"Or":[{"And":[[{"Token":14},{"Rule":94},{"Rule":85},{"Opt":{"Or":[{"And":[[{"Token":15},{"Or":[{"And":[[{"Rule":85}],null]},{"And":[[{"Rule":92}],null]}]}],null]}]}}],1]}]},"replaceable":false}},{"Pub":{"ty_idx":162,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":25},{"Rule":94},{"Rule":85}],2]}]},"replaceable":false}},{"Pub":{"ty_idx":163,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":24},{"Rule":85}],2]}]},"replaceable":false}},{"Pub":{"ty_idx":164,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":23},{"Rule":59},{"Token":32},{"Rule":95},{"Rule":85}],2]}]},"replaceable":false}},{"Pub":{"ty_idx":165,"body":{"Or":[{"And":[[{"Token":30},{"Rule":95},{"Call":[{"Rule":141},[[3,{"Rep":{"Rule":100}}]]]}],1]}]},"replaceable":false}},{"Pub":{"ty_idx":168,"body":{"Or":[{"And":[[{"Rule":137}],null]}]},"replaceable":false}},{"Pub":{"ty_idx":169,"body":{"Or":[{"And":[[{"Rule":138}],null]}]},"replaceable":false}}],"prefixes":[{"ty":177,"op":{"Or":[{"And":[[{"Token":75},{"Opt":{"Token":27}}],null]}]},"priority":999},{"ty":178,"op":{"Token":65},"priority":999},{"ty":179,"op":{"Token":73},"priority":999},{"ty":180,"op":{"Token":80},"priority":999},{"ty":190,"op":{"Rule":132},"priority":2}],"infixes":[{"ty":170,"op":{"Or":[{"And":[[{"Token":60},{"Token":90},{"Opt":{"Enter":[1,{"Rule":45}]}},{"Rule":107}],null]}]},"priority":999,"has_rhs":false},{"ty":171,"op":{"Rule":107},"priority":999,"has_rhs":false},{"ty":173,"op":{"Or":[{"And":[[{"Token":60},{"Or":[{"And":[[{"Token":90}],null]},{"And":[[{"Token":87}],null]}]}],null]}]},"priority":999,"has_rhs":false},{"ty":174,"op":{"Call":[{"Rule":144},[[6,{"Rule":71}]]]},"priority":999,"has_rhs":false},{"ty":175,"op":{"Token":81},"priority":999,"has_rhs":false},{"ty":176,"op":{"Or":[{"And":[[{"Token":5},{"Rule":48}],null]}]},"priority":999,"has_rhs":false},{"ty":181,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":65}],null]},{"And":[[{"Token":67}],null]},{"And":[[{"Token":69}],null]}]}]]]},"priority":11,"has_rhs":true},{"ty":182,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":71}],null]},{"And":[[{"Token":73}],null]}]}]]]},"priority":10,"has_rhs":true},{"ty":183,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"ContextualToken":[43,"<<"]}],null]},{"And":[[{"ContextualToken":[45,">>"]}],null]}]}]]]},"priority":9,"has_rhs":true},{"ty":184,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":75},{"Not":{"Token":75}}],null]}]}]]]},"priority":8,"has_rhs":true},{"ty":185,"op":{"Call":[{"Rule":120},[[1,{"Token":82}]]]},"priority":7,"has_rhs":true},{"ty":186,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":77},{"Not":{"Token":77}}],null]}]}]]]},"priority":6,"has_rhs":true},{"ty":187,"op":{"Call":[{"Rule":120},[[1,{"Rule":125}]]]},"priority":5,"has_rhs":true},{"ty":188,"op":{"Call":[{"Rule":120},[[1,{"ContextualToken":[47,"&&"]}]]]},"priority":4,"has_rhs":true},{"ty":189,"op":{"Call":[{"Rule":120},[[1,{"ContextualToken":[48,"||"]}]]]},"priority":3,"has_rhs":true},{"ty":190,"op":{"Call":[{"Rule":120},[[1,{"Rule":132}]]]},"priority":2,"has_rhs":true},{"ty":190,"op":{"Rule":131},"priority":2,"has_rhs":false},{"ty":191,"op":{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":51}],null]},{"And":[[{"Token":72}],null]},{"And":[[{"Token":74}],null]},{"And":[[{"Token":66}],null]},{"And":[[{"Token":68}],null]},{"And":[[{"Token":70}],null]},{"And":[[{"Token":76}],null]},{"And":[[{"Token":78}],null]},{"And":[[{"Token":83}],null]},{"And":[[{"ContextualToken":[46,">>="]}],null]},{"And":[[{"ContextualToken":[44,"<<="]}],null]}]}]]]},"priority":1,"has_rhs":true}]}}},{"body":{"Or":[{"And":[[{"PrevIs":[155,161,162,163,164,165,168]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[0,{"Exit":[2,{"Var":0}]}]}],null]}]}},{"body":{"Pub":{"ty_idx":145,"body":{"Or":[{"And":[[{"Token":87}],null]},{"And":[[{"Token":88}],null]},{"And":[[{"Token":89}],null]},{"And":[[{"Token":84}],null]},{"And":[[{"Token":86}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":146,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":90},{"Token":80}],null]}]}},{"Rule":40},{"Opt":{"Rule":76}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":147,"body":{"Or":[{"And":[[{"Not":{"IsIn":0}},{"Call":[{"Rule":141},[[3,{"Or":[{"And":[[{"Call":[{"Rule":140},[[2,{"Rule":77}]]]},{"Opt":{"Or":[{"And":[[{"Token":61},{"Call":[{"Rule":73},[[0,{"Rule":71}]]]}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":148,"body":{"Or":[{"And":[[{"Token":90},{"Opt":{"Or":[{"And":[[{"Token":57},{"Rule":71}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":149,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":150,"body":{"Or":[{"And":[[{"Call":[{"Rule":142},[[4,{"Or":[{"And":[[{"Call":[{"Rule":73},[[0,{"Rule":71}]]]},{"Opt":{"Rule":80}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":151,"body":{"Or":[{"And":[[{"Token":59},{"Call":[{"Rule":140},[[2,{"Call":[{"Rule":73},[[0,{"Rule":71}]]]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":152,"body":{"Or":[{"And":[[{"Call":[{"Rule":144},[[6,{"Call":[{"Rule":73},[[0,{"Call":[{"Rule":140},[[2,{"Rule":71}]]]}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":153,"body":{"Or":[{"And":[[{"Opt":{"Token":26}},{"Token":77},{"Rep":{"Rule":83}},{"Token":77},{"Or":[{"And":[[{"Token":49},{"Rule":48},{"Rule":85}],null]},{"And":[[{"Call":[{"Rule":73},[[0,{"Rule":71}]]]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":13},{"Or":[{"And":[[{"Token":59}],null]},{"And":[[{"Not":{"Not":{"Token":77}}}],null]}]}],1]}]}},{"body":{"Pub":{"ty_idx":154,"body":{"Or":[{"And":[[{"Token":31},{"Opt":{"Rule":71}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":155,"body":{"Or":[{"And":[[{"Call":[{"Rule":73},[[0,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Call":[{"Rule":141},[[3,{"Or":[{"And":[[{"Rep":{"Rule":86}},{"Opt":{"Rule":71}}],null]}]}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":87}],null]},{"And":[[{"Rule":91}],null]},{"And":[[{"Rule":90}],null]},{"And":[[{"Rule":3}],null]}]}},{"body":{"Pub":{"ty_idx":156,"body":{"Or":[{"And":[[{"Token":9},{"Rule":59},{"Opt":{"Rule":88}},{"Opt":{"Rule":89}},{"Token":56}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":157,"body":{"Or":[{"And":[[{"Token":57},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":158,"body":{"Or":[{"And":[[{"Token":51},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":159,"body":{"Or":[{"And":[[{"Token":56}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":160,"body":{"Or":[{"And":[[{"Enter":[2,{"Or":[{"And":[[{"Rule":71},{"Or":[{"And":[[{"Rule":72},{"Not":"Eof"}],null]},{"And":[[{"Token":56}],null]}]}],null]}]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":161,"body":{"Or":[{"And":[[{"Token":14},{"Rule":94},{"Rule":85},{"Opt":{"Or":[{"And":[[{"Token":15},{"Or":[{"And":[[{"Rule":85}],null]},{"And":[[{"Rule":92}],null]}]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":162,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":25},{"Rule":94},{"Rule":85}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":9},{"Rule":59},{"Token":51}],1]}]}},{"Rule":95}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":71}]}],null]}]}},{"body":{"Pub":{"ty_idx":163,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":24},{"Rule":85}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":164,"body":{"Or":[{"And":[[{"Opt":{"Rule":98}},{"Token":23},{"Rule":59},{"Token":32},{"Rule":95},{"Rule":85}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":85},{"Token":57}],null]}]}},{"body":{"Pub":{"ty_idx":165,"body":{"Or":[{"And":[[{"Token":30},{"Rule":95},{"Call":[{"Rule":141},[[3,{"Rep":{"Rule":100}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":166,"body":{"Or":[{"And":[[{"Rule":101},{"Token":50},{"Rule":71},{"Or":[{"And":[[{"Token":59}],null]},{"And":[["Eof"],null]},{"And":[[{"Rule":72}],null]}]}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":59},{"Rep":{"Or":[{"And":[[{"Token":77},{"Rule":59}],null]}]}},{"Opt":{"Rule":102}}],null]}]}},{"body":{"Pub":{"ty_idx":167,"body":{"Or":[{"And":[[{"Token":14},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":168,"body":{"Or":[{"And":[[{"Rule":137}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":169,"body":{"Or":[{"And":[[{"Rule":138}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":170,"body":{"Or":[{"And":[[{"Rule":71},{"Or":[{"And":[[{"Token":60},{"Token":90},{"Opt":{"Enter":[1,{"Rule":45}]}},{"Rule":107}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":171,"body":{"Or":[{"And":[[{"Rule":71},{"Rule":107}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":73},[[0,{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":108}]]]}]]]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":172,"body":{"Or":[{"And":[[{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":173,"body":{"Or":[{"And":[[{"Rule":71},{"Or":[{"And":[[{"Token":60},{"Or":[{"And":[[{"Token":90}],null]},{"And":[[{"Token":87}],null]}]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":174,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":144},[[6,{"Rule":71}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":175,"body":{"Or":[{"And":[[{"Rule":71},{"Token":81}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":176,"body":{"Or":[{"And":[[{"Rule":71},{"Or":[{"And":[[{"Token":5},{"Rule":48}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":177,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":75},{"Opt":{"Token":27}}],null]}]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":178,"body":{"Or":[{"And":[[{"Token":65},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":179,"body":{"Or":[{"And":[[{"Token":73},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":180,"body":{"Or":[{"And":[[{"Token":80},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":181,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":65}],null]},{"And":[[{"Token":67}],null]},{"And":[[{"Token":69}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":182,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":71}],null]},{"And":[[{"Token":73}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":183,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"ContextualToken":[43,"<<"]}],null]},{"And":[[{"ContextualToken":[45,">>"]}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"IsIn":2},{"Not":{"Rule":72}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":2}},{"Var":1}],null]}]}},{"body":{"Pub":{"ty_idx":184,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":75},{"Not":{"Token":75}}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":185,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Token":82}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":186,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":77},{"Not":{"Token":77}}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":187,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Rule":125}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":52}],null]},{"And":[[{"Token":53}],null]},{"And":[[{"Token":39}],null]},{"And":[[{"Token":40}],null]},{"And":[[{"Token":55}],null]},{"And":[[{"Token":54}],null]}]}},{"body":{"Pub":{"ty_idx":188,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"ContextualToken":[47,"&&"]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":189,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"ContextualToken":[48,"||"]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":190,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Rule":132}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":190,"body":{"Or":[{"And":[[{"Rule":132},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":190,"body":{"Or":[{"And":[[{"Rule":71},{"Rule":131}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":61}],null]},{"And":[[{"Token":62}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":131},{"Not":{"Or":[{"And":[[{"Token":37},{"IsIn":0}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":191,"body":{"Or":[{"And":[[{"Rule":71},{"Call":[{"Rule":120},[[1,{"Or":[{"And":[[{"Token":51}],null]},{"And":[[{"Token":72}],null]},{"And":[[{"Token":74}],null]},{"And":[[{"Token":66}],null]},{"And":[[{"Token":68}],null]},{"And":[[{"Token":70}],null]},{"And":[[{"Token":76}],null]},{"And":[[{"Token":78}],null]},{"And":[[{"Token":83}],null]},{"And":[[{"ContextualToken":[46,">>="]}],null]},{"And":[[{"ContextualToken":[44,"<<="]}],null]}]}]]]},{"Rule":71}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":192,"body":{"Or":[{"And":[[{"Token":63},{"Call":[{"Rule":144},[[6,{"Call":[{"Rule":140},[[2,{"Rule":136}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":134}}],null]}]}},{"body":{"Pub":{"ty_idx":193,"body":{"Or":[{"And":[[{"Token":90},{"Opt":{"Or":[{"And":[[{"Token":51},{"Rule":71}],null]},{"And":[[{"Call":[{"Rule":142},[[4,{"Call":[{"Rule":140},[[2,{"Rule":136}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":194,"body":{"Or":[{"And":[[{"Token":90},{"Token":80},{"Opt":{"Token":90}},{"Call":[{"Rule":141},[[3,{"Rep":{"Rule":139}}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":195,"body":{"Or":[{"And":[[{"Token":90},{"Token":80},{"Opt":{"Token":90}},{"Or":[{"And":[[{"Token":35},{"Rep":{"Rule":139}},{"Token":36}],null]},{"And":[[{"Token":41},{"Rep":{"Rule":139}},{"Token":42}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":196,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":35}],null]},{"And":[[{"Token":36}],null]},{"And":[[{"Token":37}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":41}],null]},{"And":[[{"Token":42}],null]}]}},"Any"],null]},{"And":[[{"Token":35},{"Rep":{"Rule":139}},{"Token":36}],null]},{"And":[[{"Token":41},{"Rep":{"Rule":139}},{"Token":42}],null]},{"And":[[{"Token":37},{"Rep":{"Rule":139}},{"Token":38}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":59}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":145},[[7,{"Token":37}],[8,{"Token":38}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":145},[[7,{"Token":35}],[8,{"Token":36}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":145},[[7,{"Token":39}],[8,{"Token":40}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":145},[[7,{"Token":41}],[8,{"Token":42}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":146},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":146},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            whitespace_binder,
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn lexer(&self) -> &self::fall_tree::Lexer {
                    &self.lexer
                }

                fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> INode {
                    self.parser_definition.parse2(text, tokens, &LANG, metrics)
                }

                fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                    match ty {
                        ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LINE_COMMENT => NodeTypeInfo { name: "LINE_COMMENT", whitespace_like: true },
                        BLOCK_COMMENT => NodeTypeInfo { name: "BLOCK_COMMENT", whitespace_like: true },
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
                        CONST => NodeTypeInfo { name: "CONST", whitespace_like: false },
                        STATIC => NodeTypeInfo { name: "STATIC", whitespace_like: false },
                        FOR => NodeTypeInfo { name: "FOR", whitespace_like: false },
                        LOOP => NodeTypeInfo { name: "LOOP", whitespace_like: false },
                        WHILE => NodeTypeInfo { name: "WHILE", whitespace_like: false },
                        MOVE => NodeTypeInfo { name: "MOVE", whitespace_like: false },
                        MUT => NodeTypeInfo { name: "MUT", whitespace_like: false },
                        REF => NodeTypeInfo { name: "REF", whitespace_like: false },
                        TRAIT => NodeTypeInfo { name: "TRAIT", whitespace_like: false },
                        MATCH => NodeTypeInfo { name: "MATCH", whitespace_like: false },
                        RETURN => NodeTypeInfo { name: "RETURN", whitespace_like: false },
                        IN => NodeTypeInfo { name: "IN", whitespace_like: false },
                        UNSAFE => NodeTypeInfo { name: "UNSAFE", whitespace_like: false },
                        WHERE => NodeTypeInfo { name: "WHERE", whitespace_like: false },
                        L_PAREN => NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                        R_PAREN => NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                        L_CURLY => NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                        R_CURLY => NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                        L_ANGLE => NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                        R_ANGLE => NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                        L_BRACK => NodeTypeInfo { name: "L_BRACK", whitespace_like: false },
                        R_BRACK => NodeTypeInfo { name: "R_BRACK", whitespace_like: false },
                        SHL => NodeTypeInfo { name: "SHL", whitespace_like: false },
                        SHL_EQ => NodeTypeInfo { name: "SHL_EQ", whitespace_like: false },
                        SHR => NodeTypeInfo { name: "SHR", whitespace_like: false },
                        SHR_EQ => NodeTypeInfo { name: "SHR_EQ", whitespace_like: false },
                        AND => NodeTypeInfo { name: "AND", whitespace_like: false },
                        OR => NodeTypeInfo { name: "OR", whitespace_like: false },
                        THIN_ARROW => NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                        FAT_ARROW => NodeTypeInfo { name: "FAT_ARROW", whitespace_like: false },
                        EQ => NodeTypeInfo { name: "EQ", whitespace_like: false },
                        EQEQ => NodeTypeInfo { name: "EQEQ", whitespace_like: false },
                        BANGEQ => NodeTypeInfo { name: "BANGEQ", whitespace_like: false },
                        GTET => NodeTypeInfo { name: "GTET", whitespace_like: false },
                        LTEQ => NodeTypeInfo { name: "LTEQ", whitespace_like: false },
                        SEMI => NodeTypeInfo { name: "SEMI", whitespace_like: false },
                        COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                        COLONCOLON => NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                        COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                        DOT => NodeTypeInfo { name: "DOT", whitespace_like: false },
                        DOTDOT => NodeTypeInfo { name: "DOTDOT", whitespace_like: false },
                        DOTDOTDOT => NodeTypeInfo { name: "DOTDOTDOT", whitespace_like: false },
                        HASH => NodeTypeInfo { name: "HASH", whitespace_like: false },
                        DOLLAR => NodeTypeInfo { name: "DOLLAR", whitespace_like: false },
                        STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                        STAR_EQ => NodeTypeInfo { name: "STAR_EQ", whitespace_like: false },
                        SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        SLASH_EQ => NodeTypeInfo { name: "SLASH_EQ", whitespace_like: false },
                        PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                        PERCENT_EQ => NodeTypeInfo { name: "PERCENT_EQ", whitespace_like: false },
                        PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        PLUS_EQ => NodeTypeInfo { name: "PLUS_EQ", whitespace_like: false },
                        MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        MINUS_EQ => NodeTypeInfo { name: "MINUS_EQ", whitespace_like: false },
                        AMPERSAND => NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                        AMPERSAND_EQ => NodeTypeInfo { name: "AMPERSAND_EQ", whitespace_like: false },
                        PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        PIPE_EQ => NodeTypeInfo { name: "PIPE_EQ", whitespace_like: false },
                        UNDERSCORE => NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                        BANG => NodeTypeInfo { name: "BANG", whitespace_like: false },
                        QUESTION => NodeTypeInfo { name: "QUESTION", whitespace_like: false },
                        CARET => NodeTypeInfo { name: "CARET", whitespace_like: false },
                        CARET_EQ => NodeTypeInfo { name: "CARET_EQ", whitespace_like: false },
                        CHAR => NodeTypeInfo { name: "CHAR", whitespace_like: false },
                        LIFETIME => NodeTypeInfo { name: "LIFETIME", whitespace_like: false },
                        BOOL => NodeTypeInfo { name: "BOOL", whitespace_like: false },
                        NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                        RAW_STRING => NodeTypeInfo { name: "RAW_STRING", whitespace_like: false },
                        IDENT => NodeTypeInfo { name: "IDENT", whitespace_like: false },
                        FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                        USE_DECL => NodeTypeInfo { name: "USE_DECL", whitespace_like: false },
                        USE_SPEC => NodeTypeInfo { name: "USE_SPEC", whitespace_like: false },
                        USE_SPEC_ENTRY => NodeTypeInfo { name: "USE_SPEC_ENTRY", whitespace_like: false },
                        EXTERN_CRATE_DECL => NodeTypeInfo { name: "EXTERN_CRATE_DECL", whitespace_like: false },
                        FN_DEF => NodeTypeInfo { name: "FN_DEF", whitespace_like: false },
                        LINKAGE => NodeTypeInfo { name: "LINKAGE", whitespace_like: false },
                        VALUE_PARAM => NodeTypeInfo { name: "VALUE_PARAM", whitespace_like: false },
                        LAMBDA_VALUE_PARAM => NodeTypeInfo { name: "LAMBDA_VALUE_PARAM", whitespace_like: false },
                        SELF_PARAMETER => NodeTypeInfo { name: "SELF_PARAMETER", whitespace_like: false },
                        STRUCT_DEF => NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                        STRUCT_FIELD => NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                        TUPLE_FIELD => NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                        ENUM_DEF => NodeTypeInfo { name: "ENUM_DEF", whitespace_like: false },
                        ENUM_VARIANT => NodeTypeInfo { name: "ENUM_VARIANT", whitespace_like: false },
                        MOD_DEF => NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                        IMPL_DEF => NodeTypeInfo { name: "IMPL_DEF", whitespace_like: false },
                        TRAIT_DEF => NodeTypeInfo { name: "TRAIT_DEF", whitespace_like: false },
                        MEMBERS => NodeTypeInfo { name: "MEMBERS", whitespace_like: false },
                        TYPE_DEF => NodeTypeInfo { name: "TYPE_DEF", whitespace_like: false },
                        CONST_DEF => NodeTypeInfo { name: "CONST_DEF", whitespace_like: false },
                        MACRO_ITEM => NodeTypeInfo { name: "MACRO_ITEM", whitespace_like: false },
                        EXTERN_BLOCK => NodeTypeInfo { name: "EXTERN_BLOCK", whitespace_like: false },
                        TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                        TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                        TYPE_BOUND => NodeTypeInfo { name: "TYPE_BOUND", whitespace_like: false },
                        LIFETIME_PARAMETER => NodeTypeInfo { name: "LIFETIME_PARAMETER", whitespace_like: false },
                        VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                        WHERE_CLAUSE => NodeTypeInfo { name: "WHERE_CLAUSE", whitespace_like: false },
                        PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                        TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                        FN_TRAIT_SUGAR => NodeTypeInfo { name: "FN_TRAIT_SUGAR", whitespace_like: false },
                        ALIAS => NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                        PATH_TYPE => NodeTypeInfo { name: "PATH_TYPE", whitespace_like: false },
                        REFERENCE_TYPE => NodeTypeInfo { name: "REFERENCE_TYPE", whitespace_like: false },
                        PLACEHOLDER_TYPE => NodeTypeInfo { name: "PLACEHOLDER_TYPE", whitespace_like: false },
                        UNIT_TYPE => NodeTypeInfo { name: "UNIT_TYPE", whitespace_like: false },
                        PAREN_TYPE => NodeTypeInfo { name: "PAREN_TYPE", whitespace_like: false },
                        TUPLE_TYPE => NodeTypeInfo { name: "TUPLE_TYPE", whitespace_like: false },
                        NEVER_TYPE => NodeTypeInfo { name: "NEVER_TYPE", whitespace_like: false },
                        ARRAY_TYPE => NodeTypeInfo { name: "ARRAY_TYPE", whitespace_like: false },
                        FN_POINTER_TYPE => NodeTypeInfo { name: "FN_POINTER_TYPE", whitespace_like: false },
                        WILDCARD_PATTERN => NodeTypeInfo { name: "WILDCARD_PATTERN", whitespace_like: false },
                        PATH_PATTERN => NodeTypeInfo { name: "PATH_PATTERN", whitespace_like: false },
                        TUPE_STRUCT_PATTERN => NodeTypeInfo { name: "TUPE_STRUCT_PATTERN", whitespace_like: false },
                        STRUCT_PATTERN => NodeTypeInfo { name: "STRUCT_PATTERN", whitespace_like: false },
                        STRUCT_PATTERN_FIELD => NodeTypeInfo { name: "STRUCT_PATTERN_FIELD", whitespace_like: false },
                        BINDING_PATTERN => NodeTypeInfo { name: "BINDING_PATTERN", whitespace_like: false },
                        LITERAL_PATTERN => NodeTypeInfo { name: "LITERAL_PATTERN", whitespace_like: false },
                        UNIT_PATTERN => NodeTypeInfo { name: "UNIT_PATTERN", whitespace_like: false },
                        PAREN_PATTERN => NodeTypeInfo { name: "PAREN_PATTERN", whitespace_like: false },
                        TUPLE_PATTERN => NodeTypeInfo { name: "TUPLE_PATTERN", whitespace_like: false },
                        REFERENCE_PATTERN => NodeTypeInfo { name: "REFERENCE_PATTERN", whitespace_like: false },
                        EXPR => NodeTypeInfo { name: "EXPR", whitespace_like: false },
                        LITERAL => NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                        PATH_EXPR => NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                        STRUCT_LITERAL => NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                        STRUCT_LITERAL_FIELD => NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                        UNIT_EXPR => NodeTypeInfo { name: "UNIT_EXPR", whitespace_like: false },
                        PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                        TUPLE_EXPR => NodeTypeInfo { name: "TUPLE_EXPR", whitespace_like: false },
                        ARRAY_LITERAL => NodeTypeInfo { name: "ARRAY_LITERAL", whitespace_like: false },
                        LAMBDA_EXPR => NodeTypeInfo { name: "LAMBDA_EXPR", whitespace_like: false },
                        RETURN_EXPR => NodeTypeInfo { name: "RETURN_EXPR", whitespace_like: false },
                        BLOCK_EXPR => NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                        LET_STMT => NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                        TYPE_ASCRIPTION => NodeTypeInfo { name: "TYPE_ASCRIPTION", whitespace_like: false },
                        INITIALIZER => NodeTypeInfo { name: "INITIALIZER", whitespace_like: false },
                        EMPTY_STMT => NodeTypeInfo { name: "EMPTY_STMT", whitespace_like: false },
                        EXPR_STMT => NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                        IF_EXPR => NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                        WHILE_EXPR => NodeTypeInfo { name: "WHILE_EXPR", whitespace_like: false },
                        LOOP_EXPR => NodeTypeInfo { name: "LOOP_EXPR", whitespace_like: false },
                        FOR_EXPR => NodeTypeInfo { name: "FOR_EXPR", whitespace_like: false },
                        MATCH_EXPR => NodeTypeInfo { name: "MATCH_EXPR", whitespace_like: false },
                        MATCH_ARM => NodeTypeInfo { name: "MATCH_ARM", whitespace_like: false },
                        GUARD => NodeTypeInfo { name: "GUARD", whitespace_like: false },
                        BLOCK_MACRO_EXPR => NodeTypeInfo { name: "BLOCK_MACRO_EXPR", whitespace_like: false },
                        LINE_MACRO_EXPR => NodeTypeInfo { name: "LINE_MACRO_EXPR", whitespace_like: false },
                        METHOD_CALL_EXPR => NodeTypeInfo { name: "METHOD_CALL_EXPR", whitespace_like: false },
                        CALL_EXPR => NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                        VALUE_ARGUMENT => NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                        FIELD_EXPR => NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                        INDEX_EXPR => NodeTypeInfo { name: "INDEX_EXPR", whitespace_like: false },
                        TRY_EXPR => NodeTypeInfo { name: "TRY_EXPR", whitespace_like: false },
                        CAST_EXPR => NodeTypeInfo { name: "CAST_EXPR", whitespace_like: false },
                        REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                        DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                        NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                        NOT_EXPR => NodeTypeInfo { name: "NOT_EXPR", whitespace_like: false },
                        PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                        SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                        BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                        BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                        BIT_XOR => NodeTypeInfo { name: "BIT_XOR", whitespace_like: false },
                        BIT_OR => NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
                        COMPARISON => NodeTypeInfo { name: "COMPARISON", whitespace_like: false },
                        LOGICAL_AND => NodeTypeInfo { name: "LOGICAL_AND", whitespace_like: false },
                        LOGICAL_OR => NodeTypeInfo { name: "LOGICAL_OR", whitespace_like: false },
                        RANGE_EXPR => NodeTypeInfo { name: "RANGE_EXPR", whitespace_like: false },
                        ASSIGNMENT_EXPR => NodeTypeInfo { name: "ASSIGNMENT_EXPR", whitespace_like: false },
                        ATTRIBUTE => NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                        ATTR_VALUE => NodeTypeInfo { name: "ATTR_VALUE", whitespace_like: false },
                        BLOCK_MACRO => NodeTypeInfo { name: "BLOCK_MACRO", whitespace_like: false },
                        LINE_MACRO => NodeTypeInfo { name: "LINE_MACRO", whitespace_like: false },
                        TT => NodeTypeInfo { name: "TT", whitespace_like: false },
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

fn whitespace_binder(ty: NodeType, adjacent_tokens: Vec<(NodeType, &str)>, is_leading: bool) -> usize {
    if !is_leading {
        return 0;
    }
    match ty {
        STRUCT_DEF => {
            let mut has_comment = false;
            adjacent_tokens.iter().rev()
                .take_while(|&&(ty, text)| {
                    if ty == LINE_COMMENT {
                        has_comment = true;
                        true
                    } else {
                        ty == WHITESPACE && text.chars().filter(|&c| c == '\n').next().is_none()
                    }
                })
                .count()
        }
        _ => 0,
    }
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    // Who needs more then 25 hashes anyway? :)
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

fn parse_block_comment(s: &str) -> Option<usize> {
    let mut s = &s[2..];
    let mut len = 2;
    let mut level = 1;
    loop {
        if s.len() == 0 || level == 0 {
            break;
        } else if s.starts_with("/*") {
            s = &s[2..];
            len += 2;
            level += 1;
        } else if s.starts_with("*/") {
            s = &s[2..];
            len += 2;
            level -= 1;
        } else {
            let c = s.chars().next().unwrap();
            let l = c.len_utf8();
            s = &s[l..];
            len += l;
        }
    }
    Some(len)
}

