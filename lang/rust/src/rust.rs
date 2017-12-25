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
pub const FOR_TYPE: rt::NodeType = rt::NodeType(237);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(238);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(239);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(240);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(241);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(242);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(243);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(244);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(245);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(246);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(247);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(248);
pub const EXPR: rt::NodeType = rt::NodeType(249);
pub const LITERAL: rt::NodeType = rt::NodeType(250);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(251);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(252);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(253);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(254);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(255);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(256);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(257);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(258);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(259);
pub const LOOP_CF_EXPR: rt::NodeType = rt::NodeType(260);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(261);
pub const LET_STMT: rt::NodeType = rt::NodeType(262);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(263);
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
pub const INNER_ATTRIBUTE: rt::NodeType = rt::NodeType(298);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(299);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(300);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(301);
pub const TT: rt::NodeType = rt::NodeType(302);


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
        let parser_json = r##"[{"Pub":{"ty":93,"body":157,"replaceable":false}},{"Or":[160]},{"Or":[162,164,166,168,170,172,174,175]},{"Cached":192},{"Pub":{"ty":94,"body":208,"replaceable":false}},{"Pub":{"ty":95,"body":214,"replaceable":false}},{"Pub":{"ty":96,"body":220,"replaceable":false}},{"Pub":{"ty":97,"body":227,"replaceable":false}},{"Pub":{"ty":98,"body":243,"replaceable":false}},{"Or":[245]},{"Pub":{"ty":99,"body":250,"replaceable":false}},{"Or":[256]},{"Pub":{"ty":100,"body":259,"replaceable":false}},{"Pub":{"ty":101,"body":265,"replaceable":false}},{"Pub":{"ty":102,"body":282,"replaceable":false}},{"Pub":{"ty":103,"body":301,"replaceable":false}},{"Pub":{"ty":104,"body":306,"replaceable":false}},{"Pub":{"ty":105,"body":309,"replaceable":false}},{"Pub":{"ty":106,"body":316,"replaceable":false}},{"Pub":{"ty":107,"body":329,"replaceable":false}},{"Pub":{"ty":108,"body":338,"replaceable":false}},{"Pub":{"ty":109,"body":351,"replaceable":false}},{"Pub":{"ty":110,"body":359,"replaceable":false}},{"Pub":{"ty":111,"body":371,"replaceable":false}},{"Or":[372,373,374,375]},{"Or":[377,379,381,383,385,387,389,391,396]},{"Pub":{"ty":112,"body":406,"replaceable":false}},{"Pub":{"ty":113,"body":420,"replaceable":false}},{"Pub":{"ty":114,"body":424,"replaceable":false}},{"Pub":{"ty":115,"body":428,"replaceable":false}},{"Or":[436]},{"Pub":{"ty":116,"body":443,"replaceable":false}},{"Pub":{"ty":117,"body":447,"replaceable":false}},{"Or":[467]},{"Pub":{"ty":118,"body":472,"replaceable":false}},{"Pub":{"ty":119,"body":492,"replaceable":false}},{"Pub":{"ty":120,"body":502,"replaceable":false}},{"Pub":{"ty":121,"body":517,"replaceable":false}},{"Or":[518]},{"Or":[520]},{"Or":[522]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":122,"op":525,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":122,"body":529,"replaceable":false}},{"Pub":{"ty":122,"body":534,"replaceable":false}},{"Pub":{"ty":123,"body":541,"replaceable":false}},{"Pub":{"ty":124,"body":555,"replaceable":false}},{"Pub":{"ty":125,"body":581,"replaceable":false}},{"Pub":{"ty":126,"body":586,"replaceable":false}},{"Pub":{"ty":127,"body":590,"replaceable":false}},{"Or":[595]},{"Or":[596,597,598,599,600,601,602,603,604,605]},{"Pub":{"ty":128,"body":607,"replaceable":false}},{"Pub":{"ty":129,"body":609,"replaceable":false}},{"Pub":{"ty":130,"body":617,"replaceable":false}},{"Pub":{"ty":131,"body":620,"replaceable":false}},{"Pub":{"ty":132,"body":624,"replaceable":false}},{"Pub":{"ty":133,"body":630,"replaceable":true}},{"PubReplace":{"ty":134,"body":634}},{"Pub":{"ty":135,"body":637,"replaceable":false}},{"Pub":{"ty":136,"body":646,"replaceable":false}},{"Pub":{"ty":137,"body":653,"replaceable":false}},{"Pub":{"ty":100,"body":659,"replaceable":false}},{"Pub":{"ty":138,"body":662,"replaceable":false}},{"Or":[663,664,665,666,667,668,669]},{"Pub":{"ty":139,"body":672,"replaceable":false}},{"Pub":{"ty":140,"body":678,"replaceable":true}},{"PubReplace":{"ty":141,"body":690}},{"PubReplace":{"ty":142,"body":702}},{"Pub":{"ty":143,"body":709,"replaceable":false}},{"Pub":{"ty":144,"body":716,"replaceable":false}},{"Pub":{"ty":145,"body":718,"replaceable":false}},{"Pub":{"ty":146,"body":722,"replaceable":false}},{"Pub":{"ty":147,"body":728,"replaceable":true}},{"PubReplace":{"ty":148,"body":732}},{"Pub":{"ty":149,"body":737,"replaceable":false}},{"Pratt":{"atoms":[79,80,83,84,86,87,89,90,91,97,98,101,102,104,108,109,137],"prefixes":[{"ty":183,"op":119,"priority":999},{"ty":184,"op":766,"priority":999},{"ty":185,"op":767,"priority":999},{"ty":186,"op":768,"priority":999},{"ty":196,"op":139,"priority":2}],"infixes":[{"ty":176,"op":743,"priority":999,"has_rhs":false},{"ty":177,"op":752,"priority":999,"has_rhs":false},{"ty":179,"op":760,"priority":999,"has_rhs":false},{"ty":180,"op":761,"priority":999,"has_rhs":false},{"ty":181,"op":762,"priority":999,"has_rhs":false},{"ty":182,"op":765,"priority":999,"has_rhs":false},{"ty":187,"op":776,"priority":11,"has_rhs":true},{"ty":188,"op":782,"priority":10,"has_rhs":true},{"ty":189,"op":788,"priority":9,"has_rhs":true},{"ty":190,"op":794,"priority":8,"has_rhs":true},{"ty":191,"op":796,"priority":7,"has_rhs":true},{"ty":192,"op":802,"priority":6,"has_rhs":true},{"ty":193,"op":803,"priority":5,"has_rhs":true},{"ty":194,"op":805,"priority":4,"has_rhs":true},{"ty":195,"op":807,"priority":3,"has_rhs":true},{"ty":196,"op":808,"priority":2,"has_rhs":true},{"ty":196,"op":138,"priority":2,"has_rhs":false},{"ty":197,"op":832,"priority":1,"has_rhs":true}]}},{"Or":[833,835,837,839,841,843,845,847,849,851,853,855,857,859,861,863,865,867,869,871,873,875]},{"Or":[877]},{"Or":[881]},{"Pub":{"ty":151,"body":892,"replaceable":false}},{"Pub":{"ty":152,"body":900,"replaceable":true}},{"PubReplace":{"ty":153,"body":913}},{"Pub":{"ty":154,"body":920,"replaceable":false}},{"Pub":{"ty":155,"body":924,"replaceable":false}},{"Pub":{"ty":156,"body":931,"replaceable":true}},{"PubReplace":{"ty":157,"body":936}},{"Pub":{"ty":158,"body":941,"replaceable":false}},{"Pub":{"ty":159,"body":953,"replaceable":false}},{"Or":[961]},{"Pub":{"ty":160,"body":965,"replaceable":false}},{"Pub":{"ty":161,"body":974,"replaceable":false}},{"Pub":{"ty":162,"body":986,"replaceable":false}},{"Or":[987,988,989,990]},{"Pub":{"ty":163,"body":999,"replaceable":false}},{"Pub":{"ty":164,"body":1002,"replaceable":false}},{"Pub":{"ty":165,"body":1005,"replaceable":false}},{"Pub":{"ty":166,"body":1016,"replaceable":false}},{"Pub":{"ty":167,"body":1026,"replaceable":false}},{"Pub":{"ty":168,"body":1030,"replaceable":false}},{"Or":[1036]},{"Or":[1038]},{"Pub":{"ty":169,"body":1042,"replaceable":false}},{"Pub":{"ty":170,"body":1047,"replaceable":false}},{"Or":[1050]},{"Pub":{"ty":171,"body":1055,"replaceable":false}},{"Pub":{"ty":172,"body":1065,"replaceable":false}},{"Or":[1071]},{"Pub":{"ty":173,"body":1074,"replaceable":false}},{"Pub":{"ty":174,"body":1076,"replaceable":false}},{"Pub":{"ty":175,"body":1078,"replaceable":false}},{"Pub":{"ty":176,"body":1086,"replaceable":false}},{"Pub":{"ty":177,"body":1097,"replaceable":false}},{"Or":[1101]},{"Pub":{"ty":178,"body":1103,"replaceable":false}},{"Pub":{"ty":179,"body":1113,"replaceable":false}},{"Pub":{"ty":180,"body":1116,"replaceable":false}},{"Pub":{"ty":181,"body":1119,"replaceable":false}},{"Pub":{"ty":182,"body":1124,"replaceable":false}},{"Pub":{"ty":183,"body":1126,"replaceable":false}},{"Or":[1132]},{"Pub":{"ty":184,"body":1135,"replaceable":false}},{"Pub":{"ty":185,"body":1138,"replaceable":false}},{"Pub":{"ty":186,"body":1141,"replaceable":false}},{"Pub":{"ty":187,"body":1151,"replaceable":false}},{"Pub":{"ty":188,"body":1159,"replaceable":false}},{"Pub":{"ty":189,"body":1167,"replaceable":false}},{"Or":[1171,1175]},{"Pub":{"ty":190,"body":1183,"replaceable":false}},{"Pub":{"ty":191,"body":1187,"replaceable":false}},{"Pub":{"ty":192,"body":1195,"replaceable":false}},{"Pub":{"ty":193,"body":1198,"replaceable":false}},{"Or":[1200,1202,1204,1206,1208,1210]},{"Pub":{"ty":194,"body":1214,"replaceable":false}},{"Pub":{"ty":195,"body":1218,"replaceable":false}},{"Pub":{"ty":196,"body":1221,"replaceable":false}},{"Pub":{"ty":196,"body":1223,"replaceable":false}},{"Pub":{"ty":196,"body":1225,"replaceable":false}},{"Pub":{"ty":196,"body":1229,"replaceable":false}},{"Or":[1231,1233]},{"Or":[1241]},{"Pub":{"ty":197,"body":1267,"replaceable":false}},{"Pub":{"ty":198,"body":1270,"replaceable":false}},{"Pub":{"ty":199,"body":1274,"replaceable":false}},{"Or":[1277]},{"Or":[1279]},{"Pub":{"ty":200,"body":1289,"replaceable":false}},{"Pub":{"ty":201,"body":1297,"replaceable":false}},{"Pub":{"ty":202,"body":1312,"replaceable":false}},{"Pub":{"ty":203,"body":1341,"replaceable":false}},{"Or":[1351]},{"Or":[1356]},{"Or":[1361]},{"Or":[1366]},{"Or":[1371]},{"Or":[1379]},{"Or":[1394]},{"And":[[1],null]},{"Or":[156]},{"WithSkip":[2,3]},{"Rep":158},{"And":[[159],null]},{"Token":11},{"And":[[161],null]},{"ContextualToken":[4,"union"]},{"And":[[163],null]},{"Token":16},{"And":[[165],null]},{"Token":12},{"And":[[167],null]},{"Token":13},{"And":[[169],null]},{"Token":17},{"And":[[171],null]},{"Token":29},{"And":[[173],null]},{"And":[[25],null]},{"Opt":36},{"And":[[144,176],null]},{"Or":[177]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[179,180,181,182,183,184,185,186,187]},{"Inject":[178,188]},{"And":[[189],null]},{"And":[[28],null]},{"Or":[190,191]},{"Token":12},{"And":[[48],null]},{"Token":60},{"And":[[195,5],null]},{"Or":[196]},{"Opt":197},{"And":[[198],null]},{"Or":[194,199]},{"And":[[38,200],null]},{"Token":60},{"Opt":202},{"And":[[203,5],null]},{"Or":[201,204]},{"Token":58},{"And":[[193,205,206],1]},{"Or":[207]},{"Token":67},{"And":[[209],null]},{"Call":[149,[[2,6]]]},{"Call":[150,[[3,211]]]},{"And":[[212],null]},{"Or":[210,213]},{"Token":18},{"And":[[215],null]},{"Token":92},{"Opt":48},{"And":[[217,218],1]},{"Or":[216,219]},{"Token":7},{"Token":6},{"Token":92},{"Opt":48},{"Token":58},{"And":[[221,222,223,224,225],2]},{"Or":[226]},{"Token":21},{"Opt":228},{"Token":35},{"Opt":230},{"Opt":10},{"Token":8},{"Token":92},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[91],null]},{"Token":58},{"And":[[239],null]},{"Or":[238,240]},{"And":[[229,231,232,233,234,235,11,236,237,241],4]},{"Or":[242]},{"Token":51},{"And":[[244,49],null]},{"Token":7},{"Token":90},{"Opt":247},{"And":[[246,248],null]},{"Or":[249]},{"Opt":14},{"Call":[149,[[2,12]]]},{"And":[[251,252],null]},{"Or":[253]},{"Call":[151,[[4,254]]]},{"And":[[255],null]},{"Token":59},{"And":[[63,257,49],1]},{"Or":[258]},{"Token":59},{"And":[[260,49],null]},{"Or":[261]},{"Opt":262},{"And":[[63,263],null]},{"Or":[264]},{"And":[[119],null]},{"Token":27},{"And":[[267],null]},{"Or":[266,268]},{"Opt":269},{"Token":18},{"Token":59},{"And":[[272,49],null]},{"Or":[273]},{"Opt":274},{"Token":61},{"And":[[276],null]},"Eof",{"And":[[278],null]},{"Or":[277,279]},{"And":[[270,271,275,280],2]},{"Or":[281]},{"Token":11},{"And":[[283],null]},{"ContextualToken":[4,"union"]},{"And":[[285],null]},{"Or":[284,286]},{"Token":92},{"Opt":31},{"Call":[149,[[2,16]]]},{"Call":[150,[[3,290]]]},{"And":[[291],null]},{"Token":58},{"And":[[293],null]},{"Call":[149,[[2,17]]]},{"Call":[151,[[4,295]]]},{"Token":58},{"And":[[296,297],null]},{"Or":[292,294,298]},{"And":[[287,288,289,299],1]},{"Or":[300]},{"Opt":36},{"Token":92},{"Token":59},{"And":[[302,303,304,49],2]},{"Or":[305]},{"Opt":36},{"And":[[307,49],null]},{"Or":[308]},{"Token":16},{"Token":92},{"Opt":31},{"Call":[149,[[2,19]]]},{"Call":[150,[[3,313]]]},{"And":[[310,311,312,314],1]},{"Or":[315]},{"Token":92},{"Token":53},{"And":[[318,75],null]},{"Call":[149,[[2,17]]]},{"Call":[151,[[4,320]]]},{"And":[[321],null]},{"Call":[149,[[2,16]]]},{"Call":[150,[[3,323]]]},{"And":[[324],null]},{"Or":[319,322,325]},{"Opt":326},{"And":[[317,327],1]},{"Or":[328]},{"Token":13},{"Token":92},{"Token":58},{"And":[[332],null]},{"Call":[150,[[3,1]]]},{"And":[[334],null]},{"Or":[333,335]},{"And":[[330,331,336],1]},{"Or":[337]},{"Token":35},{"Opt":339},{"Token":17},{"Opt":31},{"Token":23},{"And":[[343,49],null]},{"Or":[344]},{"Opt":345},{"And":[[49,346],null]},{"Or":[347]},{"Opt":37},{"And":[[340,341,342,348,349,23],2]},{"Or":[350]},{"Opt":36},{"Token":29},{"Token":92},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[144,352,353,354,355,356,357,23],3]},{"Or":[358]},{"Opt":36},{"And":[[144,360],null]},{"Or":[361]},{"Inject":[362,24]},{"And":[[363],null]},{"And":[[28],null]},{"Or":[364,365]},{"WithSkip":[25,366]},{"Rep":367},{"Call":[150,[[3,368]]]},{"And":[[369],null]},{"Or":[370]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[142],null]},{"Token":10},{"And":[[376],null]},{"Token":8},{"And":[[378],null]},{"Token":20},{"And":[[380],null]},{"Token":21},{"And":[[382],null]},{"Token":22},{"And":[[384],null]},{"Token":35},{"And":[[386],null]},{"Token":65},{"And":[[388],null]},{"Token":7},{"And":[[390],null]},{"Token":92},{"Token":82},{"And":[[392,393],null]},{"Or":[394]},{"And":[[395],null]},{"Token":20},{"Token":92},{"Opt":31},{"Token":53},{"And":[[400,49],null]},{"Or":[401]},{"Opt":402},{"Token":58},{"And":[[397,398,399,403,404],1]},{"Or":[405]},{"Token":21},{"And":[[407],null]},{"Token":22},{"And":[[409],null]},{"Or":[408,410]},{"Token":92},{"Token":59},{"Token":53},{"And":[[414,75],null]},{"Or":[415]},{"Opt":416},{"Token":58},{"And":[[411,412,413,49,417,418],1]},{"Or":[419]},{"And":[[146],null]},{"Token":58},{"And":[[147,422],null]},{"Or":[421,423]},{"Rep":30},{"Call":[150,[[3,425]]]},{"And":[[10,426],null]},{"Or":[427]},{"Opt":36},{"And":[[144,429],null]},{"Or":[430]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[432,433]},{"Inject":[431,434]},{"And":[[435],null]},{"Call":[149,[[2,35]]]},{"Call":[149,[[2,32]]]},{"And":[[437,438],null]},{"Or":[439]},{"Call":[152,[[5,440]]]},{"And":[[441],null]},{"Or":[442]},{"Token":92},{"Opt":33},{"And":[[444,445],1]},{"Or":[446]},{"Token":59},{"Token":73},{"And":[[449],null]},"Eof",{"And":[[451],null]},{"Token":61},{"And":[[453],null]},{"Token":39},{"And":[[455],null]},{"Token":36},{"And":[[457],null]},{"Or":[454,456,458]},{"Not":459},{"Not":460},{"And":[[461],null]},{"Or":[450,452,462]},{"And":[[34,463],1]},{"Or":[464]},{"Rep":465},{"And":[[448,466],null]},{"Token":87},{"And":[[468],null]},{"And":[[51],null]},{"And":[[62],null]},{"Or":[469,470,471]},{"Token":87},{"Token":59},{"Token":87},{"Token":73},{"And":[[476],null]},"Eof",{"And":[[478],null]},{"Token":61},{"Not":480},{"Not":481},{"And":[[482],null]},{"Or":[477,479,483]},{"And":[[475,484],1]},{"Or":[485]},{"Rep":486},{"And":[[474,487],null]},{"Or":[488]},{"Opt":489},{"And":[[473,490],1]},{"Or":[491]},{"Token":10},{"Token":6},{"And":[[494],null]},{"Token":19},{"And":[[496],null]},{"Or":[495,497]},{"Call":[151,[[4,498]]]},{"Opt":499},{"And":[[493,500],null]},{"Or":[501]},{"Token":36},{"Token":61},{"And":[[504],null]},"Eof",{"And":[[506],null]},{"Token":39},{"Not":508},{"Not":509},{"And":[[510],null]},{"Or":[505,507,511]},{"And":[[49,33,512],null]},{"Or":[513]},{"Rep":514},{"And":[[503,515],1]},{"Or":[516]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[519],null]},{"Enter":[1,41]},{"And":[[521],null]},{"Token":60},{"And":[[523,45],null]},{"Or":[524]},{"Token":60},{"Opt":526},{"And":[[527,45],null]},{"Or":[528]},{"Token":60},{"And":[[530,45],null]},{"Or":[531]},{"And":[[41,532],null]},{"Or":[533]},{"Token":5},{"And":[[49,535,49],null]},{"Or":[536]},{"Call":[152,[[5,537]]]},{"Token":60},{"And":[[538,539,45],null]},{"Or":[540]},{"Token":92},{"And":[[542],null]},{"Token":18},{"And":[[544],null]},{"Token":19},{"And":[[546],null]},{"Or":[543,545,547]},{"And":[[46],null]},{"IsIn":3},{"And":[[550,47],null]},{"Or":[549,551]},{"Opt":552},{"And":[[548,553],null]},{"Or":[554]},{"IsIn":3},{"And":[[556],null]},{"IsIn":1},{"Token":60},{"And":[[558,559],null]},{"Or":[557,560]},{"Token":87},{"Call":[149,[[2,562]]]},{"Token":92},{"Token":53},{"And":[[564,565],null]},{"Or":[566]},{"Not":567},{"And":[[568,49],null]},{"Or":[569]},{"Call":[149,[[2,570]]]},{"Token":92},{"Token":53},{"And":[[572,573,49],null]},{"Or":[574]},{"Call":[149,[[2,575]]]},{"And":[[563,571,576],null]},{"Or":[577]},{"Call":[152,[[5,578]]]},{"And":[[561,579],null]},{"Or":[580]},{"Call":[149,[[2,49]]]},{"Call":[151,[[4,582]]]},{"Opt":9},{"And":[[583,584],null]},{"Or":[585]},{"Token":5},{"Token":92},{"And":[[587,588],null]},{"Or":[589]},{"Token":73},{"And":[[591,34],null]},{"Or":[592]},{"Rep":593},{"And":[[50,594],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[62],null]},{"And":[[39],null]},{"Or":[606]},{"And":[[119,49],null]},{"Or":[608]},{"Token":67},{"Token":21},{"And":[[611],null]},{"Token":27},{"And":[[613],null]},{"Or":[612,614]},{"And":[[610,615,49],null]},{"Or":[616]},{"Token":81},{"And":[[618],null]},{"Or":[619]},{"Token":37},{"Token":38},{"And":[[621,622],null]},{"Or":[623]},{"Opt":57},{"And":[[49,625],null]},{"Or":[626]},{"Call":[151,[[4,627]]]},{"And":[[628],null]},{"Or":[629]},{"Token":61},{"Call":[149,[[2,49]]]},{"And":[[631,632],null]},{"Or":[633]},{"Token":82},{"And":[[635],null]},{"Or":[636]},{"Token":58},{"And":[[638,75],null]},{"Or":[639]},{"Opt":640},{"And":[[49,641],null]},{"Or":[642]},{"Call":[153,[[6,643]]]},{"And":[[644],null]},{"Or":[645]},{"Opt":10},{"Token":8},{"Call":[149,[[2,61]]]},{"Call":[151,[[4,649]]]},{"Opt":9},{"And":[[647,648,650,651],2]},{"Or":[652]},{"Token":59},{"And":[[63,654],null]},{"Or":[655]},{"Opt":656},{"And":[[657,49],null]},{"Or":[658]},{"Token":23},{"And":[[660,31,49],1]},{"Or":[661]},{"And":[[64],null]},{"And":[[65],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[72],null]},{"And":[[74],null]},{"Token":81},{"And":[[670],null]},{"Or":[671]},{"And":[[66],null]},{"And":[[67],null]},{"Or":[673,674]},{"Opt":675},{"And":[[40,676],null]},{"Or":[677]},{"Call":[149,[[2,63]]]},{"Token":63},{"Token":61},{"Opt":681},{"And":[[680,682],null]},{"Or":[683]},{"Opt":684},{"And":[[679,685],null]},{"Or":[686]},{"Call":[151,[[4,687]]]},{"And":[[688],null]},{"Or":[689]},{"Call":[149,[[2,68]]]},{"Token":63},{"Token":61},{"Opt":693},{"And":[[692,694],null]},{"Or":[695]},{"Opt":696},{"And":[[691,697],null]},{"Or":[698]},{"Call":[150,[[3,699]]]},{"And":[[700],null]},{"Or":[701]},{"Token":59},{"Not":703},{"And":[[69,704],null]},{"Token":92},{"Token":59},{"And":[[706,707,63],2]},{"Or":[705,708]},{"Token":28},{"Opt":710},{"Token":27},{"Opt":712},{"Token":92},{"And":[[711,713,714],null]},{"Or":[715]},{"And":[[79],null]},{"Or":[717]},{"Token":37},{"Token":38},{"And":[[719,720],null]},{"Or":[721]},{"Opt":73},{"And":[[63,723],null]},{"Or":[724]},{"Call":[151,[[4,725]]]},{"And":[[726],null]},{"Or":[727]},{"Token":61},{"Call":[149,[[2,63]]]},{"And":[[729,730],null]},{"Or":[731]},{"Token":77},{"Token":27},{"Opt":734},{"And":[[733,735,63],null]},{"Or":[736]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":740},{"And":[[738,739,741,112],null]},{"Or":[742]},{"IsIn":2},{"Not":77},{"And":[[744,745],null]},{"IsIn":2},{"Not":747},{"And":[[748],null]},{"Or":[746,749]},{"And":[[750,112],null]},{"Or":[751]},{"Token":62},{"Token":92},{"And":[[754],null]},{"Token":89},{"And":[[756],null]},{"Or":[755,757]},{"And":[[753,758],null]},{"Or":[759]},{"Call":[153,[[6,75]]]},{"Token":83},{"Token":5},{"And":[[763,49],null]},{"Or":[764]},{"Token":67},{"Token":75},{"Token":82},{"Token":67},{"And":[[769],null]},{"Token":69},{"And":[[771],null]},{"Token":71},{"And":[[773],null]},{"Or":[770,772,774]},{"Call":[126,[[1,775]]]},{"Token":73},{"And":[[777],null]},{"Token":75},{"And":[[779],null]},{"Or":[778,780]},{"Call":[126,[[1,781]]]},{"ContextualToken":[45,"<<"]},{"And":[[783],null]},{"ContextualToken":[47,">>"]},{"And":[[785],null]},{"Or":[784,786]},{"Call":[126,[[1,787]]]},{"Token":77},{"Token":77},{"Not":790},{"And":[[789,791],null]},{"Or":[792]},{"Call":[126,[[1,793]]]},{"Token":84},{"Call":[126,[[1,795]]]},{"Token":79},{"Token":79},{"Not":798},{"And":[[797,799],null]},{"Or":[800]},{"Call":[126,[[1,801]]]},{"Call":[126,[[1,131]]]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,804]]]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,806]]]},{"Call":[126,[[1,139]]]},{"Token":53},{"And":[[809],null]},{"Token":74},{"And":[[811],null]},{"Token":76},{"And":[[813],null]},{"Token":68},{"And":[[815],null]},{"Token":70},{"And":[[817],null]},{"Token":72},{"And":[[819],null]},{"Token":78},{"And":[[821],null]},{"Token":80},{"And":[[823],null]},{"Token":85},{"And":[[825],null]},{"ContextualToken":[48,">>="]},{"And":[[827],null]},{"ContextualToken":[46,"<<="]},{"And":[[829],null]},{"Or":[810,812,814,816,818,820,822,824,826,828,830]},{"Call":[126,[[1,831]]]},{"And":[[79],null]},{"Token":92},{"And":[[834],null]},{"Token":18},{"And":[[836],null]},{"Token":19},{"And":[[838],null]},{"Token":41},{"And":[[840],null]},{"Token":60},{"And":[[842],null]},{"Token":37},{"And":[[844],null]},{"Token":43},{"And":[[846],null]},{"Token":79},{"And":[[848],null]},{"Token":31},{"And":[[850],null]},{"Token":39},{"And":[[852],null]},{"Token":14},{"And":[[854],null]},{"Token":25},{"And":[[856],null]},{"Token":24},{"And":[[858],null]},{"Token":23},{"And":[[860],null]},{"Token":30},{"And":[[862],null]},{"Token":77},{"And":[[864],null]},{"Token":67},{"And":[[866],null]},{"Token":75},{"And":[[868],null]},{"Token":82},{"And":[[870],null]},{"Token":63},{"And":[[872],null]},{"Token":64},{"And":[[874],null]},{"PrevIs":[162,167,168,169,170,171,174]},{"And":[[876],null]},{"Var":0},{"Exit":[2,878]},{"Exit":[0,879]},{"And":[[880],null]},{"Token":89},{"And":[[882],null]},{"Token":90},{"And":[[884],null]},{"Token":91},{"And":[[886],null]},{"Token":86},{"And":[[888],null]},{"Token":88},{"And":[[890],null]},{"Or":[883,885,887,889,891]},{"Token":92},{"Token":82},{"And":[[893,894],null]},{"Or":[895]},{"Not":896},{"Opt":81},{"And":[[897,40,898],null]},{"Or":[899]},{"IsIn":0},{"Not":901},{"Call":[149,[[2,82]]]},{"Token":63},{"Call":[78,[[0,75]]]},{"And":[[904,905],null]},{"Or":[906]},{"Opt":907},{"And":[[903,908],null]},{"Or":[909]},{"Call":[150,[[3,910]]]},{"And":[[902,911],null]},{"Or":[912]},{"Token":92},{"Token":59},{"And":[[915,75],null]},{"Or":[916]},{"Opt":917},{"And":[[914,918],1]},{"Or":[919]},{"Token":37},{"Token":38},{"And":[[921,922],null]},{"Or":[923]},{"Call":[78,[[0,75]]]},{"Opt":85},{"And":[[925,926],null]},{"Or":[927]},{"Call":[151,[[4,928]]]},{"And":[[929],null]},{"Or":[930]},{"Token":61},{"Call":[78,[[0,75]]]},{"Call":[149,[[2,933]]]},{"And":[[932,934],null]},{"Or":[935]},{"Call":[149,[[2,75]]]},{"Call":[78,[[0,937]]]},{"Call":[153,[[6,938]]]},{"And":[[939],null]},{"Or":[940]},{"Token":26},{"Opt":942},{"Token":79},{"Rep":88},{"Token":79},{"Token":51},{"And":[[947,49,91],null]},{"Call":[78,[[0,75]]]},{"And":[[949],null]},{"Or":[948,950]},{"And":[[943,944,945,946,951],null]},{"Or":[952]},{"Token":61},{"And":[[954],null]},{"Token":79},{"Not":956},{"Not":957},{"And":[[958],null]},{"Or":[955,959]},{"And":[[13,960],1]},{"Token":31},{"Opt":75},{"And":[[962,963],null]},{"Or":[964]},{"Token":33},{"And":[[966],null]},{"Token":32},{"And":[[968],null]},{"Or":[967,969]},{"Token":87},{"Opt":971},{"And":[[970,972],null]},{"Or":[973]},{"Token":35},{"Opt":975},{"Rep":92},{"Opt":75},{"And":[[977,978],null]},{"Or":[979]},{"Call":[150,[[3,980]]]},{"And":[[976,981],null]},{"Or":[982]},{"Call":[78,[[0,983]]]},{"And":[[984],null]},{"Or":[985]},{"And":[[93],null]},{"And":[[96],null]},{"And":[[95],null]},{"And":[[3],null]},{"Token":9},{"Opt":94},{"Token":53},{"And":[[993,75],1]},{"Or":[994]},{"Opt":995},{"Token":58},{"And":[[991,63,992,996,997],1]},{"Or":[998]},{"Token":59},{"And":[[1000,49],null]},{"Or":[1001]},{"Token":58},{"And":[[1003],null]},{"Or":[1004]},"Eof",{"Not":1006},{"And":[[77,1007],null]},{"Token":58},{"And":[[1009],null]},{"Or":[1008,1010]},{"And":[[75,1011],null]},{"Or":[1012]},{"Enter":[2,1013]},{"And":[[1014],null]},{"Or":[1015]},{"Token":14},{"Token":15},{"And":[[91],null]},{"And":[[97],null]},{"Or":[1019,1020]},{"And":[[1018,1021],null]},{"Or":[1022]},{"Opt":1023},{"And":[[1017,99,91,1024],1]},{"Or":[1025]},{"Opt":103},{"Token":25},{"And":[[1027,1028,99,91],2]},{"Or":[1029]},{"Token":9},{"Token":53},{"And":[[1031,63,1032],1]},{"Or":[1033]},{"Opt":1034},{"And":[[1035,100],1]},{"Enter":[0,75]},{"And":[[1037],null]},{"Opt":103},{"Token":24},{"And":[[1039,1040,91],2]},{"Or":[1041]},{"Opt":103},{"Token":23},{"Token":34},{"And":[[1043,1044,63,1045,100,91],2]},{"Or":[1046]},{"Token":87},{"Token":59},{"And":[[1048,1049],null]},{"Token":30},{"Rep":105},{"Call":[150,[[3,1052]]]},{"And":[[1051,100,1053],1]},{"Or":[1054]},{"Token":52},{"Enter":[2,75]},{"Token":61},{"And":[[1058],null]},"Eof",{"And":[[1060],null]},{"And":[[77],null]},{"Or":[1059,1061,1062]},{"And":[[106,1056,1057,1063],1]},{"Or":[1064]},{"Token":79},{"And":[[1066,63],null]},{"Or":[1067]},{"Rep":1068},{"Opt":107},{"And":[[63,1069,1070],null]},{"Token":14},{"And":[[1072,75],null]},{"Or":[1073]},{"And":[[146],null]},{"Or":[1075]},{"And":[[147],null]},{"Or":[1077]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":1081},{"And":[[1079,1080,1082,112],null]},{"Or":[1083]},{"And":[[75,1084],null]},{"Or":[1085]},{"IsIn":2},{"Not":77},{"And":[[1087,1088],null]},{"IsIn":2},{"Not":1090},{"And":[[1091],null]},{"Or":[1089,1092]},{"And":[[1093,112],null]},{"Or":[1094]},{"And":[[75,1095],null]},{"Or":[1096]},{"Call":[149,[[2,113]]]},{"Call":[151,[[4,1098]]]},{"Call":[78,[[0,1099]]]},{"And":[[1100],null]},{"And":[[75],null]},{"Or":[1102]},{"Token":62},{"Token":92},{"And":[[1105],null]},{"Token":89},{"And":[[1107],null]},{"Or":[1106,1108]},{"And":[[1104,1109],null]},{"Or":[1110]},{"And":[[75,1111],null]},{"Or":[1112]},{"Call":[153,[[6,75]]]},{"And":[[75,1114],null]},{"Or":[1115]},{"Token":83},{"And":[[75,1117],null]},{"Or":[1118]},{"Token":5},{"And":[[1120,49],null]},{"Or":[1121]},{"And":[[75,1122],null]},{"Or":[1123]},{"And":[[119,75],null]},{"Or":[1125]},{"Token":77},{"Token":87},{"Opt":1128},{"Token":27},{"Opt":1130},{"And":[[1127,1129,1131],null]},{"Token":67},{"And":[[1133,75],null]},{"Or":[1134]},{"Token":75},{"And":[[1136,75],null]},{"Or":[1137]},{"Token":82},{"And":[[1139,75],null]},{"Or":[1140]},{"Token":67},{"And":[[1142],null]},{"Token":69},{"And":[[1144],null]},{"Token":71},{"And":[[1146],null]},{"Or":[1143,1145,1147]},{"Call":[126,[[1,1148]]]},{"And":[[75,1149,75],null]},{"Or":[1150]},{"Token":73},{"And":[[1152],null]},{"Token":75},{"And":[[1154],null]},{"Or":[1153,1155]},{"Call":[126,[[1,1156]]]},{"And":[[75,1157,75],null]},{"Or":[1158]},{"ContextualToken":[45,"<<"]},{"And":[[1160],null]},{"ContextualToken":[47,">>"]},{"And":[[1162],null]},{"Or":[1161,1163]},{"Call":[126,[[1,1164]]]},{"And":[[75,1165,75],null]},{"Or":[1166]},{"IsIn":2},{"Not":77},{"Var":1},{"And":[[1168,1169,1170],null]},{"IsIn":2},{"Not":1172},{"Var":1},{"And":[[1173,1174],null]},{"Token":77},{"Token":77},{"Not":1177},{"And":[[1176,1178],null]},{"Or":[1179]},{"Call":[126,[[1,1180]]]},{"And":[[75,1181,75],null]},{"Or":[1182]},{"Token":84},{"Call":[126,[[1,1184]]]},{"And":[[75,1185,75],null]},{"Or":[1186]},{"Token":79},{"Token":79},{"Not":1189},{"And":[[1188,1190],null]},{"Or":[1191]},{"Call":[126,[[1,1192]]]},{"And":[[75,1193,75],null]},{"Or":[1194]},{"Call":[126,[[1,131]]]},{"And":[[75,1196,75],null]},{"Or":[1197]},{"Token":54},{"And":[[1199],null]},{"Token":55},{"And":[[1201],null]},{"Token":41},{"And":[[1203],null]},{"Token":42},{"And":[[1205],null]},{"Token":57},{"And":[[1207],null]},{"Token":56},{"And":[[1209],null]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,1211]]]},{"And":[[75,1212,75],null]},{"Or":[1213]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,1215]]]},{"And":[[75,1216,75],null]},{"Or":[1217]},{"Call":[126,[[1,139]]]},{"And":[[75,1219,75],null]},{"Or":[1220]},{"And":[[139,75],null]},{"Or":[1222]},{"And":[[75,138],null]},{"Or":[1224]},{"Token":63},{"Not":76},{"And":[[1226,1227],null]},{"Or":[1228]},{"Token":63},{"And":[[1230],null]},{"Token":64},{"And":[[1232],null]},{"Not":76},{"Not":1234},{"Token":39},{"IsIn":0},{"And":[[1236,1237],null]},{"Or":[1238]},{"Not":1239},{"And":[[138,1235,1240],null]},{"Token":53},{"And":[[1242],null]},{"Token":74},{"And":[[1244],null]},{"Token":76},{"And":[[1246],null]},{"Token":68},{"And":[[1248],null]},{"Token":70},{"And":[[1250],null]},{"Token":72},{"And":[[1252],null]},{"Token":78},{"And":[[1254],null]},{"Token":80},{"And":[[1256],null]},{"Token":85},{"And":[[1258],null]},{"ContextualToken":[48,">>="]},{"And":[[1260],null]},{"ContextualToken":[46,"<<="]},{"And":[[1262],null]},{"Or":[1243,1245,1247,1249,1251,1253,1255,1257,1259,1261,1263]},{"Call":[126,[[1,1264]]]},{"And":[[75,1265,75],null]},{"Or":[1266]},{"Token":65},{"And":[[1268,143],null]},{"Or":[1269]},{"Token":65},{"Token":82},{"And":[[1271,1272,143],null]},{"Or":[1273]},{"Call":[149,[[2,145]]]},{"Call":[153,[[6,1275]]]},{"And":[[1276],null]},{"Rep":141},{"And":[[1278],null]},{"Token":92},{"Token":53},{"And":[[1281,75],null]},{"Call":[149,[[2,145]]]},{"Call":[151,[[4,1283]]]},{"And":[[1284],null]},{"Or":[1282,1285]},{"Opt":1286},{"And":[[1280,1287],1]},{"Or":[1288]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1292},{"Rep":148},{"Call":[150,[[3,1294]]]},{"And":[[1290,1291,1293,1295],null]},{"Or":[1296]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1300},{"Token":37},{"Rep":148},{"Token":38},{"And":[[1302,1303,1304],null]},{"Token":43},{"Rep":148},{"Token":44},{"And":[[1306,1307,1308],null]},{"Or":[1305,1309]},{"And":[[1298,1299,1301,1310],null]},{"Or":[1311]},{"Token":37},{"And":[[1313],null]},{"Token":38},{"And":[[1315],null]},{"Token":39},{"And":[[1317],null]},{"Token":40},{"And":[[1319],null]},{"Token":43},{"And":[[1321],null]},{"Token":44},{"And":[[1323],null]},{"Or":[1314,1316,1318,1320,1322,1324]},{"Not":1325},"Any",{"And":[[1326,1327],null]},{"Token":37},{"Rep":148},{"Token":38},{"And":[[1329,1330,1331],null]},{"Token":43},{"Rep":148},{"Token":44},{"And":[[1333,1334,1335],null]},{"Token":39},{"Rep":148},{"Token":40},{"And":[[1337,1338,1339],null]},{"Or":[1328,1332,1336,1340]},{"Var":2},"Eof",{"And":[[1343],null]},{"Token":61},{"And":[[1345],null]},{"Or":[1344,1346]},{"And":[[1342,1347],1]},{"Or":[1348]},{"Rep":1349},{"And":[[1350],null]},{"Token":39},{"Token":40},{"Var":3},{"Call":[154,[[7,1352],[8,1353],[9,1354]]]},{"And":[[1355],null]},{"Token":37},{"Token":38},{"Var":4},{"Call":[154,[[7,1357],[8,1358],[9,1359]]]},{"And":[[1360],null]},{"Token":41},{"Token":42},{"Var":5},{"Call":[154,[[7,1362],[8,1363],[9,1364]]]},{"And":[[1365],null]},{"Token":43},{"Token":44},{"Var":6},{"Call":[154,[[7,1367],[8,1368],[9,1369]]]},{"And":[[1370],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[155,[[10,1373],[11,1374]]]},{"Var":9},{"Layer":[1375,1376]},{"Var":8},{"And":[[1372,1377,1378],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[155,[[10,1381],[11,1382]]]},{"Var":11},{"And":[[1380,1383,1384],1]},{"Var":11},{"Not":1386},"Any",{"And":[[1387,1388],null]},{"Or":[1389]},{"And":[[1390],null]},{"Or":[1385,1391]},{"Rep":1392},{"And":[[1393],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, CONTINUE, BREAK, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, FOR_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, LOOP_CF_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, INNER_ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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