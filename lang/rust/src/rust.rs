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
        let parser_json = r##"[{"Pub":{"ty":93,"body":157,"replaceable":false}},{"Or":[160]},{"Or":[162,164,166,168,170,172,174,175]},{"Cached":192},{"Pub":{"ty":94,"body":208,"replaceable":false}},{"Pub":{"ty":95,"body":214,"replaceable":false}},{"Pub":{"ty":96,"body":220,"replaceable":false}},{"Pub":{"ty":97,"body":227,"replaceable":false}},{"Pub":{"ty":98,"body":243,"replaceable":false}},{"Or":[245]},{"Pub":{"ty":99,"body":250,"replaceable":false}},{"Or":[256]},{"Pub":{"ty":100,"body":259,"replaceable":false}},{"Pub":{"ty":101,"body":265,"replaceable":false}},{"Pub":{"ty":102,"body":282,"replaceable":false}},{"Pub":{"ty":103,"body":301,"replaceable":false}},{"Pub":{"ty":104,"body":306,"replaceable":false}},{"Pub":{"ty":105,"body":309,"replaceable":false}},{"Pub":{"ty":106,"body":316,"replaceable":false}},{"Pub":{"ty":107,"body":329,"replaceable":false}},{"Pub":{"ty":108,"body":338,"replaceable":false}},{"Pub":{"ty":109,"body":351,"replaceable":false}},{"Pub":{"ty":110,"body":359,"replaceable":false}},{"Pub":{"ty":111,"body":371,"replaceable":false}},{"Or":[372,373,374,375]},{"Or":[377,379,381,383,385,387,389,391,396]},{"Pub":{"ty":112,"body":406,"replaceable":false}},{"Pub":{"ty":113,"body":420,"replaceable":false}},{"Pub":{"ty":114,"body":424,"replaceable":false}},{"Pub":{"ty":115,"body":428,"replaceable":false}},{"Or":[436]},{"Pub":{"ty":116,"body":443,"replaceable":false}},{"Pub":{"ty":117,"body":447,"replaceable":false}},{"Or":[467]},{"Pub":{"ty":118,"body":471,"replaceable":false}},{"Pub":{"ty":119,"body":491,"replaceable":false}},{"Pub":{"ty":120,"body":501,"replaceable":false}},{"Pub":{"ty":121,"body":516,"replaceable":false}},{"Or":[517]},{"Or":[519]},{"Or":[521]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":122,"op":524,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":122,"body":528,"replaceable":false}},{"Pub":{"ty":122,"body":533,"replaceable":false}},{"Pub":{"ty":123,"body":540,"replaceable":false}},{"Pub":{"ty":124,"body":554,"replaceable":false}},{"Pub":{"ty":125,"body":580,"replaceable":false}},{"Pub":{"ty":126,"body":585,"replaceable":false}},{"Pub":{"ty":127,"body":589,"replaceable":false}},{"Or":[594]},{"Or":[595,596,597,598,599,600,601,602,603]},{"Pub":{"ty":128,"body":605,"replaceable":false}},{"Pub":{"ty":129,"body":607,"replaceable":false}},{"Pub":{"ty":130,"body":615,"replaceable":false}},{"Pub":{"ty":131,"body":618,"replaceable":false}},{"Pub":{"ty":132,"body":622,"replaceable":false}},{"Pub":{"ty":133,"body":628,"replaceable":true}},{"PubReplace":{"ty":134,"body":632}},{"Pub":{"ty":135,"body":635,"replaceable":false}},{"Pub":{"ty":136,"body":644,"replaceable":false}},{"Pub":{"ty":137,"body":651,"replaceable":false}},{"Pub":{"ty":100,"body":657,"replaceable":false}},{"Or":[658,659,660,661,662,663,664]},{"Pub":{"ty":138,"body":667,"replaceable":false}},{"Pub":{"ty":139,"body":673,"replaceable":true}},{"PubReplace":{"ty":140,"body":685}},{"PubReplace":{"ty":141,"body":697}},{"Pub":{"ty":142,"body":704,"replaceable":false}},{"Pub":{"ty":143,"body":711,"replaceable":false}},{"Pub":{"ty":144,"body":713,"replaceable":false}},{"Pub":{"ty":145,"body":717,"replaceable":false}},{"Pub":{"ty":146,"body":723,"replaceable":true}},{"PubReplace":{"ty":147,"body":727}},{"Pub":{"ty":148,"body":732,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,90,97,98,101,102,104,108,109,137],"prefixes":[{"ty":183,"op":119,"priority":999},{"ty":184,"op":761,"priority":999},{"ty":185,"op":762,"priority":999},{"ty":186,"op":763,"priority":999},{"ty":196,"op":139,"priority":2}],"infixes":[{"ty":176,"op":738,"priority":999,"has_rhs":false},{"ty":177,"op":747,"priority":999,"has_rhs":false},{"ty":179,"op":755,"priority":999,"has_rhs":false},{"ty":180,"op":756,"priority":999,"has_rhs":false},{"ty":181,"op":757,"priority":999,"has_rhs":false},{"ty":182,"op":760,"priority":999,"has_rhs":false},{"ty":187,"op":771,"priority":11,"has_rhs":true},{"ty":188,"op":777,"priority":10,"has_rhs":true},{"ty":189,"op":783,"priority":9,"has_rhs":true},{"ty":190,"op":789,"priority":8,"has_rhs":true},{"ty":191,"op":791,"priority":7,"has_rhs":true},{"ty":192,"op":797,"priority":6,"has_rhs":true},{"ty":193,"op":798,"priority":5,"has_rhs":true},{"ty":194,"op":800,"priority":4,"has_rhs":true},{"ty":195,"op":802,"priority":3,"has_rhs":true},{"ty":196,"op":803,"priority":2,"has_rhs":true},{"ty":196,"op":138,"priority":2,"has_rhs":false},{"ty":197,"op":827,"priority":1,"has_rhs":true}]}},{"Or":[828,830,832,834,836,838,840,842,844,846,848,850,852,854,856,858,860,862,864,866,868,870]},{"Or":[872]},{"Or":[876]},{"Pub":{"ty":150,"body":887,"replaceable":false}},{"Pub":{"ty":151,"body":895,"replaceable":true}},{"PubReplace":{"ty":152,"body":908}},{"Pub":{"ty":153,"body":915,"replaceable":false}},{"Pub":{"ty":154,"body":919,"replaceable":false}},{"Pub":{"ty":155,"body":926,"replaceable":true}},{"PubReplace":{"ty":156,"body":931}},{"Pub":{"ty":157,"body":936,"replaceable":false}},{"Pub":{"ty":158,"body":948,"replaceable":false}},{"Or":[956]},{"Pub":{"ty":159,"body":960,"replaceable":false}},{"Pub":{"ty":160,"body":969,"replaceable":false}},{"Pub":{"ty":161,"body":981,"replaceable":false}},{"Or":[982,983,984,985]},{"Pub":{"ty":162,"body":991,"replaceable":false}},{"Pub":{"ty":163,"body":994,"replaceable":false}},{"Pub":{"ty":164,"body":997,"replaceable":false}},{"Pub":{"ty":165,"body":1000,"replaceable":false}},{"Pub":{"ty":166,"body":1011,"replaceable":false}},{"Pub":{"ty":167,"body":1021,"replaceable":false}},{"Pub":{"ty":168,"body":1025,"replaceable":false}},{"Or":[1031]},{"Or":[1033]},{"Pub":{"ty":169,"body":1037,"replaceable":false}},{"Pub":{"ty":170,"body":1042,"replaceable":false}},{"Or":[1045]},{"Pub":{"ty":171,"body":1050,"replaceable":false}},{"Pub":{"ty":172,"body":1060,"replaceable":false}},{"Or":[1066]},{"Pub":{"ty":173,"body":1069,"replaceable":false}},{"Pub":{"ty":174,"body":1071,"replaceable":false}},{"Pub":{"ty":175,"body":1073,"replaceable":false}},{"Pub":{"ty":176,"body":1081,"replaceable":false}},{"Pub":{"ty":177,"body":1092,"replaceable":false}},{"Or":[1096]},{"Pub":{"ty":178,"body":1098,"replaceable":false}},{"Pub":{"ty":179,"body":1108,"replaceable":false}},{"Pub":{"ty":180,"body":1111,"replaceable":false}},{"Pub":{"ty":181,"body":1114,"replaceable":false}},{"Pub":{"ty":182,"body":1119,"replaceable":false}},{"Pub":{"ty":183,"body":1121,"replaceable":false}},{"Or":[1127]},{"Pub":{"ty":184,"body":1130,"replaceable":false}},{"Pub":{"ty":185,"body":1133,"replaceable":false}},{"Pub":{"ty":186,"body":1136,"replaceable":false}},{"Pub":{"ty":187,"body":1146,"replaceable":false}},{"Pub":{"ty":188,"body":1154,"replaceable":false}},{"Pub":{"ty":189,"body":1162,"replaceable":false}},{"Or":[1166,1170]},{"Pub":{"ty":190,"body":1178,"replaceable":false}},{"Pub":{"ty":191,"body":1182,"replaceable":false}},{"Pub":{"ty":192,"body":1190,"replaceable":false}},{"Pub":{"ty":193,"body":1193,"replaceable":false}},{"Or":[1195,1197,1199,1201,1203,1205]},{"Pub":{"ty":194,"body":1209,"replaceable":false}},{"Pub":{"ty":195,"body":1213,"replaceable":false}},{"Pub":{"ty":196,"body":1216,"replaceable":false}},{"Pub":{"ty":196,"body":1218,"replaceable":false}},{"Pub":{"ty":196,"body":1220,"replaceable":false}},{"Pub":{"ty":196,"body":1224,"replaceable":false}},{"Or":[1226,1228]},{"Or":[1236]},{"Pub":{"ty":197,"body":1262,"replaceable":false}},{"Pub":{"ty":198,"body":1265,"replaceable":false}},{"Pub":{"ty":199,"body":1269,"replaceable":false}},{"Or":[1272]},{"Or":[1274]},{"Pub":{"ty":200,"body":1284,"replaceable":false}},{"Pub":{"ty":201,"body":1292,"replaceable":false}},{"Pub":{"ty":202,"body":1307,"replaceable":false}},{"Pub":{"ty":203,"body":1336,"replaceable":false}},{"Or":[1346]},{"Or":[1351]},{"Or":[1356]},{"Or":[1361]},{"Or":[1366]},{"Or":[1374]},{"Or":[1389]},{"And":[[1],null]},{"Or":[156]},{"WithSkip":[2,3]},{"Rep":158},{"And":[[159],null]},{"Token":11},{"And":[[161],null]},{"ContextualToken":[4,"union"]},{"And":[[163],null]},{"Token":16},{"And":[[165],null]},{"Token":12},{"And":[[167],null]},{"Token":13},{"And":[[169],null]},{"Token":17},{"And":[[171],null]},{"Token":29},{"And":[[173],null]},{"And":[[25],null]},{"Opt":36},{"And":[[144,176],null]},{"Or":[177]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[179,180,181,182,183,184,185,186,187]},{"Inject":[178,188]},{"And":[[189],null]},{"And":[[28],null]},{"Or":[190,191]},{"Token":12},{"And":[[48],null]},{"Token":60},{"And":[[195,5],null]},{"Or":[196]},{"Opt":197},{"And":[[198],null]},{"Or":[194,199]},{"And":[[38,200],null]},{"Token":60},{"Opt":202},{"And":[[203,5],null]},{"Or":[201,204]},{"Token":58},{"And":[[193,205,206],1]},{"Or":[207]},{"Token":67},{"And":[[209],null]},{"Call":[149,[[2,6]]]},{"Call":[150,[[3,211]]]},{"And":[[212],null]},{"Or":[210,213]},{"Token":18},{"And":[[215],null]},{"Token":92},{"Opt":48},{"And":[[217,218],1]},{"Or":[216,219]},{"Token":7},{"Token":6},{"Token":92},{"Opt":48},{"Token":58},{"And":[[221,222,223,224,225],2]},{"Or":[226]},{"Token":21},{"Opt":228},{"Token":35},{"Opt":230},{"Opt":10},{"Token":8},{"Token":92},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[90],null]},{"Token":58},{"And":[[239],null]},{"Or":[238,240]},{"And":[[229,231,232,233,234,235,11,236,237,241],4]},{"Or":[242]},{"Token":51},{"And":[[244,49],null]},{"Token":7},{"Token":90},{"Opt":247},{"And":[[246,248],null]},{"Or":[249]},{"Opt":14},{"Call":[149,[[2,12]]]},{"And":[[251,252],null]},{"Or":[253]},{"Call":[151,[[4,254]]]},{"And":[[255],null]},{"Token":59},{"And":[[62,257,49],1]},{"Or":[258]},{"Token":59},{"And":[[260,49],null]},{"Or":[261]},{"Opt":262},{"And":[[62,263],null]},{"Or":[264]},{"And":[[119],null]},{"Token":27},{"And":[[267],null]},{"Or":[266,268]},{"Opt":269},{"Token":18},{"Token":59},{"And":[[272,49],null]},{"Or":[273]},{"Opt":274},{"Token":61},{"And":[[276],null]},"Eof",{"And":[[278],null]},{"Or":[277,279]},{"And":[[270,271,275,280],2]},{"Or":[281]},{"Token":11},{"And":[[283],null]},{"ContextualToken":[4,"union"]},{"And":[[285],null]},{"Or":[284,286]},{"Token":92},{"Opt":31},{"Call":[149,[[2,16]]]},{"Call":[150,[[3,290]]]},{"And":[[291],null]},{"Token":58},{"And":[[293],null]},{"Call":[149,[[2,17]]]},{"Call":[151,[[4,295]]]},{"Token":58},{"And":[[296,297],null]},{"Or":[292,294,298]},{"And":[[287,288,289,299],1]},{"Or":[300]},{"Opt":36},{"Token":92},{"Token":59},{"And":[[302,303,304,49],2]},{"Or":[305]},{"Opt":36},{"And":[[307,49],null]},{"Or":[308]},{"Token":16},{"Token":92},{"Opt":31},{"Call":[149,[[2,19]]]},{"Call":[150,[[3,313]]]},{"And":[[310,311,312,314],1]},{"Or":[315]},{"Token":92},{"Token":53},{"And":[[318,74],null]},{"Call":[149,[[2,17]]]},{"Call":[151,[[4,320]]]},{"And":[[321],null]},{"Call":[149,[[2,16]]]},{"Call":[150,[[3,323]]]},{"And":[[324],null]},{"Or":[319,322,325]},{"Opt":326},{"And":[[317,327],1]},{"Or":[328]},{"Token":13},{"Token":92},{"Token":58},{"And":[[332],null]},{"Call":[150,[[3,1]]]},{"And":[[334],null]},{"Or":[333,335]},{"And":[[330,331,336],1]},{"Or":[337]},{"Token":35},{"Opt":339},{"Token":17},{"Opt":31},{"Token":23},{"And":[[343,49],null]},{"Or":[344]},{"Opt":345},{"And":[[49,346],null]},{"Or":[347]},{"Opt":37},{"And":[[340,341,342,348,349,23],2]},{"Or":[350]},{"Opt":36},{"Token":29},{"Token":92},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[144,352,353,354,355,356,357,23],3]},{"Or":[358]},{"Opt":36},{"And":[[144,360],null]},{"Or":[361]},{"Inject":[362,24]},{"And":[[363],null]},{"And":[[28],null]},{"Or":[364,365]},{"WithSkip":[25,366]},{"Rep":367},{"Call":[150,[[3,368]]]},{"And":[[369],null]},{"Or":[370]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[142],null]},{"Token":10},{"And":[[376],null]},{"Token":8},{"And":[[378],null]},{"Token":20},{"And":[[380],null]},{"Token":21},{"And":[[382],null]},{"Token":22},{"And":[[384],null]},{"Token":35},{"And":[[386],null]},{"Token":65},{"And":[[388],null]},{"Token":7},{"And":[[390],null]},{"Token":92},{"Token":82},{"And":[[392,393],null]},{"Or":[394]},{"And":[[395],null]},{"Token":20},{"Token":92},{"Opt":31},{"Token":53},{"And":[[400,49],null]},{"Or":[401]},{"Opt":402},{"Token":58},{"And":[[397,398,399,403,404],1]},{"Or":[405]},{"Token":21},{"And":[[407],null]},{"Token":22},{"And":[[409],null]},{"Or":[408,410]},{"Token":92},{"Token":59},{"Token":53},{"And":[[414,74],null]},{"Or":[415]},{"Opt":416},{"Token":58},{"And":[[411,412,413,49,417,418],1]},{"Or":[419]},{"And":[[146],null]},{"Token":58},{"And":[[147,422],null]},{"Or":[421,423]},{"Rep":30},{"Call":[150,[[3,425]]]},{"And":[[10,426],null]},{"Or":[427]},{"Opt":36},{"And":[[144,429],null]},{"Or":[430]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[432,433]},{"Inject":[431,434]},{"And":[[435],null]},{"Call":[149,[[2,35]]]},{"Call":[149,[[2,32]]]},{"And":[[437,438],null]},{"Or":[439]},{"Call":[152,[[5,440]]]},{"And":[[441],null]},{"Or":[442]},{"Token":92},{"Opt":33},{"And":[[444,445],1]},{"Or":[446]},{"Token":59},{"Token":73},{"And":[[449],null]},"Eof",{"And":[[451],null]},{"Token":61},{"And":[[453],null]},{"Token":39},{"And":[[455],null]},{"Token":36},{"And":[[457],null]},{"Or":[454,456,458]},{"Not":459},{"Not":460},{"And":[[461],null]},{"Or":[450,452,462]},{"And":[[34,463],1]},{"Or":[464]},{"Rep":465},{"And":[[448,466],null]},{"Token":87},{"And":[[468],null]},{"And":[[51],null]},{"Or":[469,470]},{"Token":87},{"Token":59},{"Token":87},{"Token":73},{"And":[[475],null]},"Eof",{"And":[[477],null]},{"Token":61},{"Not":479},{"Not":480},{"And":[[481],null]},{"Or":[476,478,482]},{"And":[[474,483],1]},{"Or":[484]},{"Rep":485},{"And":[[473,486],null]},{"Or":[487]},{"Opt":488},{"And":[[472,489],1]},{"Or":[490]},{"Token":10},{"Token":6},{"And":[[493],null]},{"Token":19},{"And":[[495],null]},{"Or":[494,496]},{"Call":[151,[[4,497]]]},{"Opt":498},{"And":[[492,499],null]},{"Or":[500]},{"Token":36},{"Token":61},{"And":[[503],null]},"Eof",{"And":[[505],null]},{"Token":39},{"Not":507},{"Not":508},{"And":[[509],null]},{"Or":[504,506,510]},{"And":[[49,33,511],null]},{"Or":[512]},{"Rep":513},{"And":[[502,514],1]},{"Or":[515]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[518],null]},{"Enter":[1,41]},{"And":[[520],null]},{"Token":60},{"And":[[522,45],null]},{"Or":[523]},{"Token":60},{"Opt":525},{"And":[[526,45],null]},{"Or":[527]},{"Token":60},{"And":[[529,45],null]},{"Or":[530]},{"And":[[41,531],null]},{"Or":[532]},{"Token":5},{"And":[[49,534,49],null]},{"Or":[535]},{"Call":[152,[[5,536]]]},{"Token":60},{"And":[[537,538,45],null]},{"Or":[539]},{"Token":92},{"And":[[541],null]},{"Token":18},{"And":[[543],null]},{"Token":19},{"And":[[545],null]},{"Or":[542,544,546]},{"And":[[46],null]},{"IsIn":3},{"And":[[549,47],null]},{"Or":[548,550]},{"Opt":551},{"And":[[547,552],null]},{"Or":[553]},{"IsIn":3},{"And":[[555],null]},{"IsIn":1},{"Token":60},{"And":[[557,558],null]},{"Or":[556,559]},{"Token":87},{"Call":[149,[[2,561]]]},{"Token":92},{"Token":53},{"And":[[563,564],null]},{"Or":[565]},{"Not":566},{"And":[[567,49],null]},{"Or":[568]},{"Call":[149,[[2,569]]]},{"Token":92},{"Token":53},{"And":[[571,572,49],null]},{"Or":[573]},{"Call":[149,[[2,574]]]},{"And":[[562,570,575],null]},{"Or":[576]},{"Call":[152,[[5,577]]]},{"And":[[560,578],null]},{"Or":[579]},{"Call":[149,[[2,49]]]},{"Call":[151,[[4,581]]]},{"Opt":9},{"And":[[582,583],null]},{"Or":[584]},{"Token":5},{"Token":92},{"And":[[586,587],null]},{"Or":[588]},{"Token":73},{"And":[[590,34],null]},{"Or":[591]},{"Rep":592},{"And":[[50,593],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[604]},{"And":[[119,49],null]},{"Or":[606]},{"Token":67},{"Token":21},{"And":[[609],null]},{"Token":27},{"And":[[611],null]},{"Or":[610,612]},{"And":[[608,613,49],null]},{"Or":[614]},{"Token":81},{"And":[[616],null]},{"Or":[617]},{"Token":37},{"Token":38},{"And":[[619,620],null]},{"Or":[621]},{"Opt":57},{"And":[[49,623],null]},{"Or":[624]},{"Call":[151,[[4,625]]]},{"And":[[626],null]},{"Or":[627]},{"Token":61},{"Call":[149,[[2,49]]]},{"And":[[629,630],null]},{"Or":[631]},{"Token":82},{"And":[[633],null]},{"Or":[634]},{"Token":58},{"And":[[636,74],null]},{"Or":[637]},{"Opt":638},{"And":[[49,639],null]},{"Or":[640]},{"Call":[153,[[6,641]]]},{"And":[[642],null]},{"Or":[643]},{"Opt":10},{"Token":8},{"Call":[149,[[2,61]]]},{"Call":[151,[[4,647]]]},{"Opt":9},{"And":[[645,646,648,649],2]},{"Or":[650]},{"Token":59},{"And":[[62,652],null]},{"Or":[653]},{"Opt":654},{"And":[[655,49],null]},{"Or":[656]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":81},{"And":[[665],null]},{"Or":[666]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[668,669]},{"Opt":670},{"And":[[40,671],null]},{"Or":[672]},{"Call":[149,[[2,62]]]},{"Token":63},{"Token":61},{"Opt":676},{"And":[[675,677],null]},{"Or":[678]},{"Opt":679},{"And":[[674,680],null]},{"Or":[681]},{"Call":[151,[[4,682]]]},{"And":[[683],null]},{"Or":[684]},{"Call":[149,[[2,67]]]},{"Token":63},{"Token":61},{"Opt":688},{"And":[[687,689],null]},{"Or":[690]},{"Opt":691},{"And":[[686,692],null]},{"Or":[693]},{"Call":[150,[[3,694]]]},{"And":[[695],null]},{"Or":[696]},{"Token":59},{"Not":698},{"And":[[68,699],null]},{"Token":92},{"Token":59},{"And":[[701,702,62],2]},{"Or":[700,703]},{"Token":28},{"Opt":705},{"Token":27},{"Opt":707},{"Token":92},{"And":[[706,708,709],null]},{"Or":[710]},{"And":[[78],null]},{"Or":[712]},{"Token":37},{"Token":38},{"And":[[714,715],null]},{"Or":[716]},{"Opt":72},{"And":[[62,718],null]},{"Or":[719]},{"Call":[151,[[4,720]]]},{"And":[[721],null]},{"Or":[722]},{"Token":61},{"Call":[149,[[2,62]]]},{"And":[[724,725],null]},{"Or":[726]},{"Token":77},{"Token":27},{"Opt":729},{"And":[[728,730,62],null]},{"Or":[731]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":735},{"And":[[733,734,736,112],null]},{"Or":[737]},{"IsIn":2},{"Not":76},{"And":[[739,740],null]},{"IsIn":2},{"Not":742},{"And":[[743],null]},{"Or":[741,744]},{"And":[[745,112],null]},{"Or":[746]},{"Token":62},{"Token":92},{"And":[[749],null]},{"Token":89},{"And":[[751],null]},{"Or":[750,752]},{"And":[[748,753],null]},{"Or":[754]},{"Call":[153,[[6,74]]]},{"Token":83},{"Token":5},{"And":[[758,49],null]},{"Or":[759]},{"Token":67},{"Token":75},{"Token":82},{"Token":67},{"And":[[764],null]},{"Token":69},{"And":[[766],null]},{"Token":71},{"And":[[768],null]},{"Or":[765,767,769]},{"Call":[126,[[1,770]]]},{"Token":73},{"And":[[772],null]},{"Token":75},{"And":[[774],null]},{"Or":[773,775]},{"Call":[126,[[1,776]]]},{"ContextualToken":[45,"<<"]},{"And":[[778],null]},{"ContextualToken":[47,">>"]},{"And":[[780],null]},{"Or":[779,781]},{"Call":[126,[[1,782]]]},{"Token":77},{"Token":77},{"Not":785},{"And":[[784,786],null]},{"Or":[787]},{"Call":[126,[[1,788]]]},{"Token":84},{"Call":[126,[[1,790]]]},{"Token":79},{"Token":79},{"Not":793},{"And":[[792,794],null]},{"Or":[795]},{"Call":[126,[[1,796]]]},{"Call":[126,[[1,131]]]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,799]]]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,801]]]},{"Call":[126,[[1,139]]]},{"Token":53},{"And":[[804],null]},{"Token":74},{"And":[[806],null]},{"Token":76},{"And":[[808],null]},{"Token":68},{"And":[[810],null]},{"Token":70},{"And":[[812],null]},{"Token":72},{"And":[[814],null]},{"Token":78},{"And":[[816],null]},{"Token":80},{"And":[[818],null]},{"Token":85},{"And":[[820],null]},{"ContextualToken":[48,">>="]},{"And":[[822],null]},{"ContextualToken":[46,"<<="]},{"And":[[824],null]},{"Or":[805,807,809,811,813,815,817,819,821,823,825]},{"Call":[126,[[1,826]]]},{"And":[[78],null]},{"Token":92},{"And":[[829],null]},{"Token":18},{"And":[[831],null]},{"Token":19},{"And":[[833],null]},{"Token":41},{"And":[[835],null]},{"Token":60},{"And":[[837],null]},{"Token":37},{"And":[[839],null]},{"Token":43},{"And":[[841],null]},{"Token":79},{"And":[[843],null]},{"Token":31},{"And":[[845],null]},{"Token":39},{"And":[[847],null]},{"Token":14},{"And":[[849],null]},{"Token":25},{"And":[[851],null]},{"Token":24},{"And":[[853],null]},{"Token":23},{"And":[[855],null]},{"Token":30},{"And":[[857],null]},{"Token":77},{"And":[[859],null]},{"Token":67},{"And":[[861],null]},{"Token":75},{"And":[[863],null]},{"Token":82},{"And":[[865],null]},{"Token":63},{"And":[[867],null]},{"Token":64},{"And":[[869],null]},{"PrevIs":[161,167,168,169,170,171,174]},{"And":[[871],null]},{"Var":0},{"Exit":[2,873]},{"Exit":[0,874]},{"And":[[875],null]},{"Token":89},{"And":[[877],null]},{"Token":90},{"And":[[879],null]},{"Token":91},{"And":[[881],null]},{"Token":86},{"And":[[883],null]},{"Token":88},{"And":[[885],null]},{"Or":[878,880,882,884,886]},{"Token":92},{"Token":82},{"And":[[888,889],null]},{"Or":[890]},{"Not":891},{"Opt":80},{"And":[[892,40,893],null]},{"Or":[894]},{"IsIn":0},{"Not":896},{"Call":[149,[[2,81]]]},{"Token":63},{"Call":[77,[[0,74]]]},{"And":[[899,900],null]},{"Or":[901]},{"Opt":902},{"And":[[898,903],null]},{"Or":[904]},{"Call":[150,[[3,905]]]},{"And":[[897,906],null]},{"Or":[907]},{"Token":92},{"Token":59},{"And":[[910,74],null]},{"Or":[911]},{"Opt":912},{"And":[[909,913],1]},{"Or":[914]},{"Token":37},{"Token":38},{"And":[[916,917],null]},{"Or":[918]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[920,921],null]},{"Or":[922]},{"Call":[151,[[4,923]]]},{"And":[[924],null]},{"Or":[925]},{"Token":61},{"Call":[77,[[0,74]]]},{"Call":[149,[[2,928]]]},{"And":[[927,929],null]},{"Or":[930]},{"Call":[149,[[2,74]]]},{"Call":[77,[[0,932]]]},{"Call":[153,[[6,933]]]},{"And":[[934],null]},{"Or":[935]},{"Token":26},{"Opt":937},{"Token":79},{"Rep":87},{"Token":79},{"Token":51},{"And":[[942,49,90],null]},{"Call":[77,[[0,74]]]},{"And":[[944],null]},{"Or":[943,945]},{"And":[[938,939,940,941,946],null]},{"Or":[947]},{"Token":61},{"And":[[949],null]},{"Token":79},{"Not":951},{"Not":952},{"And":[[953],null]},{"Or":[950,954]},{"And":[[13,955],1]},{"Token":31},{"Opt":74},{"And":[[957,958],null]},{"Or":[959]},{"Token":33},{"And":[[961],null]},{"Token":32},{"And":[[963],null]},{"Or":[962,964]},{"Token":87},{"Opt":966},{"And":[[965,967],null]},{"Or":[968]},{"Token":35},{"Opt":970},{"Rep":91},{"Opt":74},{"And":[[972,973],null]},{"Or":[974]},{"Call":[150,[[3,975]]]},{"And":[[971,976],null]},{"Or":[977]},{"Call":[77,[[0,978]]]},{"And":[[979],null]},{"Or":[980]},{"And":[[92],null]},{"And":[[96],null]},{"And":[[95],null]},{"And":[[3],null]},{"Token":9},{"Opt":93},{"Opt":94},{"Token":58},{"And":[[986,62,987,988,989],1]},{"Or":[990]},{"Token":59},{"And":[[992,49],null]},{"Or":[993]},{"Token":53},{"And":[[995,74],null]},{"Or":[996]},{"Token":58},{"And":[[998],null]},{"Or":[999]},"Eof",{"Not":1001},{"And":[[76,1002],null]},{"Token":58},{"And":[[1004],null]},{"Or":[1003,1005]},{"And":[[74,1006],null]},{"Or":[1007]},{"Enter":[2,1008]},{"And":[[1009],null]},{"Or":[1010]},{"Token":14},{"Token":15},{"And":[[90],null]},{"And":[[97],null]},{"Or":[1014,1015]},{"And":[[1013,1016],null]},{"Or":[1017]},{"Opt":1018},{"And":[[1012,99,90,1019],1]},{"Or":[1020]},{"Opt":103},{"Token":25},{"And":[[1022,1023,99,90],2]},{"Or":[1024]},{"Token":9},{"Token":53},{"And":[[1026,62,1027],1]},{"Or":[1028]},{"Opt":1029},{"And":[[1030,100],null]},{"Enter":[0,74]},{"And":[[1032],null]},{"Opt":103},{"Token":24},{"And":[[1034,1035,90],2]},{"Or":[1036]},{"Opt":103},{"Token":23},{"Token":34},{"And":[[1038,1039,62,1040,100,90],2]},{"Or":[1041]},{"Token":87},{"Token":59},{"And":[[1043,1044],null]},{"Token":30},{"Rep":105},{"Call":[150,[[3,1047]]]},{"And":[[1046,100,1048],1]},{"Or":[1049]},{"Token":52},{"Enter":[2,74]},{"Token":61},{"And":[[1053],null]},"Eof",{"And":[[1055],null]},{"And":[[76],null]},{"Or":[1054,1056,1057]},{"And":[[106,1051,1052,1058],1]},{"Or":[1059]},{"Token":79},{"And":[[1061,62],null]},{"Or":[1062]},{"Rep":1063},{"Opt":107},{"And":[[62,1064,1065],null]},{"Token":14},{"And":[[1067,74],null]},{"Or":[1068]},{"And":[[146],null]},{"Or":[1070]},{"And":[[147],null]},{"Or":[1072]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":1076},{"And":[[1074,1075,1077,112],null]},{"Or":[1078]},{"And":[[74,1079],null]},{"Or":[1080]},{"IsIn":2},{"Not":76},{"And":[[1082,1083],null]},{"IsIn":2},{"Not":1085},{"And":[[1086],null]},{"Or":[1084,1087]},{"And":[[1088,112],null]},{"Or":[1089]},{"And":[[74,1090],null]},{"Or":[1091]},{"Call":[149,[[2,113]]]},{"Call":[151,[[4,1093]]]},{"Call":[77,[[0,1094]]]},{"And":[[1095],null]},{"And":[[74],null]},{"Or":[1097]},{"Token":62},{"Token":92},{"And":[[1100],null]},{"Token":89},{"And":[[1102],null]},{"Or":[1101,1103]},{"And":[[1099,1104],null]},{"Or":[1105]},{"And":[[74,1106],null]},{"Or":[1107]},{"Call":[153,[[6,74]]]},{"And":[[74,1109],null]},{"Or":[1110]},{"Token":83},{"And":[[74,1112],null]},{"Or":[1113]},{"Token":5},{"And":[[1115,49],null]},{"Or":[1116]},{"And":[[74,1117],null]},{"Or":[1118]},{"And":[[119,74],null]},{"Or":[1120]},{"Token":77},{"Token":87},{"Opt":1123},{"Token":27},{"Opt":1125},{"And":[[1122,1124,1126],null]},{"Token":67},{"And":[[1128,74],null]},{"Or":[1129]},{"Token":75},{"And":[[1131,74],null]},{"Or":[1132]},{"Token":82},{"And":[[1134,74],null]},{"Or":[1135]},{"Token":67},{"And":[[1137],null]},{"Token":69},{"And":[[1139],null]},{"Token":71},{"And":[[1141],null]},{"Or":[1138,1140,1142]},{"Call":[126,[[1,1143]]]},{"And":[[74,1144,74],null]},{"Or":[1145]},{"Token":73},{"And":[[1147],null]},{"Token":75},{"And":[[1149],null]},{"Or":[1148,1150]},{"Call":[126,[[1,1151]]]},{"And":[[74,1152,74],null]},{"Or":[1153]},{"ContextualToken":[45,"<<"]},{"And":[[1155],null]},{"ContextualToken":[47,">>"]},{"And":[[1157],null]},{"Or":[1156,1158]},{"Call":[126,[[1,1159]]]},{"And":[[74,1160,74],null]},{"Or":[1161]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1163,1164,1165],null]},{"IsIn":2},{"Not":1167},{"Var":1},{"And":[[1168,1169],null]},{"Token":77},{"Token":77},{"Not":1172},{"And":[[1171,1173],null]},{"Or":[1174]},{"Call":[126,[[1,1175]]]},{"And":[[74,1176,74],null]},{"Or":[1177]},{"Token":84},{"Call":[126,[[1,1179]]]},{"And":[[74,1180,74],null]},{"Or":[1181]},{"Token":79},{"Token":79},{"Not":1184},{"And":[[1183,1185],null]},{"Or":[1186]},{"Call":[126,[[1,1187]]]},{"And":[[74,1188,74],null]},{"Or":[1189]},{"Call":[126,[[1,131]]]},{"And":[[74,1191,74],null]},{"Or":[1192]},{"Token":54},{"And":[[1194],null]},{"Token":55},{"And":[[1196],null]},{"Token":41},{"And":[[1198],null]},{"Token":42},{"And":[[1200],null]},{"Token":57},{"And":[[1202],null]},{"Token":56},{"And":[[1204],null]},{"ContextualToken":[49,"&&"]},{"Call":[126,[[1,1206]]]},{"And":[[74,1207,74],null]},{"Or":[1208]},{"ContextualToken":[50,"||"]},{"Call":[126,[[1,1210]]]},{"And":[[74,1211,74],null]},{"Or":[1212]},{"Call":[126,[[1,139]]]},{"And":[[74,1214,74],null]},{"Or":[1215]},{"And":[[139,74],null]},{"Or":[1217]},{"And":[[74,138],null]},{"Or":[1219]},{"Token":63},{"Not":75},{"And":[[1221,1222],null]},{"Or":[1223]},{"Token":63},{"And":[[1225],null]},{"Token":64},{"And":[[1227],null]},{"Not":75},{"Not":1229},{"Token":39},{"IsIn":0},{"And":[[1231,1232],null]},{"Or":[1233]},{"Not":1234},{"And":[[138,1230,1235],null]},{"Token":53},{"And":[[1237],null]},{"Token":74},{"And":[[1239],null]},{"Token":76},{"And":[[1241],null]},{"Token":68},{"And":[[1243],null]},{"Token":70},{"And":[[1245],null]},{"Token":72},{"And":[[1247],null]},{"Token":78},{"And":[[1249],null]},{"Token":80},{"And":[[1251],null]},{"Token":85},{"And":[[1253],null]},{"ContextualToken":[48,">>="]},{"And":[[1255],null]},{"ContextualToken":[46,"<<="]},{"And":[[1257],null]},{"Or":[1238,1240,1242,1244,1246,1248,1250,1252,1254,1256,1258]},{"Call":[126,[[1,1259]]]},{"And":[[74,1260,74],null]},{"Or":[1261]},{"Token":65},{"And":[[1263,143],null]},{"Or":[1264]},{"Token":65},{"Token":82},{"And":[[1266,1267,143],null]},{"Or":[1268]},{"Call":[149,[[2,145]]]},{"Call":[153,[[6,1270]]]},{"And":[[1271],null]},{"Rep":141},{"And":[[1273],null]},{"Token":92},{"Token":53},{"And":[[1276,74],null]},{"Call":[149,[[2,145]]]},{"Call":[151,[[4,1278]]]},{"And":[[1279],null]},{"Or":[1277,1280]},{"Opt":1281},{"And":[[1275,1282],1]},{"Or":[1283]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1287},{"Rep":148},{"Call":[150,[[3,1289]]]},{"And":[[1285,1286,1288,1290],null]},{"Or":[1291]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1295},{"Token":37},{"Rep":148},{"Token":38},{"And":[[1297,1298,1299],null]},{"Token":43},{"Rep":148},{"Token":44},{"And":[[1301,1302,1303],null]},{"Or":[1300,1304]},{"And":[[1293,1294,1296,1305],null]},{"Or":[1306]},{"Token":37},{"And":[[1308],null]},{"Token":38},{"And":[[1310],null]},{"Token":39},{"And":[[1312],null]},{"Token":40},{"And":[[1314],null]},{"Token":43},{"And":[[1316],null]},{"Token":44},{"And":[[1318],null]},{"Or":[1309,1311,1313,1315,1317,1319]},{"Not":1320},"Any",{"And":[[1321,1322],null]},{"Token":37},{"Rep":148},{"Token":38},{"And":[[1324,1325,1326],null]},{"Token":43},{"Rep":148},{"Token":44},{"And":[[1328,1329,1330],null]},{"Token":39},{"Rep":148},{"Token":40},{"And":[[1332,1333,1334],null]},{"Or":[1323,1327,1331,1335]},{"Var":2},"Eof",{"And":[[1338],null]},{"Token":61},{"And":[[1340],null]},{"Or":[1339,1341]},{"And":[[1337,1342],1]},{"Or":[1343]},{"Rep":1344},{"And":[[1345],null]},{"Token":39},{"Token":40},{"Var":3},{"Call":[154,[[7,1347],[8,1348],[9,1349]]]},{"And":[[1350],null]},{"Token":37},{"Token":38},{"Var":4},{"Call":[154,[[7,1352],[8,1353],[9,1354]]]},{"And":[[1355],null]},{"Token":41},{"Token":42},{"Var":5},{"Call":[154,[[7,1357],[8,1358],[9,1359]]]},{"And":[[1360],null]},{"Token":43},{"Token":44},{"Var":6},{"Call":[154,[[7,1362],[8,1363],[9,1364]]]},{"And":[[1365],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[155,[[10,1368],[11,1369]]]},{"Var":9},{"Layer":[1370,1371]},{"Var":8},{"And":[[1367,1372,1373],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[155,[[10,1376],[11,1377]]]},{"Var":11},{"And":[[1375,1378,1379],1]},{"Var":11},{"Not":1381},"Any",{"And":[[1382,1383],null]},{"Or":[1384]},{"And":[[1385],null]},{"Or":[1380,1386]},{"Rep":1387},{"And":[[1388],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, CONTINUE, BREAK, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, LOOP_CF_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, INNER_ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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
                        INITIALIZER => rt::NodeTypeInfo { name: "INITIALIZER", whitespace_like: false },
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
impl<'f> NameOwner<'f> for TypeParameter<'f> {}
pub trait TypeParametersOwner<'f>: rt::AstNode<'f> {
    fn type_parameters(&self) -> Option<TypeParameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}
impl<'f> TypeParametersOwner<'f> for StructDef<'f> {}
impl<'f> TypeParametersOwner<'f> for EnumDef<'f> {}