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
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(263);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(264);
pub const IF_EXPR: rt::NodeType = rt::NodeType(265);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(266);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(267);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(268);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(269);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(270);
pub const GUARD: rt::NodeType = rt::NodeType(271);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(272);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(273);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(274);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(275);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(276);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(277);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(278);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(279);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(280);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(281);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(282);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(283);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(284);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(285);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(286);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(287);
pub const BIT_AND: rt::NodeType = rt::NodeType(288);
pub const BIT_XOR: rt::NodeType = rt::NodeType(289);
pub const BIT_OR: rt::NodeType = rt::NodeType(290);
pub const COMPARISON: rt::NodeType = rt::NodeType(291);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(292);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(293);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(294);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(295);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(296);
pub const INNER_ATTRIBUTE: rt::NodeType = rt::NodeType(297);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(298);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(299);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(300);
pub const TT: rt::NodeType = rt::NodeType(301);


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
        let parser_json = r##"[{"Pub":{"ty":93,"body":156,"replaceable":false}},{"Or":[159]},{"Or":[161,163,165,167,169,171,173,174]},{"Cached":191},{"Pub":{"ty":94,"body":207,"replaceable":false}},{"Pub":{"ty":95,"body":213,"replaceable":false}},{"Pub":{"ty":96,"body":219,"replaceable":false}},{"Pub":{"ty":97,"body":226,"replaceable":false}},{"Pub":{"ty":98,"body":242,"replaceable":false}},{"Or":[244]},{"Pub":{"ty":99,"body":249,"replaceable":false}},{"Or":[255]},{"Pub":{"ty":100,"body":258,"replaceable":false}},{"Pub":{"ty":101,"body":264,"replaceable":false}},{"Pub":{"ty":102,"body":281,"replaceable":false}},{"Pub":{"ty":103,"body":300,"replaceable":false}},{"Pub":{"ty":104,"body":305,"replaceable":false}},{"Pub":{"ty":105,"body":308,"replaceable":false}},{"Pub":{"ty":106,"body":315,"replaceable":false}},{"Pub":{"ty":107,"body":328,"replaceable":false}},{"Pub":{"ty":108,"body":337,"replaceable":false}},{"Pub":{"ty":109,"body":350,"replaceable":false}},{"Pub":{"ty":110,"body":358,"replaceable":false}},{"Pub":{"ty":111,"body":370,"replaceable":false}},{"Or":[371,372,373,374]},{"Or":[376,378,380,382,384,386,388,390,395]},{"Pub":{"ty":112,"body":405,"replaceable":false}},{"Pub":{"ty":113,"body":419,"replaceable":false}},{"Pub":{"ty":114,"body":423,"replaceable":false}},{"Pub":{"ty":115,"body":427,"replaceable":false}},{"Or":[435]},{"Pub":{"ty":116,"body":442,"replaceable":false}},{"Pub":{"ty":117,"body":446,"replaceable":false}},{"Or":[466]},{"Pub":{"ty":118,"body":470,"replaceable":false}},{"Pub":{"ty":119,"body":490,"replaceable":false}},{"Pub":{"ty":120,"body":500,"replaceable":false}},{"Pub":{"ty":121,"body":515,"replaceable":false}},{"Or":[516]},{"Or":[518]},{"Or":[520]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":122,"op":523,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":122,"body":527,"replaceable":false}},{"Pub":{"ty":122,"body":532,"replaceable":false}},{"Pub":{"ty":123,"body":539,"replaceable":false}},{"Pub":{"ty":124,"body":553,"replaceable":false}},{"Pub":{"ty":125,"body":579,"replaceable":false}},{"Pub":{"ty":126,"body":584,"replaceable":false}},{"Pub":{"ty":127,"body":588,"replaceable":false}},{"Or":[593]},{"Or":[594,595,596,597,598,599,600,601,602]},{"Pub":{"ty":128,"body":604,"replaceable":false}},{"Pub":{"ty":129,"body":606,"replaceable":false}},{"Pub":{"ty":130,"body":614,"replaceable":false}},{"Pub":{"ty":131,"body":617,"replaceable":false}},{"Pub":{"ty":132,"body":621,"replaceable":false}},{"Pub":{"ty":133,"body":627,"replaceable":true}},{"PubReplace":{"ty":134,"body":631}},{"Pub":{"ty":135,"body":634,"replaceable":false}},{"Pub":{"ty":136,"body":643,"replaceable":false}},{"Pub":{"ty":137,"body":650,"replaceable":false}},{"Pub":{"ty":100,"body":656,"replaceable":false}},{"Or":[657,658,659,660,661,662,663]},{"Pub":{"ty":138,"body":666,"replaceable":false}},{"Pub":{"ty":139,"body":672,"replaceable":true}},{"PubReplace":{"ty":140,"body":684}},{"PubReplace":{"ty":141,"body":696}},{"Pub":{"ty":142,"body":703,"replaceable":false}},{"Pub":{"ty":143,"body":710,"replaceable":false}},{"Pub":{"ty":144,"body":712,"replaceable":false}},{"Pub":{"ty":145,"body":716,"replaceable":false}},{"Pub":{"ty":146,"body":722,"replaceable":true}},{"PubReplace":{"ty":147,"body":726}},{"Pub":{"ty":148,"body":731,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,90,96,97,100,101,103,107,108,136],"prefixes":[{"ty":182,"op":118,"priority":999},{"ty":183,"op":760,"priority":999},{"ty":184,"op":761,"priority":999},{"ty":185,"op":762,"priority":999},{"ty":195,"op":138,"priority":2}],"infixes":[{"ty":175,"op":737,"priority":999,"has_rhs":false},{"ty":176,"op":746,"priority":999,"has_rhs":false},{"ty":178,"op":754,"priority":999,"has_rhs":false},{"ty":179,"op":755,"priority":999,"has_rhs":false},{"ty":180,"op":756,"priority":999,"has_rhs":false},{"ty":181,"op":759,"priority":999,"has_rhs":false},{"ty":186,"op":770,"priority":11,"has_rhs":true},{"ty":187,"op":776,"priority":10,"has_rhs":true},{"ty":188,"op":782,"priority":9,"has_rhs":true},{"ty":189,"op":788,"priority":8,"has_rhs":true},{"ty":190,"op":790,"priority":7,"has_rhs":true},{"ty":191,"op":796,"priority":6,"has_rhs":true},{"ty":192,"op":797,"priority":5,"has_rhs":true},{"ty":193,"op":799,"priority":4,"has_rhs":true},{"ty":194,"op":801,"priority":3,"has_rhs":true},{"ty":195,"op":802,"priority":2,"has_rhs":true},{"ty":195,"op":137,"priority":2,"has_rhs":false},{"ty":196,"op":826,"priority":1,"has_rhs":true}]}},{"Or":[827,829,831,833,835,837,839,841,843,845,847,849,851,853,855,857,859,861,863,865,867,869]},{"Or":[871]},{"Or":[875]},{"Pub":{"ty":150,"body":886,"replaceable":false}},{"Pub":{"ty":151,"body":894,"replaceable":true}},{"PubReplace":{"ty":152,"body":907}},{"Pub":{"ty":153,"body":914,"replaceable":false}},{"Pub":{"ty":154,"body":918,"replaceable":false}},{"Pub":{"ty":155,"body":925,"replaceable":true}},{"PubReplace":{"ty":156,"body":930}},{"Pub":{"ty":157,"body":935,"replaceable":false}},{"Pub":{"ty":158,"body":947,"replaceable":false}},{"Or":[955]},{"Pub":{"ty":159,"body":959,"replaceable":false}},{"Pub":{"ty":160,"body":968,"replaceable":false}},{"Pub":{"ty":161,"body":980,"replaceable":false}},{"Or":[981,982,983,984]},{"Pub":{"ty":162,"body":993,"replaceable":false}},{"Pub":{"ty":163,"body":996,"replaceable":false}},{"Pub":{"ty":164,"body":999,"replaceable":false}},{"Pub":{"ty":165,"body":1010,"replaceable":false}},{"Pub":{"ty":166,"body":1020,"replaceable":false}},{"Pub":{"ty":167,"body":1024,"replaceable":false}},{"Or":[1030]},{"Or":[1032]},{"Pub":{"ty":168,"body":1036,"replaceable":false}},{"Pub":{"ty":169,"body":1041,"replaceable":false}},{"Or":[1044]},{"Pub":{"ty":170,"body":1049,"replaceable":false}},{"Pub":{"ty":171,"body":1059,"replaceable":false}},{"Or":[1065]},{"Pub":{"ty":172,"body":1068,"replaceable":false}},{"Pub":{"ty":173,"body":1070,"replaceable":false}},{"Pub":{"ty":174,"body":1072,"replaceable":false}},{"Pub":{"ty":175,"body":1080,"replaceable":false}},{"Pub":{"ty":176,"body":1091,"replaceable":false}},{"Or":[1095]},{"Pub":{"ty":177,"body":1097,"replaceable":false}},{"Pub":{"ty":178,"body":1107,"replaceable":false}},{"Pub":{"ty":179,"body":1110,"replaceable":false}},{"Pub":{"ty":180,"body":1113,"replaceable":false}},{"Pub":{"ty":181,"body":1118,"replaceable":false}},{"Pub":{"ty":182,"body":1120,"replaceable":false}},{"Or":[1126]},{"Pub":{"ty":183,"body":1129,"replaceable":false}},{"Pub":{"ty":184,"body":1132,"replaceable":false}},{"Pub":{"ty":185,"body":1135,"replaceable":false}},{"Pub":{"ty":186,"body":1145,"replaceable":false}},{"Pub":{"ty":187,"body":1153,"replaceable":false}},{"Pub":{"ty":188,"body":1161,"replaceable":false}},{"Or":[1165,1169]},{"Pub":{"ty":189,"body":1177,"replaceable":false}},{"Pub":{"ty":190,"body":1181,"replaceable":false}},{"Pub":{"ty":191,"body":1189,"replaceable":false}},{"Pub":{"ty":192,"body":1192,"replaceable":false}},{"Or":[1194,1196,1198,1200,1202,1204]},{"Pub":{"ty":193,"body":1208,"replaceable":false}},{"Pub":{"ty":194,"body":1212,"replaceable":false}},{"Pub":{"ty":195,"body":1215,"replaceable":false}},{"Pub":{"ty":195,"body":1217,"replaceable":false}},{"Pub":{"ty":195,"body":1219,"replaceable":false}},{"Pub":{"ty":195,"body":1223,"replaceable":false}},{"Or":[1225,1227]},{"Or":[1235]},{"Pub":{"ty":196,"body":1261,"replaceable":false}},{"Pub":{"ty":197,"body":1264,"replaceable":false}},{"Pub":{"ty":198,"body":1268,"replaceable":false}},{"Or":[1271]},{"Or":[1273]},{"Pub":{"ty":199,"body":1283,"replaceable":false}},{"Pub":{"ty":200,"body":1291,"replaceable":false}},{"Pub":{"ty":201,"body":1306,"replaceable":false}},{"Pub":{"ty":202,"body":1335,"replaceable":false}},{"Or":[1345]},{"Or":[1350]},{"Or":[1355]},{"Or":[1360]},{"Or":[1365]},{"Or":[1373]},{"Or":[1388]},{"And":[[1],null]},{"Or":[155]},{"WithSkip":[2,3]},{"Rep":157},{"And":[[158],null]},{"Token":11},{"And":[[160],null]},{"ContextualToken":[4,"union"]},{"And":[[162],null]},{"Token":16},{"And":[[164],null]},{"Token":12},{"And":[[166],null]},{"Token":13},{"And":[[168],null]},{"Token":17},{"And":[[170],null]},{"Token":29},{"And":[[172],null]},{"And":[[25],null]},{"Opt":36},{"And":[[143,175],null]},{"Or":[176]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[178,179,180,181,182,183,184,185,186]},{"Inject":[177,187]},{"And":[[188],null]},{"And":[[28],null]},{"Or":[189,190]},{"Token":12},{"And":[[48],null]},{"Token":60},{"And":[[194,5],null]},{"Or":[195]},{"Opt":196},{"And":[[197],null]},{"Or":[193,198]},{"And":[[38,199],null]},{"Token":60},{"Opt":201},{"And":[[202,5],null]},{"Or":[200,203]},{"Token":58},{"And":[[192,204,205],1]},{"Or":[206]},{"Token":67},{"And":[[208],null]},{"Call":[148,[[2,6]]]},{"Call":[149,[[3,210]]]},{"And":[[211],null]},{"Or":[209,212]},{"Token":18},{"And":[[214],null]},{"Token":92},{"Opt":48},{"And":[[216,217],1]},{"Or":[215,218]},{"Token":7},{"Token":6},{"Token":92},{"Opt":48},{"Token":58},{"And":[[220,221,222,223,224],2]},{"Or":[225]},{"Token":21},{"Opt":227},{"Token":35},{"Opt":229},{"Opt":10},{"Token":8},{"Token":92},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[90],null]},{"Token":58},{"And":[[238],null]},{"Or":[237,239]},{"And":[[228,230,231,232,233,234,11,235,236,240],4]},{"Or":[241]},{"Token":51},{"And":[[243,49],null]},{"Token":7},{"Token":90},{"Opt":246},{"And":[[245,247],null]},{"Or":[248]},{"Opt":14},{"Call":[148,[[2,12]]]},{"And":[[250,251],null]},{"Or":[252]},{"Call":[150,[[4,253]]]},{"And":[[254],null]},{"Token":59},{"And":[[62,256,49],1]},{"Or":[257]},{"Token":59},{"And":[[259,49],null]},{"Or":[260]},{"Opt":261},{"And":[[62,262],null]},{"Or":[263]},{"And":[[118],null]},{"Token":27},{"And":[[266],null]},{"Or":[265,267]},{"Opt":268},{"Token":18},{"Token":59},{"And":[[271,49],null]},{"Or":[272]},{"Opt":273},{"Token":61},{"And":[[275],null]},"Eof",{"And":[[277],null]},{"Or":[276,278]},{"And":[[269,270,274,279],2]},{"Or":[280]},{"Token":11},{"And":[[282],null]},{"ContextualToken":[4,"union"]},{"And":[[284],null]},{"Or":[283,285]},{"Token":92},{"Opt":31},{"Call":[148,[[2,16]]]},{"Call":[149,[[3,289]]]},{"And":[[290],null]},{"Token":58},{"And":[[292],null]},{"Call":[148,[[2,17]]]},{"Call":[150,[[4,294]]]},{"Token":58},{"And":[[295,296],null]},{"Or":[291,293,297]},{"And":[[286,287,288,298],1]},{"Or":[299]},{"Opt":36},{"Token":92},{"Token":59},{"And":[[301,302,303,49],2]},{"Or":[304]},{"Opt":36},{"And":[[306,49],null]},{"Or":[307]},{"Token":16},{"Token":92},{"Opt":31},{"Call":[148,[[2,19]]]},{"Call":[149,[[3,312]]]},{"And":[[309,310,311,313],1]},{"Or":[314]},{"Token":92},{"Token":53},{"And":[[317,74],null]},{"Call":[148,[[2,17]]]},{"Call":[150,[[4,319]]]},{"And":[[320],null]},{"Call":[148,[[2,16]]]},{"Call":[149,[[3,322]]]},{"And":[[323],null]},{"Or":[318,321,324]},{"Opt":325},{"And":[[316,326],1]},{"Or":[327]},{"Token":13},{"Token":92},{"Token":58},{"And":[[331],null]},{"Call":[149,[[3,1]]]},{"And":[[333],null]},{"Or":[332,334]},{"And":[[329,330,335],1]},{"Or":[336]},{"Token":35},{"Opt":338},{"Token":17},{"Opt":31},{"Token":23},{"And":[[342,49],null]},{"Or":[343]},{"Opt":344},{"And":[[49,345],null]},{"Or":[346]},{"Opt":37},{"And":[[339,340,341,347,348,23],2]},{"Or":[349]},{"Opt":36},{"Token":29},{"Token":92},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[143,351,352,353,354,355,356,23],3]},{"Or":[357]},{"Opt":36},{"And":[[143,359],null]},{"Or":[360]},{"Inject":[361,24]},{"And":[[362],null]},{"And":[[28],null]},{"Or":[363,364]},{"WithSkip":[25,365]},{"Rep":366},{"Call":[149,[[3,367]]]},{"And":[[368],null]},{"Or":[369]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"And":[[141],null]},{"Token":10},{"And":[[375],null]},{"Token":8},{"And":[[377],null]},{"Token":20},{"And":[[379],null]},{"Token":21},{"And":[[381],null]},{"Token":22},{"And":[[383],null]},{"Token":35},{"And":[[385],null]},{"Token":65},{"And":[[387],null]},{"Token":7},{"And":[[389],null]},{"Token":92},{"Token":82},{"And":[[391,392],null]},{"Or":[393]},{"And":[[394],null]},{"Token":20},{"Token":92},{"Opt":31},{"Token":53},{"And":[[399,49],null]},{"Or":[400]},{"Opt":401},{"Token":58},{"And":[[396,397,398,402,403],1]},{"Or":[404]},{"Token":21},{"And":[[406],null]},{"Token":22},{"And":[[408],null]},{"Or":[407,409]},{"Token":92},{"Token":59},{"Token":53},{"And":[[413,74],null]},{"Or":[414]},{"Opt":415},{"Token":58},{"And":[[410,411,412,49,416,417],1]},{"Or":[418]},{"And":[[145],null]},{"Token":58},{"And":[[146,421],null]},{"Or":[420,422]},{"Rep":30},{"Call":[149,[[3,424]]]},{"And":[[10,425],null]},{"Or":[426]},{"Opt":36},{"And":[[143,428],null]},{"Or":[429]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[431,432]},{"Inject":[430,433]},{"And":[[434],null]},{"Call":[148,[[2,35]]]},{"Call":[148,[[2,32]]]},{"And":[[436,437],null]},{"Or":[438]},{"Call":[151,[[5,439]]]},{"And":[[440],null]},{"Or":[441]},{"Token":92},{"Opt":33},{"And":[[443,444],1]},{"Or":[445]},{"Token":59},{"Token":73},{"And":[[448],null]},"Eof",{"And":[[450],null]},{"Token":61},{"And":[[452],null]},{"Token":39},{"And":[[454],null]},{"Token":36},{"And":[[456],null]},{"Or":[453,455,457]},{"Not":458},{"Not":459},{"And":[[460],null]},{"Or":[449,451,461]},{"And":[[34,462],1]},{"Or":[463]},{"Rep":464},{"And":[[447,465],null]},{"Token":87},{"And":[[467],null]},{"And":[[51],null]},{"Or":[468,469]},{"Token":87},{"Token":59},{"Token":87},{"Token":73},{"And":[[474],null]},"Eof",{"And":[[476],null]},{"Token":61},{"Not":478},{"Not":479},{"And":[[480],null]},{"Or":[475,477,481]},{"And":[[473,482],1]},{"Or":[483]},{"Rep":484},{"And":[[472,485],null]},{"Or":[486]},{"Opt":487},{"And":[[471,488],1]},{"Or":[489]},{"Token":10},{"Token":6},{"And":[[492],null]},{"Token":19},{"And":[[494],null]},{"Or":[493,495]},{"Call":[150,[[4,496]]]},{"Opt":497},{"And":[[491,498],null]},{"Or":[499]},{"Token":36},{"Token":61},{"And":[[502],null]},"Eof",{"And":[[504],null]},{"Token":39},{"Not":506},{"Not":507},{"And":[[508],null]},{"Or":[503,505,509]},{"And":[[49,33,510],null]},{"Or":[511]},{"Rep":512},{"And":[[501,513],1]},{"Or":[514]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[517],null]},{"Enter":[1,41]},{"And":[[519],null]},{"Token":60},{"And":[[521,45],null]},{"Or":[522]},{"Token":60},{"Opt":524},{"And":[[525,45],null]},{"Or":[526]},{"Token":60},{"And":[[528,45],null]},{"Or":[529]},{"And":[[41,530],null]},{"Or":[531]},{"Token":5},{"And":[[49,533,49],null]},{"Or":[534]},{"Call":[151,[[5,535]]]},{"Token":60},{"And":[[536,537,45],null]},{"Or":[538]},{"Token":92},{"And":[[540],null]},{"Token":18},{"And":[[542],null]},{"Token":19},{"And":[[544],null]},{"Or":[541,543,545]},{"And":[[46],null]},{"IsIn":3},{"And":[[548,47],null]},{"Or":[547,549]},{"Opt":550},{"And":[[546,551],null]},{"Or":[552]},{"IsIn":3},{"And":[[554],null]},{"IsIn":1},{"Token":60},{"And":[[556,557],null]},{"Or":[555,558]},{"Token":87},{"Call":[148,[[2,560]]]},{"Token":92},{"Token":53},{"And":[[562,563],null]},{"Or":[564]},{"Not":565},{"And":[[566,49],null]},{"Or":[567]},{"Call":[148,[[2,568]]]},{"Token":92},{"Token":53},{"And":[[570,571,49],null]},{"Or":[572]},{"Call":[148,[[2,573]]]},{"And":[[561,569,574],null]},{"Or":[575]},{"Call":[151,[[5,576]]]},{"And":[[559,577],null]},{"Or":[578]},{"Call":[148,[[2,49]]]},{"Call":[150,[[4,580]]]},{"Opt":9},{"And":[[581,582],null]},{"Or":[583]},{"Token":5},{"Token":92},{"And":[[585,586],null]},{"Or":[587]},{"Token":73},{"And":[[589,34],null]},{"Or":[590]},{"Rep":591},{"And":[[50,592],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[603]},{"And":[[118,49],null]},{"Or":[605]},{"Token":67},{"Token":21},{"And":[[608],null]},{"Token":27},{"And":[[610],null]},{"Or":[609,611]},{"And":[[607,612,49],null]},{"Or":[613]},{"Token":81},{"And":[[615],null]},{"Or":[616]},{"Token":37},{"Token":38},{"And":[[618,619],null]},{"Or":[620]},{"Opt":57},{"And":[[49,622],null]},{"Or":[623]},{"Call":[150,[[4,624]]]},{"And":[[625],null]},{"Or":[626]},{"Token":61},{"Call":[148,[[2,49]]]},{"And":[[628,629],null]},{"Or":[630]},{"Token":82},{"And":[[632],null]},{"Or":[633]},{"Token":58},{"And":[[635,74],null]},{"Or":[636]},{"Opt":637},{"And":[[49,638],null]},{"Or":[639]},{"Call":[152,[[6,640]]]},{"And":[[641],null]},{"Or":[642]},{"Opt":10},{"Token":8},{"Call":[148,[[2,61]]]},{"Call":[150,[[4,646]]]},{"Opt":9},{"And":[[644,645,647,648],2]},{"Or":[649]},{"Token":59},{"And":[[62,651],null]},{"Or":[652]},{"Opt":653},{"And":[[654,49],null]},{"Or":[655]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":81},{"And":[[664],null]},{"Or":[665]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[667,668]},{"Opt":669},{"And":[[40,670],null]},{"Or":[671]},{"Call":[148,[[2,62]]]},{"Token":63},{"Token":61},{"Opt":675},{"And":[[674,676],null]},{"Or":[677]},{"Opt":678},{"And":[[673,679],null]},{"Or":[680]},{"Call":[150,[[4,681]]]},{"And":[[682],null]},{"Or":[683]},{"Call":[148,[[2,67]]]},{"Token":63},{"Token":61},{"Opt":687},{"And":[[686,688],null]},{"Or":[689]},{"Opt":690},{"And":[[685,691],null]},{"Or":[692]},{"Call":[149,[[3,693]]]},{"And":[[694],null]},{"Or":[695]},{"Token":59},{"Not":697},{"And":[[68,698],null]},{"Token":92},{"Token":59},{"And":[[700,701,62],2]},{"Or":[699,702]},{"Token":28},{"Opt":704},{"Token":27},{"Opt":706},{"Token":92},{"And":[[705,707,708],null]},{"Or":[709]},{"And":[[78],null]},{"Or":[711]},{"Token":37},{"Token":38},{"And":[[713,714],null]},{"Or":[715]},{"Opt":72},{"And":[[62,717],null]},{"Or":[718]},{"Call":[150,[[4,719]]]},{"And":[[720],null]},{"Or":[721]},{"Token":61},{"Call":[148,[[2,62]]]},{"And":[[723,724],null]},{"Or":[725]},{"Token":77},{"Token":27},{"Opt":728},{"And":[[727,729,62],null]},{"Or":[730]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":734},{"And":[[732,733,735,111],null]},{"Or":[736]},{"IsIn":2},{"Not":76},{"And":[[738,739],null]},{"IsIn":2},{"Not":741},{"And":[[742],null]},{"Or":[740,743]},{"And":[[744,111],null]},{"Or":[745]},{"Token":62},{"Token":92},{"And":[[748],null]},{"Token":89},{"And":[[750],null]},{"Or":[749,751]},{"And":[[747,752],null]},{"Or":[753]},{"Call":[152,[[6,74]]]},{"Token":83},{"Token":5},{"And":[[757,49],null]},{"Or":[758]},{"Token":67},{"Token":75},{"Token":82},{"Token":67},{"And":[[763],null]},{"Token":69},{"And":[[765],null]},{"Token":71},{"And":[[767],null]},{"Or":[764,766,768]},{"Call":[125,[[1,769]]]},{"Token":73},{"And":[[771],null]},{"Token":75},{"And":[[773],null]},{"Or":[772,774]},{"Call":[125,[[1,775]]]},{"ContextualToken":[45,"<<"]},{"And":[[777],null]},{"ContextualToken":[47,">>"]},{"And":[[779],null]},{"Or":[778,780]},{"Call":[125,[[1,781]]]},{"Token":77},{"Token":77},{"Not":784},{"And":[[783,785],null]},{"Or":[786]},{"Call":[125,[[1,787]]]},{"Token":84},{"Call":[125,[[1,789]]]},{"Token":79},{"Token":79},{"Not":792},{"And":[[791,793],null]},{"Or":[794]},{"Call":[125,[[1,795]]]},{"Call":[125,[[1,130]]]},{"ContextualToken":[49,"&&"]},{"Call":[125,[[1,798]]]},{"ContextualToken":[50,"||"]},{"Call":[125,[[1,800]]]},{"Call":[125,[[1,138]]]},{"Token":53},{"And":[[803],null]},{"Token":74},{"And":[[805],null]},{"Token":76},{"And":[[807],null]},{"Token":68},{"And":[[809],null]},{"Token":70},{"And":[[811],null]},{"Token":72},{"And":[[813],null]},{"Token":78},{"And":[[815],null]},{"Token":80},{"And":[[817],null]},{"Token":85},{"And":[[819],null]},{"ContextualToken":[48,">>="]},{"And":[[821],null]},{"ContextualToken":[46,"<<="]},{"And":[[823],null]},{"Or":[804,806,808,810,812,814,816,818,820,822,824]},{"Call":[125,[[1,825]]]},{"And":[[78],null]},{"Token":92},{"And":[[828],null]},{"Token":18},{"And":[[830],null]},{"Token":19},{"And":[[832],null]},{"Token":41},{"And":[[834],null]},{"Token":60},{"And":[[836],null]},{"Token":37},{"And":[[838],null]},{"Token":43},{"And":[[840],null]},{"Token":79},{"And":[[842],null]},{"Token":31},{"And":[[844],null]},{"Token":39},{"And":[[846],null]},{"Token":14},{"And":[[848],null]},{"Token":25},{"And":[[850],null]},{"Token":24},{"And":[[852],null]},{"Token":23},{"And":[[854],null]},{"Token":30},{"And":[[856],null]},{"Token":77},{"And":[[858],null]},{"Token":67},{"And":[[860],null]},{"Token":75},{"And":[[862],null]},{"Token":82},{"And":[[864],null]},{"Token":63},{"And":[[866],null]},{"Token":64},{"And":[[868],null]},{"PrevIs":[161,166,167,168,169,170,173]},{"And":[[870],null]},{"Var":0},{"Exit":[2,872]},{"Exit":[0,873]},{"And":[[874],null]},{"Token":89},{"And":[[876],null]},{"Token":90},{"And":[[878],null]},{"Token":91},{"And":[[880],null]},{"Token":86},{"And":[[882],null]},{"Token":88},{"And":[[884],null]},{"Or":[877,879,881,883,885]},{"Token":92},{"Token":82},{"And":[[887,888],null]},{"Or":[889]},{"Not":890},{"Opt":80},{"And":[[891,40,892],null]},{"Or":[893]},{"IsIn":0},{"Not":895},{"Call":[148,[[2,81]]]},{"Token":63},{"Call":[77,[[0,74]]]},{"And":[[898,899],null]},{"Or":[900]},{"Opt":901},{"And":[[897,902],null]},{"Or":[903]},{"Call":[149,[[3,904]]]},{"And":[[896,905],null]},{"Or":[906]},{"Token":92},{"Token":59},{"And":[[909,74],null]},{"Or":[910]},{"Opt":911},{"And":[[908,912],1]},{"Or":[913]},{"Token":37},{"Token":38},{"And":[[915,916],null]},{"Or":[917]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[919,920],null]},{"Or":[921]},{"Call":[150,[[4,922]]]},{"And":[[923],null]},{"Or":[924]},{"Token":61},{"Call":[77,[[0,74]]]},{"Call":[148,[[2,927]]]},{"And":[[926,928],null]},{"Or":[929]},{"Call":[148,[[2,74]]]},{"Call":[77,[[0,931]]]},{"Call":[152,[[6,932]]]},{"And":[[933],null]},{"Or":[934]},{"Token":26},{"Opt":936},{"Token":79},{"Rep":87},{"Token":79},{"Token":51},{"And":[[941,49,90],null]},{"Call":[77,[[0,74]]]},{"And":[[943],null]},{"Or":[942,944]},{"And":[[937,938,939,940,945],null]},{"Or":[946]},{"Token":61},{"And":[[948],null]},{"Token":79},{"Not":950},{"Not":951},{"And":[[952],null]},{"Or":[949,953]},{"And":[[13,954],1]},{"Token":31},{"Opt":74},{"And":[[956,957],null]},{"Or":[958]},{"Token":33},{"And":[[960],null]},{"Token":32},{"And":[[962],null]},{"Or":[961,963]},{"Token":87},{"Opt":965},{"And":[[964,966],null]},{"Or":[967]},{"Token":35},{"Opt":969},{"Rep":91},{"Opt":74},{"And":[[971,972],null]},{"Or":[973]},{"Call":[149,[[3,974]]]},{"And":[[970,975],null]},{"Or":[976]},{"Call":[77,[[0,977]]]},{"And":[[978],null]},{"Or":[979]},{"And":[[92],null]},{"And":[[95],null]},{"And":[[94],null]},{"And":[[3],null]},{"Token":9},{"Opt":93},{"Token":53},{"And":[[987,74],1]},{"Or":[988]},{"Opt":989},{"Token":58},{"And":[[985,62,986,990,991],1]},{"Or":[992]},{"Token":59},{"And":[[994,49],null]},{"Or":[995]},{"Token":58},{"And":[[997],null]},{"Or":[998]},"Eof",{"Not":1000},{"And":[[76,1001],null]},{"Token":58},{"And":[[1003],null]},{"Or":[1002,1004]},{"And":[[74,1005],null]},{"Or":[1006]},{"Enter":[2,1007]},{"And":[[1008],null]},{"Or":[1009]},{"Token":14},{"Token":15},{"And":[[90],null]},{"And":[[96],null]},{"Or":[1013,1014]},{"And":[[1012,1015],null]},{"Or":[1016]},{"Opt":1017},{"And":[[1011,98,90,1018],1]},{"Or":[1019]},{"Opt":102},{"Token":25},{"And":[[1021,1022,98,90],2]},{"Or":[1023]},{"Token":9},{"Token":53},{"And":[[1025,62,1026],1]},{"Or":[1027]},{"Opt":1028},{"And":[[1029,99],1]},{"Enter":[0,74]},{"And":[[1031],null]},{"Opt":102},{"Token":24},{"And":[[1033,1034,90],2]},{"Or":[1035]},{"Opt":102},{"Token":23},{"Token":34},{"And":[[1037,1038,62,1039,99,90],2]},{"Or":[1040]},{"Token":87},{"Token":59},{"And":[[1042,1043],null]},{"Token":30},{"Rep":104},{"Call":[149,[[3,1046]]]},{"And":[[1045,99,1047],1]},{"Or":[1048]},{"Token":52},{"Enter":[2,74]},{"Token":61},{"And":[[1052],null]},"Eof",{"And":[[1054],null]},{"And":[[76],null]},{"Or":[1053,1055,1056]},{"And":[[105,1050,1051,1057],1]},{"Or":[1058]},{"Token":79},{"And":[[1060,62],null]},{"Or":[1061]},{"Rep":1062},{"Opt":106},{"And":[[62,1063,1064],null]},{"Token":14},{"And":[[1066,74],null]},{"Or":[1067]},{"And":[[145],null]},{"Or":[1069]},{"And":[[146],null]},{"Or":[1071]},{"Token":62},{"Token":92},{"Enter":[1,46]},{"Opt":1075},{"And":[[1073,1074,1076,111],null]},{"Or":[1077]},{"And":[[74,1078],null]},{"Or":[1079]},{"IsIn":2},{"Not":76},{"And":[[1081,1082],null]},{"IsIn":2},{"Not":1084},{"And":[[1085],null]},{"Or":[1083,1086]},{"And":[[1087,111],null]},{"Or":[1088]},{"And":[[74,1089],null]},{"Or":[1090]},{"Call":[148,[[2,112]]]},{"Call":[150,[[4,1092]]]},{"Call":[77,[[0,1093]]]},{"And":[[1094],null]},{"And":[[74],null]},{"Or":[1096]},{"Token":62},{"Token":92},{"And":[[1099],null]},{"Token":89},{"And":[[1101],null]},{"Or":[1100,1102]},{"And":[[1098,1103],null]},{"Or":[1104]},{"And":[[74,1105],null]},{"Or":[1106]},{"Call":[152,[[6,74]]]},{"And":[[74,1108],null]},{"Or":[1109]},{"Token":83},{"And":[[74,1111],null]},{"Or":[1112]},{"Token":5},{"And":[[1114,49],null]},{"Or":[1115]},{"And":[[74,1116],null]},{"Or":[1117]},{"And":[[118,74],null]},{"Or":[1119]},{"Token":77},{"Token":87},{"Opt":1122},{"Token":27},{"Opt":1124},{"And":[[1121,1123,1125],null]},{"Token":67},{"And":[[1127,74],null]},{"Or":[1128]},{"Token":75},{"And":[[1130,74],null]},{"Or":[1131]},{"Token":82},{"And":[[1133,74],null]},{"Or":[1134]},{"Token":67},{"And":[[1136],null]},{"Token":69},{"And":[[1138],null]},{"Token":71},{"And":[[1140],null]},{"Or":[1137,1139,1141]},{"Call":[125,[[1,1142]]]},{"And":[[74,1143,74],null]},{"Or":[1144]},{"Token":73},{"And":[[1146],null]},{"Token":75},{"And":[[1148],null]},{"Or":[1147,1149]},{"Call":[125,[[1,1150]]]},{"And":[[74,1151,74],null]},{"Or":[1152]},{"ContextualToken":[45,"<<"]},{"And":[[1154],null]},{"ContextualToken":[47,">>"]},{"And":[[1156],null]},{"Or":[1155,1157]},{"Call":[125,[[1,1158]]]},{"And":[[74,1159,74],null]},{"Or":[1160]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1162,1163,1164],null]},{"IsIn":2},{"Not":1166},{"Var":1},{"And":[[1167,1168],null]},{"Token":77},{"Token":77},{"Not":1171},{"And":[[1170,1172],null]},{"Or":[1173]},{"Call":[125,[[1,1174]]]},{"And":[[74,1175,74],null]},{"Or":[1176]},{"Token":84},{"Call":[125,[[1,1178]]]},{"And":[[74,1179,74],null]},{"Or":[1180]},{"Token":79},{"Token":79},{"Not":1183},{"And":[[1182,1184],null]},{"Or":[1185]},{"Call":[125,[[1,1186]]]},{"And":[[74,1187,74],null]},{"Or":[1188]},{"Call":[125,[[1,130]]]},{"And":[[74,1190,74],null]},{"Or":[1191]},{"Token":54},{"And":[[1193],null]},{"Token":55},{"And":[[1195],null]},{"Token":41},{"And":[[1197],null]},{"Token":42},{"And":[[1199],null]},{"Token":57},{"And":[[1201],null]},{"Token":56},{"And":[[1203],null]},{"ContextualToken":[49,"&&"]},{"Call":[125,[[1,1205]]]},{"And":[[74,1206,74],null]},{"Or":[1207]},{"ContextualToken":[50,"||"]},{"Call":[125,[[1,1209]]]},{"And":[[74,1210,74],null]},{"Or":[1211]},{"Call":[125,[[1,138]]]},{"And":[[74,1213,74],null]},{"Or":[1214]},{"And":[[138,74],null]},{"Or":[1216]},{"And":[[74,137],null]},{"Or":[1218]},{"Token":63},{"Not":75},{"And":[[1220,1221],null]},{"Or":[1222]},{"Token":63},{"And":[[1224],null]},{"Token":64},{"And":[[1226],null]},{"Not":75},{"Not":1228},{"Token":39},{"IsIn":0},{"And":[[1230,1231],null]},{"Or":[1232]},{"Not":1233},{"And":[[137,1229,1234],null]},{"Token":53},{"And":[[1236],null]},{"Token":74},{"And":[[1238],null]},{"Token":76},{"And":[[1240],null]},{"Token":68},{"And":[[1242],null]},{"Token":70},{"And":[[1244],null]},{"Token":72},{"And":[[1246],null]},{"Token":78},{"And":[[1248],null]},{"Token":80},{"And":[[1250],null]},{"Token":85},{"And":[[1252],null]},{"ContextualToken":[48,">>="]},{"And":[[1254],null]},{"ContextualToken":[46,"<<="]},{"And":[[1256],null]},{"Or":[1237,1239,1241,1243,1245,1247,1249,1251,1253,1255,1257]},{"Call":[125,[[1,1258]]]},{"And":[[74,1259,74],null]},{"Or":[1260]},{"Token":65},{"And":[[1262,142],null]},{"Or":[1263]},{"Token":65},{"Token":82},{"And":[[1265,1266,142],null]},{"Or":[1267]},{"Call":[148,[[2,144]]]},{"Call":[152,[[6,1269]]]},{"And":[[1270],null]},{"Rep":140},{"And":[[1272],null]},{"Token":92},{"Token":53},{"And":[[1275,74],null]},{"Call":[148,[[2,144]]]},{"Call":[150,[[4,1277]]]},{"And":[[1278],null]},{"Or":[1276,1279]},{"Opt":1280},{"And":[[1274,1281],1]},{"Or":[1282]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1286},{"Rep":147},{"Call":[149,[[3,1288]]]},{"And":[[1284,1285,1287,1289],null]},{"Or":[1290]},{"Token":92},{"Token":82},{"Token":92},{"Opt":1294},{"Token":37},{"Rep":147},{"Token":38},{"And":[[1296,1297,1298],null]},{"Token":43},{"Rep":147},{"Token":44},{"And":[[1300,1301,1302],null]},{"Or":[1299,1303]},{"And":[[1292,1293,1295,1304],null]},{"Or":[1305]},{"Token":37},{"And":[[1307],null]},{"Token":38},{"And":[[1309],null]},{"Token":39},{"And":[[1311],null]},{"Token":40},{"And":[[1313],null]},{"Token":43},{"And":[[1315],null]},{"Token":44},{"And":[[1317],null]},{"Or":[1308,1310,1312,1314,1316,1318]},{"Not":1319},"Any",{"And":[[1320,1321],null]},{"Token":37},{"Rep":147},{"Token":38},{"And":[[1323,1324,1325],null]},{"Token":43},{"Rep":147},{"Token":44},{"And":[[1327,1328,1329],null]},{"Token":39},{"Rep":147},{"Token":40},{"And":[[1331,1332,1333],null]},{"Or":[1322,1326,1330,1334]},{"Var":2},"Eof",{"And":[[1337],null]},{"Token":61},{"And":[[1339],null]},{"Or":[1338,1340]},{"And":[[1336,1341],1]},{"Or":[1342]},{"Rep":1343},{"And":[[1344],null]},{"Token":39},{"Token":40},{"Var":3},{"Call":[153,[[7,1346],[8,1347],[9,1348]]]},{"And":[[1349],null]},{"Token":37},{"Token":38},{"Var":4},{"Call":[153,[[7,1351],[8,1352],[9,1353]]]},{"And":[[1354],null]},{"Token":41},{"Token":42},{"Var":5},{"Call":[153,[[7,1356],[8,1357],[9,1358]]]},{"And":[[1359],null]},{"Token":43},{"Token":44},{"Var":6},{"Call":[153,[[7,1361],[8,1362],[9,1363]]]},{"And":[[1364],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[154,[[10,1367],[11,1368]]]},{"Var":9},{"Layer":[1369,1370]},{"Var":8},{"And":[[1366,1371,1372],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[154,[[10,1375],[11,1376]]]},{"Var":11},{"And":[[1374,1377,1378],1]},{"Var":11},{"Not":1380},"Any",{"And":[[1381,1382],null]},{"Or":[1383]},{"And":[[1384],null]},{"Or":[1379,1385]},{"Rep":1386},{"And":[[1387],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, CONTINUE, BREAK, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, LOOP_CF_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, INNER_ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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
impl<'f> NameOwner<'f> for TypeParameter<'f> {}
pub trait TypeParametersOwner<'f>: rt::AstNode<'f> {
    fn type_parameters(&self) -> Option<TypeParameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}
impl<'f> TypeParametersOwner<'f> for StructDef<'f> {}
impl<'f> TypeParametersOwner<'f> for EnumDef<'f> {}