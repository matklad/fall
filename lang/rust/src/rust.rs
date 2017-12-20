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
pub const POINTER_TYPE: rt::NodeType = rt::NodeType(227);
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(228);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(229);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(230);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(231);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(232);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(233);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(234);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(235);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(236);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(237);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(238);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(239);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(240);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(241);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(242);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(243);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(244);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(245);
pub const EXPR: rt::NodeType = rt::NodeType(246);
pub const LITERAL: rt::NodeType = rt::NodeType(247);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(248);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(249);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(250);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(251);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(252);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(253);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(254);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(255);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(256);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(257);
pub const LET_STMT: rt::NodeType = rt::NodeType(258);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(259);
pub const INITIALIZER: rt::NodeType = rt::NodeType(260);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(261);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(262);
pub const IF_EXPR: rt::NodeType = rt::NodeType(263);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(264);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(265);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(266);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(267);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(268);
pub const GUARD: rt::NodeType = rt::NodeType(269);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(270);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(271);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(272);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(273);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(274);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(275);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(276);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(277);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(278);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(279);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(280);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(281);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(282);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(283);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(284);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(285);
pub const BIT_AND: rt::NodeType = rt::NodeType(286);
pub const BIT_XOR: rt::NodeType = rt::NodeType(287);
pub const BIT_OR: rt::NodeType = rt::NodeType(288);
pub const COMPARISON: rt::NodeType = rt::NodeType(289);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(290);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(291);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(292);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(293);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(294);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(295);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(296);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(297);
pub const TT: rt::NodeType = rt::NodeType(298);


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
        let parser_json = r##"[{"Pub":{"ty":91,"body":154,"replaceable":false}},{"Or":[157]},{"Or":[159,161,163,165,167,169,171,172]},{"Cached":189},{"Pub":{"ty":92,"body":205,"replaceable":false}},{"Pub":{"ty":93,"body":211,"replaceable":false}},{"Pub":{"ty":94,"body":217,"replaceable":false}},{"Pub":{"ty":95,"body":224,"replaceable":false}},{"Pub":{"ty":96,"body":240,"replaceable":false}},{"Or":[242]},{"Pub":{"ty":97,"body":247,"replaceable":false}},{"Or":[253]},{"Pub":{"ty":98,"body":256,"replaceable":false}},{"Pub":{"ty":99,"body":262,"replaceable":false}},{"Pub":{"ty":100,"body":279,"replaceable":false}},{"Pub":{"ty":101,"body":298,"replaceable":false}},{"Pub":{"ty":102,"body":303,"replaceable":false}},{"Pub":{"ty":103,"body":306,"replaceable":false}},{"Pub":{"ty":104,"body":313,"replaceable":false}},{"Pub":{"ty":105,"body":326,"replaceable":false}},{"Pub":{"ty":106,"body":335,"replaceable":false}},{"Pub":{"ty":107,"body":348,"replaceable":false}},{"Pub":{"ty":108,"body":356,"replaceable":false}},{"Pub":{"ty":109,"body":368,"replaceable":false}},{"Or":[369,370,371]},{"Or":[373,375,377,379,381,383,385,387,392]},{"Pub":{"ty":110,"body":402,"replaceable":false}},{"Pub":{"ty":111,"body":416,"replaceable":false}},{"Pub":{"ty":112,"body":420,"replaceable":false}},{"Pub":{"ty":113,"body":424,"replaceable":false}},{"Or":[432]},{"Pub":{"ty":114,"body":439,"replaceable":false}},{"Pub":{"ty":115,"body":443,"replaceable":false}},{"Or":[463]},{"Pub":{"ty":116,"body":467,"replaceable":false}},{"Pub":{"ty":117,"body":487,"replaceable":false}},{"Pub":{"ty":118,"body":497,"replaceable":false}},{"Pub":{"ty":119,"body":512,"replaceable":false}},{"Or":[513]},{"Or":[515]},{"Or":[517]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":520,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":524,"replaceable":false}},{"Pub":{"ty":120,"body":529,"replaceable":false}},{"Pub":{"ty":121,"body":536,"replaceable":false}},{"Pub":{"ty":122,"body":550,"replaceable":false}},{"Pub":{"ty":123,"body":576,"replaceable":false}},{"Pub":{"ty":124,"body":581,"replaceable":false}},{"Pub":{"ty":125,"body":585,"replaceable":false}},{"Or":[590]},{"Or":[591,592,593,594,595,596,597,598,599]},{"Pub":{"ty":126,"body":601,"replaceable":false}},{"Pub":{"ty":127,"body":603,"replaceable":false}},{"Pub":{"ty":128,"body":611,"replaceable":false}},{"Pub":{"ty":129,"body":614,"replaceable":false}},{"Pub":{"ty":130,"body":618,"replaceable":false}},{"Pub":{"ty":131,"body":624,"replaceable":true}},{"PubReplace":{"ty":132,"body":628}},{"Pub":{"ty":133,"body":631,"replaceable":false}},{"Pub":{"ty":134,"body":640,"replaceable":false}},{"Pub":{"ty":135,"body":645,"replaceable":false}},{"Pub":{"ty":98,"body":651,"replaceable":false}},{"Or":[652,653,654,655,656,657,658]},{"Pub":{"ty":136,"body":661,"replaceable":false}},{"Pub":{"ty":137,"body":667,"replaceable":true}},{"PubReplace":{"ty":138,"body":679}},{"PubReplace":{"ty":139,"body":691}},{"Pub":{"ty":140,"body":698,"replaceable":false}},{"Pub":{"ty":141,"body":705,"replaceable":false}},{"Pub":{"ty":142,"body":707,"replaceable":false}},{"Pub":{"ty":143,"body":711,"replaceable":false}},{"Pub":{"ty":144,"body":717,"replaceable":true}},{"PubReplace":{"ty":145,"body":721}},{"Pub":{"ty":146,"body":726,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,96,97,100,101,103,107,108,136],"prefixes":[{"ty":180,"op":118,"priority":999},{"ty":181,"op":755,"priority":999},{"ty":182,"op":756,"priority":999},{"ty":183,"op":757,"priority":999},{"ty":193,"op":138,"priority":2}],"infixes":[{"ty":173,"op":732,"priority":999,"has_rhs":false},{"ty":174,"op":741,"priority":999,"has_rhs":false},{"ty":176,"op":749,"priority":999,"has_rhs":false},{"ty":177,"op":750,"priority":999,"has_rhs":false},{"ty":178,"op":751,"priority":999,"has_rhs":false},{"ty":179,"op":754,"priority":999,"has_rhs":false},{"ty":184,"op":765,"priority":11,"has_rhs":true},{"ty":185,"op":771,"priority":10,"has_rhs":true},{"ty":186,"op":777,"priority":9,"has_rhs":true},{"ty":187,"op":783,"priority":8,"has_rhs":true},{"ty":188,"op":785,"priority":7,"has_rhs":true},{"ty":189,"op":791,"priority":6,"has_rhs":true},{"ty":190,"op":792,"priority":5,"has_rhs":true},{"ty":191,"op":794,"priority":4,"has_rhs":true},{"ty":192,"op":796,"priority":3,"has_rhs":true},{"ty":193,"op":797,"priority":2,"has_rhs":true},{"ty":193,"op":137,"priority":2,"has_rhs":false},{"ty":194,"op":821,"priority":1,"has_rhs":true}]}},{"Or":[822,824,826,828,830,832,834,836,838,840,842,844,846,848,850,852,854,856,858,860,862,864]},{"Or":[866]},{"Or":[870]},{"Pub":{"ty":148,"body":881,"replaceable":false}},{"Pub":{"ty":149,"body":889,"replaceable":true}},{"PubReplace":{"ty":150,"body":902}},{"Pub":{"ty":151,"body":909,"replaceable":false}},{"Pub":{"ty":152,"body":913,"replaceable":false}},{"Pub":{"ty":153,"body":920,"replaceable":true}},{"PubReplace":{"ty":154,"body":925}},{"Pub":{"ty":155,"body":930,"replaceable":false}},{"Pub":{"ty":156,"body":942,"replaceable":false}},{"Or":[950]},{"Pub":{"ty":157,"body":954,"replaceable":false}},{"Pub":{"ty":158,"body":966,"replaceable":false}},{"Or":[967,968,969,970]},{"Pub":{"ty":159,"body":976,"replaceable":false}},{"Pub":{"ty":160,"body":979,"replaceable":false}},{"Pub":{"ty":161,"body":982,"replaceable":false}},{"Pub":{"ty":162,"body":985,"replaceable":false}},{"Pub":{"ty":163,"body":996,"replaceable":false}},{"Pub":{"ty":164,"body":1006,"replaceable":false}},{"Pub":{"ty":165,"body":1010,"replaceable":false}},{"Or":[1016]},{"Or":[1018]},{"Pub":{"ty":166,"body":1022,"replaceable":false}},{"Pub":{"ty":167,"body":1027,"replaceable":false}},{"Or":[1030]},{"Pub":{"ty":168,"body":1035,"replaceable":false}},{"Pub":{"ty":169,"body":1045,"replaceable":false}},{"Or":[1051]},{"Pub":{"ty":170,"body":1054,"replaceable":false}},{"Pub":{"ty":171,"body":1056,"replaceable":false}},{"Pub":{"ty":172,"body":1058,"replaceable":false}},{"Pub":{"ty":173,"body":1066,"replaceable":false}},{"Pub":{"ty":174,"body":1077,"replaceable":false}},{"Or":[1081]},{"Pub":{"ty":175,"body":1083,"replaceable":false}},{"Pub":{"ty":176,"body":1093,"replaceable":false}},{"Pub":{"ty":177,"body":1096,"replaceable":false}},{"Pub":{"ty":178,"body":1099,"replaceable":false}},{"Pub":{"ty":179,"body":1104,"replaceable":false}},{"Pub":{"ty":180,"body":1106,"replaceable":false}},{"Or":[1112]},{"Pub":{"ty":181,"body":1115,"replaceable":false}},{"Pub":{"ty":182,"body":1118,"replaceable":false}},{"Pub":{"ty":183,"body":1121,"replaceable":false}},{"Pub":{"ty":184,"body":1131,"replaceable":false}},{"Pub":{"ty":185,"body":1139,"replaceable":false}},{"Pub":{"ty":186,"body":1147,"replaceable":false}},{"Or":[1151,1155]},{"Pub":{"ty":187,"body":1163,"replaceable":false}},{"Pub":{"ty":188,"body":1167,"replaceable":false}},{"Pub":{"ty":189,"body":1175,"replaceable":false}},{"Pub":{"ty":190,"body":1178,"replaceable":false}},{"Or":[1180,1182,1184,1186,1188,1190]},{"Pub":{"ty":191,"body":1194,"replaceable":false}},{"Pub":{"ty":192,"body":1198,"replaceable":false}},{"Pub":{"ty":193,"body":1201,"replaceable":false}},{"Pub":{"ty":193,"body":1203,"replaceable":false}},{"Pub":{"ty":193,"body":1205,"replaceable":false}},{"Pub":{"ty":193,"body":1209,"replaceable":false}},{"Or":[1211,1213]},{"Or":[1221]},{"Pub":{"ty":194,"body":1247,"replaceable":false}},{"Pub":{"ty":195,"body":1252,"replaceable":false}},{"Or":[1254]},{"Pub":{"ty":196,"body":1264,"replaceable":false}},{"Pub":{"ty":197,"body":1272,"replaceable":false}},{"Pub":{"ty":198,"body":1287,"replaceable":false}},{"Pub":{"ty":199,"body":1316,"replaceable":false}},{"Or":[1326]},{"Or":[1331]},{"Or":[1336]},{"Or":[1341]},{"Or":[1346]},{"Or":[1354]},{"Or":[1369]},{"And":[[1],null]},{"Or":[153]},{"WithSkip":[2,3]},{"Rep":155},{"And":[[156],null]},{"Token":11},{"And":[[158],null]},{"ContextualToken":[4,"union"]},{"And":[[160],null]},{"Token":16},{"And":[[162],null]},{"Token":12},{"And":[[164],null]},{"Token":13},{"And":[[166],null]},{"Token":17},{"And":[[168],null]},{"Token":29},{"And":[[170],null]},{"And":[[25],null]},{"Opt":36},{"And":[[141,173],null]},{"Or":[174]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[176,177,178,179,180,181,182,183,184]},{"Inject":[175,185]},{"And":[[186],null]},{"And":[[28],null]},{"Or":[187,188]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[192,5],null]},{"Or":[193]},{"Opt":194},{"And":[[195],null]},{"Or":[191,196]},{"And":[[38,197],null]},{"Token":58},{"Opt":199},{"And":[[200,5],null]},{"Or":[198,201]},{"Token":56},{"And":[[190,202,203],1]},{"Or":[204]},{"Token":65},{"And":[[206],null]},{"Call":[146,[[2,6]]]},{"Call":[147,[[3,208]]]},{"And":[[209],null]},{"Or":[207,210]},{"Token":18},{"And":[[212],null]},{"Token":90},{"Opt":48},{"And":[[214,215],1]},{"Or":[213,216]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[218,219,220,221,222],2]},{"Or":[223]},{"Token":21},{"Opt":225},{"Token":33},{"Opt":227},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[89],null]},{"Token":56},{"And":[[236],null]},{"Or":[235,237]},{"And":[[226,228,229,230,231,232,11,233,234,238],4]},{"Or":[239]},{"Token":49},{"And":[[241,49],null]},{"Token":7},{"Token":88},{"Opt":244},{"And":[[243,245],null]},{"Or":[246]},{"Opt":14},{"Call":[146,[[2,12]]]},{"And":[[248,249],null]},{"Or":[250]},{"Call":[148,[[4,251]]]},{"And":[[252],null]},{"Token":57},{"And":[[62,254,49],1]},{"Or":[255]},{"Token":57},{"And":[[257,49],null]},{"Or":[258]},{"Opt":259},{"And":[[62,260],null]},{"Or":[261]},{"And":[[118],null]},{"Token":27},{"And":[[264],null]},{"Or":[263,265]},{"Opt":266},{"Token":18},{"Token":57},{"And":[[269,49],null]},{"Or":[270]},{"Opt":271},{"Token":59},{"And":[[273],null]},"Eof",{"And":[[275],null]},{"Or":[274,276]},{"And":[[267,268,272,277],2]},{"Or":[278]},{"Token":11},{"And":[[280],null]},{"ContextualToken":[4,"union"]},{"And":[[282],null]},{"Or":[281,283]},{"Token":90},{"Opt":31},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,287]]]},{"And":[[288],null]},{"Token":56},{"And":[[290],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,292]]]},{"Token":56},{"And":[[293,294],null]},{"Or":[289,291,295]},{"And":[[284,285,286,296],1]},{"Or":[297]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[299,300,301,49],2]},{"Or":[302]},{"Opt":36},{"And":[[304,49],null]},{"Or":[305]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[146,[[2,19]]]},{"Call":[147,[[3,310]]]},{"And":[[307,308,309,311],1]},{"Or":[312]},{"Token":90},{"Token":51},{"And":[[315,74],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,317]]]},{"And":[[318],null]},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,320]]]},{"And":[[321],null]},{"Or":[316,319,322]},{"Opt":323},{"And":[[314,324],1]},{"Or":[325]},{"Token":13},{"Token":90},{"Token":56},{"And":[[329],null]},{"Call":[147,[[3,1]]]},{"And":[[331],null]},{"Or":[330,332]},{"And":[[327,328,333],1]},{"Or":[334]},{"Token":33},{"Opt":336},{"Token":17},{"Opt":31},{"Token":23},{"And":[[340,49],null]},{"Or":[341]},{"Opt":342},{"And":[[49,343],null]},{"Or":[344]},{"Opt":37},{"And":[[337,338,339,345,346,23],2]},{"Or":[347]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[141,349,350,351,352,353,354,23],3]},{"Or":[355]},{"Opt":36},{"And":[[141,357],null]},{"Or":[358]},{"Inject":[359,24]},{"And":[[360],null]},{"And":[[28],null]},{"Or":[361,362]},{"WithSkip":[25,363]},{"Rep":364},{"Call":[147,[[3,365]]]},{"And":[[366],null]},{"Or":[367]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[372],null]},{"Token":8},{"And":[[374],null]},{"Token":20},{"And":[[376],null]},{"Token":21},{"And":[[378],null]},{"Token":22},{"And":[[380],null]},{"Token":33},{"And":[[382],null]},{"Token":63},{"And":[[384],null]},{"Token":7},{"And":[[386],null]},{"Token":90},{"Token":80},{"And":[[388,389],null]},{"Or":[390]},{"And":[[391],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[396,49],null]},{"Or":[397]},{"Opt":398},{"Token":56},{"And":[[393,394,395,399,400],1]},{"Or":[401]},{"Token":21},{"And":[[403],null]},{"Token":22},{"And":[[405],null]},{"Or":[404,406]},{"Token":90},{"Token":57},{"Token":51},{"And":[[410,74],null]},{"Or":[411]},{"Opt":412},{"Token":56},{"And":[[407,408,409,49,413,414],1]},{"Or":[415]},{"And":[[143],null]},{"Token":56},{"And":[[144,418],null]},{"Or":[417,419]},{"Rep":30},{"Call":[147,[[3,421]]]},{"And":[[10,422],null]},{"Or":[423]},{"Opt":36},{"And":[[141,425],null]},{"Or":[426]},{"And":[[8],null]},{"And":[[27],null]},{"Or":[428,429]},{"Inject":[427,430]},{"And":[[431],null]},{"Call":[146,[[2,35]]]},{"Call":[146,[[2,32]]]},{"And":[[433,434],null]},{"Or":[435]},{"Call":[149,[[5,436]]]},{"And":[[437],null]},{"Or":[438]},{"Token":90},{"Opt":33},{"And":[[440,441],1]},{"Or":[442]},{"Token":57},{"Token":71},{"And":[[445],null]},"Eof",{"And":[[447],null]},{"Token":59},{"And":[[449],null]},{"Token":37},{"And":[[451],null]},{"Token":34},{"And":[[453],null]},{"Or":[450,452,454]},{"Not":455},{"Not":456},{"And":[[457],null]},{"Or":[446,448,458]},{"And":[[34,459],1]},{"Or":[460]},{"Rep":461},{"And":[[444,462],null]},{"Token":85},{"And":[[464],null]},{"And":[[51],null]},{"Or":[465,466]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[471],null]},"Eof",{"And":[[473],null]},{"Token":59},{"Not":475},{"Not":476},{"And":[[477],null]},{"Or":[472,474,478]},{"And":[[470,479],1]},{"Or":[480]},{"Rep":481},{"And":[[469,482],null]},{"Or":[483]},{"Opt":484},{"And":[[468,485],1]},{"Or":[486]},{"Token":10},{"Token":6},{"And":[[489],null]},{"Token":19},{"And":[[491],null]},{"Or":[490,492]},{"Call":[148,[[4,493]]]},{"Opt":494},{"And":[[488,495],null]},{"Or":[496]},{"Token":34},{"Token":59},{"And":[[499],null]},"Eof",{"And":[[501],null]},{"Token":37},{"Not":503},{"Not":504},{"And":[[505],null]},{"Or":[500,502,506]},{"And":[[49,33,507],null]},{"Or":[508]},{"Rep":509},{"And":[[498,510],1]},{"Or":[511]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[514],null]},{"Enter":[1,41]},{"And":[[516],null]},{"Token":58},{"And":[[518,45],null]},{"Or":[519]},{"Token":58},{"Opt":521},{"And":[[522,45],null]},{"Or":[523]},{"Token":58},{"And":[[525,45],null]},{"Or":[526]},{"And":[[41,527],null]},{"Or":[528]},{"Token":5},{"And":[[49,530,49],null]},{"Or":[531]},{"Call":[149,[[5,532]]]},{"Token":58},{"And":[[533,534,45],null]},{"Or":[535]},{"Token":90},{"And":[[537],null]},{"Token":18},{"And":[[539],null]},{"Token":19},{"And":[[541],null]},{"Or":[538,540,542]},{"And":[[46],null]},{"IsIn":3},{"And":[[545,47],null]},{"Or":[544,546]},{"Opt":547},{"And":[[543,548],null]},{"Or":[549]},{"IsIn":3},{"And":[[551],null]},{"IsIn":1},{"Token":58},{"And":[[553,554],null]},{"Or":[552,555]},{"Token":85},{"Call":[146,[[2,557]]]},{"Token":90},{"Token":51},{"And":[[559,560],null]},{"Or":[561]},{"Not":562},{"And":[[563,49],null]},{"Or":[564]},{"Call":[146,[[2,565]]]},{"Token":90},{"Token":51},{"And":[[567,568,49],null]},{"Or":[569]},{"Call":[146,[[2,570]]]},{"And":[[558,566,571],null]},{"Or":[572]},{"Call":[149,[[5,573]]]},{"And":[[556,574],null]},{"Or":[575]},{"Call":[146,[[2,49]]]},{"Call":[148,[[4,577]]]},{"Opt":9},{"And":[[578,579],null]},{"Or":[580]},{"Token":5},{"Token":90},{"And":[[582,583],null]},{"Or":[584]},{"Token":71},{"And":[[586,34],null]},{"Or":[587]},{"Rep":588},{"And":[[50,589],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[600]},{"And":[[118,49],null]},{"Or":[602]},{"Token":65},{"Token":21},{"And":[[605],null]},{"Token":27},{"And":[[607],null]},{"Or":[606,608]},{"And":[[604,609,49],null]},{"Or":[610]},{"Token":79},{"And":[[612],null]},{"Or":[613]},{"Token":35},{"Token":36},{"And":[[615,616],null]},{"Or":[617]},{"Opt":57},{"And":[[49,619],null]},{"Or":[620]},{"Call":[148,[[4,621]]]},{"And":[[622],null]},{"Or":[623]},{"Token":59},{"Call":[146,[[2,49]]]},{"And":[[625,626],null]},{"Or":[627]},{"Token":80},{"And":[[629],null]},{"Or":[630]},{"Token":56},{"And":[[632,74],null]},{"Or":[633]},{"Opt":634},{"And":[[49,635],null]},{"Or":[636]},{"Call":[150,[[6,637]]]},{"And":[[638],null]},{"Or":[639]},{"Token":8},{"Call":[146,[[2,61]]]},{"Call":[148,[[4,642]]]},{"And":[[641,643,9],1]},{"Or":[644]},{"Token":57},{"And":[[62,646],null]},{"Or":[647]},{"Opt":648},{"And":[[649,49],null]},{"Or":[650]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":79},{"And":[[659],null]},{"Or":[660]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[662,663]},{"Opt":664},{"And":[[40,665],null]},{"Or":[666]},{"Call":[146,[[2,62]]]},{"Token":61},{"Token":59},{"Opt":670},{"And":[[669,671],null]},{"Or":[672]},{"Opt":673},{"And":[[668,674],null]},{"Or":[675]},{"Call":[148,[[4,676]]]},{"And":[[677],null]},{"Or":[678]},{"Call":[146,[[2,67]]]},{"Token":61},{"Token":59},{"Opt":682},{"And":[[681,683],null]},{"Or":[684]},{"Opt":685},{"And":[[680,686],null]},{"Or":[687]},{"Call":[147,[[3,688]]]},{"And":[[689],null]},{"Or":[690]},{"Token":57},{"Not":692},{"And":[[68,693],null]},{"Token":90},{"Token":57},{"And":[[695,696,62],2]},{"Or":[694,697]},{"Token":28},{"Opt":699},{"Token":27},{"Opt":701},{"Token":90},{"And":[[700,702,703],null]},{"Or":[704]},{"And":[[78],null]},{"Or":[706]},{"Token":35},{"Token":36},{"And":[[708,709],null]},{"Or":[710]},{"Opt":72},{"And":[[62,712],null]},{"Or":[713]},{"Call":[148,[[4,714]]]},{"And":[[715],null]},{"Or":[716]},{"Token":59},{"Call":[146,[[2,62]]]},{"And":[[718,719],null]},{"Or":[720]},{"Token":75},{"Token":27},{"Opt":723},{"And":[[722,724,62],null]},{"Or":[725]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":729},{"And":[[727,728,730,111],null]},{"Or":[731]},{"IsIn":2},{"Not":76},{"And":[[733,734],null]},{"IsIn":2},{"Not":736},{"And":[[737],null]},{"Or":[735,738]},{"And":[[739,111],null]},{"Or":[740]},{"Token":60},{"Token":90},{"And":[[743],null]},{"Token":87},{"And":[[745],null]},{"Or":[744,746]},{"And":[[742,747],null]},{"Or":[748]},{"Call":[150,[[6,74]]]},{"Token":81},{"Token":5},{"And":[[752,49],null]},{"Or":[753]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[758],null]},{"Token":67},{"And":[[760],null]},{"Token":69},{"And":[[762],null]},{"Or":[759,761,763]},{"Call":[125,[[1,764]]]},{"Token":71},{"And":[[766],null]},{"Token":73},{"And":[[768],null]},{"Or":[767,769]},{"Call":[125,[[1,770]]]},{"ContextualToken":[43,"<<"]},{"And":[[772],null]},{"ContextualToken":[45,">>"]},{"And":[[774],null]},{"Or":[773,775]},{"Call":[125,[[1,776]]]},{"Token":75},{"Token":75},{"Not":779},{"And":[[778,780],null]},{"Or":[781]},{"Call":[125,[[1,782]]]},{"Token":82},{"Call":[125,[[1,784]]]},{"Token":77},{"Token":77},{"Not":787},{"And":[[786,788],null]},{"Or":[789]},{"Call":[125,[[1,790]]]},{"Call":[125,[[1,130]]]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,793]]]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,795]]]},{"Call":[125,[[1,138]]]},{"Token":51},{"And":[[798],null]},{"Token":72},{"And":[[800],null]},{"Token":74},{"And":[[802],null]},{"Token":66},{"And":[[804],null]},{"Token":68},{"And":[[806],null]},{"Token":70},{"And":[[808],null]},{"Token":76},{"And":[[810],null]},{"Token":78},{"And":[[812],null]},{"Token":83},{"And":[[814],null]},{"ContextualToken":[46,">>="]},{"And":[[816],null]},{"ContextualToken":[44,"<<="]},{"And":[[818],null]},{"Or":[799,801,803,805,807,809,811,813,815,817,819]},{"Call":[125,[[1,820]]]},{"And":[[78],null]},{"Token":90},{"And":[[823],null]},{"Token":18},{"And":[[825],null]},{"Token":19},{"And":[[827],null]},{"Token":39},{"And":[[829],null]},{"Token":58},{"And":[[831],null]},{"Token":35},{"And":[[833],null]},{"Token":41},{"And":[[835],null]},{"Token":77},{"And":[[837],null]},{"Token":31},{"And":[[839],null]},{"Token":37},{"And":[[841],null]},{"Token":14},{"And":[[843],null]},{"Token":25},{"And":[[845],null]},{"Token":24},{"And":[[847],null]},{"Token":23},{"And":[[849],null]},{"Token":30},{"And":[[851],null]},{"Token":75},{"And":[[853],null]},{"Token":65},{"And":[[855],null]},{"Token":73},{"And":[[857],null]},{"Token":80},{"And":[[859],null]},{"Token":61},{"And":[[861],null]},{"Token":62},{"And":[[863],null]},{"PrevIs":[158,164,165,166,167,168,171]},{"And":[[865],null]},{"Var":0},{"Exit":[2,867]},{"Exit":[0,868]},{"And":[[869],null]},{"Token":87},{"And":[[871],null]},{"Token":88},{"And":[[873],null]},{"Token":89},{"And":[[875],null]},{"Token":84},{"And":[[877],null]},{"Token":86},{"And":[[879],null]},{"Or":[872,874,876,878,880]},{"Token":90},{"Token":80},{"And":[[882,883],null]},{"Or":[884]},{"Not":885},{"Opt":80},{"And":[[886,40,887],null]},{"Or":[888]},{"IsIn":0},{"Not":890},{"Call":[146,[[2,81]]]},{"Token":61},{"Call":[77,[[0,74]]]},{"And":[[893,894],null]},{"Or":[895]},{"Opt":896},{"And":[[892,897],null]},{"Or":[898]},{"Call":[147,[[3,899]]]},{"And":[[891,900],null]},{"Or":[901]},{"Token":90},{"Token":57},{"And":[[904,74],null]},{"Or":[905]},{"Opt":906},{"And":[[903,907],1]},{"Or":[908]},{"Token":35},{"Token":36},{"And":[[910,911],null]},{"Or":[912]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[914,915],null]},{"Or":[916]},{"Call":[148,[[4,917]]]},{"And":[[918],null]},{"Or":[919]},{"Token":59},{"Call":[77,[[0,74]]]},{"Call":[146,[[2,922]]]},{"And":[[921,923],null]},{"Or":[924]},{"Call":[146,[[2,74]]]},{"Call":[77,[[0,926]]]},{"Call":[150,[[6,927]]]},{"And":[[928],null]},{"Or":[929]},{"Token":26},{"Opt":931},{"Token":77},{"Rep":87},{"Token":77},{"Token":49},{"And":[[936,49,89],null]},{"Call":[77,[[0,74]]]},{"And":[[938],null]},{"Or":[937,939]},{"And":[[932,933,934,935,940],null]},{"Or":[941]},{"Token":59},{"And":[[943],null]},{"Token":77},{"Not":945},{"Not":946},{"And":[[947],null]},{"Or":[944,948]},{"And":[[13,949],1]},{"Token":31},{"Opt":74},{"And":[[951,952],null]},{"Or":[953]},{"Token":33},{"Opt":955},{"Rep":90},{"Opt":74},{"And":[[957,958],null]},{"Or":[959]},{"Call":[147,[[3,960]]]},{"And":[[956,961],null]},{"Or":[962]},{"Call":[77,[[0,963]]]},{"And":[[964],null]},{"Or":[965]},{"And":[[91],null]},{"And":[[95],null]},{"And":[[94],null]},{"And":[[3],null]},{"Token":9},{"Opt":92},{"Opt":93},{"Token":56},{"And":[[971,62,972,973,974],1]},{"Or":[975]},{"Token":57},{"And":[[977,49],null]},{"Or":[978]},{"Token":51},{"And":[[980,74],null]},{"Or":[981]},{"Token":56},{"And":[[983],null]},{"Or":[984]},"Eof",{"Not":986},{"And":[[76,987],null]},{"Token":56},{"And":[[989],null]},{"Or":[988,990]},{"And":[[74,991],null]},{"Or":[992]},{"Enter":[2,993]},{"And":[[994],null]},{"Or":[995]},{"Token":14},{"Token":15},{"And":[[89],null]},{"And":[[96],null]},{"Or":[999,1000]},{"And":[[998,1001],null]},{"Or":[1002]},{"Opt":1003},{"And":[[997,98,89,1004],1]},{"Or":[1005]},{"Opt":102},{"Token":25},{"And":[[1007,1008,98,89],2]},{"Or":[1009]},{"Token":9},{"Token":51},{"And":[[1011,62,1012],1]},{"Or":[1013]},{"Opt":1014},{"And":[[1015,99],null]},{"Enter":[0,74]},{"And":[[1017],null]},{"Opt":102},{"Token":24},{"And":[[1019,1020,89],2]},{"Or":[1021]},{"Opt":102},{"Token":23},{"Token":32},{"And":[[1023,1024,62,1025,99,89],2]},{"Or":[1026]},{"Token":85},{"Token":57},{"And":[[1028,1029],null]},{"Token":30},{"Rep":104},{"Call":[147,[[3,1032]]]},{"And":[[1031,99,1033],1]},{"Or":[1034]},{"Token":50},{"Enter":[2,74]},{"Token":59},{"And":[[1038],null]},"Eof",{"And":[[1040],null]},{"And":[[76],null]},{"Or":[1039,1041,1042]},{"And":[[105,1036,1037,1043],1]},{"Or":[1044]},{"Token":77},{"And":[[1046,62],null]},{"Or":[1047]},{"Rep":1048},{"Opt":106},{"And":[[62,1049,1050],null]},{"Token":14},{"And":[[1052,74],null]},{"Or":[1053]},{"And":[[143],null]},{"Or":[1055]},{"And":[[144],null]},{"Or":[1057]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1061},{"And":[[1059,1060,1062,111],null]},{"Or":[1063]},{"And":[[74,1064],null]},{"Or":[1065]},{"IsIn":2},{"Not":76},{"And":[[1067,1068],null]},{"IsIn":2},{"Not":1070},{"And":[[1071],null]},{"Or":[1069,1072]},{"And":[[1073,111],null]},{"Or":[1074]},{"And":[[74,1075],null]},{"Or":[1076]},{"Call":[146,[[2,112]]]},{"Call":[148,[[4,1078]]]},{"Call":[77,[[0,1079]]]},{"And":[[1080],null]},{"And":[[74],null]},{"Or":[1082]},{"Token":60},{"Token":90},{"And":[[1085],null]},{"Token":87},{"And":[[1087],null]},{"Or":[1086,1088]},{"And":[[1084,1089],null]},{"Or":[1090]},{"And":[[74,1091],null]},{"Or":[1092]},{"Call":[150,[[6,74]]]},{"And":[[74,1094],null]},{"Or":[1095]},{"Token":81},{"And":[[74,1097],null]},{"Or":[1098]},{"Token":5},{"And":[[1100,49],null]},{"Or":[1101]},{"And":[[74,1102],null]},{"Or":[1103]},{"And":[[118,74],null]},{"Or":[1105]},{"Token":75},{"Token":85},{"Opt":1108},{"Token":27},{"Opt":1110},{"And":[[1107,1109,1111],null]},{"Token":65},{"And":[[1113,74],null]},{"Or":[1114]},{"Token":73},{"And":[[1116,74],null]},{"Or":[1117]},{"Token":80},{"And":[[1119,74],null]},{"Or":[1120]},{"Token":65},{"And":[[1122],null]},{"Token":67},{"And":[[1124],null]},{"Token":69},{"And":[[1126],null]},{"Or":[1123,1125,1127]},{"Call":[125,[[1,1128]]]},{"And":[[74,1129,74],null]},{"Or":[1130]},{"Token":71},{"And":[[1132],null]},{"Token":73},{"And":[[1134],null]},{"Or":[1133,1135]},{"Call":[125,[[1,1136]]]},{"And":[[74,1137,74],null]},{"Or":[1138]},{"ContextualToken":[43,"<<"]},{"And":[[1140],null]},{"ContextualToken":[45,">>"]},{"And":[[1142],null]},{"Or":[1141,1143]},{"Call":[125,[[1,1144]]]},{"And":[[74,1145,74],null]},{"Or":[1146]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1148,1149,1150],null]},{"IsIn":2},{"Not":1152},{"Var":1},{"And":[[1153,1154],null]},{"Token":75},{"Token":75},{"Not":1157},{"And":[[1156,1158],null]},{"Or":[1159]},{"Call":[125,[[1,1160]]]},{"And":[[74,1161,74],null]},{"Or":[1162]},{"Token":82},{"Call":[125,[[1,1164]]]},{"And":[[74,1165,74],null]},{"Or":[1166]},{"Token":77},{"Token":77},{"Not":1169},{"And":[[1168,1170],null]},{"Or":[1171]},{"Call":[125,[[1,1172]]]},{"And":[[74,1173,74],null]},{"Or":[1174]},{"Call":[125,[[1,130]]]},{"And":[[74,1176,74],null]},{"Or":[1177]},{"Token":52},{"And":[[1179],null]},{"Token":53},{"And":[[1181],null]},{"Token":39},{"And":[[1183],null]},{"Token":40},{"And":[[1185],null]},{"Token":55},{"And":[[1187],null]},{"Token":54},{"And":[[1189],null]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,1191]]]},{"And":[[74,1192,74],null]},{"Or":[1193]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,1195]]]},{"And":[[74,1196,74],null]},{"Or":[1197]},{"Call":[125,[[1,138]]]},{"And":[[74,1199,74],null]},{"Or":[1200]},{"And":[[138,74],null]},{"Or":[1202]},{"And":[[74,137],null]},{"Or":[1204]},{"Token":61},{"Not":75},{"And":[[1206,1207],null]},{"Or":[1208]},{"Token":61},{"And":[[1210],null]},{"Token":62},{"And":[[1212],null]},{"Not":75},{"Not":1214},{"Token":37},{"IsIn":0},{"And":[[1216,1217],null]},{"Or":[1218]},{"Not":1219},{"And":[[137,1215,1220],null]},{"Token":51},{"And":[[1222],null]},{"Token":72},{"And":[[1224],null]},{"Token":74},{"And":[[1226],null]},{"Token":66},{"And":[[1228],null]},{"Token":68},{"And":[[1230],null]},{"Token":70},{"And":[[1232],null]},{"Token":76},{"And":[[1234],null]},{"Token":78},{"And":[[1236],null]},{"Token":83},{"And":[[1238],null]},{"ContextualToken":[46,">>="]},{"And":[[1240],null]},{"ContextualToken":[44,"<<="]},{"And":[[1242],null]},{"Or":[1223,1225,1227,1229,1231,1233,1235,1237,1239,1241,1243]},{"Call":[125,[[1,1244]]]},{"And":[[74,1245,74],null]},{"Or":[1246]},{"Token":63},{"Call":[146,[[2,142]]]},{"Call":[150,[[6,1249]]]},{"And":[[1248,1250],null]},{"Or":[1251]},{"Rep":140},{"And":[[1253],null]},{"Token":90},{"Token":51},{"And":[[1256,74],null]},{"Call":[146,[[2,142]]]},{"Call":[148,[[4,1258]]]},{"And":[[1259],null]},{"Or":[1257,1260]},{"Opt":1261},{"And":[[1255,1262],1]},{"Or":[1263]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1267},{"Rep":145},{"Call":[147,[[3,1269]]]},{"And":[[1265,1266,1268,1270],null]},{"Or":[1271]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1275},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1277,1278,1279],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1281,1282,1283],null]},{"Or":[1280,1284]},{"And":[[1273,1274,1276,1285],null]},{"Or":[1286]},{"Token":35},{"And":[[1288],null]},{"Token":36},{"And":[[1290],null]},{"Token":37},{"And":[[1292],null]},{"Token":38},{"And":[[1294],null]},{"Token":41},{"And":[[1296],null]},{"Token":42},{"And":[[1298],null]},{"Or":[1289,1291,1293,1295,1297,1299]},{"Not":1300},"Any",{"And":[[1301,1302],null]},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1304,1305,1306],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1308,1309,1310],null]},{"Token":37},{"Rep":145},{"Token":38},{"And":[[1312,1313,1314],null]},{"Or":[1303,1307,1311,1315]},{"Var":2},"Eof",{"And":[[1318],null]},{"Token":59},{"And":[[1320],null]},{"Or":[1319,1321]},{"And":[[1317,1322],1]},{"Or":[1323]},{"Rep":1324},{"And":[[1325],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[151,[[7,1327],[8,1328],[9,1329]]]},{"And":[[1330],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[151,[[7,1332],[8,1333],[9,1334]]]},{"And":[[1335],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[151,[[7,1337],[8,1338],[9,1339]]]},{"And":[[1340],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[151,[[7,1342],[8,1343],[9,1344]]]},{"And":[[1345],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[152,[[10,1348],[11,1349]]]},{"Var":9},{"Layer":[1350,1351]},{"Var":8},{"And":[[1347,1352,1353],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[152,[[10,1356],[11,1357]]]},{"Var":11},{"And":[[1355,1358,1359],1]},{"Var":11},{"Not":1361},"Any",{"And":[[1362,1363],null]},{"Or":[1364]},{"And":[[1365],null]},{"Or":[1360,1366]},{"Rep":1367},{"And":[[1368],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, POINTER_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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
impl<'f> NameOwner<'f> for TypeParameter<'f> {}
pub trait TypeParametersOwner<'f>: rt::AstNode<'f> {
    fn type_parameters(&self) -> Option<TypeParameters<'f>> {
        rt::AstChildren::new(self.node().children()).next()
    }
}
impl<'f> TypeParametersOwner<'f> for StructDef<'f> {}
impl<'f> TypeParametersOwner<'f> for EnumDef<'f> {}