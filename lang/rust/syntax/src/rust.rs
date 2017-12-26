use fall_parse::runtime as rt;
pub use self::rt::ERROR;

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
pub const TYPE_REFERENCE: rt::NodeType = rt::NodeType(227);
pub const PATH_TYPE: rt::NodeType = rt::NodeType(228);
pub const REFERENCE_TYPE: rt::NodeType = rt::NodeType(229);
pub const POINTER_TYPE: rt::NodeType = rt::NodeType(230);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(231);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(232);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(233);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(234);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(235);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(236);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(237);
pub const FOR_TYPE: rt::NodeType = rt::NodeType(238);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(239);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(240);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(241);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(242);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(243);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(244);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(245);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(246);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(247);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(248);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(249);
pub const EXPR: rt::NodeType = rt::NodeType(250);
pub const LITERAL: rt::NodeType = rt::NodeType(251);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(252);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(253);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(254);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(255);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(256);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(257);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(258);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(259);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(260);
pub const LOOP_CF_EXPR: rt::NodeType = rt::NodeType(261);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(262);
pub const LET_STMT: rt::NodeType = rt::NodeType(263);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(264);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(265);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(266);
pub const IF_EXPR: rt::NodeType = rt::NodeType(267);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(268);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(269);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(270);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(271);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(272);
pub const GUARD: rt::NodeType = rt::NodeType(273);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(274);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(275);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(276);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(277);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(278);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(279);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(280);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(281);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(282);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(283);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(284);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(285);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(286);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(287);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(288);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(289);
pub const BIT_AND: rt::NodeType = rt::NodeType(290);
pub const BIT_XOR: rt::NodeType = rt::NodeType(291);
pub const BIT_OR: rt::NodeType = rt::NodeType(292);
pub const COMPARISON: rt::NodeType = rt::NodeType(293);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(294);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(295);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(296);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(297);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(298);
pub const INNER_ATTRIBUTE: rt::NodeType = rt::NodeType(299);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(300);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(301);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(302);
pub const TT: rt::NodeType = rt::NodeType(303);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(LINE_COMMENT, "//.*\\n?", None),
            rt::LexRule::new(BLOCK_COMMENT, "/\\*", Some(parse_block_comment)),
            rt::LexRule::new(AS, "as", None),
            rt::LexRule::new(CRATE, "crate", None),
            rt::LexRule::new(EXTERN, "extern", None),
            rt::LexRule::new(FN, "fn", None),
            rt::LexRule::new(LET, "let", None),
            rt::LexRule::new(PUB, "pub", None),
            rt::LexRule::new(STRUCT, "struct", None),
            rt::LexRule::new(USE, "use", None),
            rt::LexRule::new(MOD, "mod", None),
            rt::LexRule::new(IF, "if", None),
            rt::LexRule::new(ELSE, "else", None),
            rt::LexRule::new(ENUM, "enum", None),
            rt::LexRule::new(IMPL, "impl", None),
            rt::LexRule::new(SELF, "self", None),
            rt::LexRule::new(SUPER, "super", None),
            rt::LexRule::new(TYPE, "type", None),
            rt::LexRule::new(CONST, "const", None),
            rt::LexRule::new(STATIC, "static", None),
            rt::LexRule::new(FOR, "for", None),
            rt::LexRule::new(LOOP, "loop", None),
            rt::LexRule::new(WHILE, "while", None),
            rt::LexRule::new(MOVE, "move", None),
            rt::LexRule::new(MUT, "mut", None),
            rt::LexRule::new(REF, "ref", None),
            rt::LexRule::new(TRAIT, "trait", None),
            rt::LexRule::new(MATCH, "match", None),
            rt::LexRule::new(RETURN, "return", None),
            rt::LexRule::new(CONTINUE, "continue", None),
            rt::LexRule::new(BREAK, "break", None),
            rt::LexRule::new(IN, "in", None),
            rt::LexRule::new(UNSAFE, "unsafe", None),
            rt::LexRule::new(WHERE, "where", None),
            rt::LexRule::new(L_PAREN, "\\(", None),
            rt::LexRule::new(R_PAREN, "\\)", None),
            rt::LexRule::new(L_CURLY, "\\{", None),
            rt::LexRule::new(R_CURLY, "\\}", None),
            rt::LexRule::new(L_ANGLE, "<", None),
            rt::LexRule::new(R_ANGLE, ">", None),
            rt::LexRule::new(L_BRACK, "\\[", None),
            rt::LexRule::new(R_BRACK, "\\]", None),
            rt::LexRule::new(THIN_ARROW, "\\->", None),
            rt::LexRule::new(FAT_ARROW, "=>", None),
            rt::LexRule::new(EQ, "=", None),
            rt::LexRule::new(EQEQ, "==", None),
            rt::LexRule::new(BANGEQ, "!=", None),
            rt::LexRule::new(GTET, ">=", None),
            rt::LexRule::new(LTEQ, "<=", None),
            rt::LexRule::new(SEMI, ";", None),
            rt::LexRule::new(COLON, ":", None),
            rt::LexRule::new(COLONCOLON, "::", None),
            rt::LexRule::new(COMMA, ",", None),
            rt::LexRule::new(DOT, "\\.", None),
            rt::LexRule::new(DOTDOT, "\\.\\.", None),
            rt::LexRule::new(DOTDOTDOT, "\\.\\.\\.", None),
            rt::LexRule::new(HASH, "\\#", None),
            rt::LexRule::new(DOLLAR, "\\$", None),
            rt::LexRule::new(STAR, "\\*", None),
            rt::LexRule::new(STAR_EQ, "\\*=", None),
            rt::LexRule::new(SLASH, "/", None),
            rt::LexRule::new(SLASH_EQ, "/=", None),
            rt::LexRule::new(PERCENT, "%", None),
            rt::LexRule::new(PERCENT_EQ, "%=", None),
            rt::LexRule::new(PLUS, "\\+", None),
            rt::LexRule::new(PLUS_EQ, "\\+=", None),
            rt::LexRule::new(MINUS, "\\-", None),
            rt::LexRule::new(MINUS_EQ, "\\-=", None),
            rt::LexRule::new(AMPERSAND, "\\&", None),
            rt::LexRule::new(AMPERSAND_EQ, "\\&=", None),
            rt::LexRule::new(PIPE, "\\|", None),
            rt::LexRule::new(PIPE_EQ, "\\|=", None),
            rt::LexRule::new(UNDERSCORE, "_", None),
            rt::LexRule::new(BANG, "!", None),
            rt::LexRule::new(QUESTION, "\\?", None),
            rt::LexRule::new(CARET, "\\^", None),
            rt::LexRule::new(CARET_EQ, "\\^=", None),
            rt::LexRule::new(CHAR, "\'[^\']\'|\'\\\\\'\'|\'(\\\\|\\p{XID_Continue}|\\{|\\})*\'", None),
            rt::LexRule::new(LIFETIME, "\'\\p{XID_Continue}*", None),
            rt::LexRule::new(BOOL, "true|false", None),
            rt::LexRule::new(NUMBER, "\\d+", None),
            rt::LexRule::new(STRING, "\"(\\\\\"|[^\"])*\"", None),
            rt::LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
            rt::LexRule::new(IDENT, "(\\p{XID_Start}|_)\\p{XID_Continue}*", None),
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":93,"body":158,"replaceable":false}},{"Or":[161]},{"Or":[163,165,167,169,171,173,175,176]},{"Cached":193},{"Pub":{"ty":94,"body":209,"replaceable":false}},{"Pub":{"ty":95,"body":215,"replaceable":false}},{"Pub":{"ty":96,"body":221,"replaceable":false}},{"Pub":{"ty":97,"body":228,"replaceable":false}},{"Pub":{"ty":98,"body":244,"replaceable":false}},{"Or":[246]},{"Pub":{"ty":99,"body":251,"replaceable":false}},{"Or":[257]},{"Pub":{"ty":100,"body":260,"replaceable":false}},{"Pub":{"ty":101,"body":266,"replaceable":false}},{"Pub":{"ty":102,"body":283,"replaceable":false}},{"Pub":{"ty":103,"body":302,"replaceable":false}},{"Pub":{"ty":104,"body":307,"replaceable":false}},{"Pub":{"ty":105,"body":310,"replaceable":false}},{"Pub":{"ty":106,"body":317,"replaceable":false}},{"Pub":{"ty":107,"body":330,"replaceable":false}},{"Pub":{"ty":108,"body":339,"replaceable":false}},{"Pub":{"ty":109,"body":352,"replaceable":false}},{"Pub":{"ty":110,"body":360,"replaceable":false}},{"Pub":{"ty":111,"body":372,"replaceable":false}},{"Or":[373,374,375,376]},{"Or":[378,380,382,384,386,388,390,392,397]},{"Pub":{"ty":112,"body":407,"replaceable":false}},{"Pub":{"ty":113,"body":421,"replaceable":false}},{"Pub":{"ty":114,"body":425,"replaceable":false}},{"Pub":{"ty":115,"body":429,"replaceable":false}},{"Or":[437]},{"Pub":{"ty":116,"body":444,"replaceable":false}},{"Pub":{"ty":117,"body":448,"replaceable":false}},{"Or":[468]},{"Pub":{"ty":118,"body":473,"replaceable":false}},{"Pub":{"ty":119,"body":493,"replaceable":false}},{"Pub":{"ty":120,"body":503,"replaceable":false}},{"Pub":{"ty":121,"body":518,"replaceable":false}},{"Or":[519]},{"Or":[521]},{"Or":[523]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":122,"op":526,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":122,"body":530,"replaceable":false}},{"Pub":{"ty":122,"body":535,"replaceable":false}},{"Pub":{"ty":123,"body":542,"replaceable":false}},{"Pub":{"ty":124,"body":556,"replaceable":false}},{"Pub":{"ty":125,"body":582,"replaceable":false}},{"Pub":{"ty":126,"body":587,"replaceable":false}},{"Pub":{"ty":127,"body":591,"replaceable":false}},{"Pub":{"ty":128,"body":597,"replaceable":false}},{"Or":[598,599,600,601,602,603,604,605,606,607]},{"Pub":{"ty":129,"body":609,"replaceable":false}},{"Pub":{"ty":130,"body":611,"replaceable":false}},{"Pub":{"ty":131,"body":619,"replaceable":false}},{"Pub":{"ty":132,"body":622,"replaceable":false}},{"Pub":{"ty":133,"body":626,"replaceable":false}},{"Pub":{"ty":134,"body":632,"replaceable":true}},{"PubReplace":{"ty":135,"body":636}},{"Pub":{"ty":136,"body":639,"replaceable":false}},{"Pub":{"ty":137,"body":648,"replaceable":false}},{"Pub":{"ty":138,"body":655,"replaceable":false}},{"Pub":{"ty":100,"body":661,"replaceable":false}},{"Pub":{"ty":139,"body":664,"replaceable":false}},{"Or":[665,666,667,668,669,670,671]},{"Pub":{"ty":140,"body":674,"replaceable":false}},{"Pub":{"ty":141,"body":680,"replaceable":true}},{"PubReplace":{"ty":142,"body":692}},{"PubReplace":{"ty":143,"body":704}},{"Pub":{"ty":144,"body":711,"replaceable":false}},{"Pub":{"ty":145,"body":718,"replaceable":false}},{"Pub":{"ty":146,"body":720,"replaceable":false}},{"Pub":{"ty":147,"body":724,"replaceable":false}},{"Pub":{"ty":148,"body":730,"replaceable":true}},{"PubReplace":{"ty":149,"body":734}},{"Pub":{"ty":150,"body":739,"replaceable":false}},{"Pratt":{"atoms":[79,80,83,84,86,87,89,90,91,98,99,102,103,105,109,110,138],"prefixes":[{"ty":184,"op":120,"priority":999},{"ty":185,"op":768,"priority":999},{"ty":186,"op":769,"priority":999},{"ty":187,"op":770,"priority":999},{"ty":197,"op":140,"priority":2}],"infixes":[{"ty":177,"op":745,"priority":999,"has_rhs":false},{"ty":178,"op":754,"priority":999,"has_rhs":false},{"ty":180,"op":762,"priority":999,"has_rhs":false},{"ty":181,"op":763,"priority":999,"has_rhs":false},{"ty":182,"op":764,"priority":999,"has_rhs":false},{"ty":183,"op":767,"priority":999,"has_rhs":false},{"ty":188,"op":778,"priority":11,"has_rhs":true},{"ty":189,"op":784,"priority":10,"has_rhs":true},{"ty":190,"op":790,"priority":9,"has_rhs":true},{"ty":191,"op":796,"priority":8,"has_rhs":true},{"ty":192,"op":798,"priority":7,"has_rhs":true},{"ty":193,"op":804,"priority":6,"has_rhs":true},{"ty":194,"op":805,"priority":5,"has_rhs":true},{"ty":195,"op":807,"priority":4,"has_rhs":true},{"ty":196,"op":809,"priority":3,"has_rhs":true},{"ty":197,"op":810,"priority":2,"has_rhs":true},{"ty":197,"op":139,"priority":2,"has_rhs":false},{"ty":198,"op":834,"priority":1,"has_rhs":true}]}},{"Or":[835,837,839,841,843,845,847,849,851,853,855,857,859,861,863,865,867,869,871,873,875,877,879,881,883,885]},{"Or":[887]},{"Or":[891]},{"Pub":{"ty":152,"body":902,"replaceable":false}},{"Pub":{"ty":153,"body":910,"replaceable":true}},{"PubReplace":{"ty":154,"body":923}},{"Pub":{"ty":155,"body":930,"replaceable":false}},{"Pub":{"ty":156,"body":934,"replaceable":false}},{"Pub":{"ty":157,"body":941,"replaceable":true}},{"PubReplace":{"ty":158,"body":946}},{"Pub":{"ty":159,"body":959,"replaceable":false}},{"Pub":{"ty":160,"body":971,"replaceable":false}},{"Or":[979]},{"Pub":{"ty":161,"body":983,"replaceable":false}},{"Pub":{"ty":162,"body":992,"replaceable":false}},{"Pub":{"ty":163,"body":1002,"replaceable":false}},{"Or":[1003,1004,1005,1006]},{"Or":[1008,1010,1011,1012]},{"Pub":{"ty":164,"body":1021,"replaceable":false}},{"Pub":{"ty":165,"body":1024,"replaceable":false}},{"Pub":{"ty":166,"body":1027,"replaceable":false}},{"Pub":{"ty":167,"body":1038,"replaceable":false}},{"Pub":{"ty":168,"body":1048,"replaceable":false}},{"Pub":{"ty":169,"body":1052,"replaceable":false}},{"Or":[1058]},{"Or":[1060]},{"Pub":{"ty":170,"body":1064,"replaceable":false}},{"Pub":{"ty":171,"body":1069,"replaceable":false}},{"Or":[1072]},{"Pub":{"ty":172,"body":1077,"replaceable":false}},{"Pub":{"ty":173,"body":1087,"replaceable":false}},{"Or":[1093]},{"Pub":{"ty":174,"body":1096,"replaceable":false}},{"Pub":{"ty":175,"body":1098,"replaceable":false}},{"Pub":{"ty":176,"body":1100,"replaceable":false}},{"Pub":{"ty":177,"body":1108,"replaceable":false}},{"Pub":{"ty":178,"body":1119,"replaceable":false}},{"Or":[1123]},{"Pub":{"ty":179,"body":1125,"replaceable":false}},{"Pub":{"ty":180,"body":1135,"replaceable":false}},{"Pub":{"ty":181,"body":1138,"replaceable":false}},{"Pub":{"ty":182,"body":1141,"replaceable":false}},{"Pub":{"ty":183,"body":1146,"replaceable":false}},{"Pub":{"ty":184,"body":1148,"replaceable":false}},{"Or":[1154]},{"Pub":{"ty":185,"body":1157,"replaceable":false}},{"Pub":{"ty":186,"body":1160,"replaceable":false}},{"Pub":{"ty":187,"body":1163,"replaceable":false}},{"Pub":{"ty":188,"body":1173,"replaceable":false}},{"Pub":{"ty":189,"body":1181,"replaceable":false}},{"Pub":{"ty":190,"body":1189,"replaceable":false}},{"Or":[1193,1197]},{"Pub":{"ty":191,"body":1205,"replaceable":false}},{"Pub":{"ty":192,"body":1209,"replaceable":false}},{"Pub":{"ty":193,"body":1217,"replaceable":false}},{"Pub":{"ty":194,"body":1220,"replaceable":false}},{"Or":[1222,1224,1226,1228,1230,1232]},{"Pub":{"ty":195,"body":1236,"replaceable":false}},{"Pub":{"ty":196,"body":1240,"replaceable":false}},{"Pub":{"ty":197,"body":1243,"replaceable":false}},{"Pub":{"ty":197,"body":1245,"replaceable":false}},{"Pub":{"ty":197,"body":1247,"replaceable":false}},{"Pub":{"ty":197,"body":1251,"replaceable":false}},{"Or":[1253,1255]},{"Or":[1263]},{"Pub":{"ty":198,"body":1289,"replaceable":false}},{"Pub":{"ty":199,"body":1292,"replaceable":false}},{"Pub":{"ty":200,"body":1296,"replaceable":false}},{"Or":[1299]},{"Or":[1301]},{"Pub":{"ty":201,"body":1311,"replaceable":false}},{"Pub":{"ty":202,"body":1319,"replaceable":false}},{"Pub":{"ty":203,"body":1334,"replaceable":false}},{"Pub":{"ty":204,"body":1363,"replaceable":false}},{"Or":[1373]},{"Or":[1378]},{"Or":[1383]},{"Or":[1388]},{"Or":[1393]},{"Or":[1401]},{"Or":[1416]},{"And":[[1],null]},{"Or":[157]},{"WithSkip":[2,3]},{"Rep":159},{"And":[[160],null]},{"Token":11},{"And":[[162],null]},{"ContextualToken":[4,"union"]},{"And":[[164],null]},{"Token":16},{"And":[[166],null]},{"Token":12},{"And":[[168],null]},{"Token":13},{"And":[[170],null]},{"Token":17},{"And":[[172],null]},{"Token":29},{"And":[[174],null]},{"And":[[25],null]},{"Opt":36},{"And":[[145,177],null]},{"Or":[178]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[180,181,182,183,184,185,186,187,188]},{"Inject":[179,189]},{"And":[[190],null]},{"And":[[28],null]},{"Or":[191,192]},{"Token":12},{"And":[[48],null]},{"Token":60},{"And":[[196,5],null]},{"Or":[197]},{"Opt":198},{"And":[[199],null]},{"Or":[195,200]},{"And":[[38,201],null]},{"Token":60},{"Opt":203},{"And":[[204,5],null]},{"Or":[202,205]},{"Token":58},{"And":[[194,206,207],1]},{"Or":[208]},{"Token":67},{"And":[[210],null]},{"Call":[150,[[2,6]]]},{"Call":[151,[[3,212]]]},{"And":[[213],null]},{"Or":[211,214]},{"Token":18},{"And":[[216],null]},{"Token":92},{"Opt":48},{"And":[[218,219],1]},{"Or":[217,220]},{"Token":7},{"Token":6},{"Token":92},{"Opt":48},{"Token":58},{"And":[[222,223,224,225,226],2]},{"Or":[227]},{"Token":21},{"Opt":229},{"Token":35},{"Opt":231},{"Opt":10},{"Token":8},{"Token":92},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[91],null]},{"Token":58},{"And":[[240],null]},{"Or":[239,241]},{"And":[[230,232,233,234,235,236,11,237,238,242],4]},{"Or":[243]},{"Token":51},{"And":[[245,49],null]},{"Token":7},{"Token":90},{"Opt":248},{"And":[[247,249],null]},{"Or":[250]},{"Opt":14},{"Call":[150,[[2,12]]]},{"And":[[252,253],null]},{"Or":[254]},{"Call":[152,[[4,255]]]},{"And":[[256],null]},{"Token":59},{"And":[[63,258,49],1]},{"Or":[259]},{"Token":59},{"And":[[261,49],null]},{"Or":[262]},{"Opt":263},{"And":[[63,264],null]},{"Or":[265]},{"And":[[120],null]},{"Token":27},{"And":[[268],null]},{"Or":[267,269]},{"Opt":270},{"Token":18},{"Token":59},{"And":[[273,49],null]},{"Or":[274]},{"Opt":275},{"Token":61},{"And":[[277],null]},"Eof",{"And":[[279],null]},{"Or":[278,280]},{"And":[[271,272,276,281],2]},{"Or":[282]},{"Token":11},{"And":[[284],null]},{"ContextualToken":[4,"union"]},{"And":[[286],null]},{"Or":[285,287]},{"Token":92},{"Opt":31},{"Call":[150,[[2,16]]]},{"Call":[151,[[3,291]]]},{"And":[[292],null]},{"Token":58},{"And":[[294],null]},{"Call":[150,[[2,17]]]},{"Call":[152,[[4,296]]]},{"Token":58},{"And":[[297,298],null]},{"Or":[293,295,299]},{"And":[[288,289,290,300],1]},{"Or":[301]},{"Opt":36},{"Token":92},{"Token":59},{"And":[[303,304,305,49],2]},{"Or":[306]},{"Opt":36},{"And":[[308,49],null]},{"Or":[309]},{"Token":16},{"Token":92},{"Opt":31},{"Call":[150,[[2,19]]]},{"Call":[151,[[3,314]]]},{"And":[[311,312,313,315],1]},{"Or":[316]},{"Token":92},{"Token":53},{"And":[[319,75],null]},{"Call":[150,[[2,17]]]},{"Call":[152,[[4,321]]]},{"And":[[322],null]},{"Call":[150,[[2,16]]]},{"Call":[151,[[3,324]]]},{"And":[[325],null]},{"Or":[320,323,326]},{"Opt":327},{"And":[[318,328],1]},{"Or":[329]},{"Token":13},{"Token":92},{"Token":58},{"And":[[333],null]},{"Call":[151,[[3,1]]]},{"And":[[335],null]},{"Or":[334,336]},{"And":[[331,332,337],1]},{"Or":[338]},{"Token":35},{"Opt":340},{"Token":17},{"Opt":31},{"Token":23},{"And":[[344,49],null]},{"Or":[345]},{"Opt":346},{"And":[[49,347],null]},{"Or":[348]},{"Opt":37},{"And":[[341,342,343,349,350,23],2]},{"Or":[351]},{"Opt":36},{"Token":29},{"Token":92},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[145,353,354,355,356,357,358,23],3]},{"Or":[359]},{"Opt":36},{"And":[[145,361],null]},{"Or":[362]},{"Inject":[363,24]},{"And":[[364],null]},{"And":[[28],null]},{"Or":[365,366]},{"WithSkip":[25,367]},{"Rep":368},{"Call":[151,[[3,369]]]},{"And":[[370],null]},{"Or":[371]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[143],null]},{"Token":10},{"And":[[377],null]},{"Token":8},{"And":[[379],null]},{"Token":20},{"And":[[381],null]},{"Token":21},{"And":[[383],null]},{"Token":22},{"And":[[385],null]},{"Token":35},{"And":[[387],null]},{"Token":65},{"And":[[389],null]},{"Token":7},{"And":[[391],null]},{"Token":92},{"Token":82},{"And":[[393,394],null]},{"Or":[395]},{"And":[[396],null]},{"Token":20},{"Token":92},{"Opt":31},{"Token":53},{"And":[[401,49],null]},{"Or":[402]},{"Opt":403},{"Token":58},{"And":[[398,399,400,404,405],1]},{"Or":[406]},{"Token":21},{"And":[[408],null]},{"Token":22},{"And":[[410],null]},{"Or":[409,411]},{"Token":92},{"Token":59},{"Token":53},{"And":[[415,75],null]},{"Or":[416]},{"Opt":417},{"Token":58},{"And":[[412,413,414,49,418,419],1]},{"Or":[420]},{"And":[[147],null]},{"Token":58},{"And":[[148,423],null]},{"Or":[422,424]},{"Rep":30},{"Call":[151,[[3,426]]]},{"And":[[10,427],null]},{"Or":[428]},{"Opt":36},{"And":[[145,430],null]},{"Or":[431]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[433,434]},{"Inject":[432,435]},{"And":[[436],null]},{"Call":[150,[[2,35]]]},{"Call":[150,[[2,32]]]},{"And":[[438,439],null]},{"Or":[440]},{"Call":[153,[[5,441]]]},{"And":[[442],null]},{"Or":[443]},{"Token":92},{"Opt":33},{"And":[[445,446],1]},{"Or":[447]},{"Token":59},{"Token":73},{"And":[[450],null]},"Eof",{"And":[[452],null]},{"Token":61},{"And":[[454],null]},{"Token":39},{"And":[[456],null]},{"Token":36},{"And":[[458],null]},{"Or":[455,457,459]},{"Not":460},{"Not":461},{"And":[[462],null]},{"Or":[451,453,463]},{"And":[[34,464],1]},{"Or":[465]},{"Rep":466},{"And":[[449,467],null]},{"Token":87},{"And":[[469],null]},{"And":[[51],null]},{"And":[[62],null]},{"Or":[470,471,472]},{"Token":87},{"Token":59},{"Token":87},{"Token":73},{"And":[[477],null]},"Eof",{"And":[[479],null]},{"Token":61},{"Not":481},{"Not":482},{"And":[[483],null]},{"Or":[478,480,484]},{"And":[[476,485],1]},{"Or":[486]},{"Rep":487},{"And":[[475,488],null]},{"Or":[489]},{"Opt":490},{"And":[[474,491],1]},{"Or":[492]},{"Token":10},{"Token":6},{"And":[[495],null]},{"Token":19},{"And":[[497],null]},{"Or":[496,498]},{"Call":[152,[[4,499]]]},{"Opt":500},{"And":[[494,501],null]},{"Or":[502]},{"Token":36},{"Token":61},{"And":[[505],null]},"Eof",{"And":[[507],null]},{"Token":39},{"Not":509},{"Not":510},{"And":[[511],null]},{"Or":[506,508,512]},{"And":[[49,33,513],null]},{"Or":[514]},{"Rep":515},{"And":[[504,516],1]},{"Or":[517]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[520],null]},{"Enter":[1,41]},{"And":[[522],null]},{"Token":60},{"And":[[524,45],null]},{"Or":[525]},{"Token":60},{"Opt":527},{"And":[[528,45],null]},{"Or":[529]},{"Token":60},{"And":[[531,45],null]},{"Or":[532]},{"And":[[41,533],null]},{"Or":[534]},{"Token":5},{"And":[[49,536,49],null]},{"Or":[537]},{"Call":[153,[[5,538]]]},{"Token":60},{"And":[[539,540,45],null]},{"Or":[541]},{"Token":92},{"And":[[543],null]},{"Token":18},{"And":[[545],null]},{"Token":19},{"And":[[547],null]},{"Or":[544,546,548]},{"And":[[46],null]},{"IsIn":3},{"And":[[551,47],null]},{"Or":[550,552]},{"Opt":553},{"And":[[549,554],null]},{"Or":[555]},{"IsIn":3},{"And":[[557],null]},{"IsIn":1},{"Token":60},{"And":[[559,560],null]},{"Or":[558,561]},{"Token":87},{"Call":[150,[[2,563]]]},{"Token":92},{"Token":53},{"And":[[565,566],null]},{"Or":[567]},{"Not":568},{"And":[[569,49],null]},{"Or":[570]},{"Call":[150,[[2,571]]]},{"Token":92},{"Token":53},{"And":[[573,574,49],null]},{"Or":[575]},{"Call":[150,[[2,576]]]},{"And":[[564,572,577],null]},{"Or":[578]},{"Call":[153,[[5,579]]]},{"And":[[562,580],null]},{"Or":[581]},{"Call":[150,[[2,49]]]},{"Call":[152,[[4,583]]]},{"Opt":9},{"And":[[584,585],null]},{"Or":[586]},{"Token":5},{"Token":92},{"And":[[588,589],null]},{"Or":[590]},{"Token":73},{"And":[[592,34],null]},{"Or":[593]},{"Rep":594},{"And":[[50,595],null]},{"Or":[596]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[62],null]},{"And":[[39],null]},{"Or":[608]},{"And":[[120,49],null]},{"Or":[610]},{"Token":67},{"Token":21},{"And":[[613],null]},{"Token":27},{"And":[[615],null]},{"Or":[614,616]},{"And":[[612,617,49],null]},{"Or":[618]},{"Token":81},{"And":[[620],null]},{"Or":[621]},{"Token":37},{"Token":38},{"And":[[623,624],null]},{"Or":[625]},{"Opt":57},{"And":[[49,627],null]},{"Or":[628]},{"Call":[152,[[4,629]]]},{"And":[[630],null]},{"Or":[631]},{"Token":61},{"Call":[150,[[2,49]]]},{"And":[[633,634],null]},{"Or":[635]},{"Token":82},{"And":[[637],null]},{"Or":[638]},{"Token":58},{"And":[[640,75],null]},{"Or":[641]},{"Opt":642},{"And":[[49,643],null]},{"Or":[644]},{"Call":[154,[[6,645]]]},{"And":[[646],null]},{"Or":[647]},{"Opt":10},{"Token":8},{"Call":[150,[[2,61]]]},{"Call":[152,[[4,651]]]},{"Opt":9},{"And":[[649,650,652,653],2]},{"Or":[654]},{"Token":59},{"And":[[63,656],null]},{"Or":[657]},{"Opt":658},{"And":[[659,49],null]},{"Or":[660]},{"Token":23},{"And":[[662,31,49],1]},{"Or":[663]},{"And":[[64],null]},{"And":[[65],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[72],null]},{"And":[[74],null]},{"Token":81},{"And":[[672],null]},{"Or":[673]},{"And":[[66],null]},{"And":[[67],null]},{"Or":[675,676]},{"Opt":677},{"And":[[40,678],null]},{"Or":[679]},{"Call":[150,[[2,63]]]},{"Token":63},{"Token":61},{"Opt":683},{"And":[[682,684],null]},{"Or":[685]},{"Opt":686},{"And":[[681,687],null]},{"Or":[688]},{"Call":[152,[[4,689]]]},{"And":[[690],null]},{"Or":[691]},{"Call":[150,[[2,68]]]},{"Token":63},{"Token":61},{"Opt":695},{"And":[[694,696],null]},{"Or":[697]},{"Opt":698},{"And":[[693,699],null]},{"Or":[700]},{"Call":[151,[[3,701]]]},{"And":[[702],null]},{"Or":[703]},{"Token":59},{"Not":705},{"And":[[69,706],null]},{"Token":92},{"Token":59},{"And":[[708,709,63],2]},{"Or":[707,710]},{"Token":28},{"Opt":712},{"Token":27},{"Opt":714},{"Token":92},{"And":[[713,715,716],null]},{"Or":[717]},{"And":[[79],null]},{"Or":[719]},{"Token":37},{"Token":38},{"And":[[721,722],null]},{"Or":[723]},{"Opt":73},{"And":[[63,725],null]},{"Or":[726]},{"Call":[152,[[4,727]]]},{"And":[[728],null]},{"Or":[729]},{"Token":61},{"Call":[150,[[2,63]]]},{"And":[[731,732],null]},{"Or":[733]},{"Token":77},{"Token":27},{"Opt":736},{"And":[[735,737,63],null]},{"Or":[738]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":742},{"And":[[740,741,743,113],null]},{"Or":[744]},{"IsIn":2},{"Not":77},{"And":[[746,747],null]},{"IsIn":2},{"Not":749},{"And":[[750],null]},{"Or":[748,751]},{"And":[[752,113],null]},{"Or":[753]},{"Token":62},{"Token":92},{"And":[[756],null]},{"Token":89},{"And":[[758],null]},{"Or":[757,759]},{"And":[[755,760],null]},{"Or":[761]},{"Call":[154,[[6,75]]]},{"Token":83},{"Token":5},{"And":[[765,49],null]},{"Or":[766]},{"Token":67},{"Token":75},{"Token":82},{"Token":67},{"And":[[771],null]},{"Token":69},{"And":[[773],null]},{"Token":71},{"And":[[775],null]},{"Or":[772,774,776]},{"Call":[127,[[1,777]]]},{"Token":73},{"And":[[779],null]},{"Token":75},{"And":[[781],null]},{"Or":[780,782]},{"Call":[127,[[1,783]]]},{"ContextualToken":[45,"<<"]},{"And":[[785],null]},{"ContextualToken":[47,">>"]},{"And":[[787],null]},{"Or":[786,788]},{"Call":[127,[[1,789]]]},{"Token":77},{"Token":77},{"Not":792},{"And":[[791,793],null]},{"Or":[794]},{"Call":[127,[[1,795]]]},{"Token":84},{"Call":[127,[[1,797]]]},{"Token":79},{"Token":79},{"Not":800},{"And":[[799,801],null]},{"Or":[802]},{"Call":[127,[[1,803]]]},{"Call":[127,[[1,132]]]},{"ContextualToken":[49,"&&"]},{"Call":[127,[[1,806]]]},{"ContextualToken":[50,"||"]},{"Call":[127,[[1,808]]]},{"Call":[127,[[1,140]]]},{"Token":53},{"And":[[811],null]},{"Token":74},{"And":[[813],null]},{"Token":76},{"And":[[815],null]},{"Token":68},{"And":[[817],null]},{"Token":70},{"And":[[819],null]},{"Token":72},{"And":[[821],null]},{"Token":78},{"And":[[823],null]},{"Token":80},{"And":[[825],null]},{"Token":85},{"And":[[827],null]},{"ContextualToken":[48,">>="]},{"And":[[829],null]},{"ContextualToken":[46,"<<="]},{"And":[[831],null]},{"Or":[812,814,816,818,820,822,824,826,828,830,832]},{"Call":[127,[[1,833]]]},{"And":[[79],null]},{"Token":92},{"And":[[836],null]},{"Token":87},{"And":[[838],null]},{"Token":18},{"And":[[840],null]},{"Token":19},{"And":[[842],null]},{"Token":41},{"And":[[844],null]},{"Token":60},{"And":[[846],null]},{"Token":37},{"And":[[848],null]},{"Token":43},{"And":[[850],null]},{"Token":79},{"And":[[852],null]},{"Token":31},{"And":[[854],null]},{"Token":39},{"And":[[856],null]},{"Token":14},{"And":[[858],null]},{"Token":25},{"And":[[860],null]},{"Token":24},{"And":[[862],null]},{"Token":23},{"And":[[864],null]},{"Token":30},{"And":[[866],null]},{"Token":26},{"And":[[868],null]},{"Token":32},{"And":[[870],null]},{"Token":33},{"And":[[872],null]},{"Token":77},{"And":[[874],null]},{"Token":67},{"And":[[876],null]},{"Token":75},{"And":[[878],null]},{"Token":82},{"And":[[880],null]},{"Token":63},{"And":[[882],null]},{"Token":64},{"And":[[884],null]},{"PrevIs":[163,168,169,170,171,172,175]},{"And":[[886],null]},{"Var":0},{"Exit":[2,888]},{"Exit":[0,889]},{"And":[[890],null]},{"Token":89},{"And":[[892],null]},{"Token":90},{"And":[[894],null]},{"Token":91},{"And":[[896],null]},{"Token":86},{"And":[[898],null]},{"Token":88},{"And":[[900],null]},{"Or":[893,895,897,899,901]},{"Token":92},{"Token":82},{"And":[[903,904],null]},{"Or":[905]},{"Not":906},{"Opt":81},{"And":[[907,40,908],null]},{"Or":[909]},{"IsIn":0},{"Not":911},{"Call":[150,[[2,82]]]},{"Token":63},{"Call":[78,[[0,75]]]},{"And":[[914,915],null]},{"Or":[916]},{"Opt":917},{"And":[[913,918],null]},{"Or":[919]},{"Call":[151,[[3,920]]]},{"And":[[912,921],null]},{"Or":[922]},{"Token":92},{"Token":59},{"And":[[925,75],null]},{"Or":[926]},{"Opt":927},{"And":[[924,928],1]},{"Or":[929]},{"Token":37},{"Token":38},{"And":[[931,932],null]},{"Or":[933]},{"Call":[78,[[0,75]]]},{"Opt":85},{"And":[[935,936],null]},{"Or":[937]},{"Call":[152,[[4,938]]]},{"And":[[939],null]},{"Or":[940]},{"Token":61},{"Call":[78,[[0,75]]]},{"Call":[150,[[2,943]]]},{"And":[[942,944],null]},{"Or":[945]},{"Token":58},{"And":[[947,75],null]},{"Token":61},{"Call":[150,[[2,75]]]},{"And":[[949,950],null]},{"Or":[948,951]},{"Opt":952},{"And":[[75,953],null]},{"Or":[954]},{"Call":[78,[[0,955]]]},{"Call":[154,[[6,956]]]},{"And":[[957],null]},{"Or":[958]},{"Token":26},{"Opt":960},{"Token":79},{"Rep":88},{"Token":79},{"Token":51},{"And":[[965,49,91],null]},{"Call":[78,[[0,75]]]},{"And":[[967],null]},{"Or":[966,968]},{"And":[[961,962,963,964,969],null]},{"Or":[970]},{"Token":61},{"And":[[972],null]},{"Token":79},{"Not":974},{"Not":975},{"And":[[976],null]},{"Or":[973,977]},{"And":[[13,978],1]},{"Token":31},{"Opt":75},{"And":[[980,981],null]},{"Or":[982]},{"Token":33},{"And":[[984],null]},{"Token":32},{"And":[[986],null]},{"Or":[985,987]},{"Token":87},{"Opt":989},{"And":[[988,990],null]},{"Or":[991]},{"Token":35},{"Opt":993},{"WithSkip":[93,92]},{"Rep":995},{"Call":[151,[[3,996]]]},{"And":[[994,997],null]},{"Or":[998]},{"Call":[78,[[0,999]]]},{"And":[[1000],null]},{"Or":[1001]},{"And":[[94],null]},{"And":[[96],null]},{"And":[[3],null]},{"And":[[97],null]},{"Token":9},{"And":[[1007],null]},{"Token":58},{"And":[[1009],null]},{"And":[[2],null]},{"And":[[76],null]},{"Token":9},{"Opt":95},{"Token":53},{"And":[[1015,75],1]},{"Or":[1016]},{"Opt":1017},{"Token":58},{"And":[[1013,63,1014,1018,1019],1]},{"Or":[1020]},{"Token":59},{"And":[[1022,49],null]},{"Or":[1023]},{"Token":58},{"And":[[1025],null]},{"Or":[1026]},{"And":[[77],null]},"Eof",{"And":[[1029],null]},{"Token":58},{"And":[[1031],null]},{"Or":[1028,1030,1032]},{"And":[[75,1033],1]},{"Or":[1034]},{"Enter":[2,1035]},{"And":[[1036],null]},{"Or":[1037]},{"Token":14},{"Token":15},{"And":[[91],null]},{"And":[[98],null]},{"Or":[1041,1042]},{"And":[[1040,1043],null]},{"Or":[1044]},{"Opt":1045},{"And":[[1039,100,91,1046],1]},{"Or":[1047]},{"Opt":104},{"Token":25},{"And":[[1049,1050,100,91],2]},{"Or":[1051]},{"Token":9},{"Token":53},{"And":[[1053,63,1054],1]},{"Or":[1055]},{"Opt":1056},{"And":[[1057,101],1]},{"Enter":[0,75]},{"And":[[1059],null]},{"Opt":104},{"Token":24},{"And":[[1061,1062,91],2]},{"Or":[1063]},{"Opt":104},{"Token":23},{"Token":34},{"And":[[1065,1066,63,1067,101,91],2]},{"Or":[1068]},{"Token":87},{"Token":59},{"And":[[1070,1071],null]},{"Token":30},{"Rep":106},{"Call":[151,[[3,1074]]]},{"And":[[1073,101,1075],1]},{"Or":[1076]},{"Token":52},{"Enter":[2,75]},{"Token":61},{"And":[[1080],null]},"Eof",{"And":[[1082],null]},{"And":[[77],null]},{"Or":[1081,1083,1084]},{"And":[[107,1078,1079,1085],1]},{"Or":[1086]},{"Token":79},{"And":[[1088,63],null]},{"Or":[1089]},{"Rep":1090},{"Opt":108},{"And":[[63,1091,1092],null]},{"Token":14},{"And":[[1094,75],null]},{"Or":[1095]},{"And":[[147],null]},{"Or":[1097]},{"And":[[148],null]},{"Or":[1099]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":1103},{"And":[[1101,1102,1104,113],null]},{"Or":[1105]},{"And":[[75,1106],null]},{"Or":[1107]},{"IsIn":2},{"Not":77},{"And":[[1109,1110],null]},{"IsIn":2},{"Not":1112},{"And":[[1113],null]},{"Or":[1111,1114]},{"And":[[1115,113],null]},{"Or":[1116]},{"And":[[75,1117],null]},{"Or":[1118]},{"Call":[150,[[2,114]]]},{"Call":[152,[[4,1120]]]},{"Call":[78,[[0,1121]]]},{"And":[[1122],null]},{"And":[[75],null]},{"Or":[1124]},{"Token":62},{"Token":92},{"And":[[1127],null]},{"Token":89},{"And":[[1129],null]},{"Or":[1128,1130]},{"And":[[1126,1131],null]},{"Or":[1132]},{"And":[[75,1133],null]},{"Or":[1134]},{"Call":[154,[[6,75]]]},{"And":[[75,1136],null]},{"Or":[1137]},{"Token":83},{"And":[[75,1139],null]},{"Or":[1140]},{"Token":5},{"And":[[1142,49],null]},{"Or":[1143]},{"And":[[75,1144],null]},{"Or":[1145]},{"And":[[120,75],null]},{"Or":[1147]},{"Token":77},{"Token":87},{"Opt":1150},{"Token":27},{"Opt":1152},{"And":[[1149,1151,1153],null]},{"Token":67},{"And":[[1155,75],null]},{"Or":[1156]},{"Token":75},{"And":[[1158,75],null]},{"Or":[1159]},{"Token":82},{"And":[[1161,75],null]},{"Or":[1162]},{"Token":67},{"And":[[1164],null]},{"Token":69},{"And":[[1166],null]},{"Token":71},{"And":[[1168],null]},{"Or":[1165,1167,1169]},{"Call":[127,[[1,1170]]]},{"And":[[75,1171,75],null]},{"Or":[1172]},{"Token":73},{"And":[[1174],null]},{"Token":75},{"And":[[1176],null]},{"Or":[1175,1177]},{"Call":[127,[[1,1178]]]},{"And":[[75,1179,75],null]},{"Or":[1180]},{"ContextualToken":[45,"<<"]},{"And":[[1182],null]},{"ContextualToken":[47,">>"]},{"And":[[1184],null]},{"Or":[1183,1185]},{"Call":[127,[[1,1186]]]},{"And":[[75,1187,75],null]},{"Or":[1188]},{"IsIn":2},{"Not":77},{"Var":1},{"And":[[1190,1191,1192],null]},{"IsIn":2},{"Not":1194},{"Var":1},{"And":[[1195,1196],null]},{"Token":77},{"Token":77},{"Not":1199},{"And":[[1198,1200],null]},{"Or":[1201]},{"Call":[127,[[1,1202]]]},{"And":[[75,1203,75],null]},{"Or":[1204]},{"Token":84},{"Call":[127,[[1,1206]]]},{"And":[[75,1207,75],null]},{"Or":[1208]},{"Token":79},{"Token":79},{"Not":1211},{"And":[[1210,1212],null]},{"Or":[1213]},{"Call":[127,[[1,1214]]]},{"And":[[75,1215,75],null]},{"Or":[1216]},{"Call":[127,[[1,132]]]},{"And":[[75,1218,75],null]},{"Or":[1219]},{"Token":54},{"And":[[1221],null]},{"Token":55},{"And":[[1223],null]},{"Token":41},{"And":[[1225],null]},{"Token":42},{"And":[[1227],null]},{"Token":57},{"And":[[1229],null]},{"Token":56},{"And":[[1231],null]},{"ContextualToken":[49,"&&"]},{"Call":[127,[[1,1233]]]},{"And":[[75,1234,75],null]},{"Or":[1235]},{"ContextualToken":[50,"||"]},{"Call":[127,[[1,1237]]]},{"And":[[75,1238,75],null]},{"Or":[1239]},{"Call":[127,[[1,140]]]},{"And":[[75,1241,75],null]},{"Or":[1242]},{"And":[[140,75],null]},{"Or":[1244]},{"And":[[75,139],null]},{"Or":[1246]},{"Token":63},{"Not":76},{"And":[[1248,1249],null]},{"Or":[1250]},{"Token":63},{"And":[[1252],null]},{"Token":64},{"And":[[1254],null]},{"Not":76},{"Not":1256},{"Token":39},{"IsIn":0},{"And":[[1258,1259],null]},{"Or":[1260]},{"Not":1261},{"And":[[139,1257,1262],null]},{"Token":53},{"And":[[1264],null]},{"Token":74},{"And":[[1266],null]},{"Token":76},{"And":[[1268],null]},{"Token":68},{"And":[[1270],null]},{"Token":70},{"And":[[1272],null]},{"Token":72},{"And":[[1274],null]},{"Token":78},{"And":[[1276],null]},{"Token":80},{"And":[[1278],null]},{"Token":85},{"And":[[1280],null]},{"ContextualToken":[48,">>="]},{"And":[[1282],null]},{"ContextualToken":[46,"<<="]},{"And":[[1284],null]},{"Or":[1265,1267,1269,1271,1273,1275,1277,1279,1281,1283,1285]},{"Call":[127,[[1,1286]]]},{"And":[[75,1287,75],null]},{"Or":[1288]},{"Token":65},{"And":[[1290,144],null]},{"Or":[1291]},{"Token":65},{"Token":82},{"And":[[1293,1294,144],null]},{"Or":[1295]},{"Call":[150,[[2,146]]]},{"Call":[154,[[6,1297]]]},{"And":[[1298],null]},{"Rep":142},{"And":[[1300],null]},{"Token":92},{"Token":53},{"And":[[1303,75],null]},{"Call":[150,[[2,146]]]},{"Call":[152,[[4,1305]]]},{"And":[[1306],null]},{"Or":[1304,1307]},{"Opt":1308},{"And":[[1302,1309],1]},{"Or":[1310]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1314},{"Rep":149},{"Call":[151,[[3,1316]]]},{"And":[[1312,1313,1315,1317],null]},{"Or":[1318]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1322},{"Token":37},{"Rep":149},{"Token":38},{"And":[[1324,1325,1326],null]},{"Token":43},{"Rep":149},{"Token":44},{"And":[[1328,1329,1330],null]},{"Or":[1327,1331]},{"And":[[1320,1321,1323,1332],null]},{"Or":[1333]},{"Token":37},{"And":[[1335],null]},{"Token":38},{"And":[[1337],null]},{"Token":39},{"And":[[1339],null]},{"Token":40},{"And":[[1341],null]},{"Token":43},{"And":[[1343],null]},{"Token":44},{"And":[[1345],null]},{"Or":[1336,1338,1340,1342,1344,1346]},{"Not":1347},"Any",{"And":[[1348,1349],null]},{"Token":37},{"Rep":149},{"Token":38},{"And":[[1351,1352,1353],null]},{"Token":43},{"Rep":149},{"Token":44},{"And":[[1355,1356,1357],null]},{"Token":39},{"Rep":149},{"Token":40},{"And":[[1359,1360,1361],null]},{"Or":[1350,1354,1358,1362]},{"Var":2},"Eof",{"And":[[1365],null]},{"Token":61},{"And":[[1367],null]},{"Or":[1366,1368]},{"And":[[1364,1369],1]},{"Or":[1370]},{"Rep":1371},{"And":[[1372],null]},{"Token":39},{"Token":40},{"Var":3},{"Call":[155,[[7,1374],[8,1375],[9,1376]]]},{"And":[[1377],null]},{"Token":37},{"Token":38},{"Var":4},{"Call":[155,[[7,1379],[8,1380],[9,1381]]]},{"And":[[1382],null]},{"Token":41},{"Token":42},{"Var":5},{"Call":[155,[[7,1384],[8,1385],[9,1386]]]},{"And":[[1387],null]},{"Token":43},{"Token":44},{"Var":6},{"Call":[155,[[7,1389],[8,1390],[9,1391]]]},{"And":[[1392],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[156,[[10,1395],[11,1396]]]},{"Var":9},{"Layer":[1397,1398]},{"Var":8},{"And":[[1394,1399,1400],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[156,[[10,1403],[11,1404]]]},{"Var":11},{"And":[[1402,1405,1406],1]},{"Var":11},{"Not":1408},"Any",{"And":[[1409,1410],null]},{"Or":[1411]},{"And":[[1412],null]},{"Or":[1407,1413]},{"Rep":1414},{"And":[[1415],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, CONTINUE, BREAK, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, TYPE_REFERENCE, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, FOR_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, LOOP_CF_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, INNER_ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
            ],
            syntactical_rules: rt::parser_from_str(parser_json),
            whitespace_binder,
            .. Default::default()
        }
    }
    use self::rt::*;
    lazy_static! {
        static ref LANG: rt::Language = {
            struct Impl { parser_definition: rt::ParserDefinition, lexer: rt::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &::std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => rt::NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LINE_COMMENT => rt::NodeTypeInfo { name: "LINE_COMMENT", whitespace_like: true },
                        BLOCK_COMMENT => rt::NodeTypeInfo { name: "BLOCK_COMMENT", whitespace_like: true },
                        UNION => rt::NodeTypeInfo { name: "UNION", whitespace_like: false },
                        AS => rt::NodeTypeInfo { name: "AS", whitespace_like: false },
                        CRATE => rt::NodeTypeInfo { name: "CRATE", whitespace_like: false },
                        EXTERN => rt::NodeTypeInfo { name: "EXTERN", whitespace_like: false },
                        FN => rt::NodeTypeInfo { name: "FN", whitespace_like: false },
                        LET => rt::NodeTypeInfo { name: "LET", whitespace_like: false },
                        PUB => rt::NodeTypeInfo { name: "PUB", whitespace_like: false },
                        STRUCT => rt::NodeTypeInfo { name: "STRUCT", whitespace_like: false },
                        USE => rt::NodeTypeInfo { name: "USE", whitespace_like: false },
                        MOD => rt::NodeTypeInfo { name: "MOD", whitespace_like: false },
                        IF => rt::NodeTypeInfo { name: "IF", whitespace_like: false },
                        ELSE => rt::NodeTypeInfo { name: "ELSE", whitespace_like: false },
                        ENUM => rt::NodeTypeInfo { name: "ENUM", whitespace_like: false },
                        IMPL => rt::NodeTypeInfo { name: "IMPL", whitespace_like: false },
                        SELF => rt::NodeTypeInfo { name: "SELF", whitespace_like: false },
                        SUPER => rt::NodeTypeInfo { name: "SUPER", whitespace_like: false },
                        TYPE => rt::NodeTypeInfo { name: "TYPE", whitespace_like: false },
                        CONST => rt::NodeTypeInfo { name: "CONST", whitespace_like: false },
                        STATIC => rt::NodeTypeInfo { name: "STATIC", whitespace_like: false },
                        FOR => rt::NodeTypeInfo { name: "FOR", whitespace_like: false },
                        LOOP => rt::NodeTypeInfo { name: "LOOP", whitespace_like: false },
                        WHILE => rt::NodeTypeInfo { name: "WHILE", whitespace_like: false },
                        MOVE => rt::NodeTypeInfo { name: "MOVE", whitespace_like: false },
                        MUT => rt::NodeTypeInfo { name: "MUT", whitespace_like: false },
                        REF => rt::NodeTypeInfo { name: "REF", whitespace_like: false },
                        TRAIT => rt::NodeTypeInfo { name: "TRAIT", whitespace_like: false },
                        MATCH => rt::NodeTypeInfo { name: "MATCH", whitespace_like: false },
                        RETURN => rt::NodeTypeInfo { name: "RETURN", whitespace_like: false },
                        CONTINUE => rt::NodeTypeInfo { name: "CONTINUE", whitespace_like: false },
                        BREAK => rt::NodeTypeInfo { name: "BREAK", whitespace_like: false },
                        IN => rt::NodeTypeInfo { name: "IN", whitespace_like: false },
                        UNSAFE => rt::NodeTypeInfo { name: "UNSAFE", whitespace_like: false },
                        WHERE => rt::NodeTypeInfo { name: "WHERE", whitespace_like: false },
                        L_PAREN => rt::NodeTypeInfo { name: "L_PAREN", whitespace_like: false },
                        R_PAREN => rt::NodeTypeInfo { name: "R_PAREN", whitespace_like: false },
                        L_CURLY => rt::NodeTypeInfo { name: "L_CURLY", whitespace_like: false },
                        R_CURLY => rt::NodeTypeInfo { name: "R_CURLY", whitespace_like: false },
                        L_ANGLE => rt::NodeTypeInfo { name: "L_ANGLE", whitespace_like: false },
                        R_ANGLE => rt::NodeTypeInfo { name: "R_ANGLE", whitespace_like: false },
                        L_BRACK => rt::NodeTypeInfo { name: "L_BRACK", whitespace_like: false },
                        R_BRACK => rt::NodeTypeInfo { name: "R_BRACK", whitespace_like: false },
                        SHL => rt::NodeTypeInfo { name: "SHL", whitespace_like: false },
                        SHL_EQ => rt::NodeTypeInfo { name: "SHL_EQ", whitespace_like: false },
                        SHR => rt::NodeTypeInfo { name: "SHR", whitespace_like: false },
                        SHR_EQ => rt::NodeTypeInfo { name: "SHR_EQ", whitespace_like: false },
                        AND => rt::NodeTypeInfo { name: "AND", whitespace_like: false },
                        OR => rt::NodeTypeInfo { name: "OR", whitespace_like: false },
                        THIN_ARROW => rt::NodeTypeInfo { name: "THIN_ARROW", whitespace_like: false },
                        FAT_ARROW => rt::NodeTypeInfo { name: "FAT_ARROW", whitespace_like: false },
                        EQ => rt::NodeTypeInfo { name: "EQ", whitespace_like: false },
                        EQEQ => rt::NodeTypeInfo { name: "EQEQ", whitespace_like: false },
                        BANGEQ => rt::NodeTypeInfo { name: "BANGEQ", whitespace_like: false },
                        GTET => rt::NodeTypeInfo { name: "GTET", whitespace_like: false },
                        LTEQ => rt::NodeTypeInfo { name: "LTEQ", whitespace_like: false },
                        SEMI => rt::NodeTypeInfo { name: "SEMI", whitespace_like: false },
                        COLON => rt::NodeTypeInfo { name: "COLON", whitespace_like: false },
                        COLONCOLON => rt::NodeTypeInfo { name: "COLONCOLON", whitespace_like: false },
                        COMMA => rt::NodeTypeInfo { name: "COMMA", whitespace_like: false },
                        DOT => rt::NodeTypeInfo { name: "DOT", whitespace_like: false },
                        DOTDOT => rt::NodeTypeInfo { name: "DOTDOT", whitespace_like: false },
                        DOTDOTDOT => rt::NodeTypeInfo { name: "DOTDOTDOT", whitespace_like: false },
                        HASH => rt::NodeTypeInfo { name: "HASH", whitespace_like: false },
                        DOLLAR => rt::NodeTypeInfo { name: "DOLLAR", whitespace_like: false },
                        STAR => rt::NodeTypeInfo { name: "STAR", whitespace_like: false },
                        STAR_EQ => rt::NodeTypeInfo { name: "STAR_EQ", whitespace_like: false },
                        SLASH => rt::NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        SLASH_EQ => rt::NodeTypeInfo { name: "SLASH_EQ", whitespace_like: false },
                        PERCENT => rt::NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                        PERCENT_EQ => rt::NodeTypeInfo { name: "PERCENT_EQ", whitespace_like: false },
                        PLUS => rt::NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        PLUS_EQ => rt::NodeTypeInfo { name: "PLUS_EQ", whitespace_like: false },
                        MINUS => rt::NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        MINUS_EQ => rt::NodeTypeInfo { name: "MINUS_EQ", whitespace_like: false },
                        AMPERSAND => rt::NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                        AMPERSAND_EQ => rt::NodeTypeInfo { name: "AMPERSAND_EQ", whitespace_like: false },
                        PIPE => rt::NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        PIPE_EQ => rt::NodeTypeInfo { name: "PIPE_EQ", whitespace_like: false },
                        UNDERSCORE => rt::NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                        BANG => rt::NodeTypeInfo { name: "BANG", whitespace_like: false },
                        QUESTION => rt::NodeTypeInfo { name: "QUESTION", whitespace_like: false },
                        CARET => rt::NodeTypeInfo { name: "CARET", whitespace_like: false },
                        CARET_EQ => rt::NodeTypeInfo { name: "CARET_EQ", whitespace_like: false },
                        CHAR => rt::NodeTypeInfo { name: "CHAR", whitespace_like: false },
                        LIFETIME => rt::NodeTypeInfo { name: "LIFETIME", whitespace_like: false },
                        BOOL => rt::NodeTypeInfo { name: "BOOL", whitespace_like: false },
                        NUMBER => rt::NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        STRING => rt::NodeTypeInfo { name: "STRING", whitespace_like: false },
                        RAW_STRING => rt::NodeTypeInfo { name: "RAW_STRING", whitespace_like: false },
                        IDENT => rt::NodeTypeInfo { name: "IDENT", whitespace_like: false },
                        FILE => rt::NodeTypeInfo { name: "FILE", whitespace_like: false },
                        USE_DECL => rt::NodeTypeInfo { name: "USE_DECL", whitespace_like: false },
                        USE_SPEC => rt::NodeTypeInfo { name: "USE_SPEC", whitespace_like: false },
                        USE_SPEC_ENTRY => rt::NodeTypeInfo { name: "USE_SPEC_ENTRY", whitespace_like: false },
                        EXTERN_CRATE_DECL => rt::NodeTypeInfo { name: "EXTERN_CRATE_DECL", whitespace_like: false },
                        FN_DEF => rt::NodeTypeInfo { name: "FN_DEF", whitespace_like: false },
                        LINKAGE => rt::NodeTypeInfo { name: "LINKAGE", whitespace_like: false },
                        VALUE_PARAM => rt::NodeTypeInfo { name: "VALUE_PARAM", whitespace_like: false },
                        LAMBDA_VALUE_PARAM => rt::NodeTypeInfo { name: "LAMBDA_VALUE_PARAM", whitespace_like: false },
                        SELF_PARAMETER => rt::NodeTypeInfo { name: "SELF_PARAMETER", whitespace_like: false },
                        STRUCT_DEF => rt::NodeTypeInfo { name: "STRUCT_DEF", whitespace_like: false },
                        STRUCT_FIELD => rt::NodeTypeInfo { name: "STRUCT_FIELD", whitespace_like: false },
                        TUPLE_FIELD => rt::NodeTypeInfo { name: "TUPLE_FIELD", whitespace_like: false },
                        ENUM_DEF => rt::NodeTypeInfo { name: "ENUM_DEF", whitespace_like: false },
                        ENUM_VARIANT => rt::NodeTypeInfo { name: "ENUM_VARIANT", whitespace_like: false },
                        MOD_DEF => rt::NodeTypeInfo { name: "MOD_DEF", whitespace_like: false },
                        IMPL_DEF => rt::NodeTypeInfo { name: "IMPL_DEF", whitespace_like: false },
                        TRAIT_DEF => rt::NodeTypeInfo { name: "TRAIT_DEF", whitespace_like: false },
                        MEMBERS => rt::NodeTypeInfo { name: "MEMBERS", whitespace_like: false },
                        TYPE_DEF => rt::NodeTypeInfo { name: "TYPE_DEF", whitespace_like: false },
                        CONST_DEF => rt::NodeTypeInfo { name: "CONST_DEF", whitespace_like: false },
                        MACRO_ITEM => rt::NodeTypeInfo { name: "MACRO_ITEM", whitespace_like: false },
                        EXTERN_BLOCK => rt::NodeTypeInfo { name: "EXTERN_BLOCK", whitespace_like: false },
                        TYPE_PARAMETERS => rt::NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                        TYPE_PARAMETER => rt::NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                        TYPE_BOUND => rt::NodeTypeInfo { name: "TYPE_BOUND", whitespace_like: false },
                        LIFETIME_PARAMETER => rt::NodeTypeInfo { name: "LIFETIME_PARAMETER", whitespace_like: false },
                        VISIBILITY => rt::NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                        WHERE_CLAUSE => rt::NodeTypeInfo { name: "WHERE_CLAUSE", whitespace_like: false },
                        PATH => rt::NodeTypeInfo { name: "PATH", whitespace_like: false },
                        TRAIT_PROJECTION_PATH => rt::NodeTypeInfo { name: "TRAIT_PROJECTION_PATH", whitespace_like: false },
                        PATH_SEGMENT => rt::NodeTypeInfo { name: "PATH_SEGMENT", whitespace_like: false },
                        TYPE_ARGUMENTS => rt::NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
                        FN_TRAIT_SUGAR => rt::NodeTypeInfo { name: "FN_TRAIT_SUGAR", whitespace_like: false },
                        ALIAS => rt::NodeTypeInfo { name: "ALIAS", whitespace_like: false },
                        TYPE_REFERENCE => rt::NodeTypeInfo { name: "TYPE_REFERENCE", whitespace_like: false },
                        PATH_TYPE => rt::NodeTypeInfo { name: "PATH_TYPE", whitespace_like: false },
                        REFERENCE_TYPE => rt::NodeTypeInfo { name: "REFERENCE_TYPE", whitespace_like: false },
                        POINTER_TYPE => rt::NodeTypeInfo { name: "POINTER_TYPE", whitespace_like: false },
                        PLACEHOLDER_TYPE => rt::NodeTypeInfo { name: "PLACEHOLDER_TYPE", whitespace_like: false },
                        UNIT_TYPE => rt::NodeTypeInfo { name: "UNIT_TYPE", whitespace_like: false },
                        PAREN_TYPE => rt::NodeTypeInfo { name: "PAREN_TYPE", whitespace_like: false },
                        TUPLE_TYPE => rt::NodeTypeInfo { name: "TUPLE_TYPE", whitespace_like: false },
                        NEVER_TYPE => rt::NodeTypeInfo { name: "NEVER_TYPE", whitespace_like: false },
                        ARRAY_TYPE => rt::NodeTypeInfo { name: "ARRAY_TYPE", whitespace_like: false },
                        FN_POINTER_TYPE => rt::NodeTypeInfo { name: "FN_POINTER_TYPE", whitespace_like: false },
                        FOR_TYPE => rt::NodeTypeInfo { name: "FOR_TYPE", whitespace_like: false },
                        WILDCARD_PATTERN => rt::NodeTypeInfo { name: "WILDCARD_PATTERN", whitespace_like: false },
                        PATH_PATTERN => rt::NodeTypeInfo { name: "PATH_PATTERN", whitespace_like: false },
                        TUPE_STRUCT_PATTERN => rt::NodeTypeInfo { name: "TUPE_STRUCT_PATTERN", whitespace_like: false },
                        STRUCT_PATTERN => rt::NodeTypeInfo { name: "STRUCT_PATTERN", whitespace_like: false },
                        STRUCT_PATTERN_FIELD => rt::NodeTypeInfo { name: "STRUCT_PATTERN_FIELD", whitespace_like: false },
                        BINDING_PATTERN => rt::NodeTypeInfo { name: "BINDING_PATTERN", whitespace_like: false },
                        LITERAL_PATTERN => rt::NodeTypeInfo { name: "LITERAL_PATTERN", whitespace_like: false },
                        UNIT_PATTERN => rt::NodeTypeInfo { name: "UNIT_PATTERN", whitespace_like: false },
                        PAREN_PATTERN => rt::NodeTypeInfo { name: "PAREN_PATTERN", whitespace_like: false },
                        TUPLE_PATTERN => rt::NodeTypeInfo { name: "TUPLE_PATTERN", whitespace_like: false },
                        REFERENCE_PATTERN => rt::NodeTypeInfo { name: "REFERENCE_PATTERN", whitespace_like: false },
                        EXPR => rt::NodeTypeInfo { name: "EXPR", whitespace_like: false },
                        LITERAL => rt::NodeTypeInfo { name: "LITERAL", whitespace_like: false },
                        PATH_EXPR => rt::NodeTypeInfo { name: "PATH_EXPR", whitespace_like: false },
                        STRUCT_LITERAL => rt::NodeTypeInfo { name: "STRUCT_LITERAL", whitespace_like: false },
                        STRUCT_LITERAL_FIELD => rt::NodeTypeInfo { name: "STRUCT_LITERAL_FIELD", whitespace_like: false },
                        UNIT_EXPR => rt::NodeTypeInfo { name: "UNIT_EXPR", whitespace_like: false },
                        PAREN_EXPR => rt::NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                        TUPLE_EXPR => rt::NodeTypeInfo { name: "TUPLE_EXPR", whitespace_like: false },
                        ARRAY_LITERAL => rt::NodeTypeInfo { name: "ARRAY_LITERAL", whitespace_like: false },
                        LAMBDA_EXPR => rt::NodeTypeInfo { name: "LAMBDA_EXPR", whitespace_like: false },
                        RETURN_EXPR => rt::NodeTypeInfo { name: "RETURN_EXPR", whitespace_like: false },
                        LOOP_CF_EXPR => rt::NodeTypeInfo { name: "LOOP_CF_EXPR", whitespace_like: false },
                        BLOCK_EXPR => rt::NodeTypeInfo { name: "BLOCK_EXPR", whitespace_like: false },
                        LET_STMT => rt::NodeTypeInfo { name: "LET_STMT", whitespace_like: false },
                        TYPE_ASCRIPTION => rt::NodeTypeInfo { name: "TYPE_ASCRIPTION", whitespace_like: false },
                        EMPTY_STMT => rt::NodeTypeInfo { name: "EMPTY_STMT", whitespace_like: false },
                        EXPR_STMT => rt::NodeTypeInfo { name: "EXPR_STMT", whitespace_like: false },
                        IF_EXPR => rt::NodeTypeInfo { name: "IF_EXPR", whitespace_like: false },
                        WHILE_EXPR => rt::NodeTypeInfo { name: "WHILE_EXPR", whitespace_like: false },
                        LOOP_EXPR => rt::NodeTypeInfo { name: "LOOP_EXPR", whitespace_like: false },
                        FOR_EXPR => rt::NodeTypeInfo { name: "FOR_EXPR", whitespace_like: false },
                        MATCH_EXPR => rt::NodeTypeInfo { name: "MATCH_EXPR", whitespace_like: false },
                        MATCH_ARM => rt::NodeTypeInfo { name: "MATCH_ARM", whitespace_like: false },
                        GUARD => rt::NodeTypeInfo { name: "GUARD", whitespace_like: false },
                        BLOCK_MACRO_EXPR => rt::NodeTypeInfo { name: "BLOCK_MACRO_EXPR", whitespace_like: false },
                        LINE_MACRO_EXPR => rt::NodeTypeInfo { name: "LINE_MACRO_EXPR", whitespace_like: false },
                        METHOD_CALL_EXPR => rt::NodeTypeInfo { name: "METHOD_CALL_EXPR", whitespace_like: false },
                        CALL_EXPR => rt::NodeTypeInfo { name: "CALL_EXPR", whitespace_like: false },
                        VALUE_ARGUMENT => rt::NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                        FIELD_EXPR => rt::NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                        INDEX_EXPR => rt::NodeTypeInfo { name: "INDEX_EXPR", whitespace_like: false },
                        TRY_EXPR => rt::NodeTypeInfo { name: "TRY_EXPR", whitespace_like: false },
                        CAST_EXPR => rt::NodeTypeInfo { name: "CAST_EXPR", whitespace_like: false },
                        REFERENCE_EXPR => rt::NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                        DEREFERENCE_EXPR => rt::NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                        NEGATION_EXPR => rt::NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                        NOT_EXPR => rt::NodeTypeInfo { name: "NOT_EXPR", whitespace_like: false },
                        PRODUCT_EXPR => rt::NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                        SUM_EXPR => rt::NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                        BIT_SHIFT => rt::NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                        BIT_AND => rt::NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
                        BIT_XOR => rt::NodeTypeInfo { name: "BIT_XOR", whitespace_like: false },
                        BIT_OR => rt::NodeTypeInfo { name: "BIT_OR", whitespace_like: false },
                        COMPARISON => rt::NodeTypeInfo { name: "COMPARISON", whitespace_like: false },
                        LOGICAL_AND => rt::NodeTypeInfo { name: "LOGICAL_AND", whitespace_like: false },
                        LOGICAL_OR => rt::NodeTypeInfo { name: "LOGICAL_OR", whitespace_like: false },
                        RANGE_EXPR => rt::NodeTypeInfo { name: "RANGE_EXPR", whitespace_like: false },
                        ASSIGNMENT_EXPR => rt::NodeTypeInfo { name: "ASSIGNMENT_EXPR", whitespace_like: false },
                        ATTRIBUTE => rt::NodeTypeInfo { name: "ATTRIBUTE", whitespace_like: false },
                        INNER_ATTRIBUTE => rt::NodeTypeInfo { name: "INNER_ATTRIBUTE", whitespace_like: false },
                        ATTR_VALUE => rt::NodeTypeInfo { name: "ATTR_VALUE", whitespace_like: false },
                        BLOCK_MACRO => rt::NodeTypeInfo { name: "BLOCK_MACRO", whitespace_like: false },
                        LINE_MACRO => rt::NodeTypeInfo { name: "LINE_MACRO", whitespace_like: false },
                        TT => rt::NodeTypeInfo { name: "TT", whitespace_like: false },
                        _ => panic!("Unknown rt::NodeType: {:?}", ty)
                    }
                }
            }

            rt::Language::new(Impl {
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
        };
    }

    &*LANG
}

fn whitespace_binder(ty: rt::NodeType, adjacent_tokens: Vec<(rt::NodeType, &str)>, is_leading: bool) -> usize {
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

#[allow(unused)]
use self::rt::AstNode;

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
pub struct ModDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for ModDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == MOD_DEF {
            Some(ModDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> ModDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for ModDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("ModDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImplDef<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for ImplDef<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == IMPL_DEF {
            Some(ImplDef { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> ImplDef<'f> {
    
}

impl<'f> ::std::fmt::Debug for ImplDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("ImplDef@")?;
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
    pub fn lifetime(&self) -> rt::Text<'f> {
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LetStmt<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for LetStmt<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == LET_STMT {
            Some(LetStmt { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> LetStmt<'f> {
    
}

impl<'f> ::std::fmt::Debug for LetStmt<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("LetStmt@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeReference<'f> { node: rt::Node<'f> }

impl<'f> rt::AstNode<'f> for TypeReference<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TYPE_REFERENCE {
            Some(TypeReference { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}

impl<'f> TypeReference<'f> {
    
}

impl<'f> ::std::fmt::Debug for TypeReference<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TypeReference@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}



pub trait NameOwner<'f>: rt::AstNode<'f> {
    fn name_ident(&self) -> Option<rt::Node<'f>> {
        self.node().children().find(|n| n.ty() == IDENT)
    }
    fn name(&self) -> Option<rt::Text<'f>> {
        rt::child_of_type(self.node(), IDENT).map(|n| n.text())
    }
}
impl<'f> NameOwner<'f> for FnDef<'f> {}
impl<'f> NameOwner<'f> for StructDef<'f> {}
impl<'f> NameOwner<'f> for EnumDef<'f> {}
impl<'f> NameOwner<'f> for TraitDef<'f> {}
impl<'f> NameOwner<'f> for TypeDef<'f> {}
impl<'f> NameOwner<'f> for ModDef<'f> {}
impl<'f> NameOwner<'f> for TypeParameter<'f> {}
pub trait TypeParametersOwner<'f>: rt::AstNode<'f> {
    fn type_parameters(&self) -> Option<TypeParameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}
impl<'f> TypeParametersOwner<'f> for StructDef<'f> {}
impl<'f> TypeParametersOwner<'f> for EnumDef<'f> {}