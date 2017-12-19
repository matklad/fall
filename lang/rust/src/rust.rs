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
pub const TYPE_ARGUMENTS: rt::NodeType = rt::NodeType(220);
pub const FN_TRAIT_SUGAR: rt::NodeType = rt::NodeType(221);
pub const ALIAS: rt::NodeType = rt::NodeType(222);
pub const PATH_TYPE: rt::NodeType = rt::NodeType(223);
pub const REFERENCE_TYPE: rt::NodeType = rt::NodeType(224);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(225);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(226);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(227);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(228);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(229);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(230);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(231);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(232);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(233);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(234);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(235);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(236);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(237);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(238);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(239);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(240);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(241);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(242);
pub const EXPR: rt::NodeType = rt::NodeType(243);
pub const LITERAL: rt::NodeType = rt::NodeType(244);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(245);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(246);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(247);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(248);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(249);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(250);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(251);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(252);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(253);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(254);
pub const LET_STMT: rt::NodeType = rt::NodeType(255);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(256);
pub const INITIALIZER: rt::NodeType = rt::NodeType(257);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(258);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(259);
pub const IF_EXPR: rt::NodeType = rt::NodeType(260);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(261);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(262);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(263);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(264);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(265);
pub const GUARD: rt::NodeType = rt::NodeType(266);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(267);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(268);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(269);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(270);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(271);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(272);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(273);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(274);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(275);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(276);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(277);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(278);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(279);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(280);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(281);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(282);
pub const BIT_AND: rt::NodeType = rt::NodeType(283);
pub const BIT_XOR: rt::NodeType = rt::NodeType(284);
pub const BIT_OR: rt::NodeType = rt::NodeType(285);
pub const COMPARISON: rt::NodeType = rt::NodeType(286);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(287);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(288);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(289);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(290);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(291);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(292);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(293);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(294);
pub const TT: rt::NodeType = rt::NodeType(295);


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
            ::fall_parse::LexRule::new(STRING, "\"([^\"]|\\\\\")*\"", None),
            ::fall_parse::LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(IDENT, "(\\p{XID_Start}|_)\\p{XID_Continue}*", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":91,"body":150,"replaceable":false}},{"Or":[153]},{"Or":[155,157,159,161,163,165,167,168]},{"Cached":185},{"Pub":{"ty":92,"body":201,"replaceable":false}},{"Pub":{"ty":93,"body":207,"replaceable":false}},{"Pub":{"ty":94,"body":213,"replaceable":false}},{"Pub":{"ty":95,"body":220,"replaceable":false}},{"Pub":{"ty":96,"body":232,"replaceable":false}},{"Or":[234]},{"Pub":{"ty":97,"body":239,"replaceable":false}},{"Or":[245]},{"Pub":{"ty":98,"body":248,"replaceable":false}},{"Pub":{"ty":99,"body":254,"replaceable":false}},{"Pub":{"ty":100,"body":270,"replaceable":false}},{"Pub":{"ty":101,"body":289,"replaceable":false}},{"Pub":{"ty":102,"body":294,"replaceable":false}},{"Pub":{"ty":103,"body":297,"replaceable":false}},{"Pub":{"ty":104,"body":304,"replaceable":false}},{"Pub":{"ty":105,"body":317,"replaceable":false}},{"Pub":{"ty":106,"body":326,"replaceable":false}},{"Pub":{"ty":107,"body":336,"replaceable":false}},{"Pub":{"ty":108,"body":342,"replaceable":false}},{"Pub":{"ty":109,"body":354,"replaceable":false}},{"Or":[355,356,357]},{"Or":[359,361,363,365,367,369,371,376]},{"Pub":{"ty":110,"body":386,"replaceable":false}},{"Pub":{"ty":111,"body":400,"replaceable":false}},{"Pub":{"ty":112,"body":404,"replaceable":false}},{"Pub":{"ty":113,"body":408,"replaceable":false}},{"Or":[409,410]},{"Pub":{"ty":114,"body":417,"replaceable":false}},{"Pub":{"ty":115,"body":421,"replaceable":false}},{"Or":[439]},{"Pub":{"ty":116,"body":443,"replaceable":false}},{"Pub":{"ty":117,"body":463,"replaceable":false}},{"Pub":{"ty":118,"body":473,"replaceable":false}},{"Pub":{"ty":119,"body":488,"replaceable":false}},{"Or":[489]},{"Or":[491]},{"Or":[493]},{"Pratt":{"atoms":[42],"prefixes":[],"infixes":[{"ty":120,"op":496,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":500,"replaceable":false}},{"Pub":{"ty":120,"body":505,"replaceable":false}},{"Or":[518]},{"Pub":{"ty":121,"body":532,"replaceable":false}},{"Pub":{"ty":122,"body":537,"replaceable":false}},{"Pub":{"ty":123,"body":541,"replaceable":false}},{"Or":[542,543,544,545,546,547,548,549]},{"Pub":{"ty":124,"body":551,"replaceable":false}},{"Pub":{"ty":125,"body":558,"replaceable":false}},{"Pub":{"ty":126,"body":561,"replaceable":false}},{"Pub":{"ty":127,"body":565,"replaceable":false}},{"Pub":{"ty":128,"body":571,"replaceable":true}},{"PubReplace":{"ty":129,"body":575}},{"Pub":{"ty":130,"body":578,"replaceable":false}},{"Pub":{"ty":131,"body":587,"replaceable":false}},{"Pub":{"ty":132,"body":592,"replaceable":false}},{"Pub":{"ty":98,"body":598,"replaceable":false}},{"Or":[599,600,601,602,603,604,605]},{"Pub":{"ty":133,"body":608,"replaceable":false}},{"Pub":{"ty":134,"body":614,"replaceable":true}},{"PubReplace":{"ty":135,"body":626}},{"PubReplace":{"ty":136,"body":638}},{"Pub":{"ty":137,"body":645,"replaceable":false}},{"Pub":{"ty":138,"body":652,"replaceable":false}},{"Pub":{"ty":139,"body":654,"replaceable":false}},{"Pub":{"ty":140,"body":658,"replaceable":false}},{"Pub":{"ty":141,"body":664,"replaceable":true}},{"PubReplace":{"ty":142,"body":668}},{"Pub":{"ty":143,"body":673,"replaceable":false}},{"Pratt":{"atoms":[75,76,79,80,82,83,85,86,93,94,97,98,100,104,105,132],"prefixes":[{"ty":177,"op":706,"priority":999},{"ty":178,"op":707,"priority":999},{"ty":179,"op":708,"priority":999},{"ty":180,"op":709,"priority":999},{"ty":190,"op":134,"priority":2}],"infixes":[{"ty":170,"op":679,"priority":999,"has_rhs":false},{"ty":171,"op":688,"priority":999,"has_rhs":false},{"ty":173,"op":696,"priority":999,"has_rhs":false},{"ty":174,"op":697,"priority":999,"has_rhs":false},{"ty":175,"op":698,"priority":999,"has_rhs":false},{"ty":176,"op":701,"priority":999,"has_rhs":false},{"ty":181,"op":717,"priority":11,"has_rhs":true},{"ty":182,"op":723,"priority":10,"has_rhs":true},{"ty":183,"op":729,"priority":9,"has_rhs":true},{"ty":184,"op":735,"priority":8,"has_rhs":true},{"ty":185,"op":737,"priority":7,"has_rhs":true},{"ty":186,"op":743,"priority":6,"has_rhs":true},{"ty":187,"op":744,"priority":5,"has_rhs":true},{"ty":188,"op":746,"priority":4,"has_rhs":true},{"ty":189,"op":748,"priority":3,"has_rhs":true},{"ty":190,"op":749,"priority":2,"has_rhs":true},{"ty":190,"op":133,"priority":2,"has_rhs":false},{"ty":191,"op":773,"priority":1,"has_rhs":true}]}},{"Or":[774,776,778,780,782,784,786,788,790,792,794,796,798,800,802,804,806,808,810,812,814,816]},{"Or":[818]},{"Or":[822]},{"Pub":{"ty":145,"body":833,"replaceable":false}},{"Pub":{"ty":146,"body":841,"replaceable":true}},{"PubReplace":{"ty":147,"body":854}},{"Pub":{"ty":148,"body":861,"replaceable":false}},{"Pub":{"ty":149,"body":865,"replaceable":false}},{"Pub":{"ty":150,"body":872,"replaceable":true}},{"PubReplace":{"ty":151,"body":877}},{"Pub":{"ty":152,"body":882,"replaceable":false}},{"Pub":{"ty":153,"body":894,"replaceable":false}},{"Or":[902]},{"Pub":{"ty":154,"body":906,"replaceable":false}},{"Pub":{"ty":155,"body":918,"replaceable":false}},{"Or":[919,920,921,922]},{"Pub":{"ty":156,"body":928,"replaceable":false}},{"Pub":{"ty":157,"body":931,"replaceable":false}},{"Pub":{"ty":158,"body":934,"replaceable":false}},{"Pub":{"ty":159,"body":937,"replaceable":false}},{"Pub":{"ty":160,"body":948,"replaceable":false}},{"Pub":{"ty":161,"body":958,"replaceable":false}},{"Pub":{"ty":162,"body":962,"replaceable":false}},{"Or":[968]},{"Or":[970]},{"Pub":{"ty":163,"body":974,"replaceable":false}},{"Pub":{"ty":164,"body":979,"replaceable":false}},{"Or":[982]},{"Pub":{"ty":165,"body":987,"replaceable":false}},{"Pub":{"ty":166,"body":997,"replaceable":false}},{"Or":[1003]},{"Pub":{"ty":167,"body":1006,"replaceable":false}},{"Pub":{"ty":168,"body":1008,"replaceable":false}},{"Pub":{"ty":169,"body":1010,"replaceable":false}},{"Pub":{"ty":170,"body":1018,"replaceable":false}},{"Pub":{"ty":171,"body":1029,"replaceable":false}},{"Or":[1033]},{"Pub":{"ty":172,"body":1035,"replaceable":false}},{"Pub":{"ty":173,"body":1045,"replaceable":false}},{"Pub":{"ty":174,"body":1048,"replaceable":false}},{"Pub":{"ty":175,"body":1051,"replaceable":false}},{"Pub":{"ty":176,"body":1056,"replaceable":false}},{"Pub":{"ty":177,"body":1063,"replaceable":false}},{"Pub":{"ty":178,"body":1066,"replaceable":false}},{"Pub":{"ty":179,"body":1069,"replaceable":false}},{"Pub":{"ty":180,"body":1072,"replaceable":false}},{"Pub":{"ty":181,"body":1082,"replaceable":false}},{"Pub":{"ty":182,"body":1090,"replaceable":false}},{"Pub":{"ty":183,"body":1098,"replaceable":false}},{"Or":[1102,1106]},{"Pub":{"ty":184,"body":1114,"replaceable":false}},{"Pub":{"ty":185,"body":1118,"replaceable":false}},{"Pub":{"ty":186,"body":1126,"replaceable":false}},{"Pub":{"ty":187,"body":1129,"replaceable":false}},{"Or":[1131,1133,1135,1137,1139,1141]},{"Pub":{"ty":188,"body":1145,"replaceable":false}},{"Pub":{"ty":189,"body":1149,"replaceable":false}},{"Pub":{"ty":190,"body":1152,"replaceable":false}},{"Pub":{"ty":190,"body":1154,"replaceable":false}},{"Pub":{"ty":190,"body":1156,"replaceable":false}},{"Pub":{"ty":190,"body":1160,"replaceable":false}},{"Or":[1162,1164]},{"Or":[1170]},{"Pub":{"ty":191,"body":1196,"replaceable":false}},{"Pub":{"ty":192,"body":1201,"replaceable":false}},{"Or":[1203]},{"Pub":{"ty":193,"body":1213,"replaceable":false}},{"Pub":{"ty":194,"body":1221,"replaceable":false}},{"Pub":{"ty":195,"body":1236,"replaceable":false}},{"Pub":{"ty":196,"body":1265,"replaceable":false}},{"Or":[1275]},{"Or":[1280]},{"Or":[1285]},{"Or":[1290]},{"Or":[1295]},{"Or":[1303]},{"Or":[1318]},{"And":[[1],null]},{"Or":[149]},{"WithSkip":[2,3]},{"Rep":151},{"And":[[152],null]},{"Token":11},{"And":[[154],null]},{"ContextualToken":[4,"union"]},{"And":[[156],null]},{"Token":16},{"And":[[158],null]},{"Token":12},{"And":[[160],null]},{"Token":13},{"And":[[162],null]},{"Token":17},{"And":[[164],null]},{"Token":29},{"And":[[166],null]},{"And":[[25],null]},{"Opt":36},{"And":[[137,169],null]},{"Or":[170]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[172,173,174,175,176,177,178,179,180]},{"Inject":[171,181]},{"And":[[182],null]},{"And":[[28],null]},{"Or":[183,184]},{"Token":12},{"And":[[47],null]},{"Token":58},{"And":[[188,5],null]},{"Or":[189]},{"Opt":190},{"And":[[191],null]},{"Or":[187,192]},{"And":[[38,193],null]},{"Token":58},{"Opt":195},{"And":[[196,5],null]},{"Or":[194,197]},{"Token":56},{"And":[[186,198,199],1]},{"Or":[200]},{"Token":65},{"And":[[202],null]},{"Call":[142,[[2,6]]]},{"Call":[143,[[3,204]]]},{"And":[[205],null]},{"Or":[203,206]},{"Token":18},{"And":[[208],null]},{"Token":90},{"Opt":47},{"And":[[210,211],1]},{"Or":[209,212]},{"Token":7},{"Token":6},{"Token":90},{"Opt":47},{"Token":56},{"And":[[214,215,216,217,218],2]},{"Or":[219]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[86],null]},{"Token":56},{"And":[[228],null]},{"Or":[227,229]},{"And":[[221,222,223,224,11,225,226,230],2]},{"Or":[231]},{"Token":49},{"And":[[233,48],null]},{"Token":7},{"Token":88},{"Opt":236},{"And":[[235,237],null]},{"Or":[238]},{"Opt":14},{"Call":[142,[[2,12]]]},{"And":[[240,241],null]},{"Or":[242]},{"Call":[144,[[4,243]]]},{"And":[[244],null]},{"Token":57},{"And":[[59,246,48],1]},{"Or":[247]},{"Token":57},{"And":[[249,48],null]},{"Or":[250]},{"Opt":251},{"And":[[59,252],null]},{"Or":[253]},{"Token":75},{"Opt":255},{"Token":27},{"Opt":257},{"Token":18},{"Token":57},{"And":[[260,48],null]},{"Or":[261]},{"Opt":262},{"Token":59},{"And":[[264],null]},"Eof",{"And":[[266],null]},{"Or":[265,267]},{"And":[[256,258,259,263,268],3]},{"Or":[269]},{"Token":11},{"And":[[271],null]},{"ContextualToken":[4,"union"]},{"And":[[273],null]},{"Or":[272,274]},{"Token":90},{"Opt":31},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,278]]]},{"And":[[279],null]},{"Token":56},{"And":[[281],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,283]]]},{"Token":56},{"And":[[284,285],null]},{"Or":[280,282,286]},{"And":[[275,276,277,287],1]},{"Or":[288]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[290,291,292,48],2]},{"Or":[293]},{"Opt":36},{"And":[[295,48],null]},{"Or":[296]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[142,[[2,19]]]},{"Call":[143,[[3,301]]]},{"And":[[298,299,300,302],1]},{"Or":[303]},{"Token":90},{"Token":51},{"And":[[306,71],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,308]]]},{"And":[[309],null]},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,311]]]},{"And":[[312],null]},{"Or":[307,310,313]},{"Opt":314},{"And":[[305,315],1]},{"Or":[316]},{"Token":13},{"Token":90},{"Token":56},{"And":[[320],null]},{"Call":[143,[[3,1]]]},{"And":[[322],null]},{"Or":[321,323]},{"And":[[318,319,324],1]},{"Or":[325]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[329,48],null]},{"Or":[330]},{"Opt":331},{"And":[[48,332],null]},{"Or":[333]},{"And":[[327,328,334,23],1]},{"Or":[335]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"And":[[137,337,338,339,340,23],3]},{"Or":[341]},{"Opt":36},{"And":[[137,343],null]},{"Or":[344]},{"Inject":[345,24]},{"And":[[346],null]},{"And":[[28],null]},{"Or":[347,348]},{"WithSkip":[25,349]},{"Rep":350},{"Call":[143,[[3,351]]]},{"And":[[352],null]},{"Or":[353]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[358],null]},{"Token":8},{"And":[[360],null]},{"Token":20},{"And":[[362],null]},{"Token":21},{"And":[[364],null]},{"Token":22},{"And":[[366],null]},{"Token":63},{"And":[[368],null]},{"Token":7},{"And":[[370],null]},{"Token":90},{"Token":80},{"And":[[372,373],null]},{"Or":[374]},{"And":[[375],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[380,48],null]},{"Or":[381]},{"Opt":382},{"Token":56},{"And":[[377,378,379,383,384],1]},{"Or":[385]},{"Token":21},{"And":[[387],null]},{"Token":22},{"And":[[389],null]},{"Or":[388,390]},{"Token":90},{"Token":57},{"Token":51},{"And":[[394,71],null]},{"Or":[395]},{"Opt":396},{"Token":56},{"And":[[391,392,393,48,397,398],1]},{"Or":[399]},{"And":[[139],null]},{"Token":56},{"And":[[140,402],null]},{"Or":[401,403]},{"Rep":30},{"Call":[143,[[3,405]]]},{"And":[[10,406],null]},{"Or":[407]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[142,[[2,35]]]},{"Call":[142,[[2,32]]]},{"And":[[411,412],null]},{"Or":[413]},{"Call":[145,[[5,414]]]},{"And":[[415],null]},{"Or":[416]},{"Token":90},{"Opt":33},{"And":[[418,419],1]},{"Or":[420]},{"Token":57},{"Token":71},{"And":[[423],null]},"Eof",{"And":[[425],null]},{"Token":59},{"Not":427},{"Not":428},{"And":[[429],null]},{"Token":37},{"Not":431},{"Not":432},{"And":[[433],null]},{"Or":[424,426,430,434]},{"And":[[34,435],1]},{"Or":[436]},{"Rep":437},{"And":[[422,438],null]},{"Token":85},{"And":[[440],null]},{"And":[[48],null]},{"Or":[441,442]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[447],null]},"Eof",{"And":[[449],null]},{"Token":59},{"Not":451},{"Not":452},{"And":[[453],null]},{"Or":[448,450,454]},{"And":[[446,455],1]},{"Or":[456]},{"Rep":457},{"And":[[445,458],null]},{"Or":[459]},{"Opt":460},{"And":[[444,461],1]},{"Or":[462]},{"Token":10},{"Token":6},{"And":[[465],null]},{"Token":19},{"And":[[467],null]},{"Or":[466,468]},{"Call":[144,[[4,469]]]},{"Opt":470},{"And":[[464,471],null]},{"Or":[472]},{"Token":34},{"Token":59},{"And":[[475],null]},"Eof",{"And":[[477],null]},{"Token":37},{"Not":479},{"Not":480},{"And":[[481],null]},{"Or":[476,478,482]},{"And":[[48,33,483],null]},{"Or":[484]},{"Rep":485},{"And":[[474,486],1]},{"Or":[487]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[490],null]},{"Enter":[1,41]},{"And":[[492],null]},{"Token":58},{"And":[[494,44],null]},{"Or":[495]},{"Token":58},{"Opt":497},{"And":[[498,44],null]},{"Or":[499]},{"Token":58},{"And":[[501,44],null]},{"Or":[502]},{"And":[[41,503],null]},{"Or":[504]},{"Token":90},{"And":[[506],null]},{"Token":18},{"And":[[508],null]},{"Token":19},{"And":[[510],null]},{"Or":[507,509,511]},{"And":[[45],null]},{"IsIn":3},{"And":[[514,46],null]},{"Or":[513,515]},{"Opt":516},{"And":[[512,517],null]},{"IsIn":3},{"And":[[519],null]},{"IsIn":1},{"Token":58},{"And":[[521,522],null]},{"Or":[520,523]},{"Token":85},{"Call":[142,[[2,525]]]},{"Call":[142,[[2,48]]]},{"And":[[526,527],null]},{"Or":[528]},{"Call":[145,[[5,529]]]},{"And":[[524,530],null]},{"Or":[531]},{"Call":[142,[[2,48]]]},{"Call":[144,[[4,533]]]},{"Opt":9},{"And":[[534,535],null]},{"Or":[536]},{"Token":5},{"Token":90},{"And":[[538,539],null]},{"Or":[540]},{"And":[[49],null]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[39],null]},{"Or":[550]},{"Token":75},{"Token":85},{"Opt":553},{"Token":27},{"Opt":555},{"And":[[552,554,556,48],null]},{"Or":[557]},{"Token":79},{"And":[[559],null]},{"Or":[560]},{"Token":35},{"Token":36},{"And":[[562,563],null]},{"Or":[564]},{"Opt":54},{"And":[[48,566],null]},{"Or":[567]},{"Call":[144,[[4,568]]]},{"And":[[569],null]},{"Or":[570]},{"Token":59},{"Call":[142,[[2,48]]]},{"And":[[572,573],null]},{"Or":[574]},{"Token":80},{"And":[[576],null]},{"Or":[577]},{"Token":56},{"And":[[579,71],null]},{"Or":[580]},{"Opt":581},{"And":[[48,582],null]},{"Or":[583]},{"Call":[146,[[6,584]]]},{"And":[[585],null]},{"Or":[586]},{"Token":8},{"Call":[142,[[2,58]]]},{"Call":[144,[[4,589]]]},{"And":[[588,590,9],1]},{"Or":[591]},{"Token":57},{"And":[[59,593],null]},{"Or":[594]},{"Opt":595},{"And":[[596,48],null]},{"Or":[597]},{"And":[[60],null]},{"And":[[61],null]},{"And":[[65],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[70],null]},{"Token":79},{"And":[[606],null]},{"Or":[607]},{"And":[[62],null]},{"And":[[63],null]},{"Or":[609,610]},{"Opt":611},{"And":[[40,612],null]},{"Or":[613]},{"Call":[142,[[2,59]]]},{"Token":61},{"Token":59},{"Opt":617},{"And":[[616,618],null]},{"Or":[619]},{"Opt":620},{"And":[[615,621],null]},{"Or":[622]},{"Call":[144,[[4,623]]]},{"And":[[624],null]},{"Or":[625]},{"Call":[142,[[2,64]]]},{"Token":61},{"Token":59},{"Opt":629},{"And":[[628,630],null]},{"Or":[631]},{"Opt":632},{"And":[[627,633],null]},{"Or":[634]},{"Call":[143,[[3,635]]]},{"And":[[636],null]},{"Or":[637]},{"Token":57},{"Not":639},{"And":[[65,640],null]},{"Token":90},{"Token":57},{"And":[[642,643,59],2]},{"Or":[641,644]},{"Token":28},{"Opt":646},{"Token":27},{"Opt":648},{"Token":90},{"And":[[647,649,650],null]},{"Or":[651]},{"And":[[75],null]},{"Or":[653]},{"Token":35},{"Token":36},{"And":[[655,656],null]},{"Or":[657]},{"Opt":69},{"And":[[59,659],null]},{"Or":[660]},{"Call":[144,[[4,661]]]},{"And":[[662],null]},{"Or":[663]},{"Token":59},{"Call":[142,[[2,59]]]},{"And":[[665,666],null]},{"Or":[667]},{"Token":75},{"Token":27},{"Opt":670},{"And":[[669,671,59],null]},{"Or":[672]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":676},{"And":[[674,675,677,108],null]},{"Or":[678]},{"IsIn":2},{"Not":73},{"And":[[680,681],null]},{"IsIn":2},{"Not":683},{"And":[[684],null]},{"Or":[682,685]},{"And":[[686,108],null]},{"Or":[687]},{"Token":60},{"Token":90},{"And":[[690],null]},{"Token":87},{"And":[[692],null]},{"Or":[691,693]},{"And":[[689,694],null]},{"Or":[695]},{"Call":[146,[[6,71]]]},{"Token":81},{"Token":5},{"And":[[699,48],null]},{"Or":[700]},{"Token":75},{"Token":27},{"Opt":703},{"And":[[702,704],null]},{"Or":[705]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[710],null]},{"Token":67},{"And":[[712],null]},{"Token":69},{"And":[[714],null]},{"Or":[711,713,715]},{"Call":[121,[[1,716]]]},{"Token":71},{"And":[[718],null]},{"Token":73},{"And":[[720],null]},{"Or":[719,721]},{"Call":[121,[[1,722]]]},{"ContextualToken":[43,"<<"]},{"And":[[724],null]},{"ContextualToken":[45,">>"]},{"And":[[726],null]},{"Or":[725,727]},{"Call":[121,[[1,728]]]},{"Token":75},{"Token":75},{"Not":731},{"And":[[730,732],null]},{"Or":[733]},{"Call":[121,[[1,734]]]},{"Token":82},{"Call":[121,[[1,736]]]},{"Token":77},{"Token":77},{"Not":739},{"And":[[738,740],null]},{"Or":[741]},{"Call":[121,[[1,742]]]},{"Call":[121,[[1,126]]]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,745]]]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,747]]]},{"Call":[121,[[1,134]]]},{"Token":51},{"And":[[750],null]},{"Token":72},{"And":[[752],null]},{"Token":74},{"And":[[754],null]},{"Token":66},{"And":[[756],null]},{"Token":68},{"And":[[758],null]},{"Token":70},{"And":[[760],null]},{"Token":76},{"And":[[762],null]},{"Token":78},{"And":[[764],null]},{"Token":83},{"And":[[766],null]},{"ContextualToken":[46,">>="]},{"And":[[768],null]},{"ContextualToken":[44,"<<="]},{"And":[[770],null]},{"Or":[751,753,755,757,759,761,763,765,767,769,771]},{"Call":[121,[[1,772]]]},{"And":[[75],null]},{"Token":90},{"And":[[775],null]},{"Token":18},{"And":[[777],null]},{"Token":19},{"And":[[779],null]},{"Token":39},{"And":[[781],null]},{"Token":58},{"And":[[783],null]},{"Token":35},{"And":[[785],null]},{"Token":41},{"And":[[787],null]},{"Token":77},{"And":[[789],null]},{"Token":31},{"And":[[791],null]},{"Token":37},{"And":[[793],null]},{"Token":14},{"And":[[795],null]},{"Token":25},{"And":[[797],null]},{"Token":24},{"And":[[799],null]},{"Token":23},{"And":[[801],null]},{"Token":30},{"And":[[803],null]},{"Token":75},{"And":[[805],null]},{"Token":65},{"And":[[807],null]},{"Token":73},{"And":[[809],null]},{"Token":80},{"And":[[811],null]},{"Token":61},{"And":[[813],null]},{"Token":62},{"And":[[815],null]},{"PrevIs":[155,161,162,163,164,165,168]},{"And":[[817],null]},{"Var":0},{"Exit":[2,819]},{"Exit":[0,820]},{"And":[[821],null]},{"Token":87},{"And":[[823],null]},{"Token":88},{"And":[[825],null]},{"Token":89},{"And":[[827],null]},{"Token":84},{"And":[[829],null]},{"Token":86},{"And":[[831],null]},{"Or":[824,826,828,830,832]},{"Token":90},{"Token":80},{"And":[[834,835],null]},{"Or":[836]},{"Not":837},{"Opt":77},{"And":[[838,40,839],null]},{"Or":[840]},{"IsIn":0},{"Not":842},{"Call":[142,[[2,78]]]},{"Token":61},{"Call":[74,[[0,71]]]},{"And":[[845,846],null]},{"Or":[847]},{"Opt":848},{"And":[[844,849],null]},{"Or":[850]},{"Call":[143,[[3,851]]]},{"And":[[843,852],null]},{"Or":[853]},{"Token":90},{"Token":57},{"And":[[856,71],null]},{"Or":[857]},{"Opt":858},{"And":[[855,859],1]},{"Or":[860]},{"Token":35},{"Token":36},{"And":[[862,863],null]},{"Or":[864]},{"Call":[74,[[0,71]]]},{"Opt":81},{"And":[[866,867],null]},{"Or":[868]},{"Call":[144,[[4,869]]]},{"And":[[870],null]},{"Or":[871]},{"Token":59},{"Call":[74,[[0,71]]]},{"Call":[142,[[2,874]]]},{"And":[[873,875],null]},{"Or":[876]},{"Call":[142,[[2,71]]]},{"Call":[74,[[0,878]]]},{"Call":[146,[[6,879]]]},{"And":[[880],null]},{"Or":[881]},{"Token":26},{"Opt":883},{"Token":77},{"Rep":84},{"Token":77},{"Token":49},{"And":[[888,48,86],null]},{"Call":[74,[[0,71]]]},{"And":[[890],null]},{"Or":[889,891]},{"And":[[884,885,886,887,892],null]},{"Or":[893]},{"Token":59},{"And":[[895],null]},{"Token":77},{"Not":897},{"Not":898},{"And":[[899],null]},{"Or":[896,900]},{"And":[[13,901],1]},{"Token":31},{"Opt":71},{"And":[[903,904],null]},{"Or":[905]},{"Token":33},{"Opt":907},{"Rep":87},{"Opt":71},{"And":[[909,910],null]},{"Or":[911]},{"Call":[143,[[3,912]]]},{"And":[[908,913],null]},{"Or":[914]},{"Call":[74,[[0,915]]]},{"And":[[916],null]},{"Or":[917]},{"And":[[88],null]},{"And":[[92],null]},{"And":[[91],null]},{"And":[[3],null]},{"Token":9},{"Opt":89},{"Opt":90},{"Token":56},{"And":[[923,59,924,925,926],1]},{"Or":[927]},{"Token":57},{"And":[[929,48],null]},{"Or":[930]},{"Token":51},{"And":[[932,71],null]},{"Or":[933]},{"Token":56},{"And":[[935],null]},{"Or":[936]},"Eof",{"Not":938},{"And":[[73,939],null]},{"Token":56},{"And":[[941],null]},{"Or":[940,942]},{"And":[[71,943],null]},{"Or":[944]},{"Enter":[2,945]},{"And":[[946],null]},{"Or":[947]},{"Token":14},{"Token":15},{"And":[[86],null]},{"And":[[93],null]},{"Or":[951,952]},{"And":[[950,953],null]},{"Or":[954]},{"Opt":955},{"And":[[949,95,86,956],1]},{"Or":[957]},{"Opt":99},{"Token":25},{"And":[[959,960,95,86],2]},{"Or":[961]},{"Token":9},{"Token":51},{"And":[[963,59,964],1]},{"Or":[965]},{"Opt":966},{"And":[[967,96],null]},{"Enter":[0,71]},{"And":[[969],null]},{"Opt":99},{"Token":24},{"And":[[971,972,86],2]},{"Or":[973]},{"Opt":99},{"Token":23},{"Token":32},{"And":[[975,976,59,977,96,86],2]},{"Or":[978]},{"Token":85},{"Token":57},{"And":[[980,981],null]},{"Token":30},{"Rep":101},{"Call":[143,[[3,984]]]},{"And":[[983,96,985],1]},{"Or":[986]},{"Token":50},{"Enter":[2,71]},{"Token":59},{"And":[[990],null]},"Eof",{"And":[[992],null]},{"And":[[73],null]},{"Or":[991,993,994]},{"And":[[102,988,989,995],1]},{"Or":[996]},{"Token":77},{"And":[[998,59],null]},{"Or":[999]},{"Rep":1000},{"Opt":103},{"And":[[59,1001,1002],null]},{"Token":14},{"And":[[1004,71],null]},{"Or":[1005]},{"And":[[139],null]},{"Or":[1007]},{"And":[[140],null]},{"Or":[1009]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":1013},{"And":[[1011,1012,1014,108],null]},{"Or":[1015]},{"And":[[71,1016],null]},{"Or":[1017]},{"IsIn":2},{"Not":73},{"And":[[1019,1020],null]},{"IsIn":2},{"Not":1022},{"And":[[1023],null]},{"Or":[1021,1024]},{"And":[[1025,108],null]},{"Or":[1026]},{"And":[[71,1027],null]},{"Or":[1028]},{"Call":[142,[[2,109]]]},{"Call":[144,[[4,1030]]]},{"Call":[74,[[0,1031]]]},{"And":[[1032],null]},{"And":[[71],null]},{"Or":[1034]},{"Token":60},{"Token":90},{"And":[[1037],null]},{"Token":87},{"And":[[1039],null]},{"Or":[1038,1040]},{"And":[[1036,1041],null]},{"Or":[1042]},{"And":[[71,1043],null]},{"Or":[1044]},{"Call":[146,[[6,71]]]},{"And":[[71,1046],null]},{"Or":[1047]},{"Token":81},{"And":[[71,1049],null]},{"Or":[1050]},{"Token":5},{"And":[[1052,48],null]},{"Or":[1053]},{"And":[[71,1054],null]},{"Or":[1055]},{"Token":75},{"Token":27},{"Opt":1058},{"And":[[1057,1059],null]},{"Or":[1060]},{"And":[[1061,71],null]},{"Or":[1062]},{"Token":65},{"And":[[1064,71],null]},{"Or":[1065]},{"Token":73},{"And":[[1067,71],null]},{"Or":[1068]},{"Token":80},{"And":[[1070,71],null]},{"Or":[1071]},{"Token":65},{"And":[[1073],null]},{"Token":67},{"And":[[1075],null]},{"Token":69},{"And":[[1077],null]},{"Or":[1074,1076,1078]},{"Call":[121,[[1,1079]]]},{"And":[[71,1080,71],null]},{"Or":[1081]},{"Token":71},{"And":[[1083],null]},{"Token":73},{"And":[[1085],null]},{"Or":[1084,1086]},{"Call":[121,[[1,1087]]]},{"And":[[71,1088,71],null]},{"Or":[1089]},{"ContextualToken":[43,"<<"]},{"And":[[1091],null]},{"ContextualToken":[45,">>"]},{"And":[[1093],null]},{"Or":[1092,1094]},{"Call":[121,[[1,1095]]]},{"And":[[71,1096,71],null]},{"Or":[1097]},{"IsIn":2},{"Not":73},{"Var":1},{"And":[[1099,1100,1101],null]},{"IsIn":2},{"Not":1103},{"Var":1},{"And":[[1104,1105],null]},{"Token":75},{"Token":75},{"Not":1108},{"And":[[1107,1109],null]},{"Or":[1110]},{"Call":[121,[[1,1111]]]},{"And":[[71,1112,71],null]},{"Or":[1113]},{"Token":82},{"Call":[121,[[1,1115]]]},{"And":[[71,1116,71],null]},{"Or":[1117]},{"Token":77},{"Token":77},{"Not":1120},{"And":[[1119,1121],null]},{"Or":[1122]},{"Call":[121,[[1,1123]]]},{"And":[[71,1124,71],null]},{"Or":[1125]},{"Call":[121,[[1,126]]]},{"And":[[71,1127,71],null]},{"Or":[1128]},{"Token":52},{"And":[[1130],null]},{"Token":53},{"And":[[1132],null]},{"Token":39},{"And":[[1134],null]},{"Token":40},{"And":[[1136],null]},{"Token":55},{"And":[[1138],null]},{"Token":54},{"And":[[1140],null]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,1142]]]},{"And":[[71,1143,71],null]},{"Or":[1144]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,1146]]]},{"And":[[71,1147,71],null]},{"Or":[1148]},{"Call":[121,[[1,134]]]},{"And":[[71,1150,71],null]},{"Or":[1151]},{"And":[[134,71],null]},{"Or":[1153]},{"And":[[71,133],null]},{"Or":[1155]},{"Token":61},{"Not":72},{"And":[[1157,1158],null]},{"Or":[1159]},{"Token":61},{"And":[[1161],null]},{"Token":62},{"And":[[1163],null]},{"Token":37},{"IsIn":0},{"And":[[1165,1166],null]},{"Or":[1167]},{"Not":1168},{"And":[[133,1169],null]},{"Token":51},{"And":[[1171],null]},{"Token":72},{"And":[[1173],null]},{"Token":74},{"And":[[1175],null]},{"Token":66},{"And":[[1177],null]},{"Token":68},{"And":[[1179],null]},{"Token":70},{"And":[[1181],null]},{"Token":76},{"And":[[1183],null]},{"Token":78},{"And":[[1185],null]},{"Token":83},{"And":[[1187],null]},{"ContextualToken":[46,">>="]},{"And":[[1189],null]},{"ContextualToken":[44,"<<="]},{"And":[[1191],null]},{"Or":[1172,1174,1176,1178,1180,1182,1184,1186,1188,1190,1192]},{"Call":[121,[[1,1193]]]},{"And":[[71,1194,71],null]},{"Or":[1195]},{"Token":63},{"Call":[142,[[2,138]]]},{"Call":[146,[[6,1198]]]},{"And":[[1197,1199],null]},{"Or":[1200]},{"Rep":136},{"And":[[1202],null]},{"Token":90},{"Token":51},{"And":[[1205,71],null]},{"Call":[142,[[2,138]]]},{"Call":[144,[[4,1207]]]},{"And":[[1208],null]},{"Or":[1206,1209]},{"Opt":1210},{"And":[[1204,1211],1]},{"Or":[1212]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1216},{"Rep":141},{"Call":[143,[[3,1218]]]},{"And":[[1214,1215,1217,1219],null]},{"Or":[1220]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1224},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1226,1227,1228],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1230,1231,1232],null]},{"Or":[1229,1233]},{"And":[[1222,1223,1225,1234],null]},{"Or":[1235]},{"Token":35},{"And":[[1237],null]},{"Token":36},{"And":[[1239],null]},{"Token":37},{"And":[[1241],null]},{"Token":38},{"And":[[1243],null]},{"Token":41},{"And":[[1245],null]},{"Token":42},{"And":[[1247],null]},{"Or":[1238,1240,1242,1244,1246,1248]},{"Not":1249},"Any",{"And":[[1250,1251],null]},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1253,1254,1255],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1257,1258,1259],null]},{"Token":37},{"Rep":141},{"Token":38},{"And":[[1261,1262,1263],null]},{"Or":[1252,1256,1260,1264]},{"Var":2},"Eof",{"And":[[1267],null]},{"Token":59},{"And":[[1269],null]},{"Or":[1268,1270]},{"And":[[1266,1271],1]},{"Or":[1272]},{"Rep":1273},{"And":[[1274],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[147,[[7,1276],[8,1277],[9,1278]]]},{"And":[[1279],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[147,[[7,1281],[8,1282],[9,1283]]]},{"And":[[1284],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[147,[[7,1286],[8,1287],[9,1288]]]},{"And":[[1289],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[147,[[7,1291],[8,1292],[9,1293]]]},{"And":[[1294],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[148,[[10,1297],[11,1298]]]},{"Var":9},{"Layer":[1299,1300]},{"Var":8},{"And":[[1296,1301,1302],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[148,[[10,1305],[11,1306]]]},{"Var":11},{"And":[[1304,1307,1308],1]},{"Var":11},{"Not":1310},"Any",{"And":[[1311,1312],null]},{"Or":[1313]},{"And":[[1314],null]},{"Or":[1309,1315]},{"Rep":1316},{"And":[[1317],null]}]"##;

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
    pub fn name_ident(&self) -> Option<Node<'f>> {
        self.node().children().find(|n| n.ty() == IDENT)
    }
}

impl<'f> ::std::fmt::Debug for FnDef<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("FnDef@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}