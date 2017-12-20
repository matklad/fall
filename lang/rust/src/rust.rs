use fall_parse::runtime as rt;
use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeTypeInfo, Metrics, TextEdit, TreeBuilder};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const LINE_COMMENT: rt::NodeType = rt::NodeType(101);
pub const BLOCK_COMMENT: rt::NodeType = rt::NodeType(102);
pub const UNION: rt::NodeType = rt::NodeType(103);
pub const AS: rt::NodeType = rt::NodeType(104);
pub const CRATE: rt::NodeType = rt::NodeType(105);
pub const EXTERN: rt::NodeType = rt::NodeType(106);
pub const FN: rt::NodeType = rt::NodeType(107);
pub const LET: rt::NodeType = rt::NodeType(108);
pub const PUB: rt::NodeType = rt::NodeType(109);
pub const STRUCT: rt::NodeType = rt::NodeType(110);
pub const USE: rt::NodeType = rt::NodeType(111);
pub const MOD: rt::NodeType = rt::NodeType(112);
pub const IF: rt::NodeType = rt::NodeType(113);
pub const ELSE: rt::NodeType = rt::NodeType(114);
pub const ENUM: rt::NodeType = rt::NodeType(115);
pub const IMPL: rt::NodeType = rt::NodeType(116);
pub const SELF: rt::NodeType = rt::NodeType(117);
pub const SUPER: rt::NodeType = rt::NodeType(118);
pub const TYPE: rt::NodeType = rt::NodeType(119);
pub const CONST: rt::NodeType = rt::NodeType(120);
pub const STATIC: rt::NodeType = rt::NodeType(121);
pub const FOR: rt::NodeType = rt::NodeType(122);
pub const LOOP: rt::NodeType = rt::NodeType(123);
pub const WHILE: rt::NodeType = rt::NodeType(124);
pub const MOVE: rt::NodeType = rt::NodeType(125);
pub const MUT: rt::NodeType = rt::NodeType(126);
pub const REF: rt::NodeType = rt::NodeType(127);
pub const TRAIT: rt::NodeType = rt::NodeType(128);
pub const MATCH: rt::NodeType = rt::NodeType(129);
pub const RETURN: rt::NodeType = rt::NodeType(130);
pub const IN: rt::NodeType = rt::NodeType(131);
pub const UNSAFE: rt::NodeType = rt::NodeType(132);
pub const WHERE: rt::NodeType = rt::NodeType(133);
pub const L_PAREN: rt::NodeType = rt::NodeType(134);
pub const R_PAREN: rt::NodeType = rt::NodeType(135);
pub const L_CURLY: rt::NodeType = rt::NodeType(136);
pub const R_CURLY: rt::NodeType = rt::NodeType(137);
pub const L_ANGLE: rt::NodeType = rt::NodeType(138);
pub const R_ANGLE: rt::NodeType = rt::NodeType(139);
pub const L_BRACK: rt::NodeType = rt::NodeType(140);
pub const R_BRACK: rt::NodeType = rt::NodeType(141);
pub const SHL: rt::NodeType = rt::NodeType(142);
pub const SHL_EQ: rt::NodeType = rt::NodeType(143);
pub const SHR: rt::NodeType = rt::NodeType(144);
pub const SHR_EQ: rt::NodeType = rt::NodeType(145);
pub const AND: rt::NodeType = rt::NodeType(146);
pub const OR: rt::NodeType = rt::NodeType(147);
pub const THIN_ARROW: rt::NodeType = rt::NodeType(148);
pub const FAT_ARROW: rt::NodeType = rt::NodeType(149);
pub const EQ: rt::NodeType = rt::NodeType(150);
pub const EQEQ: rt::NodeType = rt::NodeType(151);
pub const BANGEQ: rt::NodeType = rt::NodeType(152);
pub const GTET: rt::NodeType = rt::NodeType(153);
pub const LTEQ: rt::NodeType = rt::NodeType(154);
pub const SEMI: rt::NodeType = rt::NodeType(155);
pub const COLON: rt::NodeType = rt::NodeType(156);
pub const COLONCOLON: rt::NodeType = rt::NodeType(157);
pub const COMMA: rt::NodeType = rt::NodeType(158);
pub const DOT: rt::NodeType = rt::NodeType(159);
pub const DOTDOT: rt::NodeType = rt::NodeType(160);
pub const DOTDOTDOT: rt::NodeType = rt::NodeType(161);
pub const HASH: rt::NodeType = rt::NodeType(162);
pub const DOLLAR: rt::NodeType = rt::NodeType(163);
pub const STAR: rt::NodeType = rt::NodeType(164);
pub const STAR_EQ: rt::NodeType = rt::NodeType(165);
pub const SLASH: rt::NodeType = rt::NodeType(166);
pub const SLASH_EQ: rt::NodeType = rt::NodeType(167);
pub const PERCENT: rt::NodeType = rt::NodeType(168);
pub const PERCENT_EQ: rt::NodeType = rt::NodeType(169);
pub const PLUS: rt::NodeType = rt::NodeType(170);
pub const PLUS_EQ: rt::NodeType = rt::NodeType(171);
pub const MINUS: rt::NodeType = rt::NodeType(172);
pub const MINUS_EQ: rt::NodeType = rt::NodeType(173);
pub const AMPERSAND: rt::NodeType = rt::NodeType(174);
pub const AMPERSAND_EQ: rt::NodeType = rt::NodeType(175);
pub const PIPE: rt::NodeType = rt::NodeType(176);
pub const PIPE_EQ: rt::NodeType = rt::NodeType(177);
pub const UNDERSCORE: rt::NodeType = rt::NodeType(178);
pub const BANG: rt::NodeType = rt::NodeType(179);
pub const QUESTION: rt::NodeType = rt::NodeType(180);
pub const CARET: rt::NodeType = rt::NodeType(181);
pub const CARET_EQ: rt::NodeType = rt::NodeType(182);
pub const CHAR: rt::NodeType = rt::NodeType(183);
pub const LIFETIME: rt::NodeType = rt::NodeType(184);
pub const BOOL: rt::NodeType = rt::NodeType(185);
pub const NUMBER: rt::NodeType = rt::NodeType(186);
pub const STRING: rt::NodeType = rt::NodeType(187);
pub const RAW_STRING: rt::NodeType = rt::NodeType(188);
pub const IDENT: rt::NodeType = rt::NodeType(189);
pub const FILE: rt::NodeType = rt::NodeType(190);
pub const USE_DECL: rt::NodeType = rt::NodeType(191);
pub const USE_SPEC: rt::NodeType = rt::NodeType(192);
pub const USE_SPEC_ENTRY: rt::NodeType = rt::NodeType(193);
pub const EXTERN_CRATE_DECL: rt::NodeType = rt::NodeType(194);
pub const FN_DEF: rt::NodeType = rt::NodeType(195);
pub const LINKAGE: rt::NodeType = rt::NodeType(196);
pub const VALUE_PARAM: rt::NodeType = rt::NodeType(197);
pub const LAMBDA_VALUE_PARAM: rt::NodeType = rt::NodeType(198);
pub const SELF_PARAMETER: rt::NodeType = rt::NodeType(199);
pub const STRUCT_DEF: rt::NodeType = rt::NodeType(200);
pub const STRUCT_FIELD: rt::NodeType = rt::NodeType(201);
pub const TUPLE_FIELD: rt::NodeType = rt::NodeType(202);
pub const ENUM_DEF: rt::NodeType = rt::NodeType(203);
pub const ENUM_VARIANT: rt::NodeType = rt::NodeType(204);
pub const MOD_DEF: rt::NodeType = rt::NodeType(205);
pub const IMPL_DEF: rt::NodeType = rt::NodeType(206);
pub const TRAIT_DEF: rt::NodeType = rt::NodeType(207);
pub const MEMBERS: rt::NodeType = rt::NodeType(208);
pub const TYPE_DEF: rt::NodeType = rt::NodeType(209);
pub const CONST_DEF: rt::NodeType = rt::NodeType(210);
pub const MACRO_ITEM: rt::NodeType = rt::NodeType(211);
pub const EXTERN_BLOCK: rt::NodeType = rt::NodeType(212);
pub const TYPE_PARAMETERS: rt::NodeType = rt::NodeType(213);
pub const TYPE_PARAMETER: rt::NodeType = rt::NodeType(214);
pub const TYPE_BOUND: rt::NodeType = rt::NodeType(215);
pub const LIFETIME_PARAMETER: rt::NodeType = rt::NodeType(216);
pub const VISIBILITY: rt::NodeType = rt::NodeType(217);
pub const WHERE_CLAUSE: rt::NodeType = rt::NodeType(218);
pub const PATH: rt::NodeType = rt::NodeType(219);
pub const TRAIT_PROJECTION_PATH: rt::NodeType = rt::NodeType(220);
pub const PATH_SEGMENT: rt::NodeType = rt::NodeType(221);
pub const TYPE_ARGUMENTS: rt::NodeType = rt::NodeType(222);
pub const FN_TRAIT_SUGAR: rt::NodeType = rt::NodeType(223);
pub const ALIAS: rt::NodeType = rt::NodeType(224);
pub const PATH_TYPE: rt::NodeType = rt::NodeType(225);
pub const REFERENCE_TYPE: rt::NodeType = rt::NodeType(226);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(227);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(228);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(229);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(230);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(231);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(232);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(233);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(234);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(235);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(236);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(237);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(238);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(239);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(240);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(241);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(242);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(243);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(244);
pub const EXPR: rt::NodeType = rt::NodeType(245);
pub const LITERAL: rt::NodeType = rt::NodeType(246);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(247);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(248);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(249);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(250);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(251);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(252);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(253);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(254);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(255);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(256);
pub const LET_STMT: rt::NodeType = rt::NodeType(257);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(258);
pub const INITIALIZER: rt::NodeType = rt::NodeType(259);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(260);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(261);
pub const IF_EXPR: rt::NodeType = rt::NodeType(262);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(263);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(264);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(265);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(266);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(267);
pub const GUARD: rt::NodeType = rt::NodeType(268);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(269);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(270);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(271);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(272);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(273);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(274);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(275);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(276);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(277);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(278);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(279);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(280);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(281);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(282);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(283);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(284);
pub const BIT_AND: rt::NodeType = rt::NodeType(285);
pub const BIT_XOR: rt::NodeType = rt::NodeType(286);
pub const BIT_OR: rt::NodeType = rt::NodeType(287);
pub const COMPARISON: rt::NodeType = rt::NodeType(288);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(289);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(290);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(291);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(292);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(293);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(294);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(295);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(296);
pub const TT: rt::NodeType = rt::NodeType(297);


pub fn language() -> &'static rt::Language {
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
            ::fall_parse::LexRule::new(STRING, "\"(\\\\\"|[^\"])*\"", None),
            ::fall_parse::LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(IDENT, "(\\p{XID_Start}|_)\\p{XID_Continue}*", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":91,"body":152,"replaceable":false}},{"Or":[155]},{"Or":[157,159,161,163,165,167,169,170]},{"Cached":187},{"Pub":{"ty":92,"body":203,"replaceable":false}},{"Pub":{"ty":93,"body":209,"replaceable":false}},{"Pub":{"ty":94,"body":215,"replaceable":false}},{"Pub":{"ty":95,"body":222,"replaceable":false}},{"Pub":{"ty":96,"body":234,"replaceable":false}},{"Or":[236]},{"Pub":{"ty":97,"body":241,"replaceable":false}},{"Or":[247]},{"Pub":{"ty":98,"body":250,"replaceable":false}},{"Pub":{"ty":99,"body":256,"replaceable":false}},{"Pub":{"ty":100,"body":269,"replaceable":false}},{"Pub":{"ty":101,"body":288,"replaceable":false}},{"Pub":{"ty":102,"body":293,"replaceable":false}},{"Pub":{"ty":103,"body":296,"replaceable":false}},{"Pub":{"ty":104,"body":303,"replaceable":false}},{"Pub":{"ty":105,"body":316,"replaceable":false}},{"Pub":{"ty":106,"body":325,"replaceable":false}},{"Pub":{"ty":107,"body":336,"replaceable":false}},{"Pub":{"ty":108,"body":344,"replaceable":false}},{"Pub":{"ty":109,"body":356,"replaceable":false}},{"Or":[357,358,359]},{"Or":[361,363,365,367,369,371,373,378]},{"Pub":{"ty":110,"body":388,"replaceable":false}},{"Pub":{"ty":111,"body":402,"replaceable":false}},{"Pub":{"ty":112,"body":406,"replaceable":false}},{"Pub":{"ty":113,"body":410,"replaceable":false}},{"Or":[411,412]},{"Pub":{"ty":114,"body":419,"replaceable":false}},{"Pub":{"ty":115,"body":423,"replaceable":false}},{"Or":[443]},{"Pub":{"ty":116,"body":447,"replaceable":false}},{"Pub":{"ty":117,"body":467,"replaceable":false}},{"Pub":{"ty":118,"body":477,"replaceable":false}},{"Pub":{"ty":119,"body":492,"replaceable":false}},{"Or":[493]},{"Or":[495]},{"Or":[497]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":500,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":504,"replaceable":false}},{"Pub":{"ty":120,"body":509,"replaceable":false}},{"Pub":{"ty":121,"body":516,"replaceable":false}},{"Pub":{"ty":122,"body":530,"replaceable":false}},{"Pub":{"ty":123,"body":544,"replaceable":false}},{"Pub":{"ty":124,"body":549,"replaceable":false}},{"Pub":{"ty":125,"body":553,"replaceable":false}},{"Or":[554,555,556,557,558,559,560,561]},{"Pub":{"ty":126,"body":563,"replaceable":false}},{"Pub":{"ty":127,"body":565,"replaceable":false}},{"Pub":{"ty":128,"body":568,"replaceable":false}},{"Pub":{"ty":129,"body":572,"replaceable":false}},{"Pub":{"ty":130,"body":578,"replaceable":true}},{"PubReplace":{"ty":131,"body":582}},{"Pub":{"ty":132,"body":585,"replaceable":false}},{"Pub":{"ty":133,"body":594,"replaceable":false}},{"Pub":{"ty":134,"body":599,"replaceable":false}},{"Pub":{"ty":98,"body":605,"replaceable":false}},{"Or":[606,607,608,609,610,611,612]},{"Pub":{"ty":135,"body":615,"replaceable":false}},{"Pub":{"ty":136,"body":621,"replaceable":true}},{"PubReplace":{"ty":137,"body":633}},{"PubReplace":{"ty":138,"body":645}},{"Pub":{"ty":139,"body":652,"replaceable":false}},{"Pub":{"ty":140,"body":659,"replaceable":false}},{"Pub":{"ty":141,"body":661,"replaceable":false}},{"Pub":{"ty":142,"body":665,"replaceable":false}},{"Pub":{"ty":143,"body":671,"replaceable":true}},{"PubReplace":{"ty":144,"body":675}},{"Pub":{"ty":145,"body":680,"replaceable":false}},{"Pratt":{"atoms":[76,77,80,81,83,84,86,87,94,95,98,99,101,105,106,134],"prefixes":[{"ty":179,"op":116,"priority":999},{"ty":180,"op":709,"priority":999},{"ty":181,"op":710,"priority":999},{"ty":182,"op":711,"priority":999},{"ty":192,"op":136,"priority":2}],"infixes":[{"ty":172,"op":686,"priority":999,"has_rhs":false},{"ty":173,"op":695,"priority":999,"has_rhs":false},{"ty":175,"op":703,"priority":999,"has_rhs":false},{"ty":176,"op":704,"priority":999,"has_rhs":false},{"ty":177,"op":705,"priority":999,"has_rhs":false},{"ty":178,"op":708,"priority":999,"has_rhs":false},{"ty":183,"op":719,"priority":11,"has_rhs":true},{"ty":184,"op":725,"priority":10,"has_rhs":true},{"ty":185,"op":731,"priority":9,"has_rhs":true},{"ty":186,"op":737,"priority":8,"has_rhs":true},{"ty":187,"op":739,"priority":7,"has_rhs":true},{"ty":188,"op":745,"priority":6,"has_rhs":true},{"ty":189,"op":746,"priority":5,"has_rhs":true},{"ty":190,"op":748,"priority":4,"has_rhs":true},{"ty":191,"op":750,"priority":3,"has_rhs":true},{"ty":192,"op":751,"priority":2,"has_rhs":true},{"ty":192,"op":135,"priority":2,"has_rhs":false},{"ty":193,"op":775,"priority":1,"has_rhs":true}]}},{"Or":[776,778,780,782,784,786,788,790,792,794,796,798,800,802,804,806,808,810,812,814,816,818]},{"Or":[820]},{"Or":[824]},{"Pub":{"ty":147,"body":835,"replaceable":false}},{"Pub":{"ty":148,"body":843,"replaceable":true}},{"PubReplace":{"ty":149,"body":856}},{"Pub":{"ty":150,"body":863,"replaceable":false}},{"Pub":{"ty":151,"body":867,"replaceable":false}},{"Pub":{"ty":152,"body":874,"replaceable":true}},{"PubReplace":{"ty":153,"body":879}},{"Pub":{"ty":154,"body":884,"replaceable":false}},{"Pub":{"ty":155,"body":896,"replaceable":false}},{"Or":[904]},{"Pub":{"ty":156,"body":908,"replaceable":false}},{"Pub":{"ty":157,"body":920,"replaceable":false}},{"Or":[921,922,923,924]},{"Pub":{"ty":158,"body":930,"replaceable":false}},{"Pub":{"ty":159,"body":933,"replaceable":false}},{"Pub":{"ty":160,"body":936,"replaceable":false}},{"Pub":{"ty":161,"body":939,"replaceable":false}},{"Pub":{"ty":162,"body":950,"replaceable":false}},{"Pub":{"ty":163,"body":960,"replaceable":false}},{"Pub":{"ty":164,"body":964,"replaceable":false}},{"Or":[970]},{"Or":[972]},{"Pub":{"ty":165,"body":976,"replaceable":false}},{"Pub":{"ty":166,"body":981,"replaceable":false}},{"Or":[984]},{"Pub":{"ty":167,"body":989,"replaceable":false}},{"Pub":{"ty":168,"body":999,"replaceable":false}},{"Or":[1005]},{"Pub":{"ty":169,"body":1008,"replaceable":false}},{"Pub":{"ty":170,"body":1010,"replaceable":false}},{"Pub":{"ty":171,"body":1012,"replaceable":false}},{"Pub":{"ty":172,"body":1020,"replaceable":false}},{"Pub":{"ty":173,"body":1031,"replaceable":false}},{"Or":[1035]},{"Pub":{"ty":174,"body":1037,"replaceable":false}},{"Pub":{"ty":175,"body":1047,"replaceable":false}},{"Pub":{"ty":176,"body":1050,"replaceable":false}},{"Pub":{"ty":177,"body":1053,"replaceable":false}},{"Pub":{"ty":178,"body":1058,"replaceable":false}},{"Pub":{"ty":179,"body":1060,"replaceable":false}},{"Or":[1066]},{"Pub":{"ty":180,"body":1069,"replaceable":false}},{"Pub":{"ty":181,"body":1072,"replaceable":false}},{"Pub":{"ty":182,"body":1075,"replaceable":false}},{"Pub":{"ty":183,"body":1085,"replaceable":false}},{"Pub":{"ty":184,"body":1093,"replaceable":false}},{"Pub":{"ty":185,"body":1101,"replaceable":false}},{"Or":[1105,1109]},{"Pub":{"ty":186,"body":1117,"replaceable":false}},{"Pub":{"ty":187,"body":1121,"replaceable":false}},{"Pub":{"ty":188,"body":1129,"replaceable":false}},{"Pub":{"ty":189,"body":1132,"replaceable":false}},{"Or":[1134,1136,1138,1140,1142,1144]},{"Pub":{"ty":190,"body":1148,"replaceable":false}},{"Pub":{"ty":191,"body":1152,"replaceable":false}},{"Pub":{"ty":192,"body":1155,"replaceable":false}},{"Pub":{"ty":192,"body":1157,"replaceable":false}},{"Pub":{"ty":192,"body":1159,"replaceable":false}},{"Pub":{"ty":192,"body":1163,"replaceable":false}},{"Or":[1165,1167]},{"Or":[1175]},{"Pub":{"ty":193,"body":1201,"replaceable":false}},{"Pub":{"ty":194,"body":1206,"replaceable":false}},{"Or":[1208]},{"Pub":{"ty":195,"body":1218,"replaceable":false}},{"Pub":{"ty":196,"body":1226,"replaceable":false}},{"Pub":{"ty":197,"body":1241,"replaceable":false}},{"Pub":{"ty":198,"body":1270,"replaceable":false}},{"Or":[1280]},{"Or":[1285]},{"Or":[1290]},{"Or":[1295]},{"Or":[1300]},{"Or":[1308]},{"Or":[1323]},{"And":[[1],null]},{"Or":[151]},{"WithSkip":[2,3]},{"Rep":153},{"And":[[154],null]},{"Token":11},{"And":[[156],null]},{"ContextualToken":[4,"union"]},{"And":[[158],null]},{"Token":16},{"And":[[160],null]},{"Token":12},{"And":[[162],null]},{"Token":13},{"And":[[164],null]},{"Token":17},{"And":[[166],null]},{"Token":29},{"And":[[168],null]},{"And":[[25],null]},{"Opt":36},{"And":[[139,171],null]},{"Or":[172]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[174,175,176,177,178,179,180,181,182]},{"Inject":[173,183]},{"And":[[184],null]},{"And":[[28],null]},{"Or":[185,186]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[190,5],null]},{"Or":[191]},{"Opt":192},{"And":[[193],null]},{"Or":[189,194]},{"And":[[38,195],null]},{"Token":58},{"Opt":197},{"And":[[198,5],null]},{"Or":[196,199]},{"Token":56},{"And":[[188,200,201],1]},{"Or":[202]},{"Token":65},{"And":[[204],null]},{"Call":[144,[[2,6]]]},{"Call":[145,[[3,206]]]},{"And":[[207],null]},{"Or":[205,208]},{"Token":18},{"And":[[210],null]},{"Token":90},{"Opt":48},{"And":[[212,213],1]},{"Or":[211,214]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[216,217,218,219,220],2]},{"Or":[221]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[87],null]},{"Token":56},{"And":[[230],null]},{"Or":[229,231]},{"And":[[223,224,225,226,11,227,228,232],2]},{"Or":[233]},{"Token":49},{"And":[[235,49],null]},{"Token":7},{"Token":88},{"Opt":238},{"And":[[237,239],null]},{"Or":[240]},{"Opt":14},{"Call":[144,[[2,12]]]},{"And":[[242,243],null]},{"Or":[244]},{"Call":[146,[[4,245]]]},{"And":[[246],null]},{"Token":57},{"And":[[60,248,49],1]},{"Or":[249]},{"Token":57},{"And":[[251,49],null]},{"Or":[252]},{"Opt":253},{"And":[[60,254],null]},{"Or":[255]},{"Opt":116},{"Token":18},{"Token":57},{"And":[[259,49],null]},{"Or":[260]},{"Opt":261},{"Token":59},{"And":[[263],null]},"Eof",{"And":[[265],null]},{"Or":[264,266]},{"And":[[257,258,262,267],2]},{"Or":[268]},{"Token":11},{"And":[[270],null]},{"ContextualToken":[4,"union"]},{"And":[[272],null]},{"Or":[271,273]},{"Token":90},{"Opt":31},{"Call":[144,[[2,16]]]},{"Call":[145,[[3,277]]]},{"And":[[278],null]},{"Token":56},{"And":[[280],null]},{"Call":[144,[[2,17]]]},{"Call":[146,[[4,282]]]},{"Token":56},{"And":[[283,284],null]},{"Or":[279,281,285]},{"And":[[274,275,276,286],1]},{"Or":[287]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[289,290,291,49],2]},{"Or":[292]},{"Opt":36},{"And":[[294,49],null]},{"Or":[295]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[144,[[2,19]]]},{"Call":[145,[[3,300]]]},{"And":[[297,298,299,301],1]},{"Or":[302]},{"Token":90},{"Token":51},{"And":[[305,72],null]},{"Call":[144,[[2,17]]]},{"Call":[146,[[4,307]]]},{"And":[[308],null]},{"Call":[144,[[2,16]]]},{"Call":[145,[[3,310]]]},{"And":[[311],null]},{"Or":[306,309,312]},{"Opt":313},{"And":[[304,314],1]},{"Or":[315]},{"Token":13},{"Token":90},{"Token":56},{"And":[[319],null]},{"Call":[145,[[3,1]]]},{"And":[[321],null]},{"Or":[320,322]},{"And":[[317,318,323],1]},{"Or":[324]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[328,49],null]},{"Or":[329]},{"Opt":330},{"And":[[49,331],null]},{"Or":[332]},{"Opt":37},{"And":[[326,327,333,334,23],1]},{"Or":[335]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[139,337,338,339,340,341,342,23],3]},{"Or":[343]},{"Opt":36},{"And":[[139,345],null]},{"Or":[346]},{"Inject":[347,24]},{"And":[[348],null]},{"And":[[28],null]},{"Or":[349,350]},{"WithSkip":[25,351]},{"Rep":352},{"Call":[145,[[3,353]]]},{"And":[[354],null]},{"Or":[355]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[360],null]},{"Token":8},{"And":[[362],null]},{"Token":20},{"And":[[364],null]},{"Token":21},{"And":[[366],null]},{"Token":22},{"And":[[368],null]},{"Token":63},{"And":[[370],null]},{"Token":7},{"And":[[372],null]},{"Token":90},{"Token":80},{"And":[[374,375],null]},{"Or":[376]},{"And":[[377],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[382,49],null]},{"Or":[383]},{"Opt":384},{"Token":56},{"And":[[379,380,381,385,386],1]},{"Or":[387]},{"Token":21},{"And":[[389],null]},{"Token":22},{"And":[[391],null]},{"Or":[390,392]},{"Token":90},{"Token":57},{"Token":51},{"And":[[396,72],null]},{"Or":[397]},{"Opt":398},{"Token":56},{"And":[[393,394,395,49,399,400],1]},{"Or":[401]},{"And":[[141],null]},{"Token":56},{"And":[[142,404],null]},{"Or":[403,405]},{"Rep":30},{"Call":[145,[[3,407]]]},{"And":[[10,408],null]},{"Or":[409]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[144,[[2,35]]]},{"Call":[144,[[2,32]]]},{"And":[[413,414],null]},{"Or":[415]},{"Call":[147,[[5,416]]]},{"And":[[417],null]},{"Or":[418]},{"Token":90},{"Opt":33},{"And":[[420,421],1]},{"Or":[422]},{"Token":57},{"Token":71},{"And":[[425],null]},"Eof",{"And":[[427],null]},{"Token":59},{"And":[[429],null]},{"Token":37},{"And":[[431],null]},{"Token":34},{"And":[[433],null]},{"Or":[430,432,434]},{"Not":435},{"Not":436},{"And":[[437],null]},{"Or":[426,428,438]},{"And":[[34,439],1]},{"Or":[440]},{"Rep":441},{"And":[[424,442],null]},{"Token":85},{"And":[[444],null]},{"And":[[49],null]},{"Or":[445,446]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[451],null]},"Eof",{"And":[[453],null]},{"Token":59},{"Not":455},{"Not":456},{"And":[[457],null]},{"Or":[452,454,458]},{"And":[[450,459],1]},{"Or":[460]},{"Rep":461},{"And":[[449,462],null]},{"Or":[463]},{"Opt":464},{"And":[[448,465],1]},{"Or":[466]},{"Token":10},{"Token":6},{"And":[[469],null]},{"Token":19},{"And":[[471],null]},{"Or":[470,472]},{"Call":[146,[[4,473]]]},{"Opt":474},{"And":[[468,475],null]},{"Or":[476]},{"Token":34},{"Token":59},{"And":[[479],null]},"Eof",{"And":[[481],null]},{"Token":37},{"Not":483},{"Not":484},{"And":[[485],null]},{"Or":[480,482,486]},{"And":[[49,33,487],null]},{"Or":[488]},{"Rep":489},{"And":[[478,490],1]},{"Or":[491]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[494],null]},{"Enter":[1,41]},{"And":[[496],null]},{"Token":58},{"And":[[498,45],null]},{"Or":[499]},{"Token":58},{"Opt":501},{"And":[[502,45],null]},{"Or":[503]},{"Token":58},{"And":[[505,45],null]},{"Or":[506]},{"And":[[41,507],null]},{"Or":[508]},{"Token":5},{"And":[[49,510,49],null]},{"Or":[511]},{"Call":[147,[[5,512]]]},{"Token":58},{"And":[[513,514,45],null]},{"Or":[515]},{"Token":90},{"And":[[517],null]},{"Token":18},{"And":[[519],null]},{"Token":19},{"And":[[521],null]},{"Or":[518,520,522]},{"And":[[46],null]},{"IsIn":3},{"And":[[525,47],null]},{"Or":[524,526]},{"Opt":527},{"And":[[523,528],null]},{"Or":[529]},{"IsIn":3},{"And":[[531],null]},{"IsIn":1},{"Token":58},{"And":[[533,534],null]},{"Or":[532,535]},{"Token":85},{"Call":[144,[[2,537]]]},{"Call":[144,[[2,49]]]},{"And":[[538,539],null]},{"Or":[540]},{"Call":[147,[[5,541]]]},{"And":[[536,542],null]},{"Or":[543]},{"Call":[144,[[2,49]]]},{"Call":[146,[[4,545]]]},{"Opt":9},{"And":[[546,547],null]},{"Or":[548]},{"Token":5},{"Token":90},{"And":[[550,551],null]},{"Or":[552]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[58],null]},{"And":[[39],null]},{"Or":[562]},{"And":[[116,49],null]},{"Or":[564]},{"Token":79},{"And":[[566],null]},{"Or":[567]},{"Token":35},{"Token":36},{"And":[[569,570],null]},{"Or":[571]},{"Opt":55},{"And":[[49,573],null]},{"Or":[574]},{"Call":[146,[[4,575]]]},{"And":[[576],null]},{"Or":[577]},{"Token":59},{"Call":[144,[[2,49]]]},{"And":[[579,580],null]},{"Or":[581]},{"Token":80},{"And":[[583],null]},{"Or":[584]},{"Token":56},{"And":[[586,72],null]},{"Or":[587]},{"Opt":588},{"And":[[49,589],null]},{"Or":[590]},{"Call":[148,[[6,591]]]},{"And":[[592],null]},{"Or":[593]},{"Token":8},{"Call":[144,[[2,59]]]},{"Call":[146,[[4,596]]]},{"And":[[595,597,9],1]},{"Or":[598]},{"Token":57},{"And":[[60,600],null]},{"Or":[601]},{"Opt":602},{"And":[[603,49],null]},{"Or":[604]},{"And":[[61],null]},{"And":[[62],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[71],null]},{"Token":79},{"And":[[613],null]},{"Or":[614]},{"And":[[63],null]},{"And":[[64],null]},{"Or":[616,617]},{"Opt":618},{"And":[[40,619],null]},{"Or":[620]},{"Call":[144,[[2,60]]]},{"Token":61},{"Token":59},{"Opt":624},{"And":[[623,625],null]},{"Or":[626]},{"Opt":627},{"And":[[622,628],null]},{"Or":[629]},{"Call":[146,[[4,630]]]},{"And":[[631],null]},{"Or":[632]},{"Call":[144,[[2,65]]]},{"Token":61},{"Token":59},{"Opt":636},{"And":[[635,637],null]},{"Or":[638]},{"Opt":639},{"And":[[634,640],null]},{"Or":[641]},{"Call":[145,[[3,642]]]},{"And":[[643],null]},{"Or":[644]},{"Token":57},{"Not":646},{"And":[[66,647],null]},{"Token":90},{"Token":57},{"And":[[649,650,60],2]},{"Or":[648,651]},{"Token":28},{"Opt":653},{"Token":27},{"Opt":655},{"Token":90},{"And":[[654,656,657],null]},{"Or":[658]},{"And":[[76],null]},{"Or":[660]},{"Token":35},{"Token":36},{"And":[[662,663],null]},{"Or":[664]},{"Opt":70},{"And":[[60,666],null]},{"Or":[667]},{"Call":[146,[[4,668]]]},{"And":[[669],null]},{"Or":[670]},{"Token":59},{"Call":[144,[[2,60]]]},{"And":[[672,673],null]},{"Or":[674]},{"Token":75},{"Token":27},{"Opt":677},{"And":[[676,678,60],null]},{"Or":[679]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":683},{"And":[[681,682,684,109],null]},{"Or":[685]},{"IsIn":2},{"Not":74},{"And":[[687,688],null]},{"IsIn":2},{"Not":690},{"And":[[691],null]},{"Or":[689,692]},{"And":[[693,109],null]},{"Or":[694]},{"Token":60},{"Token":90},{"And":[[697],null]},{"Token":87},{"And":[[699],null]},{"Or":[698,700]},{"And":[[696,701],null]},{"Or":[702]},{"Call":[148,[[6,72]]]},{"Token":81},{"Token":5},{"And":[[706,49],null]},{"Or":[707]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[712],null]},{"Token":67},{"And":[[714],null]},{"Token":69},{"And":[[716],null]},{"Or":[713,715,717]},{"Call":[123,[[1,718]]]},{"Token":71},{"And":[[720],null]},{"Token":73},{"And":[[722],null]},{"Or":[721,723]},{"Call":[123,[[1,724]]]},{"ContextualToken":[43,"<<"]},{"And":[[726],null]},{"ContextualToken":[45,">>"]},{"And":[[728],null]},{"Or":[727,729]},{"Call":[123,[[1,730]]]},{"Token":75},{"Token":75},{"Not":733},{"And":[[732,734],null]},{"Or":[735]},{"Call":[123,[[1,736]]]},{"Token":82},{"Call":[123,[[1,738]]]},{"Token":77},{"Token":77},{"Not":741},{"And":[[740,742],null]},{"Or":[743]},{"Call":[123,[[1,744]]]},{"Call":[123,[[1,128]]]},{"ContextualToken":[47,"&&"]},{"Call":[123,[[1,747]]]},{"ContextualToken":[48,"||"]},{"Call":[123,[[1,749]]]},{"Call":[123,[[1,136]]]},{"Token":51},{"And":[[752],null]},{"Token":72},{"And":[[754],null]},{"Token":74},{"And":[[756],null]},{"Token":66},{"And":[[758],null]},{"Token":68},{"And":[[760],null]},{"Token":70},{"And":[[762],null]},{"Token":76},{"And":[[764],null]},{"Token":78},{"And":[[766],null]},{"Token":83},{"And":[[768],null]},{"ContextualToken":[46,">>="]},{"And":[[770],null]},{"ContextualToken":[44,"<<="]},{"And":[[772],null]},{"Or":[753,755,757,759,761,763,765,767,769,771,773]},{"Call":[123,[[1,774]]]},{"And":[[76],null]},{"Token":90},{"And":[[777],null]},{"Token":18},{"And":[[779],null]},{"Token":19},{"And":[[781],null]},{"Token":39},{"And":[[783],null]},{"Token":58},{"And":[[785],null]},{"Token":35},{"And":[[787],null]},{"Token":41},{"And":[[789],null]},{"Token":77},{"And":[[791],null]},{"Token":31},{"And":[[793],null]},{"Token":37},{"And":[[795],null]},{"Token":14},{"And":[[797],null]},{"Token":25},{"And":[[799],null]},{"Token":24},{"And":[[801],null]},{"Token":23},{"And":[[803],null]},{"Token":30},{"And":[[805],null]},{"Token":75},{"And":[[807],null]},{"Token":65},{"And":[[809],null]},{"Token":73},{"And":[[811],null]},{"Token":80},{"And":[[813],null]},{"Token":61},{"And":[[815],null]},{"Token":62},{"And":[[817],null]},{"PrevIs":[157,163,164,165,166,167,170]},{"And":[[819],null]},{"Var":0},{"Exit":[2,821]},{"Exit":[0,822]},{"And":[[823],null]},{"Token":87},{"And":[[825],null]},{"Token":88},{"And":[[827],null]},{"Token":89},{"And":[[829],null]},{"Token":84},{"And":[[831],null]},{"Token":86},{"And":[[833],null]},{"Or":[826,828,830,832,834]},{"Token":90},{"Token":80},{"And":[[836,837],null]},{"Or":[838]},{"Not":839},{"Opt":78},{"And":[[840,40,841],null]},{"Or":[842]},{"IsIn":0},{"Not":844},{"Call":[144,[[2,79]]]},{"Token":61},{"Call":[75,[[0,72]]]},{"And":[[847,848],null]},{"Or":[849]},{"Opt":850},{"And":[[846,851],null]},{"Or":[852]},{"Call":[145,[[3,853]]]},{"And":[[845,854],null]},{"Or":[855]},{"Token":90},{"Token":57},{"And":[[858,72],null]},{"Or":[859]},{"Opt":860},{"And":[[857,861],1]},{"Or":[862]},{"Token":35},{"Token":36},{"And":[[864,865],null]},{"Or":[866]},{"Call":[75,[[0,72]]]},{"Opt":82},{"And":[[868,869],null]},{"Or":[870]},{"Call":[146,[[4,871]]]},{"And":[[872],null]},{"Or":[873]},{"Token":59},{"Call":[75,[[0,72]]]},{"Call":[144,[[2,876]]]},{"And":[[875,877],null]},{"Or":[878]},{"Call":[144,[[2,72]]]},{"Call":[75,[[0,880]]]},{"Call":[148,[[6,881]]]},{"And":[[882],null]},{"Or":[883]},{"Token":26},{"Opt":885},{"Token":77},{"Rep":85},{"Token":77},{"Token":49},{"And":[[890,49,87],null]},{"Call":[75,[[0,72]]]},{"And":[[892],null]},{"Or":[891,893]},{"And":[[886,887,888,889,894],null]},{"Or":[895]},{"Token":59},{"And":[[897],null]},{"Token":77},{"Not":899},{"Not":900},{"And":[[901],null]},{"Or":[898,902]},{"And":[[13,903],1]},{"Token":31},{"Opt":72},{"And":[[905,906],null]},{"Or":[907]},{"Token":33},{"Opt":909},{"Rep":88},{"Opt":72},{"And":[[911,912],null]},{"Or":[913]},{"Call":[145,[[3,914]]]},{"And":[[910,915],null]},{"Or":[916]},{"Call":[75,[[0,917]]]},{"And":[[918],null]},{"Or":[919]},{"And":[[89],null]},{"And":[[93],null]},{"And":[[92],null]},{"And":[[3],null]},{"Token":9},{"Opt":90},{"Opt":91},{"Token":56},{"And":[[925,60,926,927,928],1]},{"Or":[929]},{"Token":57},{"And":[[931,49],null]},{"Or":[932]},{"Token":51},{"And":[[934,72],null]},{"Or":[935]},{"Token":56},{"And":[[937],null]},{"Or":[938]},"Eof",{"Not":940},{"And":[[74,941],null]},{"Token":56},{"And":[[943],null]},{"Or":[942,944]},{"And":[[72,945],null]},{"Or":[946]},{"Enter":[2,947]},{"And":[[948],null]},{"Or":[949]},{"Token":14},{"Token":15},{"And":[[87],null]},{"And":[[94],null]},{"Or":[953,954]},{"And":[[952,955],null]},{"Or":[956]},{"Opt":957},{"And":[[951,96,87,958],1]},{"Or":[959]},{"Opt":100},{"Token":25},{"And":[[961,962,96,87],2]},{"Or":[963]},{"Token":9},{"Token":51},{"And":[[965,60,966],1]},{"Or":[967]},{"Opt":968},{"And":[[969,97],null]},{"Enter":[0,72]},{"And":[[971],null]},{"Opt":100},{"Token":24},{"And":[[973,974,87],2]},{"Or":[975]},{"Opt":100},{"Token":23},{"Token":32},{"And":[[977,978,60,979,97,87],2]},{"Or":[980]},{"Token":85},{"Token":57},{"And":[[982,983],null]},{"Token":30},{"Rep":102},{"Call":[145,[[3,986]]]},{"And":[[985,97,987],1]},{"Or":[988]},{"Token":50},{"Enter":[2,72]},{"Token":59},{"And":[[992],null]},"Eof",{"And":[[994],null]},{"And":[[74],null]},{"Or":[993,995,996]},{"And":[[103,990,991,997],1]},{"Or":[998]},{"Token":77},{"And":[[1000,60],null]},{"Or":[1001]},{"Rep":1002},{"Opt":104},{"And":[[60,1003,1004],null]},{"Token":14},{"And":[[1006,72],null]},{"Or":[1007]},{"And":[[141],null]},{"Or":[1009]},{"And":[[142],null]},{"Or":[1011]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1015},{"And":[[1013,1014,1016,109],null]},{"Or":[1017]},{"And":[[72,1018],null]},{"Or":[1019]},{"IsIn":2},{"Not":74},{"And":[[1021,1022],null]},{"IsIn":2},{"Not":1024},{"And":[[1025],null]},{"Or":[1023,1026]},{"And":[[1027,109],null]},{"Or":[1028]},{"And":[[72,1029],null]},{"Or":[1030]},{"Call":[144,[[2,110]]]},{"Call":[146,[[4,1032]]]},{"Call":[75,[[0,1033]]]},{"And":[[1034],null]},{"And":[[72],null]},{"Or":[1036]},{"Token":60},{"Token":90},{"And":[[1039],null]},{"Token":87},{"And":[[1041],null]},{"Or":[1040,1042]},{"And":[[1038,1043],null]},{"Or":[1044]},{"And":[[72,1045],null]},{"Or":[1046]},{"Call":[148,[[6,72]]]},{"And":[[72,1048],null]},{"Or":[1049]},{"Token":81},{"And":[[72,1051],null]},{"Or":[1052]},{"Token":5},{"And":[[1054,49],null]},{"Or":[1055]},{"And":[[72,1056],null]},{"Or":[1057]},{"And":[[116,72],null]},{"Or":[1059]},{"Token":75},{"Token":85},{"Opt":1062},{"Token":27},{"Opt":1064},{"And":[[1061,1063,1065],null]},{"Token":65},{"And":[[1067,72],null]},{"Or":[1068]},{"Token":73},{"And":[[1070,72],null]},{"Or":[1071]},{"Token":80},{"And":[[1073,72],null]},{"Or":[1074]},{"Token":65},{"And":[[1076],null]},{"Token":67},{"And":[[1078],null]},{"Token":69},{"And":[[1080],null]},{"Or":[1077,1079,1081]},{"Call":[123,[[1,1082]]]},{"And":[[72,1083,72],null]},{"Or":[1084]},{"Token":71},{"And":[[1086],null]},{"Token":73},{"And":[[1088],null]},{"Or":[1087,1089]},{"Call":[123,[[1,1090]]]},{"And":[[72,1091,72],null]},{"Or":[1092]},{"ContextualToken":[43,"<<"]},{"And":[[1094],null]},{"ContextualToken":[45,">>"]},{"And":[[1096],null]},{"Or":[1095,1097]},{"Call":[123,[[1,1098]]]},{"And":[[72,1099,72],null]},{"Or":[1100]},{"IsIn":2},{"Not":74},{"Var":1},{"And":[[1102,1103,1104],null]},{"IsIn":2},{"Not":1106},{"Var":1},{"And":[[1107,1108],null]},{"Token":75},{"Token":75},{"Not":1111},{"And":[[1110,1112],null]},{"Or":[1113]},{"Call":[123,[[1,1114]]]},{"And":[[72,1115,72],null]},{"Or":[1116]},{"Token":82},{"Call":[123,[[1,1118]]]},{"And":[[72,1119,72],null]},{"Or":[1120]},{"Token":77},{"Token":77},{"Not":1123},{"And":[[1122,1124],null]},{"Or":[1125]},{"Call":[123,[[1,1126]]]},{"And":[[72,1127,72],null]},{"Or":[1128]},{"Call":[123,[[1,128]]]},{"And":[[72,1130,72],null]},{"Or":[1131]},{"Token":52},{"And":[[1133],null]},{"Token":53},{"And":[[1135],null]},{"Token":39},{"And":[[1137],null]},{"Token":40},{"And":[[1139],null]},{"Token":55},{"And":[[1141],null]},{"Token":54},{"And":[[1143],null]},{"ContextualToken":[47,"&&"]},{"Call":[123,[[1,1145]]]},{"And":[[72,1146,72],null]},{"Or":[1147]},{"ContextualToken":[48,"||"]},{"Call":[123,[[1,1149]]]},{"And":[[72,1150,72],null]},{"Or":[1151]},{"Call":[123,[[1,136]]]},{"And":[[72,1153,72],null]},{"Or":[1154]},{"And":[[136,72],null]},{"Or":[1156]},{"And":[[72,135],null]},{"Or":[1158]},{"Token":61},{"Not":73},{"And":[[1160,1161],null]},{"Or":[1162]},{"Token":61},{"And":[[1164],null]},{"Token":62},{"And":[[1166],null]},{"Not":73},{"Not":1168},{"Token":37},{"IsIn":0},{"And":[[1170,1171],null]},{"Or":[1172]},{"Not":1173},{"And":[[135,1169,1174],null]},{"Token":51},{"And":[[1176],null]},{"Token":72},{"And":[[1178],null]},{"Token":74},{"And":[[1180],null]},{"Token":66},{"And":[[1182],null]},{"Token":68},{"And":[[1184],null]},{"Token":70},{"And":[[1186],null]},{"Token":76},{"And":[[1188],null]},{"Token":78},{"And":[[1190],null]},{"Token":83},{"And":[[1192],null]},{"ContextualToken":[46,">>="]},{"And":[[1194],null]},{"ContextualToken":[44,"<<="]},{"And":[[1196],null]},{"Or":[1177,1179,1181,1183,1185,1187,1189,1191,1193,1195,1197]},{"Call":[123,[[1,1198]]]},{"And":[[72,1199,72],null]},{"Or":[1200]},{"Token":63},{"Call":[144,[[2,140]]]},{"Call":[148,[[6,1203]]]},{"And":[[1202,1204],null]},{"Or":[1205]},{"Rep":138},{"And":[[1207],null]},{"Token":90},{"Token":51},{"And":[[1210,72],null]},{"Call":[144,[[2,140]]]},{"Call":[146,[[4,1212]]]},{"And":[[1213],null]},{"Or":[1211,1214]},{"Opt":1215},{"And":[[1209,1216],1]},{"Or":[1217]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1221},{"Rep":143},{"Call":[145,[[3,1223]]]},{"And":[[1219,1220,1222,1224],null]},{"Or":[1225]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1229},{"Token":35},{"Rep":143},{"Token":36},{"And":[[1231,1232,1233],null]},{"Token":41},{"Rep":143},{"Token":42},{"And":[[1235,1236,1237],null]},{"Or":[1234,1238]},{"And":[[1227,1228,1230,1239],null]},{"Or":[1240]},{"Token":35},{"And":[[1242],null]},{"Token":36},{"And":[[1244],null]},{"Token":37},{"And":[[1246],null]},{"Token":38},{"And":[[1248],null]},{"Token":41},{"And":[[1250],null]},{"Token":42},{"And":[[1252],null]},{"Or":[1243,1245,1247,1249,1251,1253]},{"Not":1254},"Any",{"And":[[1255,1256],null]},{"Token":35},{"Rep":143},{"Token":36},{"And":[[1258,1259,1260],null]},{"Token":41},{"Rep":143},{"Token":42},{"And":[[1262,1263,1264],null]},{"Token":37},{"Rep":143},{"Token":38},{"And":[[1266,1267,1268],null]},{"Or":[1257,1261,1265,1269]},{"Var":2},"Eof",{"And":[[1272],null]},{"Token":59},{"And":[[1274],null]},{"Or":[1273,1275]},{"And":[[1271,1276],1]},{"Or":[1277]},{"Rep":1278},{"And":[[1279],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[149,[[7,1281],[8,1282],[9,1283]]]},{"And":[[1284],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[149,[[7,1286],[8,1287],[9,1288]]]},{"And":[[1289],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[149,[[7,1291],[8,1292],[9,1293]]]},{"And":[[1294],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[149,[[7,1296],[8,1297],[9,1298]]]},{"And":[[1299],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[150,[[10,1302],[11,1303]]]},{"Var":9},{"Layer":[1304,1305]},{"Var":8},{"And":[[1301,1306,1307],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[150,[[10,1310],[11,1311]]]},{"Var":11},{"And":[[1309,1312,1313],1]},{"Var":11},{"Not":1315},"Any",{"And":[[1316,1317],null]},{"Or":[1318]},{"And":[[1319],null]},{"Or":[1314,1320]},{"Rep":1321},{"And":[[1322],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            whitespace_binder,
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: rt::Language = {
            use fall_parse::{ParserDefinition, parse, reparse};
            use std::any::Any;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &Any,
                    edit: &TextEdit,
                    new_text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
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
                        TRAIT_PROJECTION_PATH => NodeTypeInfo { name: "TRAIT_PROJECTION_PATH", whitespace_like: false },
                        PATH_SEGMENT => NodeTypeInfo { name: "PATH_SEGMENT", whitespace_like: false },
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FnDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for FnDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == FN_DEF {
            Some(FnDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> FnDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for FnDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("FnDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for StructDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == STRUCT_DEF {
            Some(StructDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> StructDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for StructDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("StructDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for EnumDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == ENUM_DEF {
            Some(EnumDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> EnumDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for EnumDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("EnumDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TraitDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TRAIT_DEF {
            Some(TraitDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TraitDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for TraitDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TraitDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Path<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for Path<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == PATH {
            Some(Path { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> Path<'f> {
    pub fn qualifier(&self) -> Option<Path<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn segment(&self) -> Option<PathSegment<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}

impl<'f> ::std::fmt::Debug for Path<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Path@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathSegment<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for PathSegment<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == PATH_SEGMENT {
            Some(PathSegment { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> PathSegment<'f> {
    
}

impl<'f> ::std::fmt::Debug for PathSegment<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("PathSegment@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UseDecl<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for UseDecl<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == USE_DECL {
            Some(UseDecl { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> UseDecl<'f> {
    pub fn path(&self) -> Option<Path<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
    pub fn spec(&self) -> Option<UseSpec<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}

impl<'f> ::std::fmt::Debug for UseDecl<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("UseDecl@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UseSpec<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for UseSpec<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == USE_SPEC {
            Some(UseSpec { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> UseSpec<'f> {
    
}

impl<'f> ::std::fmt::Debug for UseSpec<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("UseSpec@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}



pub trait NameOwner<'f>: rt::AstNode<'f> {
    fn name_ident(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == IDENT)
    }
    fn name(&self) -> Option<Text<'f>> {
        rt::child_of_type(self.node(), IDENT).map(|n| n.text())
    }
}
impl<'f> NameOwner<'f> for FnDef<'f> {}
impl<'f> NameOwner<'f> for StructDef<'f> {}
impl<'f> NameOwner<'f> for EnumDef<'f> {}
impl<'f> NameOwner<'f> for TraitDef<'f> {}