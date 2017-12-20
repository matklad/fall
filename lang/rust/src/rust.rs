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
pub const TYPE_ARGUMENTS: rt::NodeType = rt::NodeType(221);
pub const FN_TRAIT_SUGAR: rt::NodeType = rt::NodeType(222);
pub const ALIAS: rt::NodeType = rt::NodeType(223);
pub const PATH_TYPE: rt::NodeType = rt::NodeType(224);
pub const REFERENCE_TYPE: rt::NodeType = rt::NodeType(225);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(226);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(227);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(228);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(229);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(230);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(231);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(232);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(233);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(234);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(235);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(236);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(237);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(238);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(239);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(240);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(241);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(242);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(243);
pub const EXPR: rt::NodeType = rt::NodeType(244);
pub const LITERAL: rt::NodeType = rt::NodeType(245);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(246);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(247);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(248);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(249);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(250);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(251);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(252);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(253);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(254);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(255);
pub const LET_STMT: rt::NodeType = rt::NodeType(256);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(257);
pub const INITIALIZER: rt::NodeType = rt::NodeType(258);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(259);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(260);
pub const IF_EXPR: rt::NodeType = rt::NodeType(261);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(262);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(263);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(264);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(265);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(266);
pub const GUARD: rt::NodeType = rt::NodeType(267);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(268);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(269);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(270);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(271);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(272);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(273);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(274);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(275);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(276);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(277);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(278);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(279);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(280);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(281);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(282);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(283);
pub const BIT_AND: rt::NodeType = rt::NodeType(284);
pub const BIT_XOR: rt::NodeType = rt::NodeType(285);
pub const BIT_OR: rt::NodeType = rt::NodeType(286);
pub const COMPARISON: rt::NodeType = rt::NodeType(287);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(288);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(289);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(290);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(291);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(292);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(293);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(294);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(295);
pub const TT: rt::NodeType = rt::NodeType(296);


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
        let parser_json = r##"[{"Pub":{"ty":91,"body":152,"replaceable":false}},{"Or":[155]},{"Or":[157,159,161,163,165,167,169,170]},{"Cached":187},{"Pub":{"ty":92,"body":203,"replaceable":false}},{"Pub":{"ty":93,"body":209,"replaceable":false}},{"Pub":{"ty":94,"body":215,"replaceable":false}},{"Pub":{"ty":95,"body":222,"replaceable":false}},{"Pub":{"ty":96,"body":234,"replaceable":false}},{"Or":[236]},{"Pub":{"ty":97,"body":241,"replaceable":false}},{"Or":[247]},{"Pub":{"ty":98,"body":250,"replaceable":false}},{"Pub":{"ty":99,"body":256,"replaceable":false}},{"Pub":{"ty":100,"body":269,"replaceable":false}},{"Pub":{"ty":101,"body":288,"replaceable":false}},{"Pub":{"ty":102,"body":293,"replaceable":false}},{"Pub":{"ty":103,"body":296,"replaceable":false}},{"Pub":{"ty":104,"body":303,"replaceable":false}},{"Pub":{"ty":105,"body":316,"replaceable":false}},{"Pub":{"ty":106,"body":325,"replaceable":false}},{"Pub":{"ty":107,"body":336,"replaceable":false}},{"Pub":{"ty":108,"body":344,"replaceable":false}},{"Pub":{"ty":109,"body":356,"replaceable":false}},{"Or":[357,358,359]},{"Or":[361,363,365,367,369,371,373,378]},{"Pub":{"ty":110,"body":388,"replaceable":false}},{"Pub":{"ty":111,"body":402,"replaceable":false}},{"Pub":{"ty":112,"body":406,"replaceable":false}},{"Pub":{"ty":113,"body":410,"replaceable":false}},{"Or":[411,412]},{"Pub":{"ty":114,"body":419,"replaceable":false}},{"Pub":{"ty":115,"body":423,"replaceable":false}},{"Or":[443]},{"Pub":{"ty":116,"body":447,"replaceable":false}},{"Pub":{"ty":117,"body":467,"replaceable":false}},{"Pub":{"ty":118,"body":477,"replaceable":false}},{"Pub":{"ty":119,"body":492,"replaceable":false}},{"Or":[493]},{"Or":[495]},{"Or":[497]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":500,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":504,"replaceable":false}},{"Pub":{"ty":120,"body":509,"replaceable":false}},{"Pub":{"ty":121,"body":516,"replaceable":false}},{"Or":[529]},{"Pub":{"ty":122,"body":543,"replaceable":false}},{"Pub":{"ty":123,"body":548,"replaceable":false}},{"Pub":{"ty":124,"body":552,"replaceable":false}},{"Or":[553,554,555,556,557,558,559,560]},{"Pub":{"ty":125,"body":562,"replaceable":false}},{"Pub":{"ty":126,"body":564,"replaceable":false}},{"Pub":{"ty":127,"body":567,"replaceable":false}},{"Pub":{"ty":128,"body":571,"replaceable":false}},{"Pub":{"ty":129,"body":577,"replaceable":true}},{"PubReplace":{"ty":130,"body":581}},{"Pub":{"ty":131,"body":584,"replaceable":false}},{"Pub":{"ty":132,"body":593,"replaceable":false}},{"Pub":{"ty":133,"body":598,"replaceable":false}},{"Pub":{"ty":98,"body":604,"replaceable":false}},{"Or":[605,606,607,608,609,610,611]},{"Pub":{"ty":134,"body":614,"replaceable":false}},{"Pub":{"ty":135,"body":620,"replaceable":true}},{"PubReplace":{"ty":136,"body":632}},{"PubReplace":{"ty":137,"body":644}},{"Pub":{"ty":138,"body":651,"replaceable":false}},{"Pub":{"ty":139,"body":658,"replaceable":false}},{"Pub":{"ty":140,"body":660,"replaceable":false}},{"Pub":{"ty":141,"body":664,"replaceable":false}},{"Pub":{"ty":142,"body":670,"replaceable":true}},{"PubReplace":{"ty":143,"body":674}},{"Pub":{"ty":144,"body":679,"replaceable":false}},{"Pratt":{"atoms":[76,77,80,81,83,84,86,87,94,95,98,99,101,105,106,134],"prefixes":[{"ty":178,"op":116,"priority":999},{"ty":179,"op":708,"priority":999},{"ty":180,"op":709,"priority":999},{"ty":181,"op":710,"priority":999},{"ty":191,"op":136,"priority":2}],"infixes":[{"ty":171,"op":685,"priority":999,"has_rhs":false},{"ty":172,"op":694,"priority":999,"has_rhs":false},{"ty":174,"op":702,"priority":999,"has_rhs":false},{"ty":175,"op":703,"priority":999,"has_rhs":false},{"ty":176,"op":704,"priority":999,"has_rhs":false},{"ty":177,"op":707,"priority":999,"has_rhs":false},{"ty":182,"op":718,"priority":11,"has_rhs":true},{"ty":183,"op":724,"priority":10,"has_rhs":true},{"ty":184,"op":730,"priority":9,"has_rhs":true},{"ty":185,"op":736,"priority":8,"has_rhs":true},{"ty":186,"op":738,"priority":7,"has_rhs":true},{"ty":187,"op":744,"priority":6,"has_rhs":true},{"ty":188,"op":745,"priority":5,"has_rhs":true},{"ty":189,"op":747,"priority":4,"has_rhs":true},{"ty":190,"op":749,"priority":3,"has_rhs":true},{"ty":191,"op":750,"priority":2,"has_rhs":true},{"ty":191,"op":135,"priority":2,"has_rhs":false},{"ty":192,"op":774,"priority":1,"has_rhs":true}]}},{"Or":[775,777,779,781,783,785,787,789,791,793,795,797,799,801,803,805,807,809,811,813,815,817]},{"Or":[819]},{"Or":[823]},{"Pub":{"ty":146,"body":834,"replaceable":false}},{"Pub":{"ty":147,"body":842,"replaceable":true}},{"PubReplace":{"ty":148,"body":855}},{"Pub":{"ty":149,"body":862,"replaceable":false}},{"Pub":{"ty":150,"body":866,"replaceable":false}},{"Pub":{"ty":151,"body":873,"replaceable":true}},{"PubReplace":{"ty":152,"body":878}},{"Pub":{"ty":153,"body":883,"replaceable":false}},{"Pub":{"ty":154,"body":895,"replaceable":false}},{"Or":[903]},{"Pub":{"ty":155,"body":907,"replaceable":false}},{"Pub":{"ty":156,"body":919,"replaceable":false}},{"Or":[920,921,922,923]},{"Pub":{"ty":157,"body":929,"replaceable":false}},{"Pub":{"ty":158,"body":932,"replaceable":false}},{"Pub":{"ty":159,"body":935,"replaceable":false}},{"Pub":{"ty":160,"body":938,"replaceable":false}},{"Pub":{"ty":161,"body":949,"replaceable":false}},{"Pub":{"ty":162,"body":959,"replaceable":false}},{"Pub":{"ty":163,"body":963,"replaceable":false}},{"Or":[969]},{"Or":[971]},{"Pub":{"ty":164,"body":975,"replaceable":false}},{"Pub":{"ty":165,"body":980,"replaceable":false}},{"Or":[983]},{"Pub":{"ty":166,"body":988,"replaceable":false}},{"Pub":{"ty":167,"body":998,"replaceable":false}},{"Or":[1004]},{"Pub":{"ty":168,"body":1007,"replaceable":false}},{"Pub":{"ty":169,"body":1009,"replaceable":false}},{"Pub":{"ty":170,"body":1011,"replaceable":false}},{"Pub":{"ty":171,"body":1019,"replaceable":false}},{"Pub":{"ty":172,"body":1030,"replaceable":false}},{"Or":[1034]},{"Pub":{"ty":173,"body":1036,"replaceable":false}},{"Pub":{"ty":174,"body":1046,"replaceable":false}},{"Pub":{"ty":175,"body":1049,"replaceable":false}},{"Pub":{"ty":176,"body":1052,"replaceable":false}},{"Pub":{"ty":177,"body":1057,"replaceable":false}},{"Pub":{"ty":178,"body":1059,"replaceable":false}},{"Or":[1065]},{"Pub":{"ty":179,"body":1068,"replaceable":false}},{"Pub":{"ty":180,"body":1071,"replaceable":false}},{"Pub":{"ty":181,"body":1074,"replaceable":false}},{"Pub":{"ty":182,"body":1084,"replaceable":false}},{"Pub":{"ty":183,"body":1092,"replaceable":false}},{"Pub":{"ty":184,"body":1100,"replaceable":false}},{"Or":[1104,1108]},{"Pub":{"ty":185,"body":1116,"replaceable":false}},{"Pub":{"ty":186,"body":1120,"replaceable":false}},{"Pub":{"ty":187,"body":1128,"replaceable":false}},{"Pub":{"ty":188,"body":1131,"replaceable":false}},{"Or":[1133,1135,1137,1139,1141,1143]},{"Pub":{"ty":189,"body":1147,"replaceable":false}},{"Pub":{"ty":190,"body":1151,"replaceable":false}},{"Pub":{"ty":191,"body":1154,"replaceable":false}},{"Pub":{"ty":191,"body":1156,"replaceable":false}},{"Pub":{"ty":191,"body":1158,"replaceable":false}},{"Pub":{"ty":191,"body":1162,"replaceable":false}},{"Or":[1164,1166]},{"Or":[1174]},{"Pub":{"ty":192,"body":1200,"replaceable":false}},{"Pub":{"ty":193,"body":1205,"replaceable":false}},{"Or":[1207]},{"Pub":{"ty":194,"body":1217,"replaceable":false}},{"Pub":{"ty":195,"body":1225,"replaceable":false}},{"Pub":{"ty":196,"body":1240,"replaceable":false}},{"Pub":{"ty":197,"body":1269,"replaceable":false}},{"Or":[1279]},{"Or":[1284]},{"Or":[1289]},{"Or":[1294]},{"Or":[1299]},{"Or":[1307]},{"Or":[1322]},{"And":[[1],null]},{"Or":[151]},{"WithSkip":[2,3]},{"Rep":153},{"And":[[154],null]},{"Token":11},{"And":[[156],null]},{"ContextualToken":[4,"union"]},{"And":[[158],null]},{"Token":16},{"And":[[160],null]},{"Token":12},{"And":[[162],null]},{"Token":13},{"And":[[164],null]},{"Token":17},{"And":[[166],null]},{"Token":29},{"And":[[168],null]},{"And":[[25],null]},{"Opt":36},{"And":[[139,171],null]},{"Or":[172]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[174,175,176,177,178,179,180,181,182]},{"Inject":[173,183]},{"And":[[184],null]},{"And":[[28],null]},{"Or":[185,186]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[190,5],null]},{"Or":[191]},{"Opt":192},{"And":[[193],null]},{"Or":[189,194]},{"And":[[38,195],null]},{"Token":58},{"Opt":197},{"And":[[198,5],null]},{"Or":[196,199]},{"Token":56},{"And":[[188,200,201],1]},{"Or":[202]},{"Token":65},{"And":[[204],null]},{"Call":[144,[[2,6]]]},{"Call":[145,[[3,206]]]},{"And":[[207],null]},{"Or":[205,208]},{"Token":18},{"And":[[210],null]},{"Token":90},{"Opt":48},{"And":[[212,213],1]},{"Or":[211,214]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[216,217,218,219,220],2]},{"Or":[221]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[87],null]},{"Token":56},{"And":[[230],null]},{"Or":[229,231]},{"And":[[223,224,225,226,11,227,228,232],2]},{"Or":[233]},{"Token":49},{"And":[[235,49],null]},{"Token":7},{"Token":88},{"Opt":238},{"And":[[237,239],null]},{"Or":[240]},{"Opt":14},{"Call":[144,[[2,12]]]},{"And":[[242,243],null]},{"Or":[244]},{"Call":[146,[[4,245]]]},{"And":[[246],null]},{"Token":57},{"And":[[60,248,49],1]},{"Or":[249]},{"Token":57},{"And":[[251,49],null]},{"Or":[252]},{"Opt":253},{"And":[[60,254],null]},{"Or":[255]},{"Opt":116},{"Token":18},{"Token":57},{"And":[[259,49],null]},{"Or":[260]},{"Opt":261},{"Token":59},{"And":[[263],null]},"Eof",{"And":[[265],null]},{"Or":[264,266]},{"And":[[257,258,262,267],2]},{"Or":[268]},{"Token":11},{"And":[[270],null]},{"ContextualToken":[4,"union"]},{"And":[[272],null]},{"Or":[271,273]},{"Token":90},{"Opt":31},{"Call":[144,[[2,16]]]},{"Call":[145,[[3,277]]]},{"And":[[278],null]},{"Token":56},{"And":[[280],null]},{"Call":[144,[[2,17]]]},{"Call":[146,[[4,282]]]},{"Token":56},{"And":[[283,284],null]},{"Or":[279,281,285]},{"And":[[274,275,276,286],1]},{"Or":[287]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[289,290,291,49],2]},{"Or":[292]},{"Opt":36},{"And":[[294,49],null]},{"Or":[295]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[144,[[2,19]]]},{"Call":[145,[[3,300]]]},{"And":[[297,298,299,301],1]},{"Or":[302]},{"Token":90},{"Token":51},{"And":[[305,72],null]},{"Call":[144,[[2,17]]]},{"Call":[146,[[4,307]]]},{"And":[[308],null]},{"Call":[144,[[2,16]]]},{"Call":[145,[[3,310]]]},{"And":[[311],null]},{"Or":[306,309,312]},{"Opt":313},{"And":[[304,314],1]},{"Or":[315]},{"Token":13},{"Token":90},{"Token":56},{"And":[[319],null]},{"Call":[145,[[3,1]]]},{"And":[[321],null]},{"Or":[320,322]},{"And":[[317,318,323],1]},{"Or":[324]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[328,49],null]},{"Or":[329]},{"Opt":330},{"And":[[49,331],null]},{"Or":[332]},{"Opt":37},{"And":[[326,327,333,334,23],1]},{"Or":[335]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[139,337,338,339,340,341,342,23],3]},{"Or":[343]},{"Opt":36},{"And":[[139,345],null]},{"Or":[346]},{"Inject":[347,24]},{"And":[[348],null]},{"And":[[28],null]},{"Or":[349,350]},{"WithSkip":[25,351]},{"Rep":352},{"Call":[145,[[3,353]]]},{"And":[[354],null]},{"Or":[355]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[360],null]},{"Token":8},{"And":[[362],null]},{"Token":20},{"And":[[364],null]},{"Token":21},{"And":[[366],null]},{"Token":22},{"And":[[368],null]},{"Token":63},{"And":[[370],null]},{"Token":7},{"And":[[372],null]},{"Token":90},{"Token":80},{"And":[[374,375],null]},{"Or":[376]},{"And":[[377],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[382,49],null]},{"Or":[383]},{"Opt":384},{"Token":56},{"And":[[379,380,381,385,386],1]},{"Or":[387]},{"Token":21},{"And":[[389],null]},{"Token":22},{"And":[[391],null]},{"Or":[390,392]},{"Token":90},{"Token":57},{"Token":51},{"And":[[396,72],null]},{"Or":[397]},{"Opt":398},{"Token":56},{"And":[[393,394,395,49,399,400],1]},{"Or":[401]},{"And":[[141],null]},{"Token":56},{"And":[[142,404],null]},{"Or":[403,405]},{"Rep":30},{"Call":[145,[[3,407]]]},{"And":[[10,408],null]},{"Or":[409]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[144,[[2,35]]]},{"Call":[144,[[2,32]]]},{"And":[[413,414],null]},{"Or":[415]},{"Call":[147,[[5,416]]]},{"And":[[417],null]},{"Or":[418]},{"Token":90},{"Opt":33},{"And":[[420,421],1]},{"Or":[422]},{"Token":57},{"Token":71},{"And":[[425],null]},"Eof",{"And":[[427],null]},{"Token":59},{"And":[[429],null]},{"Token":37},{"And":[[431],null]},{"Token":34},{"And":[[433],null]},{"Or":[430,432,434]},{"Not":435},{"Not":436},{"And":[[437],null]},{"Or":[426,428,438]},{"And":[[34,439],1]},{"Or":[440]},{"Rep":441},{"And":[[424,442],null]},{"Token":85},{"And":[[444],null]},{"And":[[49],null]},{"Or":[445,446]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[451],null]},"Eof",{"And":[[453],null]},{"Token":59},{"Not":455},{"Not":456},{"And":[[457],null]},{"Or":[452,454,458]},{"And":[[450,459],1]},{"Or":[460]},{"Rep":461},{"And":[[449,462],null]},{"Or":[463]},{"Opt":464},{"And":[[448,465],1]},{"Or":[466]},{"Token":10},{"Token":6},{"And":[[469],null]},{"Token":19},{"And":[[471],null]},{"Or":[470,472]},{"Call":[146,[[4,473]]]},{"Opt":474},{"And":[[468,475],null]},{"Or":[476]},{"Token":34},{"Token":59},{"And":[[479],null]},"Eof",{"And":[[481],null]},{"Token":37},{"Not":483},{"Not":484},{"And":[[485],null]},{"Or":[480,482,486]},{"And":[[49,33,487],null]},{"Or":[488]},{"Rep":489},{"And":[[478,490],1]},{"Or":[491]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[494],null]},{"Enter":[1,41]},{"And":[[496],null]},{"Token":58},{"And":[[498,45],null]},{"Or":[499]},{"Token":58},{"Opt":501},{"And":[[502,45],null]},{"Or":[503]},{"Token":58},{"And":[[505,45],null]},{"Or":[506]},{"And":[[41,507],null]},{"Or":[508]},{"Token":5},{"And":[[49,510,49],null]},{"Or":[511]},{"Call":[147,[[5,512]]]},{"Token":58},{"And":[[513,514,45],null]},{"Or":[515]},{"Token":90},{"And":[[517],null]},{"Token":18},{"And":[[519],null]},{"Token":19},{"And":[[521],null]},{"Or":[518,520,522]},{"And":[[46],null]},{"IsIn":3},{"And":[[525,47],null]},{"Or":[524,526]},{"Opt":527},{"And":[[523,528],null]},{"IsIn":3},{"And":[[530],null]},{"IsIn":1},{"Token":58},{"And":[[532,533],null]},{"Or":[531,534]},{"Token":85},{"Call":[144,[[2,536]]]},{"Call":[144,[[2,49]]]},{"And":[[537,538],null]},{"Or":[539]},{"Call":[147,[[5,540]]]},{"And":[[535,541],null]},{"Or":[542]},{"Call":[144,[[2,49]]]},{"Call":[146,[[4,544]]]},{"Opt":9},{"And":[[545,546],null]},{"Or":[547]},{"Token":5},{"Token":90},{"And":[[549,550],null]},{"Or":[551]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[58],null]},{"And":[[39],null]},{"Or":[561]},{"And":[[116,49],null]},{"Or":[563]},{"Token":79},{"And":[[565],null]},{"Or":[566]},{"Token":35},{"Token":36},{"And":[[568,569],null]},{"Or":[570]},{"Opt":55},{"And":[[49,572],null]},{"Or":[573]},{"Call":[146,[[4,574]]]},{"And":[[575],null]},{"Or":[576]},{"Token":59},{"Call":[144,[[2,49]]]},{"And":[[578,579],null]},{"Or":[580]},{"Token":80},{"And":[[582],null]},{"Or":[583]},{"Token":56},{"And":[[585,72],null]},{"Or":[586]},{"Opt":587},{"And":[[49,588],null]},{"Or":[589]},{"Call":[148,[[6,590]]]},{"And":[[591],null]},{"Or":[592]},{"Token":8},{"Call":[144,[[2,59]]]},{"Call":[146,[[4,595]]]},{"And":[[594,596,9],1]},{"Or":[597]},{"Token":57},{"And":[[60,599],null]},{"Or":[600]},{"Opt":601},{"And":[[602,49],null]},{"Or":[603]},{"And":[[61],null]},{"And":[[62],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[71],null]},{"Token":79},{"And":[[612],null]},{"Or":[613]},{"And":[[63],null]},{"And":[[64],null]},{"Or":[615,616]},{"Opt":617},{"And":[[40,618],null]},{"Or":[619]},{"Call":[144,[[2,60]]]},{"Token":61},{"Token":59},{"Opt":623},{"And":[[622,624],null]},{"Or":[625]},{"Opt":626},{"And":[[621,627],null]},{"Or":[628]},{"Call":[146,[[4,629]]]},{"And":[[630],null]},{"Or":[631]},{"Call":[144,[[2,65]]]},{"Token":61},{"Token":59},{"Opt":635},{"And":[[634,636],null]},{"Or":[637]},{"Opt":638},{"And":[[633,639],null]},{"Or":[640]},{"Call":[145,[[3,641]]]},{"And":[[642],null]},{"Or":[643]},{"Token":57},{"Not":645},{"And":[[66,646],null]},{"Token":90},{"Token":57},{"And":[[648,649,60],2]},{"Or":[647,650]},{"Token":28},{"Opt":652},{"Token":27},{"Opt":654},{"Token":90},{"And":[[653,655,656],null]},{"Or":[657]},{"And":[[76],null]},{"Or":[659]},{"Token":35},{"Token":36},{"And":[[661,662],null]},{"Or":[663]},{"Opt":70},{"And":[[60,665],null]},{"Or":[666]},{"Call":[146,[[4,667]]]},{"And":[[668],null]},{"Or":[669]},{"Token":59},{"Call":[144,[[2,60]]]},{"And":[[671,672],null]},{"Or":[673]},{"Token":75},{"Token":27},{"Opt":676},{"And":[[675,677,60],null]},{"Or":[678]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":682},{"And":[[680,681,683,109],null]},{"Or":[684]},{"IsIn":2},{"Not":74},{"And":[[686,687],null]},{"IsIn":2},{"Not":689},{"And":[[690],null]},{"Or":[688,691]},{"And":[[692,109],null]},{"Or":[693]},{"Token":60},{"Token":90},{"And":[[696],null]},{"Token":87},{"And":[[698],null]},{"Or":[697,699]},{"And":[[695,700],null]},{"Or":[701]},{"Call":[148,[[6,72]]]},{"Token":81},{"Token":5},{"And":[[705,49],null]},{"Or":[706]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[711],null]},{"Token":67},{"And":[[713],null]},{"Token":69},{"And":[[715],null]},{"Or":[712,714,716]},{"Call":[123,[[1,717]]]},{"Token":71},{"And":[[719],null]},{"Token":73},{"And":[[721],null]},{"Or":[720,722]},{"Call":[123,[[1,723]]]},{"ContextualToken":[43,"<<"]},{"And":[[725],null]},{"ContextualToken":[45,">>"]},{"And":[[727],null]},{"Or":[726,728]},{"Call":[123,[[1,729]]]},{"Token":75},{"Token":75},{"Not":732},{"And":[[731,733],null]},{"Or":[734]},{"Call":[123,[[1,735]]]},{"Token":82},{"Call":[123,[[1,737]]]},{"Token":77},{"Token":77},{"Not":740},{"And":[[739,741],null]},{"Or":[742]},{"Call":[123,[[1,743]]]},{"Call":[123,[[1,128]]]},{"ContextualToken":[47,"&&"]},{"Call":[123,[[1,746]]]},{"ContextualToken":[48,"||"]},{"Call":[123,[[1,748]]]},{"Call":[123,[[1,136]]]},{"Token":51},{"And":[[751],null]},{"Token":72},{"And":[[753],null]},{"Token":74},{"And":[[755],null]},{"Token":66},{"And":[[757],null]},{"Token":68},{"And":[[759],null]},{"Token":70},{"And":[[761],null]},{"Token":76},{"And":[[763],null]},{"Token":78},{"And":[[765],null]},{"Token":83},{"And":[[767],null]},{"ContextualToken":[46,">>="]},{"And":[[769],null]},{"ContextualToken":[44,"<<="]},{"And":[[771],null]},{"Or":[752,754,756,758,760,762,764,766,768,770,772]},{"Call":[123,[[1,773]]]},{"And":[[76],null]},{"Token":90},{"And":[[776],null]},{"Token":18},{"And":[[778],null]},{"Token":19},{"And":[[780],null]},{"Token":39},{"And":[[782],null]},{"Token":58},{"And":[[784],null]},{"Token":35},{"And":[[786],null]},{"Token":41},{"And":[[788],null]},{"Token":77},{"And":[[790],null]},{"Token":31},{"And":[[792],null]},{"Token":37},{"And":[[794],null]},{"Token":14},{"And":[[796],null]},{"Token":25},{"And":[[798],null]},{"Token":24},{"And":[[800],null]},{"Token":23},{"And":[[802],null]},{"Token":30},{"And":[[804],null]},{"Token":75},{"And":[[806],null]},{"Token":65},{"And":[[808],null]},{"Token":73},{"And":[[810],null]},{"Token":80},{"And":[[812],null]},{"Token":61},{"And":[[814],null]},{"Token":62},{"And":[[816],null]},{"PrevIs":[156,162,163,164,165,166,169]},{"And":[[818],null]},{"Var":0},{"Exit":[2,820]},{"Exit":[0,821]},{"And":[[822],null]},{"Token":87},{"And":[[824],null]},{"Token":88},{"And":[[826],null]},{"Token":89},{"And":[[828],null]},{"Token":84},{"And":[[830],null]},{"Token":86},{"And":[[832],null]},{"Or":[825,827,829,831,833]},{"Token":90},{"Token":80},{"And":[[835,836],null]},{"Or":[837]},{"Not":838},{"Opt":78},{"And":[[839,40,840],null]},{"Or":[841]},{"IsIn":0},{"Not":843},{"Call":[144,[[2,79]]]},{"Token":61},{"Call":[75,[[0,72]]]},{"And":[[846,847],null]},{"Or":[848]},{"Opt":849},{"And":[[845,850],null]},{"Or":[851]},{"Call":[145,[[3,852]]]},{"And":[[844,853],null]},{"Or":[854]},{"Token":90},{"Token":57},{"And":[[857,72],null]},{"Or":[858]},{"Opt":859},{"And":[[856,860],1]},{"Or":[861]},{"Token":35},{"Token":36},{"And":[[863,864],null]},{"Or":[865]},{"Call":[75,[[0,72]]]},{"Opt":82},{"And":[[867,868],null]},{"Or":[869]},{"Call":[146,[[4,870]]]},{"And":[[871],null]},{"Or":[872]},{"Token":59},{"Call":[75,[[0,72]]]},{"Call":[144,[[2,875]]]},{"And":[[874,876],null]},{"Or":[877]},{"Call":[144,[[2,72]]]},{"Call":[75,[[0,879]]]},{"Call":[148,[[6,880]]]},{"And":[[881],null]},{"Or":[882]},{"Token":26},{"Opt":884},{"Token":77},{"Rep":85},{"Token":77},{"Token":49},{"And":[[889,49,87],null]},{"Call":[75,[[0,72]]]},{"And":[[891],null]},{"Or":[890,892]},{"And":[[885,886,887,888,893],null]},{"Or":[894]},{"Token":59},{"And":[[896],null]},{"Token":77},{"Not":898},{"Not":899},{"And":[[900],null]},{"Or":[897,901]},{"And":[[13,902],1]},{"Token":31},{"Opt":72},{"And":[[904,905],null]},{"Or":[906]},{"Token":33},{"Opt":908},{"Rep":88},{"Opt":72},{"And":[[910,911],null]},{"Or":[912]},{"Call":[145,[[3,913]]]},{"And":[[909,914],null]},{"Or":[915]},{"Call":[75,[[0,916]]]},{"And":[[917],null]},{"Or":[918]},{"And":[[89],null]},{"And":[[93],null]},{"And":[[92],null]},{"And":[[3],null]},{"Token":9},{"Opt":90},{"Opt":91},{"Token":56},{"And":[[924,60,925,926,927],1]},{"Or":[928]},{"Token":57},{"And":[[930,49],null]},{"Or":[931]},{"Token":51},{"And":[[933,72],null]},{"Or":[934]},{"Token":56},{"And":[[936],null]},{"Or":[937]},"Eof",{"Not":939},{"And":[[74,940],null]},{"Token":56},{"And":[[942],null]},{"Or":[941,943]},{"And":[[72,944],null]},{"Or":[945]},{"Enter":[2,946]},{"And":[[947],null]},{"Or":[948]},{"Token":14},{"Token":15},{"And":[[87],null]},{"And":[[94],null]},{"Or":[952,953]},{"And":[[951,954],null]},{"Or":[955]},{"Opt":956},{"And":[[950,96,87,957],1]},{"Or":[958]},{"Opt":100},{"Token":25},{"And":[[960,961,96,87],2]},{"Or":[962]},{"Token":9},{"Token":51},{"And":[[964,60,965],1]},{"Or":[966]},{"Opt":967},{"And":[[968,97],null]},{"Enter":[0,72]},{"And":[[970],null]},{"Opt":100},{"Token":24},{"And":[[972,973,87],2]},{"Or":[974]},{"Opt":100},{"Token":23},{"Token":32},{"And":[[976,977,60,978,97,87],2]},{"Or":[979]},{"Token":85},{"Token":57},{"And":[[981,982],null]},{"Token":30},{"Rep":102},{"Call":[145,[[3,985]]]},{"And":[[984,97,986],1]},{"Or":[987]},{"Token":50},{"Enter":[2,72]},{"Token":59},{"And":[[991],null]},"Eof",{"And":[[993],null]},{"And":[[74],null]},{"Or":[992,994,995]},{"And":[[103,989,990,996],1]},{"Or":[997]},{"Token":77},{"And":[[999,60],null]},{"Or":[1000]},{"Rep":1001},{"Opt":104},{"And":[[60,1002,1003],null]},{"Token":14},{"And":[[1005,72],null]},{"Or":[1006]},{"And":[[141],null]},{"Or":[1008]},{"And":[[142],null]},{"Or":[1010]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1014},{"And":[[1012,1013,1015,109],null]},{"Or":[1016]},{"And":[[72,1017],null]},{"Or":[1018]},{"IsIn":2},{"Not":74},{"And":[[1020,1021],null]},{"IsIn":2},{"Not":1023},{"And":[[1024],null]},{"Or":[1022,1025]},{"And":[[1026,109],null]},{"Or":[1027]},{"And":[[72,1028],null]},{"Or":[1029]},{"Call":[144,[[2,110]]]},{"Call":[146,[[4,1031]]]},{"Call":[75,[[0,1032]]]},{"And":[[1033],null]},{"And":[[72],null]},{"Or":[1035]},{"Token":60},{"Token":90},{"And":[[1038],null]},{"Token":87},{"And":[[1040],null]},{"Or":[1039,1041]},{"And":[[1037,1042],null]},{"Or":[1043]},{"And":[[72,1044],null]},{"Or":[1045]},{"Call":[148,[[6,72]]]},{"And":[[72,1047],null]},{"Or":[1048]},{"Token":81},{"And":[[72,1050],null]},{"Or":[1051]},{"Token":5},{"And":[[1053,49],null]},{"Or":[1054]},{"And":[[72,1055],null]},{"Or":[1056]},{"And":[[116,72],null]},{"Or":[1058]},{"Token":75},{"Token":85},{"Opt":1061},{"Token":27},{"Opt":1063},{"And":[[1060,1062,1064],null]},{"Token":65},{"And":[[1066,72],null]},{"Or":[1067]},{"Token":73},{"And":[[1069,72],null]},{"Or":[1070]},{"Token":80},{"And":[[1072,72],null]},{"Or":[1073]},{"Token":65},{"And":[[1075],null]},{"Token":67},{"And":[[1077],null]},{"Token":69},{"And":[[1079],null]},{"Or":[1076,1078,1080]},{"Call":[123,[[1,1081]]]},{"And":[[72,1082,72],null]},{"Or":[1083]},{"Token":71},{"And":[[1085],null]},{"Token":73},{"And":[[1087],null]},{"Or":[1086,1088]},{"Call":[123,[[1,1089]]]},{"And":[[72,1090,72],null]},{"Or":[1091]},{"ContextualToken":[43,"<<"]},{"And":[[1093],null]},{"ContextualToken":[45,">>"]},{"And":[[1095],null]},{"Or":[1094,1096]},{"Call":[123,[[1,1097]]]},{"And":[[72,1098,72],null]},{"Or":[1099]},{"IsIn":2},{"Not":74},{"Var":1},{"And":[[1101,1102,1103],null]},{"IsIn":2},{"Not":1105},{"Var":1},{"And":[[1106,1107],null]},{"Token":75},{"Token":75},{"Not":1110},{"And":[[1109,1111],null]},{"Or":[1112]},{"Call":[123,[[1,1113]]]},{"And":[[72,1114,72],null]},{"Or":[1115]},{"Token":82},{"Call":[123,[[1,1117]]]},{"And":[[72,1118,72],null]},{"Or":[1119]},{"Token":77},{"Token":77},{"Not":1122},{"And":[[1121,1123],null]},{"Or":[1124]},{"Call":[123,[[1,1125]]]},{"And":[[72,1126,72],null]},{"Or":[1127]},{"Call":[123,[[1,128]]]},{"And":[[72,1129,72],null]},{"Or":[1130]},{"Token":52},{"And":[[1132],null]},{"Token":53},{"And":[[1134],null]},{"Token":39},{"And":[[1136],null]},{"Token":40},{"And":[[1138],null]},{"Token":55},{"And":[[1140],null]},{"Token":54},{"And":[[1142],null]},{"ContextualToken":[47,"&&"]},{"Call":[123,[[1,1144]]]},{"And":[[72,1145,72],null]},{"Or":[1146]},{"ContextualToken":[48,"||"]},{"Call":[123,[[1,1148]]]},{"And":[[72,1149,72],null]},{"Or":[1150]},{"Call":[123,[[1,136]]]},{"And":[[72,1152,72],null]},{"Or":[1153]},{"And":[[136,72],null]},{"Or":[1155]},{"And":[[72,135],null]},{"Or":[1157]},{"Token":61},{"Not":73},{"And":[[1159,1160],null]},{"Or":[1161]},{"Token":61},{"And":[[1163],null]},{"Token":62},{"And":[[1165],null]},{"Not":73},{"Not":1167},{"Token":37},{"IsIn":0},{"And":[[1169,1170],null]},{"Or":[1171]},{"Not":1172},{"And":[[135,1168,1173],null]},{"Token":51},{"And":[[1175],null]},{"Token":72},{"And":[[1177],null]},{"Token":74},{"And":[[1179],null]},{"Token":66},{"And":[[1181],null]},{"Token":68},{"And":[[1183],null]},{"Token":70},{"And":[[1185],null]},{"Token":76},{"And":[[1187],null]},{"Token":78},{"And":[[1189],null]},{"Token":83},{"And":[[1191],null]},{"ContextualToken":[46,">>="]},{"And":[[1193],null]},{"ContextualToken":[44,"<<="]},{"And":[[1195],null]},{"Or":[1176,1178,1180,1182,1184,1186,1188,1190,1192,1194,1196]},{"Call":[123,[[1,1197]]]},{"And":[[72,1198,72],null]},{"Or":[1199]},{"Token":63},{"Call":[144,[[2,140]]]},{"Call":[148,[[6,1202]]]},{"And":[[1201,1203],null]},{"Or":[1204]},{"Rep":138},{"And":[[1206],null]},{"Token":90},{"Token":51},{"And":[[1209,72],null]},{"Call":[144,[[2,140]]]},{"Call":[146,[[4,1211]]]},{"And":[[1212],null]},{"Or":[1210,1213]},{"Opt":1214},{"And":[[1208,1215],1]},{"Or":[1216]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1220},{"Rep":143},{"Call":[145,[[3,1222]]]},{"And":[[1218,1219,1221,1223],null]},{"Or":[1224]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1228},{"Token":35},{"Rep":143},{"Token":36},{"And":[[1230,1231,1232],null]},{"Token":41},{"Rep":143},{"Token":42},{"And":[[1234,1235,1236],null]},{"Or":[1233,1237]},{"And":[[1226,1227,1229,1238],null]},{"Or":[1239]},{"Token":35},{"And":[[1241],null]},{"Token":36},{"And":[[1243],null]},{"Token":37},{"And":[[1245],null]},{"Token":38},{"And":[[1247],null]},{"Token":41},{"And":[[1249],null]},{"Token":42},{"And":[[1251],null]},{"Or":[1242,1244,1246,1248,1250,1252]},{"Not":1253},"Any",{"And":[[1254,1255],null]},{"Token":35},{"Rep":143},{"Token":36},{"And":[[1257,1258,1259],null]},{"Token":41},{"Rep":143},{"Token":42},{"And":[[1261,1262,1263],null]},{"Token":37},{"Rep":143},{"Token":38},{"And":[[1265,1266,1267],null]},{"Or":[1256,1260,1264,1268]},{"Var":2},"Eof",{"And":[[1271],null]},{"Token":59},{"And":[[1273],null]},{"Or":[1272,1274]},{"And":[[1270,1275],1]},{"Or":[1276]},{"Rep":1277},{"And":[[1278],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[149,[[7,1280],[8,1281],[9,1282]]]},{"And":[[1283],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[149,[[7,1285],[8,1286],[9,1287]]]},{"And":[[1288],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[149,[[7,1290],[8,1291],[9,1292]]]},{"And":[[1293],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[149,[[7,1295],[8,1296],[9,1297]]]},{"And":[[1298],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[150,[[10,1301],[11,1302]]]},{"Var":9},{"Layer":[1303,1304]},{"Var":8},{"And":[[1300,1305,1306],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[150,[[10,1309],[11,1310]]]},{"Var":11},{"And":[[1308,1311,1312],1]},{"Var":11},{"Not":1314},"Any",{"And":[[1315,1316],null]},{"Or":[1317]},{"And":[[1318],null]},{"Or":[1313,1319]},{"Rep":1320},{"And":[[1321],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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
}

impl<'f> ::std::fmt::Debug for Path<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Path@")?;
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
}

impl<'f> ::std::fmt::Debug for UseDecl<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("UseDecl@")?;
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