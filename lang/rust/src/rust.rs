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
        let parser_json = r##"[{"Pub":{"ty":91,"body":154,"replaceable":false}},{"Or":[157]},{"Or":[159,161,163,165,167,169,171,172]},{"Cached":189},{"Pub":{"ty":92,"body":205,"replaceable":false}},{"Pub":{"ty":93,"body":211,"replaceable":false}},{"Pub":{"ty":94,"body":217,"replaceable":false}},{"Pub":{"ty":95,"body":224,"replaceable":false}},{"Pub":{"ty":96,"body":240,"replaceable":false}},{"Or":[242]},{"Pub":{"ty":97,"body":247,"replaceable":false}},{"Or":[253]},{"Pub":{"ty":98,"body":256,"replaceable":false}},{"Pub":{"ty":99,"body":262,"replaceable":false}},{"Pub":{"ty":100,"body":279,"replaceable":false}},{"Pub":{"ty":101,"body":298,"replaceable":false}},{"Pub":{"ty":102,"body":303,"replaceable":false}},{"Pub":{"ty":103,"body":306,"replaceable":false}},{"Pub":{"ty":104,"body":313,"replaceable":false}},{"Pub":{"ty":105,"body":326,"replaceable":false}},{"Pub":{"ty":106,"body":335,"replaceable":false}},{"Pub":{"ty":107,"body":348,"replaceable":false}},{"Pub":{"ty":108,"body":356,"replaceable":false}},{"Pub":{"ty":109,"body":368,"replaceable":false}},{"Or":[369,370,371]},{"Or":[373,375,377,379,381,383,385,387,392]},{"Pub":{"ty":110,"body":402,"replaceable":false}},{"Pub":{"ty":111,"body":416,"replaceable":false}},{"Pub":{"ty":112,"body":420,"replaceable":false}},{"Pub":{"ty":113,"body":424,"replaceable":false}},{"Or":[425,426]},{"Pub":{"ty":114,"body":433,"replaceable":false}},{"Pub":{"ty":115,"body":437,"replaceable":false}},{"Or":[457]},{"Pub":{"ty":116,"body":461,"replaceable":false}},{"Pub":{"ty":117,"body":481,"replaceable":false}},{"Pub":{"ty":118,"body":491,"replaceable":false}},{"Pub":{"ty":119,"body":506,"replaceable":false}},{"Or":[507]},{"Or":[509]},{"Or":[511]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":514,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":518,"replaceable":false}},{"Pub":{"ty":120,"body":523,"replaceable":false}},{"Pub":{"ty":121,"body":530,"replaceable":false}},{"Pub":{"ty":122,"body":544,"replaceable":false}},{"Pub":{"ty":123,"body":570,"replaceable":false}},{"Pub":{"ty":124,"body":575,"replaceable":false}},{"Pub":{"ty":125,"body":579,"replaceable":false}},{"Or":[584]},{"Or":[585,586,587,588,589,590,591,592,593]},{"Pub":{"ty":126,"body":595,"replaceable":false}},{"Pub":{"ty":127,"body":597,"replaceable":false}},{"Pub":{"ty":128,"body":605,"replaceable":false}},{"Pub":{"ty":129,"body":608,"replaceable":false}},{"Pub":{"ty":130,"body":612,"replaceable":false}},{"Pub":{"ty":131,"body":618,"replaceable":true}},{"PubReplace":{"ty":132,"body":622}},{"Pub":{"ty":133,"body":625,"replaceable":false}},{"Pub":{"ty":134,"body":634,"replaceable":false}},{"Pub":{"ty":135,"body":639,"replaceable":false}},{"Pub":{"ty":98,"body":645,"replaceable":false}},{"Or":[646,647,648,649,650,651,652]},{"Pub":{"ty":136,"body":655,"replaceable":false}},{"Pub":{"ty":137,"body":661,"replaceable":true}},{"PubReplace":{"ty":138,"body":673}},{"PubReplace":{"ty":139,"body":685}},{"Pub":{"ty":140,"body":692,"replaceable":false}},{"Pub":{"ty":141,"body":699,"replaceable":false}},{"Pub":{"ty":142,"body":701,"replaceable":false}},{"Pub":{"ty":143,"body":705,"replaceable":false}},{"Pub":{"ty":144,"body":711,"replaceable":true}},{"PubReplace":{"ty":145,"body":715}},{"Pub":{"ty":146,"body":720,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,96,97,100,101,103,107,108,136],"prefixes":[{"ty":180,"op":118,"priority":999},{"ty":181,"op":749,"priority":999},{"ty":182,"op":750,"priority":999},{"ty":183,"op":751,"priority":999},{"ty":193,"op":138,"priority":2}],"infixes":[{"ty":173,"op":726,"priority":999,"has_rhs":false},{"ty":174,"op":735,"priority":999,"has_rhs":false},{"ty":176,"op":743,"priority":999,"has_rhs":false},{"ty":177,"op":744,"priority":999,"has_rhs":false},{"ty":178,"op":745,"priority":999,"has_rhs":false},{"ty":179,"op":748,"priority":999,"has_rhs":false},{"ty":184,"op":759,"priority":11,"has_rhs":true},{"ty":185,"op":765,"priority":10,"has_rhs":true},{"ty":186,"op":771,"priority":9,"has_rhs":true},{"ty":187,"op":777,"priority":8,"has_rhs":true},{"ty":188,"op":779,"priority":7,"has_rhs":true},{"ty":189,"op":785,"priority":6,"has_rhs":true},{"ty":190,"op":786,"priority":5,"has_rhs":true},{"ty":191,"op":788,"priority":4,"has_rhs":true},{"ty":192,"op":790,"priority":3,"has_rhs":true},{"ty":193,"op":791,"priority":2,"has_rhs":true},{"ty":193,"op":137,"priority":2,"has_rhs":false},{"ty":194,"op":815,"priority":1,"has_rhs":true}]}},{"Or":[816,818,820,822,824,826,828,830,832,834,836,838,840,842,844,846,848,850,852,854,856,858]},{"Or":[860]},{"Or":[864]},{"Pub":{"ty":148,"body":875,"replaceable":false}},{"Pub":{"ty":149,"body":883,"replaceable":true}},{"PubReplace":{"ty":150,"body":896}},{"Pub":{"ty":151,"body":903,"replaceable":false}},{"Pub":{"ty":152,"body":907,"replaceable":false}},{"Pub":{"ty":153,"body":914,"replaceable":true}},{"PubReplace":{"ty":154,"body":919}},{"Pub":{"ty":155,"body":924,"replaceable":false}},{"Pub":{"ty":156,"body":936,"replaceable":false}},{"Or":[944]},{"Pub":{"ty":157,"body":948,"replaceable":false}},{"Pub":{"ty":158,"body":960,"replaceable":false}},{"Or":[961,962,963,964]},{"Pub":{"ty":159,"body":970,"replaceable":false}},{"Pub":{"ty":160,"body":973,"replaceable":false}},{"Pub":{"ty":161,"body":976,"replaceable":false}},{"Pub":{"ty":162,"body":979,"replaceable":false}},{"Pub":{"ty":163,"body":990,"replaceable":false}},{"Pub":{"ty":164,"body":1000,"replaceable":false}},{"Pub":{"ty":165,"body":1004,"replaceable":false}},{"Or":[1010]},{"Or":[1012]},{"Pub":{"ty":166,"body":1016,"replaceable":false}},{"Pub":{"ty":167,"body":1021,"replaceable":false}},{"Or":[1024]},{"Pub":{"ty":168,"body":1029,"replaceable":false}},{"Pub":{"ty":169,"body":1039,"replaceable":false}},{"Or":[1045]},{"Pub":{"ty":170,"body":1048,"replaceable":false}},{"Pub":{"ty":171,"body":1050,"replaceable":false}},{"Pub":{"ty":172,"body":1052,"replaceable":false}},{"Pub":{"ty":173,"body":1060,"replaceable":false}},{"Pub":{"ty":174,"body":1071,"replaceable":false}},{"Or":[1075]},{"Pub":{"ty":175,"body":1077,"replaceable":false}},{"Pub":{"ty":176,"body":1087,"replaceable":false}},{"Pub":{"ty":177,"body":1090,"replaceable":false}},{"Pub":{"ty":178,"body":1093,"replaceable":false}},{"Pub":{"ty":179,"body":1098,"replaceable":false}},{"Pub":{"ty":180,"body":1100,"replaceable":false}},{"Or":[1106]},{"Pub":{"ty":181,"body":1109,"replaceable":false}},{"Pub":{"ty":182,"body":1112,"replaceable":false}},{"Pub":{"ty":183,"body":1115,"replaceable":false}},{"Pub":{"ty":184,"body":1125,"replaceable":false}},{"Pub":{"ty":185,"body":1133,"replaceable":false}},{"Pub":{"ty":186,"body":1141,"replaceable":false}},{"Or":[1145,1149]},{"Pub":{"ty":187,"body":1157,"replaceable":false}},{"Pub":{"ty":188,"body":1161,"replaceable":false}},{"Pub":{"ty":189,"body":1169,"replaceable":false}},{"Pub":{"ty":190,"body":1172,"replaceable":false}},{"Or":[1174,1176,1178,1180,1182,1184]},{"Pub":{"ty":191,"body":1188,"replaceable":false}},{"Pub":{"ty":192,"body":1192,"replaceable":false}},{"Pub":{"ty":193,"body":1195,"replaceable":false}},{"Pub":{"ty":193,"body":1197,"replaceable":false}},{"Pub":{"ty":193,"body":1199,"replaceable":false}},{"Pub":{"ty":193,"body":1203,"replaceable":false}},{"Or":[1205,1207]},{"Or":[1215]},{"Pub":{"ty":194,"body":1241,"replaceable":false}},{"Pub":{"ty":195,"body":1246,"replaceable":false}},{"Or":[1248]},{"Pub":{"ty":196,"body":1258,"replaceable":false}},{"Pub":{"ty":197,"body":1266,"replaceable":false}},{"Pub":{"ty":198,"body":1281,"replaceable":false}},{"Pub":{"ty":199,"body":1310,"replaceable":false}},{"Or":[1320]},{"Or":[1325]},{"Or":[1330]},{"Or":[1335]},{"Or":[1340]},{"Or":[1348]},{"Or":[1363]},{"And":[[1],null]},{"Or":[153]},{"WithSkip":[2,3]},{"Rep":155},{"And":[[156],null]},{"Token":11},{"And":[[158],null]},{"ContextualToken":[4,"union"]},{"And":[[160],null]},{"Token":16},{"And":[[162],null]},{"Token":12},{"And":[[164],null]},{"Token":13},{"And":[[166],null]},{"Token":17},{"And":[[168],null]},{"Token":29},{"And":[[170],null]},{"And":[[25],null]},{"Opt":36},{"And":[[141,173],null]},{"Or":[174]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[176,177,178,179,180,181,182,183,184]},{"Inject":[175,185]},{"And":[[186],null]},{"And":[[28],null]},{"Or":[187,188]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[192,5],null]},{"Or":[193]},{"Opt":194},{"And":[[195],null]},{"Or":[191,196]},{"And":[[38,197],null]},{"Token":58},{"Opt":199},{"And":[[200,5],null]},{"Or":[198,201]},{"Token":56},{"And":[[190,202,203],1]},{"Or":[204]},{"Token":65},{"And":[[206],null]},{"Call":[146,[[2,6]]]},{"Call":[147,[[3,208]]]},{"And":[[209],null]},{"Or":[207,210]},{"Token":18},{"And":[[212],null]},{"Token":90},{"Opt":48},{"And":[[214,215],1]},{"Or":[213,216]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[218,219,220,221,222],2]},{"Or":[223]},{"Token":21},{"Opt":225},{"Token":33},{"Opt":227},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[89],null]},{"Token":56},{"And":[[236],null]},{"Or":[235,237]},{"And":[[226,228,229,230,231,232,11,233,234,238],4]},{"Or":[239]},{"Token":49},{"And":[[241,49],null]},{"Token":7},{"Token":88},{"Opt":244},{"And":[[243,245],null]},{"Or":[246]},{"Opt":14},{"Call":[146,[[2,12]]]},{"And":[[248,249],null]},{"Or":[250]},{"Call":[148,[[4,251]]]},{"And":[[252],null]},{"Token":57},{"And":[[62,254,49],1]},{"Or":[255]},{"Token":57},{"And":[[257,49],null]},{"Or":[258]},{"Opt":259},{"And":[[62,260],null]},{"Or":[261]},{"And":[[118],null]},{"Token":27},{"And":[[264],null]},{"Or":[263,265]},{"Opt":266},{"Token":18},{"Token":57},{"And":[[269,49],null]},{"Or":[270]},{"Opt":271},{"Token":59},{"And":[[273],null]},"Eof",{"And":[[275],null]},{"Or":[274,276]},{"And":[[267,268,272,277],2]},{"Or":[278]},{"Token":11},{"And":[[280],null]},{"ContextualToken":[4,"union"]},{"And":[[282],null]},{"Or":[281,283]},{"Token":90},{"Opt":31},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,287]]]},{"And":[[288],null]},{"Token":56},{"And":[[290],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,292]]]},{"Token":56},{"And":[[293,294],null]},{"Or":[289,291,295]},{"And":[[284,285,286,296],1]},{"Or":[297]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[299,300,301,49],2]},{"Or":[302]},{"Opt":36},{"And":[[304,49],null]},{"Or":[305]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[146,[[2,19]]]},{"Call":[147,[[3,310]]]},{"And":[[307,308,309,311],1]},{"Or":[312]},{"Token":90},{"Token":51},{"And":[[315,74],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,317]]]},{"And":[[318],null]},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,320]]]},{"And":[[321],null]},{"Or":[316,319,322]},{"Opt":323},{"And":[[314,324],1]},{"Or":[325]},{"Token":13},{"Token":90},{"Token":56},{"And":[[329],null]},{"Call":[147,[[3,1]]]},{"And":[[331],null]},{"Or":[330,332]},{"And":[[327,328,333],1]},{"Or":[334]},{"Token":33},{"Opt":336},{"Token":17},{"Opt":31},{"Token":23},{"And":[[340,49],null]},{"Or":[341]},{"Opt":342},{"And":[[49,343],null]},{"Or":[344]},{"Opt":37},{"And":[[337,338,339,345,346,23],2]},{"Or":[347]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[141,349,350,351,352,353,354,23],3]},{"Or":[355]},{"Opt":36},{"And":[[141,357],null]},{"Or":[358]},{"Inject":[359,24]},{"And":[[360],null]},{"And":[[28],null]},{"Or":[361,362]},{"WithSkip":[25,363]},{"Rep":364},{"Call":[147,[[3,365]]]},{"And":[[366],null]},{"Or":[367]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[372],null]},{"Token":8},{"And":[[374],null]},{"Token":20},{"And":[[376],null]},{"Token":21},{"And":[[378],null]},{"Token":22},{"And":[[380],null]},{"Token":33},{"And":[[382],null]},{"Token":63},{"And":[[384],null]},{"Token":7},{"And":[[386],null]},{"Token":90},{"Token":80},{"And":[[388,389],null]},{"Or":[390]},{"And":[[391],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[396,49],null]},{"Or":[397]},{"Opt":398},{"Token":56},{"And":[[393,394,395,399,400],1]},{"Or":[401]},{"Token":21},{"And":[[403],null]},{"Token":22},{"And":[[405],null]},{"Or":[404,406]},{"Token":90},{"Token":57},{"Token":51},{"And":[[410,74],null]},{"Or":[411]},{"Opt":412},{"Token":56},{"And":[[407,408,409,49,413,414],1]},{"Or":[415]},{"And":[[143],null]},{"Token":56},{"And":[[144,418],null]},{"Or":[417,419]},{"Rep":30},{"Call":[147,[[3,421]]]},{"And":[[10,422],null]},{"Or":[423]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[146,[[2,35]]]},{"Call":[146,[[2,32]]]},{"And":[[427,428],null]},{"Or":[429]},{"Call":[149,[[5,430]]]},{"And":[[431],null]},{"Or":[432]},{"Token":90},{"Opt":33},{"And":[[434,435],1]},{"Or":[436]},{"Token":57},{"Token":71},{"And":[[439],null]},"Eof",{"And":[[441],null]},{"Token":59},{"And":[[443],null]},{"Token":37},{"And":[[445],null]},{"Token":34},{"And":[[447],null]},{"Or":[444,446,448]},{"Not":449},{"Not":450},{"And":[[451],null]},{"Or":[440,442,452]},{"And":[[34,453],1]},{"Or":[454]},{"Rep":455},{"And":[[438,456],null]},{"Token":85},{"And":[[458],null]},{"And":[[51],null]},{"Or":[459,460]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[465],null]},"Eof",{"And":[[467],null]},{"Token":59},{"Not":469},{"Not":470},{"And":[[471],null]},{"Or":[466,468,472]},{"And":[[464,473],1]},{"Or":[474]},{"Rep":475},{"And":[[463,476],null]},{"Or":[477]},{"Opt":478},{"And":[[462,479],1]},{"Or":[480]},{"Token":10},{"Token":6},{"And":[[483],null]},{"Token":19},{"And":[[485],null]},{"Or":[484,486]},{"Call":[148,[[4,487]]]},{"Opt":488},{"And":[[482,489],null]},{"Or":[490]},{"Token":34},{"Token":59},{"And":[[493],null]},"Eof",{"And":[[495],null]},{"Token":37},{"Not":497},{"Not":498},{"And":[[499],null]},{"Or":[494,496,500]},{"And":[[49,33,501],null]},{"Or":[502]},{"Rep":503},{"And":[[492,504],1]},{"Or":[505]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[508],null]},{"Enter":[1,41]},{"And":[[510],null]},{"Token":58},{"And":[[512,45],null]},{"Or":[513]},{"Token":58},{"Opt":515},{"And":[[516,45],null]},{"Or":[517]},{"Token":58},{"And":[[519,45],null]},{"Or":[520]},{"And":[[41,521],null]},{"Or":[522]},{"Token":5},{"And":[[49,524,49],null]},{"Or":[525]},{"Call":[149,[[5,526]]]},{"Token":58},{"And":[[527,528,45],null]},{"Or":[529]},{"Token":90},{"And":[[531],null]},{"Token":18},{"And":[[533],null]},{"Token":19},{"And":[[535],null]},{"Or":[532,534,536]},{"And":[[46],null]},{"IsIn":3},{"And":[[539,47],null]},{"Or":[538,540]},{"Opt":541},{"And":[[537,542],null]},{"Or":[543]},{"IsIn":3},{"And":[[545],null]},{"IsIn":1},{"Token":58},{"And":[[547,548],null]},{"Or":[546,549]},{"Token":85},{"Call":[146,[[2,551]]]},{"Token":90},{"Token":51},{"And":[[553,554],null]},{"Or":[555]},{"Not":556},{"And":[[557,49],null]},{"Or":[558]},{"Call":[146,[[2,559]]]},{"Token":90},{"Token":51},{"And":[[561,562,49],null]},{"Or":[563]},{"Call":[146,[[2,564]]]},{"And":[[552,560,565],null]},{"Or":[566]},{"Call":[149,[[5,567]]]},{"And":[[550,568],null]},{"Or":[569]},{"Call":[146,[[2,49]]]},{"Call":[148,[[4,571]]]},{"Opt":9},{"And":[[572,573],null]},{"Or":[574]},{"Token":5},{"Token":90},{"And":[[576,577],null]},{"Or":[578]},{"Token":71},{"And":[[580,34],null]},{"Or":[581]},{"Rep":582},{"And":[[50,583],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[594]},{"And":[[118,49],null]},{"Or":[596]},{"Token":65},{"Token":21},{"And":[[599],null]},{"Token":27},{"And":[[601],null]},{"Or":[600,602]},{"And":[[598,603,49],null]},{"Or":[604]},{"Token":79},{"And":[[606],null]},{"Or":[607]},{"Token":35},{"Token":36},{"And":[[609,610],null]},{"Or":[611]},{"Opt":57},{"And":[[49,613],null]},{"Or":[614]},{"Call":[148,[[4,615]]]},{"And":[[616],null]},{"Or":[617]},{"Token":59},{"Call":[146,[[2,49]]]},{"And":[[619,620],null]},{"Or":[621]},{"Token":80},{"And":[[623],null]},{"Or":[624]},{"Token":56},{"And":[[626,74],null]},{"Or":[627]},{"Opt":628},{"And":[[49,629],null]},{"Or":[630]},{"Call":[150,[[6,631]]]},{"And":[[632],null]},{"Or":[633]},{"Token":8},{"Call":[146,[[2,61]]]},{"Call":[148,[[4,636]]]},{"And":[[635,637,9],1]},{"Or":[638]},{"Token":57},{"And":[[62,640],null]},{"Or":[641]},{"Opt":642},{"And":[[643,49],null]},{"Or":[644]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":79},{"And":[[653],null]},{"Or":[654]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[656,657]},{"Opt":658},{"And":[[40,659],null]},{"Or":[660]},{"Call":[146,[[2,62]]]},{"Token":61},{"Token":59},{"Opt":664},{"And":[[663,665],null]},{"Or":[666]},{"Opt":667},{"And":[[662,668],null]},{"Or":[669]},{"Call":[148,[[4,670]]]},{"And":[[671],null]},{"Or":[672]},{"Call":[146,[[2,67]]]},{"Token":61},{"Token":59},{"Opt":676},{"And":[[675,677],null]},{"Or":[678]},{"Opt":679},{"And":[[674,680],null]},{"Or":[681]},{"Call":[147,[[3,682]]]},{"And":[[683],null]},{"Or":[684]},{"Token":57},{"Not":686},{"And":[[68,687],null]},{"Token":90},{"Token":57},{"And":[[689,690,62],2]},{"Or":[688,691]},{"Token":28},{"Opt":693},{"Token":27},{"Opt":695},{"Token":90},{"And":[[694,696,697],null]},{"Or":[698]},{"And":[[78],null]},{"Or":[700]},{"Token":35},{"Token":36},{"And":[[702,703],null]},{"Or":[704]},{"Opt":72},{"And":[[62,706],null]},{"Or":[707]},{"Call":[148,[[4,708]]]},{"And":[[709],null]},{"Or":[710]},{"Token":59},{"Call":[146,[[2,62]]]},{"And":[[712,713],null]},{"Or":[714]},{"Token":75},{"Token":27},{"Opt":717},{"And":[[716,718,62],null]},{"Or":[719]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":723},{"And":[[721,722,724,111],null]},{"Or":[725]},{"IsIn":2},{"Not":76},{"And":[[727,728],null]},{"IsIn":2},{"Not":730},{"And":[[731],null]},{"Or":[729,732]},{"And":[[733,111],null]},{"Or":[734]},{"Token":60},{"Token":90},{"And":[[737],null]},{"Token":87},{"And":[[739],null]},{"Or":[738,740]},{"And":[[736,741],null]},{"Or":[742]},{"Call":[150,[[6,74]]]},{"Token":81},{"Token":5},{"And":[[746,49],null]},{"Or":[747]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[752],null]},{"Token":67},{"And":[[754],null]},{"Token":69},{"And":[[756],null]},{"Or":[753,755,757]},{"Call":[125,[[1,758]]]},{"Token":71},{"And":[[760],null]},{"Token":73},{"And":[[762],null]},{"Or":[761,763]},{"Call":[125,[[1,764]]]},{"ContextualToken":[43,"<<"]},{"And":[[766],null]},{"ContextualToken":[45,">>"]},{"And":[[768],null]},{"Or":[767,769]},{"Call":[125,[[1,770]]]},{"Token":75},{"Token":75},{"Not":773},{"And":[[772,774],null]},{"Or":[775]},{"Call":[125,[[1,776]]]},{"Token":82},{"Call":[125,[[1,778]]]},{"Token":77},{"Token":77},{"Not":781},{"And":[[780,782],null]},{"Or":[783]},{"Call":[125,[[1,784]]]},{"Call":[125,[[1,130]]]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,787]]]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,789]]]},{"Call":[125,[[1,138]]]},{"Token":51},{"And":[[792],null]},{"Token":72},{"And":[[794],null]},{"Token":74},{"And":[[796],null]},{"Token":66},{"And":[[798],null]},{"Token":68},{"And":[[800],null]},{"Token":70},{"And":[[802],null]},{"Token":76},{"And":[[804],null]},{"Token":78},{"And":[[806],null]},{"Token":83},{"And":[[808],null]},{"ContextualToken":[46,">>="]},{"And":[[810],null]},{"ContextualToken":[44,"<<="]},{"And":[[812],null]},{"Or":[793,795,797,799,801,803,805,807,809,811,813]},{"Call":[125,[[1,814]]]},{"And":[[78],null]},{"Token":90},{"And":[[817],null]},{"Token":18},{"And":[[819],null]},{"Token":19},{"And":[[821],null]},{"Token":39},{"And":[[823],null]},{"Token":58},{"And":[[825],null]},{"Token":35},{"And":[[827],null]},{"Token":41},{"And":[[829],null]},{"Token":77},{"And":[[831],null]},{"Token":31},{"And":[[833],null]},{"Token":37},{"And":[[835],null]},{"Token":14},{"And":[[837],null]},{"Token":25},{"And":[[839],null]},{"Token":24},{"And":[[841],null]},{"Token":23},{"And":[[843],null]},{"Token":30},{"And":[[845],null]},{"Token":75},{"And":[[847],null]},{"Token":65},{"And":[[849],null]},{"Token":73},{"And":[[851],null]},{"Token":80},{"And":[[853],null]},{"Token":61},{"And":[[855],null]},{"Token":62},{"And":[[857],null]},{"PrevIs":[158,164,165,166,167,168,171]},{"And":[[859],null]},{"Var":0},{"Exit":[2,861]},{"Exit":[0,862]},{"And":[[863],null]},{"Token":87},{"And":[[865],null]},{"Token":88},{"And":[[867],null]},{"Token":89},{"And":[[869],null]},{"Token":84},{"And":[[871],null]},{"Token":86},{"And":[[873],null]},{"Or":[866,868,870,872,874]},{"Token":90},{"Token":80},{"And":[[876,877],null]},{"Or":[878]},{"Not":879},{"Opt":80},{"And":[[880,40,881],null]},{"Or":[882]},{"IsIn":0},{"Not":884},{"Call":[146,[[2,81]]]},{"Token":61},{"Call":[77,[[0,74]]]},{"And":[[887,888],null]},{"Or":[889]},{"Opt":890},{"And":[[886,891],null]},{"Or":[892]},{"Call":[147,[[3,893]]]},{"And":[[885,894],null]},{"Or":[895]},{"Token":90},{"Token":57},{"And":[[898,74],null]},{"Or":[899]},{"Opt":900},{"And":[[897,901],1]},{"Or":[902]},{"Token":35},{"Token":36},{"And":[[904,905],null]},{"Or":[906]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[908,909],null]},{"Or":[910]},{"Call":[148,[[4,911]]]},{"And":[[912],null]},{"Or":[913]},{"Token":59},{"Call":[77,[[0,74]]]},{"Call":[146,[[2,916]]]},{"And":[[915,917],null]},{"Or":[918]},{"Call":[146,[[2,74]]]},{"Call":[77,[[0,920]]]},{"Call":[150,[[6,921]]]},{"And":[[922],null]},{"Or":[923]},{"Token":26},{"Opt":925},{"Token":77},{"Rep":87},{"Token":77},{"Token":49},{"And":[[930,49,89],null]},{"Call":[77,[[0,74]]]},{"And":[[932],null]},{"Or":[931,933]},{"And":[[926,927,928,929,934],null]},{"Or":[935]},{"Token":59},{"And":[[937],null]},{"Token":77},{"Not":939},{"Not":940},{"And":[[941],null]},{"Or":[938,942]},{"And":[[13,943],1]},{"Token":31},{"Opt":74},{"And":[[945,946],null]},{"Or":[947]},{"Token":33},{"Opt":949},{"Rep":90},{"Opt":74},{"And":[[951,952],null]},{"Or":[953]},{"Call":[147,[[3,954]]]},{"And":[[950,955],null]},{"Or":[956]},{"Call":[77,[[0,957]]]},{"And":[[958],null]},{"Or":[959]},{"And":[[91],null]},{"And":[[95],null]},{"And":[[94],null]},{"And":[[3],null]},{"Token":9},{"Opt":92},{"Opt":93},{"Token":56},{"And":[[965,62,966,967,968],1]},{"Or":[969]},{"Token":57},{"And":[[971,49],null]},{"Or":[972]},{"Token":51},{"And":[[974,74],null]},{"Or":[975]},{"Token":56},{"And":[[977],null]},{"Or":[978]},"Eof",{"Not":980},{"And":[[76,981],null]},{"Token":56},{"And":[[983],null]},{"Or":[982,984]},{"And":[[74,985],null]},{"Or":[986]},{"Enter":[2,987]},{"And":[[988],null]},{"Or":[989]},{"Token":14},{"Token":15},{"And":[[89],null]},{"And":[[96],null]},{"Or":[993,994]},{"And":[[992,995],null]},{"Or":[996]},{"Opt":997},{"And":[[991,98,89,998],1]},{"Or":[999]},{"Opt":102},{"Token":25},{"And":[[1001,1002,98,89],2]},{"Or":[1003]},{"Token":9},{"Token":51},{"And":[[1005,62,1006],1]},{"Or":[1007]},{"Opt":1008},{"And":[[1009,99],null]},{"Enter":[0,74]},{"And":[[1011],null]},{"Opt":102},{"Token":24},{"And":[[1013,1014,89],2]},{"Or":[1015]},{"Opt":102},{"Token":23},{"Token":32},{"And":[[1017,1018,62,1019,99,89],2]},{"Or":[1020]},{"Token":85},{"Token":57},{"And":[[1022,1023],null]},{"Token":30},{"Rep":104},{"Call":[147,[[3,1026]]]},{"And":[[1025,99,1027],1]},{"Or":[1028]},{"Token":50},{"Enter":[2,74]},{"Token":59},{"And":[[1032],null]},"Eof",{"And":[[1034],null]},{"And":[[76],null]},{"Or":[1033,1035,1036]},{"And":[[105,1030,1031,1037],1]},{"Or":[1038]},{"Token":77},{"And":[[1040,62],null]},{"Or":[1041]},{"Rep":1042},{"Opt":106},{"And":[[62,1043,1044],null]},{"Token":14},{"And":[[1046,74],null]},{"Or":[1047]},{"And":[[143],null]},{"Or":[1049]},{"And":[[144],null]},{"Or":[1051]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1055},{"And":[[1053,1054,1056,111],null]},{"Or":[1057]},{"And":[[74,1058],null]},{"Or":[1059]},{"IsIn":2},{"Not":76},{"And":[[1061,1062],null]},{"IsIn":2},{"Not":1064},{"And":[[1065],null]},{"Or":[1063,1066]},{"And":[[1067,111],null]},{"Or":[1068]},{"And":[[74,1069],null]},{"Or":[1070]},{"Call":[146,[[2,112]]]},{"Call":[148,[[4,1072]]]},{"Call":[77,[[0,1073]]]},{"And":[[1074],null]},{"And":[[74],null]},{"Or":[1076]},{"Token":60},{"Token":90},{"And":[[1079],null]},{"Token":87},{"And":[[1081],null]},{"Or":[1080,1082]},{"And":[[1078,1083],null]},{"Or":[1084]},{"And":[[74,1085],null]},{"Or":[1086]},{"Call":[150,[[6,74]]]},{"And":[[74,1088],null]},{"Or":[1089]},{"Token":81},{"And":[[74,1091],null]},{"Or":[1092]},{"Token":5},{"And":[[1094,49],null]},{"Or":[1095]},{"And":[[74,1096],null]},{"Or":[1097]},{"And":[[118,74],null]},{"Or":[1099]},{"Token":75},{"Token":85},{"Opt":1102},{"Token":27},{"Opt":1104},{"And":[[1101,1103,1105],null]},{"Token":65},{"And":[[1107,74],null]},{"Or":[1108]},{"Token":73},{"And":[[1110,74],null]},{"Or":[1111]},{"Token":80},{"And":[[1113,74],null]},{"Or":[1114]},{"Token":65},{"And":[[1116],null]},{"Token":67},{"And":[[1118],null]},{"Token":69},{"And":[[1120],null]},{"Or":[1117,1119,1121]},{"Call":[125,[[1,1122]]]},{"And":[[74,1123,74],null]},{"Or":[1124]},{"Token":71},{"And":[[1126],null]},{"Token":73},{"And":[[1128],null]},{"Or":[1127,1129]},{"Call":[125,[[1,1130]]]},{"And":[[74,1131,74],null]},{"Or":[1132]},{"ContextualToken":[43,"<<"]},{"And":[[1134],null]},{"ContextualToken":[45,">>"]},{"And":[[1136],null]},{"Or":[1135,1137]},{"Call":[125,[[1,1138]]]},{"And":[[74,1139,74],null]},{"Or":[1140]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1142,1143,1144],null]},{"IsIn":2},{"Not":1146},{"Var":1},{"And":[[1147,1148],null]},{"Token":75},{"Token":75},{"Not":1151},{"And":[[1150,1152],null]},{"Or":[1153]},{"Call":[125,[[1,1154]]]},{"And":[[74,1155,74],null]},{"Or":[1156]},{"Token":82},{"Call":[125,[[1,1158]]]},{"And":[[74,1159,74],null]},{"Or":[1160]},{"Token":77},{"Token":77},{"Not":1163},{"And":[[1162,1164],null]},{"Or":[1165]},{"Call":[125,[[1,1166]]]},{"And":[[74,1167,74],null]},{"Or":[1168]},{"Call":[125,[[1,130]]]},{"And":[[74,1170,74],null]},{"Or":[1171]},{"Token":52},{"And":[[1173],null]},{"Token":53},{"And":[[1175],null]},{"Token":39},{"And":[[1177],null]},{"Token":40},{"And":[[1179],null]},{"Token":55},{"And":[[1181],null]},{"Token":54},{"And":[[1183],null]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,1185]]]},{"And":[[74,1186,74],null]},{"Or":[1187]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,1189]]]},{"And":[[74,1190,74],null]},{"Or":[1191]},{"Call":[125,[[1,138]]]},{"And":[[74,1193,74],null]},{"Or":[1194]},{"And":[[138,74],null]},{"Or":[1196]},{"And":[[74,137],null]},{"Or":[1198]},{"Token":61},{"Not":75},{"And":[[1200,1201],null]},{"Or":[1202]},{"Token":61},{"And":[[1204],null]},{"Token":62},{"And":[[1206],null]},{"Not":75},{"Not":1208},{"Token":37},{"IsIn":0},{"And":[[1210,1211],null]},{"Or":[1212]},{"Not":1213},{"And":[[137,1209,1214],null]},{"Token":51},{"And":[[1216],null]},{"Token":72},{"And":[[1218],null]},{"Token":74},{"And":[[1220],null]},{"Token":66},{"And":[[1222],null]},{"Token":68},{"And":[[1224],null]},{"Token":70},{"And":[[1226],null]},{"Token":76},{"And":[[1228],null]},{"Token":78},{"And":[[1230],null]},{"Token":83},{"And":[[1232],null]},{"ContextualToken":[46,">>="]},{"And":[[1234],null]},{"ContextualToken":[44,"<<="]},{"And":[[1236],null]},{"Or":[1217,1219,1221,1223,1225,1227,1229,1231,1233,1235,1237]},{"Call":[125,[[1,1238]]]},{"And":[[74,1239,74],null]},{"Or":[1240]},{"Token":63},{"Call":[146,[[2,142]]]},{"Call":[150,[[6,1243]]]},{"And":[[1242,1244],null]},{"Or":[1245]},{"Rep":140},{"And":[[1247],null]},{"Token":90},{"Token":51},{"And":[[1250,74],null]},{"Call":[146,[[2,142]]]},{"Call":[148,[[4,1252]]]},{"And":[[1253],null]},{"Or":[1251,1254]},{"Opt":1255},{"And":[[1249,1256],1]},{"Or":[1257]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1261},{"Rep":145},{"Call":[147,[[3,1263]]]},{"And":[[1259,1260,1262,1264],null]},{"Or":[1265]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1269},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1271,1272,1273],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1275,1276,1277],null]},{"Or":[1274,1278]},{"And":[[1267,1268,1270,1279],null]},{"Or":[1280]},{"Token":35},{"And":[[1282],null]},{"Token":36},{"And":[[1284],null]},{"Token":37},{"And":[[1286],null]},{"Token":38},{"And":[[1288],null]},{"Token":41},{"And":[[1290],null]},{"Token":42},{"And":[[1292],null]},{"Or":[1283,1285,1287,1289,1291,1293]},{"Not":1294},"Any",{"And":[[1295,1296],null]},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1298,1299,1300],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1302,1303,1304],null]},{"Token":37},{"Rep":145},{"Token":38},{"And":[[1306,1307,1308],null]},{"Or":[1297,1301,1305,1309]},{"Var":2},"Eof",{"And":[[1312],null]},{"Token":59},{"And":[[1314],null]},{"Or":[1313,1315]},{"And":[[1311,1316],1]},{"Or":[1317]},{"Rep":1318},{"And":[[1319],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[151,[[7,1321],[8,1322],[9,1323]]]},{"And":[[1324],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[151,[[7,1326],[8,1327],[9,1328]]]},{"And":[[1329],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[151,[[7,1331],[8,1332],[9,1333]]]},{"And":[[1334],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[151,[[7,1336],[8,1337],[9,1338]]]},{"And":[[1339],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[152,[[10,1342],[11,1343]]]},{"Var":9},{"Layer":[1344,1345]},{"Var":8},{"And":[[1341,1346,1347],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[152,[[10,1350],[11,1351]]]},{"Var":11},{"And":[[1349,1352,1353],1]},{"Var":11},{"Not":1355},"Any",{"And":[[1356,1357],null]},{"Or":[1358]},{"And":[[1359],null]},{"Or":[1354,1360]},{"Rep":1361},{"And":[[1362],null]}]"##;

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