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
pub const CONTINUE: rt::NodeType = rt::NodeType(131);
pub const BREAK: rt::NodeType = rt::NodeType(132);
pub const IN: rt::NodeType = rt::NodeType(133);
pub const UNSAFE: rt::NodeType = rt::NodeType(134);
pub const WHERE: rt::NodeType = rt::NodeType(135);
pub const L_PAREN: rt::NodeType = rt::NodeType(136);
pub const R_PAREN: rt::NodeType = rt::NodeType(137);
pub const L_CURLY: rt::NodeType = rt::NodeType(138);
pub const R_CURLY: rt::NodeType = rt::NodeType(139);
pub const L_ANGLE: rt::NodeType = rt::NodeType(140);
pub const R_ANGLE: rt::NodeType = rt::NodeType(141);
pub const L_BRACK: rt::NodeType = rt::NodeType(142);
pub const R_BRACK: rt::NodeType = rt::NodeType(143);
pub const SHL: rt::NodeType = rt::NodeType(144);
pub const SHL_EQ: rt::NodeType = rt::NodeType(145);
pub const SHR: rt::NodeType = rt::NodeType(146);
pub const SHR_EQ: rt::NodeType = rt::NodeType(147);
pub const AND: rt::NodeType = rt::NodeType(148);
pub const OR: rt::NodeType = rt::NodeType(149);
pub const THIN_ARROW: rt::NodeType = rt::NodeType(150);
pub const FAT_ARROW: rt::NodeType = rt::NodeType(151);
pub const EQ: rt::NodeType = rt::NodeType(152);
pub const EQEQ: rt::NodeType = rt::NodeType(153);
pub const BANGEQ: rt::NodeType = rt::NodeType(154);
pub const GTET: rt::NodeType = rt::NodeType(155);
pub const LTEQ: rt::NodeType = rt::NodeType(156);
pub const SEMI: rt::NodeType = rt::NodeType(157);
pub const COLON: rt::NodeType = rt::NodeType(158);
pub const COLONCOLON: rt::NodeType = rt::NodeType(159);
pub const COMMA: rt::NodeType = rt::NodeType(160);
pub const DOT: rt::NodeType = rt::NodeType(161);
pub const DOTDOT: rt::NodeType = rt::NodeType(162);
pub const DOTDOTDOT: rt::NodeType = rt::NodeType(163);
pub const HASH: rt::NodeType = rt::NodeType(164);
pub const DOLLAR: rt::NodeType = rt::NodeType(165);
pub const STAR: rt::NodeType = rt::NodeType(166);
pub const STAR_EQ: rt::NodeType = rt::NodeType(167);
pub const SLASH: rt::NodeType = rt::NodeType(168);
pub const SLASH_EQ: rt::NodeType = rt::NodeType(169);
pub const PERCENT: rt::NodeType = rt::NodeType(170);
pub const PERCENT_EQ: rt::NodeType = rt::NodeType(171);
pub const PLUS: rt::NodeType = rt::NodeType(172);
pub const PLUS_EQ: rt::NodeType = rt::NodeType(173);
pub const MINUS: rt::NodeType = rt::NodeType(174);
pub const MINUS_EQ: rt::NodeType = rt::NodeType(175);
pub const AMPERSAND: rt::NodeType = rt::NodeType(176);
pub const AMPERSAND_EQ: rt::NodeType = rt::NodeType(177);
pub const PIPE: rt::NodeType = rt::NodeType(178);
pub const PIPE_EQ: rt::NodeType = rt::NodeType(179);
pub const UNDERSCORE: rt::NodeType = rt::NodeType(180);
pub const BANG: rt::NodeType = rt::NodeType(181);
pub const QUESTION: rt::NodeType = rt::NodeType(182);
pub const CARET: rt::NodeType = rt::NodeType(183);
pub const CARET_EQ: rt::NodeType = rt::NodeType(184);
pub const CHAR: rt::NodeType = rt::NodeType(185);
pub const LIFETIME: rt::NodeType = rt::NodeType(186);
pub const BOOL: rt::NodeType = rt::NodeType(187);
pub const NUMBER: rt::NodeType = rt::NodeType(188);
pub const STRING: rt::NodeType = rt::NodeType(189);
pub const RAW_STRING: rt::NodeType = rt::NodeType(190);
pub const IDENT: rt::NodeType = rt::NodeType(191);
pub const FILE: rt::NodeType = rt::NodeType(192);
pub const USE_DECL: rt::NodeType = rt::NodeType(193);
pub const USE_SPEC: rt::NodeType = rt::NodeType(194);
pub const USE_SPEC_ENTRY: rt::NodeType = rt::NodeType(195);
pub const EXTERN_CRATE_DECL: rt::NodeType = rt::NodeType(196);
pub const FN_DEF: rt::NodeType = rt::NodeType(197);
pub const LINKAGE: rt::NodeType = rt::NodeType(198);
pub const VALUE_PARAM: rt::NodeType = rt::NodeType(199);
pub const LAMBDA_VALUE_PARAM: rt::NodeType = rt::NodeType(200);
pub const SELF_PARAMETER: rt::NodeType = rt::NodeType(201);
pub const STRUCT_DEF: rt::NodeType = rt::NodeType(202);
pub const STRUCT_FIELD: rt::NodeType = rt::NodeType(203);
pub const TUPLE_FIELD: rt::NodeType = rt::NodeType(204);
pub const ENUM_DEF: rt::NodeType = rt::NodeType(205);
pub const ENUM_VARIANT: rt::NodeType = rt::NodeType(206);
pub const MOD_DEF: rt::NodeType = rt::NodeType(207);
pub const IMPL_DEF: rt::NodeType = rt::NodeType(208);
pub const TRAIT_DEF: rt::NodeType = rt::NodeType(209);
pub const MEMBERS: rt::NodeType = rt::NodeType(210);
pub const TYPE_DEF: rt::NodeType = rt::NodeType(211);
pub const CONST_DEF: rt::NodeType = rt::NodeType(212);
pub const MACRO_ITEM: rt::NodeType = rt::NodeType(213);
pub const EXTERN_BLOCK: rt::NodeType = rt::NodeType(214);
pub const TYPE_PARAMETERS: rt::NodeType = rt::NodeType(215);
pub const TYPE_PARAMETER: rt::NodeType = rt::NodeType(216);
pub const TYPE_BOUND: rt::NodeType = rt::NodeType(217);
pub const LIFETIME_PARAMETER: rt::NodeType = rt::NodeType(218);
pub const VISIBILITY: rt::NodeType = rt::NodeType(219);
pub const WHERE_CLAUSE: rt::NodeType = rt::NodeType(220);
pub const PATH: rt::NodeType = rt::NodeType(221);
pub const TRAIT_PROJECTION_PATH: rt::NodeType = rt::NodeType(222);
pub const PATH_SEGMENT: rt::NodeType = rt::NodeType(223);
pub const TYPE_ARGUMENTS: rt::NodeType = rt::NodeType(224);
pub const FN_TRAIT_SUGAR: rt::NodeType = rt::NodeType(225);
pub const ALIAS: rt::NodeType = rt::NodeType(226);
pub const PATH_TYPE: rt::NodeType = rt::NodeType(227);
pub const REFERENCE_TYPE: rt::NodeType = rt::NodeType(228);
pub const POINTER_TYPE: rt::NodeType = rt::NodeType(229);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(230);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(231);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(232);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(233);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(234);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(235);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(236);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(237);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(238);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(239);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(240);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(241);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(242);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(243);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(244);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(245);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(246);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(247);
pub const EXPR: rt::NodeType = rt::NodeType(248);
pub const LITERAL: rt::NodeType = rt::NodeType(249);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(250);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(251);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(252);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(253);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(254);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(255);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(256);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(257);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(258);
pub const LOOP_CF_EXPR: rt::NodeType = rt::NodeType(259);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(260);
pub const LET_STMT: rt::NodeType = rt::NodeType(261);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(262);
pub const INITIALIZER: rt::NodeType = rt::NodeType(263);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(264);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(265);
pub const IF_EXPR: rt::NodeType = rt::NodeType(266);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(267);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(268);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(269);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(270);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(271);
pub const GUARD: rt::NodeType = rt::NodeType(272);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(273);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(274);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(275);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(276);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(277);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(278);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(279);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(280);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(281);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(282);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(283);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(284);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(285);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(286);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(287);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(288);
pub const BIT_AND: rt::NodeType = rt::NodeType(289);
pub const BIT_XOR: rt::NodeType = rt::NodeType(290);
pub const BIT_OR: rt::NodeType = rt::NodeType(291);
pub const COMPARISON: rt::NodeType = rt::NodeType(292);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(293);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(294);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(295);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(296);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(297);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(298);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(299);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(300);
pub const TT: rt::NodeType = rt::NodeType(301);


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
            ::fall_parse::LexRule::new(CONTINUE, "continue", None),
            ::fall_parse::LexRule::new(BREAK, "break", None),
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
        let parser_json = r##"[{"Pub":{"ty":93,"body":155,"replaceable":false}},{"Or":[158]},{"Or":[160,162,164,166,168,170,172,173]},{"Cached":190},{"Pub":{"ty":94,"body":206,"replaceable":false}},{"Pub":{"ty":95,"body":212,"replaceable":false}},{"Pub":{"ty":96,"body":218,"replaceable":false}},{"Pub":{"ty":97,"body":225,"replaceable":false}},{"Pub":{"ty":98,"body":241,"replaceable":false}},{"Or":[243]},{"Pub":{"ty":99,"body":248,"replaceable":false}},{"Or":[254]},{"Pub":{"ty":100,"body":257,"replaceable":false}},{"Pub":{"ty":101,"body":263,"replaceable":false}},{"Pub":{"ty":102,"body":280,"replaceable":false}},{"Pub":{"ty":103,"body":299,"replaceable":false}},{"Pub":{"ty":104,"body":304,"replaceable":false}},{"Pub":{"ty":105,"body":307,"replaceable":false}},{"Pub":{"ty":106,"body":314,"replaceable":false}},{"Pub":{"ty":107,"body":327,"replaceable":false}},{"Pub":{"ty":108,"body":336,"replaceable":false}},{"Pub":{"ty":109,"body":349,"replaceable":false}},{"Pub":{"ty":110,"body":357,"replaceable":false}},{"Pub":{"ty":111,"body":369,"replaceable":false}},{"Or":[370,371,372]},{"Or":[374,376,378,380,382,384,386,388,393]},{"Pub":{"ty":112,"body":403,"replaceable":false}},{"Pub":{"ty":113,"body":417,"replaceable":false}},{"Pub":{"ty":114,"body":421,"replaceable":false}},{"Pub":{"ty":115,"body":425,"replaceable":false}},{"Or":[433]},{"Pub":{"ty":116,"body":440,"replaceable":false}},{"Pub":{"ty":117,"body":444,"replaceable":false}},{"Or":[464]},{"Pub":{"ty":118,"body":468,"replaceable":false}},{"Pub":{"ty":119,"body":488,"replaceable":false}},{"Pub":{"ty":120,"body":498,"replaceable":false}},{"Pub":{"ty":121,"body":513,"replaceable":false}},{"Or":[514]},{"Or":[516]},{"Or":[518]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":122,"op":521,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":122,"body":525,"replaceable":false}},{"Pub":{"ty":122,"body":530,"replaceable":false}},{"Pub":{"ty":123,"body":537,"replaceable":false}},{"Pub":{"ty":124,"body":551,"replaceable":false}},{"Pub":{"ty":125,"body":577,"replaceable":false}},{"Pub":{"ty":126,"body":582,"replaceable":false}},{"Pub":{"ty":127,"body":586,"replaceable":false}},{"Or":[591]},{"Or":[592,593,594,595,596,597,598,599,600]},{"Pub":{"ty":128,"body":602,"replaceable":false}},{"Pub":{"ty":129,"body":604,"replaceable":false}},{"Pub":{"ty":130,"body":612,"replaceable":false}},{"Pub":{"ty":131,"body":615,"replaceable":false}},{"Pub":{"ty":132,"body":619,"replaceable":false}},{"Pub":{"ty":133,"body":625,"replaceable":true}},{"PubReplace":{"ty":134,"body":629}},{"Pub":{"ty":135,"body":632,"replaceable":false}},{"Pub":{"ty":136,"body":641,"replaceable":false}},{"Pub":{"ty":137,"body":648,"replaceable":false}},{"Pub":{"ty":100,"body":654,"replaceable":false}},{"Or":[655,656,657,658,659,660,661]},{"Pub":{"ty":138,"body":664,"replaceable":false}},{"Pub":{"ty":139,"body":670,"replaceable":true}},{"PubReplace":{"ty":140,"body":682}},{"PubReplace":{"ty":141,"body":694}},{"Pub":{"ty":142,"body":701,"replaceable":false}},{"Pub":{"ty":143,"body":708,"replaceable":false}},{"Pub":{"ty":144,"body":710,"replaceable":false}},{"Pub":{"ty":145,"body":714,"replaceable":false}},{"Pub":{"ty":146,"body":720,"replaceable":true}},{"PubReplace":{"ty":147,"body":724}},{"Pub":{"ty":148,"body":729,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,90,97,98,101,102,104,108,109,137],"prefixes":[{"ty":183,"op":119,"priority":999},{"ty":184,"op":758,"priority":999},{"ty":185,"op":759,"priority":999},{"ty":186,"op":760,"priority":999},{"ty":196,"op":139,"priority":2}],"infixes":[{"ty":176,"op":735,"priority":999,"has_rhs":false},{"ty":177,"op":744,"priority":999,"has_rhs":false},{"ty":179,"op":752,"priority":999,"has_rhs":false},{"ty":180,"op":753,"priority":999,"has_rhs":false},{"ty":181,"op":754,"priority":999,"has_rhs":false},{"ty":182,"op":757,"priority":999,"has_rhs":false},{"ty":187,"op":768,"priority":11,"has_rhs":true},{"ty":188,"op":774,"priority":10,"has_rhs":true},{"ty":189,"op":780,"priority":9,"has_rhs":true},{"ty":190,"op":786,"priority":8,"has_rhs":true},{"ty":191,"op":788,"priority":7,"has_rhs":true},{"ty":192,"op":794,"priority":6,"has_rhs":true},{"ty":193,"op":795,"priority":5,"has_rhs":true},{"ty":194,"op":797,"priority":4,"has_rhs":true},{"ty":195,"op":799,"priority":3,"has_rhs":true},{"ty":196,"op":800,"priority":2,"has_rhs":true},{"ty":196,"op":138,"priority":2,"has_rhs":false},{"ty":197,"op":824,"priority":1,"has_rhs":true}]}},{"Or":[825,827,829,831,833,835,837,839,841,843,845,847,849,851,853,855,857,859,861,863,865,867]},{"Or":[869]},{"Or":[873]},{"Pub":{"ty":150,"body":884,"replaceable":false}},{"Pub":{"ty":151,"body":892,"replaceable":true}},{"PubReplace":{"ty":152,"body":905}},{"Pub":{"ty":153,"body":912,"replaceable":false}},{"Pub":{"ty":154,"body":916,"replaceable":false}},{"Pub":{"ty":155,"body":923,"replaceable":true}},{"PubReplace":{"ty":156,"body":928}},{"Pub":{"ty":157,"body":933,"replaceable":false}},{"Pub":{"ty":158,"body":945,"replaceable":false}},{"Or":[953]},{"Pub":{"ty":159,"body":957,"replaceable":false}},{"Pub":{"ty":160,"body":966,"replaceable":false}},{"Pub":{"ty":161,"body":978,"replaceable":false}},{"Or":[979,980,981,982]},{"Pub":{"ty":162,"body":988,"replaceable":false}},{"Pub":{"ty":163,"body":991,"replaceable":false}},{"Pub":{"ty":164,"body":994,"replaceable":false}},{"Pub":{"ty":165,"body":997,"replaceable":false}},{"Pub":{"ty":166,"body":1008,"replaceable":false}},{"Pub":{"ty":167,"body":1018,"replaceable":false}},{"Pub":{"ty":168,"body":1022,"replaceable":false}},{"Or":[1028]},{"Or":[1030]},{"Pub":{"ty":169,"body":1034,"replaceable":false}},{"Pub":{"ty":170,"body":1039,"replaceable":false}},{"Or":[1042]},{"Pub":{"ty":171,"body":1047,"replaceable":false}},{"Pub":{"ty":172,"body":1057,"replaceable":false}},{"Or":[1063]},{"Pub":{"ty":173,"body":1066,"replaceable":false}},{"Pub":{"ty":174,"body":1068,"replaceable":false}},{"Pub":{"ty":175,"body":1070,"replaceable":false}},{"Pub":{"ty":176,"body":1078,"replaceable":false}},{"Pub":{"ty":177,"body":1089,"replaceable":false}},{"Or":[1093]},{"Pub":{"ty":178,"body":1095,"replaceable":false}},{"Pub":{"ty":179,"body":1105,"replaceable":false}},{"Pub":{"ty":180,"body":1108,"replaceable":false}},{"Pub":{"ty":181,"body":1111,"replaceable":false}},{"Pub":{"ty":182,"body":1116,"replaceable":false}},{"Pub":{"ty":183,"body":1118,"replaceable":false}},{"Or":[1124]},{"Pub":{"ty":184,"body":1127,"replaceable":false}},{"Pub":{"ty":185,"body":1130,"replaceable":false}},{"Pub":{"ty":186,"body":1133,"replaceable":false}},{"Pub":{"ty":187,"body":1143,"replaceable":false}},{"Pub":{"ty":188,"body":1151,"replaceable":false}},{"Pub":{"ty":189,"body":1159,"replaceable":false}},{"Or":[1163,1167]},{"Pub":{"ty":190,"body":1175,"replaceable":false}},{"Pub":{"ty":191,"body":1179,"replaceable":false}},{"Pub":{"ty":192,"body":1187,"replaceable":false}},{"Pub":{"ty":193,"body":1190,"replaceable":false}},{"Or":[1192,1194,1196,1198,1200,1202]},{"Pub":{"ty":194,"body":1206,"replaceable":false}},{"Pub":{"ty":195,"body":1210,"replaceable":false}},{"Pub":{"ty":196,"body":1213,"replaceable":false}},{"Pub":{"ty":196,"body":1215,"replaceable":false}},{"Pub":{"ty":196,"body":1217,"replaceable":false}},{"Pub":{"ty":196,"body":1221,"replaceable":false}},{"Or":[1223,1225]},{"Or":[1233]},{"Pub":{"ty":197,"body":1259,"replaceable":false}},{"Pub":{"ty":198,"body":1264,"replaceable":false}},{"Or":[1266]},{"Pub":{"ty":199,"body":1276,"replaceable":false}},{"Pub":{"ty":200,"body":1284,"replaceable":false}},{"Pub":{"ty":201,"body":1299,"replaceable":false}},{"Pub":{"ty":202,"body":1328,"replaceable":false}},{"Or":[1338]},{"Or":[1343]},{"Or":[1348]},{"Or":[1353]},{"Or":[1358]},{"Or":[1366]},{"Or":[1381]},{"And":[[1],null]},{"Or":[154]},{"WithSkip":[2,3]},{"Rep":156},{"And":[[157],null]},{"Token":11},{"And":[[159],null]},{"ContextualToken":[4,"union"]},{"And":[[161],null]},{"Token":16},{"And":[[163],null]},{"Token":12},{"And":[[165],null]},{"Token":13},{"And":[[167],null]},{"Token":17},{"And":[[169],null]},{"Token":29},{"And":[[171],null]},{"And":[[25],null]},{"Opt":36},{"And":[[142,174],null]},{"Or":[175]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[177,178,179,180,181,182,183,184,185]},{"Inject":[176,186]},{"And":[[187],null]},{"And":[[28],null]},{"Or":[188,189]},{"Token":12},{"And":[[48],null]},{"Token":60},{"And":[[193,5],null]},{"Or":[194]},{"Opt":195},{"And":[[196],null]},{"Or":[192,197]},{"And":[[38,198],null]},{"Token":60},{"Opt":200},{"And":[[201,5],null]},{"Or":[199,202]},{"Token":58},{"And":[[191,203,204],1]},{"Or":[205]},{"Token":67},{"And":[[207],null]},{"Call":[147,[[2,6]]]},{"Call":[148,[[3,209]]]},{"And":[[210],null]},{"Or":[208,211]},{"Token":18},{"And":[[213],null]},{"Token":92},{"Opt":48},{"And":[[215,216],1]},{"Or":[214,217]},{"Token":7},{"Token":6},{"Token":92},{"Opt":48},{"Token":58},{"And":[[219,220,221,222,223],2]},{"Or":[224]},{"Token":21},{"Opt":226},{"Token":35},{"Opt":228},{"Opt":10},{"Token":8},{"Token":92},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[90],null]},{"Token":58},{"And":[[237],null]},{"Or":[236,238]},{"And":[[227,229,230,231,232,233,11,234,235,239],4]},{"Or":[240]},{"Token":51},{"And":[[242,49],null]},{"Token":7},{"Token":90},{"Opt":245},{"And":[[244,246],null]},{"Or":[247]},{"Opt":14},{"Call":[147,[[2,12]]]},{"And":[[249,250],null]},{"Or":[251]},{"Call":[149,[[4,252]]]},{"And":[[253],null]},{"Token":59},{"And":[[62,255,49],1]},{"Or":[256]},{"Token":59},{"And":[[258,49],null]},{"Or":[259]},{"Opt":260},{"And":[[62,261],null]},{"Or":[262]},{"And":[[119],null]},{"Token":27},{"And":[[265],null]},{"Or":[264,266]},{"Opt":267},{"Token":18},{"Token":59},{"And":[[270,49],null]},{"Or":[271]},{"Opt":272},{"Token":61},{"And":[[274],null]},"Eof",{"And":[[276],null]},{"Or":[275,277]},{"And":[[268,269,273,278],2]},{"Or":[279]},{"Token":11},{"And":[[281],null]},{"ContextualToken":[4,"union"]},{"And":[[283],null]},{"Or":[282,284]},{"Token":92},{"Opt":31},{"Call":[147,[[2,16]]]},{"Call":[148,[[3,288]]]},{"And":[[289],null]},{"Token":58},{"And":[[291],null]},{"Call":[147,[[2,17]]]},{"Call":[149,[[4,293]]]},{"Token":58},{"And":[[294,295],null]},{"Or":[290,292,296]},{"And":[[285,286,287,297],1]},{"Or":[298]},{"Opt":36},{"Token":92},{"Token":59},{"And":[[300,301,302,49],2]},{"Or":[303]},{"Opt":36},{"And":[[305,49],null]},{"Or":[306]},{"Token":16},{"Token":92},{"Opt":31},{"Call":[147,[[2,19]]]},{"Call":[148,[[3,311]]]},{"And":[[308,309,310,312],1]},{"Or":[313]},{"Token":92},{"Token":53},{"And":[[316,74],null]},{"Call":[147,[[2,17]]]},{"Call":[149,[[4,318]]]},{"And":[[319],null]},{"Call":[147,[[2,16]]]},{"Call":[148,[[3,321]]]},{"And":[[322],null]},{"Or":[317,320,323]},{"Opt":324},{"And":[[315,325],1]},{"Or":[326]},{"Token":13},{"Token":92},{"Token":58},{"And":[[330],null]},{"Call":[148,[[3,1]]]},{"And":[[332],null]},{"Or":[331,333]},{"And":[[328,329,334],1]},{"Or":[335]},{"Token":35},{"Opt":337},{"Token":17},{"Opt":31},{"Token":23},{"And":[[341,49],null]},{"Or":[342]},{"Opt":343},{"And":[[49,344],null]},{"Or":[345]},{"Opt":37},{"And":[[338,339,340,346,347,23],2]},{"Or":[348]},{"Opt":36},{"Token":29},{"Token":92},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[142,350,351,352,353,354,355,23],3]},{"Or":[356]},{"Opt":36},{"And":[[142,358],null]},{"Or":[359]},{"Inject":[360,24]},{"And":[[361],null]},{"And":[[28],null]},{"Or":[362,363]},{"WithSkip":[25,364]},{"Rep":365},{"Call":[148,[[3,366]]]},{"And":[[367],null]},{"Or":[368]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[373],null]},{"Token":8},{"And":[[375],null]},{"Token":20},{"And":[[377],null]},{"Token":21},{"And":[[379],null]},{"Token":22},{"And":[[381],null]},{"Token":35},{"And":[[383],null]},{"Token":65},{"And":[[385],null]},{"Token":7},{"And":[[387],null]},{"Token":92},{"Token":82},{"And":[[389,390],null]},{"Or":[391]},{"And":[[392],null]},{"Token":20},{"Token":92},{"Opt":31},{"Token":53},{"And":[[397,49],null]},{"Or":[398]},{"Opt":399},{"Token":58},{"And":[[394,395,396,400,401],1]},{"Or":[402]},{"Token":21},{"And":[[404],null]},{"Token":22},{"And":[[406],null]},{"Or":[405,407]},{"Token":92},{"Token":59},{"Token":53},{"And":[[411,74],null]},{"Or":[412]},{"Opt":413},{"Token":58},{"And":[[408,409,410,49,414,415],1]},{"Or":[416]},{"And":[[144],null]},{"Token":58},{"And":[[145,419],null]},{"Or":[418,420]},{"Rep":30},{"Call":[148,[[3,422]]]},{"And":[[10,423],null]},{"Or":[424]},{"Opt":36},{"And":[[142,426],null]},{"Or":[427]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[429,430]},{"Inject":[428,431]},{"And":[[432],null]},{"Call":[147,[[2,35]]]},{"Call":[147,[[2,32]]]},{"And":[[434,435],null]},{"Or":[436]},{"Call":[150,[[5,437]]]},{"And":[[438],null]},{"Or":[439]},{"Token":92},{"Opt":33},{"And":[[441,442],1]},{"Or":[443]},{"Token":59},{"Token":73},{"And":[[446],null]},"Eof",{"And":[[448],null]},{"Token":61},{"And":[[450],null]},{"Token":39},{"And":[[452],null]},{"Token":36},{"And":[[454],null]},{"Or":[451,453,455]},{"Not":456},{"Not":457},{"And":[[458],null]},{"Or":[447,449,459]},{"And":[[34,460],1]},{"Or":[461]},{"Rep":462},{"And":[[445,463],null]},{"Token":87},{"And":[[465],null]},{"And":[[51],null]},{"Or":[466,467]},{"Token":87},{"Token":59},{"Token":87},{"Token":73},{"And":[[472],null]},"Eof",{"And":[[474],null]},{"Token":61},{"Not":476},{"Not":477},{"And":[[478],null]},{"Or":[473,475,479]},{"And":[[471,480],1]},{"Or":[481]},{"Rep":482},{"And":[[470,483],null]},{"Or":[484]},{"Opt":485},{"And":[[469,486],1]},{"Or":[487]},{"Token":10},{"Token":6},{"And":[[490],null]},{"Token":19},{"And":[[492],null]},{"Or":[491,493]},{"Call":[149,[[4,494]]]},{"Opt":495},{"And":[[489,496],null]},{"Or":[497]},{"Token":36},{"Token":61},{"And":[[500],null]},"Eof",{"And":[[502],null]},{"Token":39},{"Not":504},{"Not":505},{"And":[[506],null]},{"Or":[501,503,507]},{"And":[[49,33,508],null]},{"Or":[509]},{"Rep":510},{"And":[[499,511],1]},{"Or":[512]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[515],null]},{"Enter":[1,41]},{"And":[[517],null]},{"Token":60},{"And":[[519,45],null]},{"Or":[520]},{"Token":60},{"Opt":522},{"And":[[523,45],null]},{"Or":[524]},{"Token":60},{"And":[[526,45],null]},{"Or":[527]},{"And":[[41,528],null]},{"Or":[529]},{"Token":5},{"And":[[49,531,49],null]},{"Or":[532]},{"Call":[150,[[5,533]]]},{"Token":60},{"And":[[534,535,45],null]},{"Or":[536]},{"Token":92},{"And":[[538],null]},{"Token":18},{"And":[[540],null]},{"Token":19},{"And":[[542],null]},{"Or":[539,541,543]},{"And":[[46],null]},{"IsIn":3},{"And":[[546,47],null]},{"Or":[545,547]},{"Opt":548},{"And":[[544,549],null]},{"Or":[550]},{"IsIn":3},{"And":[[552],null]},{"IsIn":1},{"Token":60},{"And":[[554,555],null]},{"Or":[553,556]},{"Token":87},{"Call":[147,[[2,558]]]},{"Token":92},{"Token":53},{"And":[[560,561],null]},{"Or":[562]},{"Not":563},{"And":[[564,49],null]},{"Or":[565]},{"Call":[147,[[2,566]]]},{"Token":92},{"Token":53},{"And":[[568,569,49],null]},{"Or":[570]},{"Call":[147,[[2,571]]]},{"And":[[559,567,572],null]},{"Or":[573]},{"Call":[150,[[5,574]]]},{"And":[[557,575],null]},{"Or":[576]},{"Call":[147,[[2,49]]]},{"Call":[149,[[4,578]]]},{"Opt":9},{"And":[[579,580],null]},{"Or":[581]},{"Token":5},{"Token":92},{"And":[[583,584],null]},{"Or":[585]},{"Token":73},{"And":[[587,34],null]},{"Or":[588]},{"Rep":589},{"And":[[50,590],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[601]},{"And":[[119,49],null]},{"Or":[603]},{"Token":67},{"Token":21},{"And":[[606],null]},{"Token":27},{"And":[[608],null]},{"Or":[607,609]},{"And":[[605,610,49],null]},{"Or":[611]},{"Token":81},{"And":[[613],null]},{"Or":[614]},{"Token":37},{"Token":38},{"And":[[616,617],null]},{"Or":[618]},{"Opt":57},{"And":[[49,620],null]},{"Or":[621]},{"Call":[149,[[4,622]]]},{"And":[[623],null]},{"Or":[624]},{"Token":61},{"Call":[147,[[2,49]]]},{"And":[[626,627],null]},{"Or":[628]},{"Token":82},{"And":[[630],null]},{"Or":[631]},{"Token":58},{"And":[[633,74],null]},{"Or":[634]},{"Opt":635},{"And":[[49,636],null]},{"Or":[637]},{"Call":[151,[[6,638]]]},{"And":[[639],null]},{"Or":[640]},{"Opt":10},{"Token":8},{"Call":[147,[[2,61]]]},{"Call":[149,[[4,644]]]},{"Opt":9},{"And":[[642,643,645,646],2]},{"Or":[647]},{"Token":59},{"And":[[62,649],null]},{"Or":[650]},{"Opt":651},{"And":[[652,49],null]},{"Or":[653]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":81},{"And":[[662],null]},{"Or":[663]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[665,666]},{"Opt":667},{"And":[[40,668],null]},{"Or":[669]},{"Call":[147,[[2,62]]]},{"Token":63},{"Token":61},{"Opt":673},{"And":[[672,674],null]},{"Or":[675]},{"Opt":676},{"And":[[671,677],null]},{"Or":[678]},{"Call":[149,[[4,679]]]},{"And":[[680],null]},{"Or":[681]},{"Call":[147,[[2,67]]]},{"Token":63},{"Token":61},{"Opt":685},{"And":[[684,686],null]},{"Or":[687]},{"Opt":688},{"And":[[683,689],null]},{"Or":[690]},{"Call":[148,[[3,691]]]},{"And":[[692],null]},{"Or":[693]},{"Token":59},{"Not":695},{"And":[[68,696],null]},{"Token":92},{"Token":59},{"And":[[698,699,62],2]},{"Or":[697,700]},{"Token":28},{"Opt":702},{"Token":27},{"Opt":704},{"Token":92},{"And":[[703,705,706],null]},{"Or":[707]},{"And":[[78],null]},{"Or":[709]},{"Token":37},{"Token":38},{"And":[[711,712],null]},{"Or":[713]},{"Opt":72},{"And":[[62,715],null]},{"Or":[716]},{"Call":[149,[[4,717]]]},{"And":[[718],null]},{"Or":[719]},{"Token":61},{"Call":[147,[[2,62]]]},{"And":[[721,722],null]},{"Or":[723]},{"Token":77},{"Token":27},{"Opt":726},{"And":[[725,727,62],null]},{"Or":[728]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":732},{"And":[[730,731,733,112],null]},{"Or":[734]},{"IsIn":2},{"Not":76},{"And":[[736,737],null]},{"IsIn":2},{"Not":739},{"And":[[740],null]},{"Or":[738,741]},{"And":[[742,112],null]},{"Or":[743]},{"Token":62},{"Token":92},{"And":[[746],null]},{"Token":89},{"And":[[748],null]},{"Or":[747,749]},{"And":[[745,750],null]},{"Or":[751]},{"Call":[151,[[6,74]]]},{"Token":83},{"Token":5},{"And":[[755,49],null]},{"Or":[756]},{"Token":67},{"Token":75},{"Token":82},{"Token":67},{"And":[[761],null]},{"Token":69},{"And":[[763],null]},{"Token":71},{"And":[[765],null]},{"Or":[762,764,766]},{"Call":[126,[[1,767]]]},{"Token":73},{"And":[[769],null]},{"Token":75},{"And":[[771],null]},{"Or":[770,772]},{"Call":[126,[[1,773]]]},{"ContextualToken":[45,"<<"]},{"And":[[775],null]},{"ContextualToken":[47,">>"]},{"And":[[777],null]},{"Or":[776,778]},{"Call":[126,[[1,779]]]},{"Token":77},{"Token":77},{"Not":782},{"And":[[781,783],null]},{"Or":[784]},{"Call":[126,[[1,785]]]},{"Token":84},{"Call":[126,[[1,787]]]},{"Token":79},{"Token":79},{"Not":790},{"And":[[789,791],null]},{"Or":[792]},{"Call":[126,[[1,793]]]},{"Call":[126,[[1,131]]]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,796]]]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,798]]]},{"Call":[126,[[1,139]]]},{"Token":53},{"And":[[801],null]},{"Token":74},{"And":[[803],null]},{"Token":76},{"And":[[805],null]},{"Token":68},{"And":[[807],null]},{"Token":70},{"And":[[809],null]},{"Token":72},{"And":[[811],null]},{"Token":78},{"And":[[813],null]},{"Token":80},{"And":[[815],null]},{"Token":85},{"And":[[817],null]},{"ContextualToken":[48,">>="]},{"And":[[819],null]},{"ContextualToken":[46,"<<="]},{"And":[[821],null]},{"Or":[802,804,806,808,810,812,814,816,818,820,822]},{"Call":[126,[[1,823]]]},{"And":[[78],null]},{"Token":92},{"And":[[826],null]},{"Token":18},{"And":[[828],null]},{"Token":19},{"And":[[830],null]},{"Token":41},{"And":[[832],null]},{"Token":60},{"And":[[834],null]},{"Token":37},{"And":[[836],null]},{"Token":43},{"And":[[838],null]},{"Token":79},{"And":[[840],null]},{"Token":31},{"And":[[842],null]},{"Token":39},{"And":[[844],null]},{"Token":14},{"And":[[846],null]},{"Token":25},{"And":[[848],null]},{"Token":24},{"And":[[850],null]},{"Token":23},{"And":[[852],null]},{"Token":30},{"And":[[854],null]},{"Token":77},{"And":[[856],null]},{"Token":67},{"And":[[858],null]},{"Token":75},{"And":[[860],null]},{"Token":82},{"And":[[862],null]},{"Token":63},{"And":[[864],null]},{"Token":64},{"And":[[866],null]},{"PrevIs":[161,167,168,169,170,171,174]},{"And":[[868],null]},{"Var":0},{"Exit":[2,870]},{"Exit":[0,871]},{"And":[[872],null]},{"Token":89},{"And":[[874],null]},{"Token":90},{"And":[[876],null]},{"Token":91},{"And":[[878],null]},{"Token":86},{"And":[[880],null]},{"Token":88},{"And":[[882],null]},{"Or":[875,877,879,881,883]},{"Token":92},{"Token":82},{"And":[[885,886],null]},{"Or":[887]},{"Not":888},{"Opt":80},{"And":[[889,40,890],null]},{"Or":[891]},{"IsIn":0},{"Not":893},{"Call":[147,[[2,81]]]},{"Token":63},{"Call":[77,[[0,74]]]},{"And":[[896,897],null]},{"Or":[898]},{"Opt":899},{"And":[[895,900],null]},{"Or":[901]},{"Call":[148,[[3,902]]]},{"And":[[894,903],null]},{"Or":[904]},{"Token":92},{"Token":59},{"And":[[907,74],null]},{"Or":[908]},{"Opt":909},{"And":[[906,910],1]},{"Or":[911]},{"Token":37},{"Token":38},{"And":[[913,914],null]},{"Or":[915]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[917,918],null]},{"Or":[919]},{"Call":[149,[[4,920]]]},{"And":[[921],null]},{"Or":[922]},{"Token":61},{"Call":[77,[[0,74]]]},{"Call":[147,[[2,925]]]},{"And":[[924,926],null]},{"Or":[927]},{"Call":[147,[[2,74]]]},{"Call":[77,[[0,929]]]},{"Call":[151,[[6,930]]]},{"And":[[931],null]},{"Or":[932]},{"Token":26},{"Opt":934},{"Token":79},{"Rep":87},{"Token":79},{"Token":51},{"And":[[939,49,90],null]},{"Call":[77,[[0,74]]]},{"And":[[941],null]},{"Or":[940,942]},{"And":[[935,936,937,938,943],null]},{"Or":[944]},{"Token":61},{"And":[[946],null]},{"Token":79},{"Not":948},{"Not":949},{"And":[[950],null]},{"Or":[947,951]},{"And":[[13,952],1]},{"Token":31},{"Opt":74},{"And":[[954,955],null]},{"Or":[956]},{"Token":33},{"And":[[958],null]},{"Token":32},{"And":[[960],null]},{"Or":[959,961]},{"Token":87},{"Opt":963},{"And":[[962,964],null]},{"Or":[965]},{"Token":35},{"Opt":967},{"Rep":91},{"Opt":74},{"And":[[969,970],null]},{"Or":[971]},{"Call":[148,[[3,972]]]},{"And":[[968,973],null]},{"Or":[974]},{"Call":[77,[[0,975]]]},{"And":[[976],null]},{"Or":[977]},{"And":[[92],null]},{"And":[[96],null]},{"And":[[95],null]},{"And":[[3],null]},{"Token":9},{"Opt":93},{"Opt":94},{"Token":58},{"And":[[983,62,984,985,986],1]},{"Or":[987]},{"Token":59},{"And":[[989,49],null]},{"Or":[990]},{"Token":53},{"And":[[992,74],null]},{"Or":[993]},{"Token":58},{"And":[[995],null]},{"Or":[996]},"Eof",{"Not":998},{"And":[[76,999],null]},{"Token":58},{"And":[[1001],null]},{"Or":[1000,1002]},{"And":[[74,1003],null]},{"Or":[1004]},{"Enter":[2,1005]},{"And":[[1006],null]},{"Or":[1007]},{"Token":14},{"Token":15},{"And":[[90],null]},{"And":[[97],null]},{"Or":[1011,1012]},{"And":[[1010,1013],null]},{"Or":[1014]},{"Opt":1015},{"And":[[1009,99,90,1016],1]},{"Or":[1017]},{"Opt":103},{"Token":25},{"And":[[1019,1020,99,90],2]},{"Or":[1021]},{"Token":9},{"Token":53},{"And":[[1023,62,1024],1]},{"Or":[1025]},{"Opt":1026},{"And":[[1027,100],null]},{"Enter":[0,74]},{"And":[[1029],null]},{"Opt":103},{"Token":24},{"And":[[1031,1032,90],2]},{"Or":[1033]},{"Opt":103},{"Token":23},{"Token":34},{"And":[[1035,1036,62,1037,100,90],2]},{"Or":[1038]},{"Token":87},{"Token":59},{"And":[[1040,1041],null]},{"Token":30},{"Rep":105},{"Call":[148,[[3,1044]]]},{"And":[[1043,100,1045],1]},{"Or":[1046]},{"Token":52},{"Enter":[2,74]},{"Token":61},{"And":[[1050],null]},"Eof",{"And":[[1052],null]},{"And":[[76],null]},{"Or":[1051,1053,1054]},{"And":[[106,1048,1049,1055],1]},{"Or":[1056]},{"Token":79},{"And":[[1058,62],null]},{"Or":[1059]},{"Rep":1060},{"Opt":107},{"And":[[62,1061,1062],null]},{"Token":14},{"And":[[1064,74],null]},{"Or":[1065]},{"And":[[144],null]},{"Or":[1067]},{"And":[[145],null]},{"Or":[1069]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":1073},{"And":[[1071,1072,1074,112],null]},{"Or":[1075]},{"And":[[74,1076],null]},{"Or":[1077]},{"IsIn":2},{"Not":76},{"And":[[1079,1080],null]},{"IsIn":2},{"Not":1082},{"And":[[1083],null]},{"Or":[1081,1084]},{"And":[[1085,112],null]},{"Or":[1086]},{"And":[[74,1087],null]},{"Or":[1088]},{"Call":[147,[[2,113]]]},{"Call":[149,[[4,1090]]]},{"Call":[77,[[0,1091]]]},{"And":[[1092],null]},{"And":[[74],null]},{"Or":[1094]},{"Token":62},{"Token":92},{"And":[[1097],null]},{"Token":89},{"And":[[1099],null]},{"Or":[1098,1100]},{"And":[[1096,1101],null]},{"Or":[1102]},{"And":[[74,1103],null]},{"Or":[1104]},{"Call":[151,[[6,74]]]},{"And":[[74,1106],null]},{"Or":[1107]},{"Token":83},{"And":[[74,1109],null]},{"Or":[1110]},{"Token":5},{"And":[[1112,49],null]},{"Or":[1113]},{"And":[[74,1114],null]},{"Or":[1115]},{"And":[[119,74],null]},{"Or":[1117]},{"Token":77},{"Token":87},{"Opt":1120},{"Token":27},{"Opt":1122},{"And":[[1119,1121,1123],null]},{"Token":67},{"And":[[1125,74],null]},{"Or":[1126]},{"Token":75},{"And":[[1128,74],null]},{"Or":[1129]},{"Token":82},{"And":[[1131,74],null]},{"Or":[1132]},{"Token":67},{"And":[[1134],null]},{"Token":69},{"And":[[1136],null]},{"Token":71},{"And":[[1138],null]},{"Or":[1135,1137,1139]},{"Call":[126,[[1,1140]]]},{"And":[[74,1141,74],null]},{"Or":[1142]},{"Token":73},{"And":[[1144],null]},{"Token":75},{"And":[[1146],null]},{"Or":[1145,1147]},{"Call":[126,[[1,1148]]]},{"And":[[74,1149,74],null]},{"Or":[1150]},{"ContextualToken":[45,"<<"]},{"And":[[1152],null]},{"ContextualToken":[47,">>"]},{"And":[[1154],null]},{"Or":[1153,1155]},{"Call":[126,[[1,1156]]]},{"And":[[74,1157,74],null]},{"Or":[1158]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1160,1161,1162],null]},{"IsIn":2},{"Not":1164},{"Var":1},{"And":[[1165,1166],null]},{"Token":77},{"Token":77},{"Not":1169},{"And":[[1168,1170],null]},{"Or":[1171]},{"Call":[126,[[1,1172]]]},{"And":[[74,1173,74],null]},{"Or":[1174]},{"Token":84},{"Call":[126,[[1,1176]]]},{"And":[[74,1177,74],null]},{"Or":[1178]},{"Token":79},{"Token":79},{"Not":1181},{"And":[[1180,1182],null]},{"Or":[1183]},{"Call":[126,[[1,1184]]]},{"And":[[74,1185,74],null]},{"Or":[1186]},{"Call":[126,[[1,131]]]},{"And":[[74,1188,74],null]},{"Or":[1189]},{"Token":54},{"And":[[1191],null]},{"Token":55},{"And":[[1193],null]},{"Token":41},{"And":[[1195],null]},{"Token":42},{"And":[[1197],null]},{"Token":57},{"And":[[1199],null]},{"Token":56},{"And":[[1201],null]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,1203]]]},{"And":[[74,1204,74],null]},{"Or":[1205]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,1207]]]},{"And":[[74,1208,74],null]},{"Or":[1209]},{"Call":[126,[[1,139]]]},{"And":[[74,1211,74],null]},{"Or":[1212]},{"And":[[139,74],null]},{"Or":[1214]},{"And":[[74,138],null]},{"Or":[1216]},{"Token":63},{"Not":75},{"And":[[1218,1219],null]},{"Or":[1220]},{"Token":63},{"And":[[1222],null]},{"Token":64},{"And":[[1224],null]},{"Not":75},{"Not":1226},{"Token":39},{"IsIn":0},{"And":[[1228,1229],null]},{"Or":[1230]},{"Not":1231},{"And":[[138,1227,1232],null]},{"Token":53},{"And":[[1234],null]},{"Token":74},{"And":[[1236],null]},{"Token":76},{"And":[[1238],null]},{"Token":68},{"And":[[1240],null]},{"Token":70},{"And":[[1242],null]},{"Token":72},{"And":[[1244],null]},{"Token":78},{"And":[[1246],null]},{"Token":80},{"And":[[1248],null]},{"Token":85},{"And":[[1250],null]},{"ContextualToken":[48,">>="]},{"And":[[1252],null]},{"ContextualToken":[46,"<<="]},{"And":[[1254],null]},{"Or":[1235,1237,1239,1241,1243,1245,1247,1249,1251,1253,1255]},{"Call":[126,[[1,1256]]]},{"And":[[74,1257,74],null]},{"Or":[1258]},{"Token":65},{"Call":[147,[[2,143]]]},{"Call":[151,[[6,1261]]]},{"And":[[1260,1262],null]},{"Or":[1263]},{"Rep":141},{"And":[[1265],null]},{"Token":92},{"Token":53},{"And":[[1268,74],null]},{"Call":[147,[[2,143]]]},{"Call":[149,[[4,1270]]]},{"And":[[1271],null]},{"Or":[1269,1272]},{"Opt":1273},{"And":[[1267,1274],1]},{"Or":[1275]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1279},{"Rep":146},{"Call":[148,[[3,1281]]]},{"And":[[1277,1278,1280,1282],null]},{"Or":[1283]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1287},{"Token":37},{"Rep":146},{"Token":38},{"And":[[1289,1290,1291],null]},{"Token":43},{"Rep":146},{"Token":44},{"And":[[1293,1294,1295],null]},{"Or":[1292,1296]},{"And":[[1285,1286,1288,1297],null]},{"Or":[1298]},{"Token":37},{"And":[[1300],null]},{"Token":38},{"And":[[1302],null]},{"Token":39},{"And":[[1304],null]},{"Token":40},{"And":[[1306],null]},{"Token":43},{"And":[[1308],null]},{"Token":44},{"And":[[1310],null]},{"Or":[1301,1303,1305,1307,1309,1311]},{"Not":1312},"Any",{"And":[[1313,1314],null]},{"Token":37},{"Rep":146},{"Token":38},{"And":[[1316,1317,1318],null]},{"Token":43},{"Rep":146},{"Token":44},{"And":[[1320,1321,1322],null]},{"Token":39},{"Rep":146},{"Token":40},{"And":[[1324,1325,1326],null]},{"Or":[1315,1319,1323,1327]},{"Var":2},"Eof",{"And":[[1330],null]},{"Token":61},{"And":[[1332],null]},{"Or":[1331,1333]},{"And":[[1329,1334],1]},{"Or":[1335]},{"Rep":1336},{"And":[[1337],null]},{"Token":39},{"Token":40},{"Var":3},{"Call":[152,[[7,1339],[8,1340],[9,1341]]]},{"And":[[1342],null]},{"Token":37},{"Token":38},{"Var":4},{"Call":[152,[[7,1344],[8,1345],[9,1346]]]},{"And":[[1347],null]},{"Token":41},{"Token":42},{"Var":5},{"Call":[152,[[7,1349],[8,1350],[9,1351]]]},{"And":[[1352],null]},{"Token":43},{"Token":44},{"Var":6},{"Call":[152,[[7,1354],[8,1355],[9,1356]]]},{"And":[[1357],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[153,[[10,1360],[11,1361]]]},{"Var":9},{"Layer":[1362,1363]},{"Var":8},{"And":[[1359,1364,1365],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[153,[[10,1368],[11,1369]]]},{"Var":11},{"And":[[1367,1370,1371],1]},{"Var":11},{"Not":1373},"Any",{"And":[[1374,1375],null]},{"Or":[1376]},{"And":[[1377],null]},{"Or":[1372,1378]},{"Rep":1379},{"And":[[1380],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, CONTINUE, BREAK, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, LOOP_CF_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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
                        CONTINUE => NodeTypeInfo { name: "CONTINUE", whitespace_like: false },
                        BREAK => NodeTypeInfo { name: "BREAK", whitespace_like: false },
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
                        POINTER_TYPE => NodeTypeInfo { name: "POINTER_TYPE", whitespace_like: false },
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
                        LOOP_CF_EXPR => NodeTypeInfo { name: "LOOP_CF_EXPR", whitespace_like: false },
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
pub struct TypeDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TypeDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TYPE_DEF {
            Some(TypeDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TypeDef<'f> {

}

impl<'f> ::std::fmt::Debug for TypeDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TypeDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeParameters<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TypeParameters<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TYPE_PARAMETERS {
            Some(TypeParameters { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TypeParameters<'f> {
    pub fn lifetime_parameters(&self) -> rt::AstChildren<'f, LifetimeParameter<'f>> {
        rt::AstChildren::new(self.node().children())
    }
    pub fn type_parameters(&self) -> rt::AstChildren<'f, TypeParameter<'f>> {
        rt::AstChildren::new(self.node().children())
    }
}

impl<'f> ::std::fmt::Debug for TypeParameters<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TypeParameters@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeParameter<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TypeParameter<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TYPE_PARAMETER {
            Some(TypeParameter { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TypeParameter<'f> {

}

impl<'f> ::std::fmt::Debug for TypeParameter<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TypeParameter@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifetimeParameter<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for LifetimeParameter<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == LIFETIME_PARAMETER {
            Some(LifetimeParameter { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> LifetimeParameter<'f> {
    pub fn lifetime(&self) -> Text<'f> {
        rt::child_of_type_exn(self.node(), LIFETIME).text()
    }
}

impl<'f> ::std::fmt::Debug for LifetimeParameter<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("LifetimeParameter@")?;
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
impl<'f> NameOwner<'f> for TypeDef<'f> {}
impl<'f> NameOwner<'f> for TypeParameter<'f> {}
pub trait TypeParametersOwner<'f>: rt::AstNode<'f> {
    fn type_parameters(&self) -> Option<TypeParameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}
impl<'f> TypeParametersOwner<'f> for StructDef<'f> {}
impl<'f> TypeParametersOwner<'f> for EnumDef<'f> {}