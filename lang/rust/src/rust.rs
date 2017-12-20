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
        let parser_json = r##"[{"Pub":{"ty":91,"body":154,"replaceable":false}},{"Or":[157]},{"Or":[159,161,163,165,167,169,171,172]},{"Cached":189},{"Pub":{"ty":92,"body":205,"replaceable":false}},{"Pub":{"ty":93,"body":211,"replaceable":false}},{"Pub":{"ty":94,"body":217,"replaceable":false}},{"Pub":{"ty":95,"body":224,"replaceable":false}},{"Pub":{"ty":96,"body":240,"replaceable":false}},{"Or":[242]},{"Pub":{"ty":97,"body":247,"replaceable":false}},{"Or":[253]},{"Pub":{"ty":98,"body":256,"replaceable":false}},{"Pub":{"ty":99,"body":262,"replaceable":false}},{"Pub":{"ty":100,"body":279,"replaceable":false}},{"Pub":{"ty":101,"body":298,"replaceable":false}},{"Pub":{"ty":102,"body":303,"replaceable":false}},{"Pub":{"ty":103,"body":306,"replaceable":false}},{"Pub":{"ty":104,"body":313,"replaceable":false}},{"Pub":{"ty":105,"body":326,"replaceable":false}},{"Pub":{"ty":106,"body":335,"replaceable":false}},{"Pub":{"ty":107,"body":346,"replaceable":false}},{"Pub":{"ty":108,"body":354,"replaceable":false}},{"Pub":{"ty":109,"body":366,"replaceable":false}},{"Or":[367,368,369]},{"Or":[371,373,375,377,379,381,383,385,390]},{"Pub":{"ty":110,"body":400,"replaceable":false}},{"Pub":{"ty":111,"body":414,"replaceable":false}},{"Pub":{"ty":112,"body":418,"replaceable":false}},{"Pub":{"ty":113,"body":422,"replaceable":false}},{"Or":[423,424]},{"Pub":{"ty":114,"body":431,"replaceable":false}},{"Pub":{"ty":115,"body":435,"replaceable":false}},{"Or":[455]},{"Pub":{"ty":116,"body":459,"replaceable":false}},{"Pub":{"ty":117,"body":479,"replaceable":false}},{"Pub":{"ty":118,"body":489,"replaceable":false}},{"Pub":{"ty":119,"body":504,"replaceable":false}},{"Or":[505]},{"Or":[507]},{"Or":[509]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":512,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":516,"replaceable":false}},{"Pub":{"ty":120,"body":521,"replaceable":false}},{"Pub":{"ty":121,"body":528,"replaceable":false}},{"Pub":{"ty":122,"body":542,"replaceable":false}},{"Pub":{"ty":123,"body":568,"replaceable":false}},{"Pub":{"ty":124,"body":573,"replaceable":false}},{"Pub":{"ty":125,"body":577,"replaceable":false}},{"Or":[582]},{"Or":[583,584,585,586,587,588,589,590,591]},{"Pub":{"ty":126,"body":593,"replaceable":false}},{"Pub":{"ty":127,"body":595,"replaceable":false}},{"Pub":{"ty":128,"body":603,"replaceable":false}},{"Pub":{"ty":129,"body":606,"replaceable":false}},{"Pub":{"ty":130,"body":610,"replaceable":false}},{"Pub":{"ty":131,"body":616,"replaceable":true}},{"PubReplace":{"ty":132,"body":620}},{"Pub":{"ty":133,"body":623,"replaceable":false}},{"Pub":{"ty":134,"body":632,"replaceable":false}},{"Pub":{"ty":135,"body":637,"replaceable":false}},{"Pub":{"ty":98,"body":643,"replaceable":false}},{"Or":[644,645,646,647,648,649,650]},{"Pub":{"ty":136,"body":653,"replaceable":false}},{"Pub":{"ty":137,"body":659,"replaceable":true}},{"PubReplace":{"ty":138,"body":671}},{"PubReplace":{"ty":139,"body":683}},{"Pub":{"ty":140,"body":690,"replaceable":false}},{"Pub":{"ty":141,"body":697,"replaceable":false}},{"Pub":{"ty":142,"body":699,"replaceable":false}},{"Pub":{"ty":143,"body":703,"replaceable":false}},{"Pub":{"ty":144,"body":709,"replaceable":true}},{"PubReplace":{"ty":145,"body":713}},{"Pub":{"ty":146,"body":718,"replaceable":false}},{"Pratt":{"atoms":[78,79,82,83,85,86,88,89,96,97,100,101,103,107,108,136],"prefixes":[{"ty":180,"op":118,"priority":999},{"ty":181,"op":747,"priority":999},{"ty":182,"op":748,"priority":999},{"ty":183,"op":749,"priority":999},{"ty":193,"op":138,"priority":2}],"infixes":[{"ty":173,"op":724,"priority":999,"has_rhs":false},{"ty":174,"op":733,"priority":999,"has_rhs":false},{"ty":176,"op":741,"priority":999,"has_rhs":false},{"ty":177,"op":742,"priority":999,"has_rhs":false},{"ty":178,"op":743,"priority":999,"has_rhs":false},{"ty":179,"op":746,"priority":999,"has_rhs":false},{"ty":184,"op":757,"priority":11,"has_rhs":true},{"ty":185,"op":763,"priority":10,"has_rhs":true},{"ty":186,"op":769,"priority":9,"has_rhs":true},{"ty":187,"op":775,"priority":8,"has_rhs":true},{"ty":188,"op":777,"priority":7,"has_rhs":true},{"ty":189,"op":783,"priority":6,"has_rhs":true},{"ty":190,"op":784,"priority":5,"has_rhs":true},{"ty":191,"op":786,"priority":4,"has_rhs":true},{"ty":192,"op":788,"priority":3,"has_rhs":true},{"ty":193,"op":789,"priority":2,"has_rhs":true},{"ty":193,"op":137,"priority":2,"has_rhs":false},{"ty":194,"op":813,"priority":1,"has_rhs":true}]}},{"Or":[814,816,818,820,822,824,826,828,830,832,834,836,838,840,842,844,846,848,850,852,854,856]},{"Or":[858]},{"Or":[862]},{"Pub":{"ty":148,"body":873,"replaceable":false}},{"Pub":{"ty":149,"body":881,"replaceable":true}},{"PubReplace":{"ty":150,"body":894}},{"Pub":{"ty":151,"body":901,"replaceable":false}},{"Pub":{"ty":152,"body":905,"replaceable":false}},{"Pub":{"ty":153,"body":912,"replaceable":true}},{"PubReplace":{"ty":154,"body":917}},{"Pub":{"ty":155,"body":922,"replaceable":false}},{"Pub":{"ty":156,"body":934,"replaceable":false}},{"Or":[942]},{"Pub":{"ty":157,"body":946,"replaceable":false}},{"Pub":{"ty":158,"body":958,"replaceable":false}},{"Or":[959,960,961,962]},{"Pub":{"ty":159,"body":968,"replaceable":false}},{"Pub":{"ty":160,"body":971,"replaceable":false}},{"Pub":{"ty":161,"body":974,"replaceable":false}},{"Pub":{"ty":162,"body":977,"replaceable":false}},{"Pub":{"ty":163,"body":988,"replaceable":false}},{"Pub":{"ty":164,"body":998,"replaceable":false}},{"Pub":{"ty":165,"body":1002,"replaceable":false}},{"Or":[1008]},{"Or":[1010]},{"Pub":{"ty":166,"body":1014,"replaceable":false}},{"Pub":{"ty":167,"body":1019,"replaceable":false}},{"Or":[1022]},{"Pub":{"ty":168,"body":1027,"replaceable":false}},{"Pub":{"ty":169,"body":1037,"replaceable":false}},{"Or":[1043]},{"Pub":{"ty":170,"body":1046,"replaceable":false}},{"Pub":{"ty":171,"body":1048,"replaceable":false}},{"Pub":{"ty":172,"body":1050,"replaceable":false}},{"Pub":{"ty":173,"body":1058,"replaceable":false}},{"Pub":{"ty":174,"body":1069,"replaceable":false}},{"Or":[1073]},{"Pub":{"ty":175,"body":1075,"replaceable":false}},{"Pub":{"ty":176,"body":1085,"replaceable":false}},{"Pub":{"ty":177,"body":1088,"replaceable":false}},{"Pub":{"ty":178,"body":1091,"replaceable":false}},{"Pub":{"ty":179,"body":1096,"replaceable":false}},{"Pub":{"ty":180,"body":1098,"replaceable":false}},{"Or":[1104]},{"Pub":{"ty":181,"body":1107,"replaceable":false}},{"Pub":{"ty":182,"body":1110,"replaceable":false}},{"Pub":{"ty":183,"body":1113,"replaceable":false}},{"Pub":{"ty":184,"body":1123,"replaceable":false}},{"Pub":{"ty":185,"body":1131,"replaceable":false}},{"Pub":{"ty":186,"body":1139,"replaceable":false}},{"Or":[1143,1147]},{"Pub":{"ty":187,"body":1155,"replaceable":false}},{"Pub":{"ty":188,"body":1159,"replaceable":false}},{"Pub":{"ty":189,"body":1167,"replaceable":false}},{"Pub":{"ty":190,"body":1170,"replaceable":false}},{"Or":[1172,1174,1176,1178,1180,1182]},{"Pub":{"ty":191,"body":1186,"replaceable":false}},{"Pub":{"ty":192,"body":1190,"replaceable":false}},{"Pub":{"ty":193,"body":1193,"replaceable":false}},{"Pub":{"ty":193,"body":1195,"replaceable":false}},{"Pub":{"ty":193,"body":1197,"replaceable":false}},{"Pub":{"ty":193,"body":1201,"replaceable":false}},{"Or":[1203,1205]},{"Or":[1213]},{"Pub":{"ty":194,"body":1239,"replaceable":false}},{"Pub":{"ty":195,"body":1244,"replaceable":false}},{"Or":[1246]},{"Pub":{"ty":196,"body":1256,"replaceable":false}},{"Pub":{"ty":197,"body":1264,"replaceable":false}},{"Pub":{"ty":198,"body":1279,"replaceable":false}},{"Pub":{"ty":199,"body":1308,"replaceable":false}},{"Or":[1318]},{"Or":[1323]},{"Or":[1328]},{"Or":[1333]},{"Or":[1338]},{"Or":[1346]},{"Or":[1361]},{"And":[[1],null]},{"Or":[153]},{"WithSkip":[2,3]},{"Rep":155},{"And":[[156],null]},{"Token":11},{"And":[[158],null]},{"ContextualToken":[4,"union"]},{"And":[[160],null]},{"Token":16},{"And":[[162],null]},{"Token":12},{"And":[[164],null]},{"Token":13},{"And":[[166],null]},{"Token":17},{"And":[[168],null]},{"Token":29},{"And":[[170],null]},{"And":[[25],null]},{"Opt":36},{"And":[[141,173],null]},{"Or":[174]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[176,177,178,179,180,181,182,183,184]},{"Inject":[175,185]},{"And":[[186],null]},{"And":[[28],null]},{"Or":[187,188]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[192,5],null]},{"Or":[193]},{"Opt":194},{"And":[[195],null]},{"Or":[191,196]},{"And":[[38,197],null]},{"Token":58},{"Opt":199},{"And":[[200,5],null]},{"Or":[198,201]},{"Token":56},{"And":[[190,202,203],1]},{"Or":[204]},{"Token":65},{"And":[[206],null]},{"Call":[146,[[2,6]]]},{"Call":[147,[[3,208]]]},{"And":[[209],null]},{"Or":[207,210]},{"Token":18},{"And":[[212],null]},{"Token":90},{"Opt":48},{"And":[[214,215],1]},{"Or":[213,216]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[218,219,220,221,222],2]},{"Or":[223]},{"Token":21},{"Opt":225},{"Token":33},{"Opt":227},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[89],null]},{"Token":56},{"And":[[236],null]},{"Or":[235,237]},{"And":[[226,228,229,230,231,232,11,233,234,238],4]},{"Or":[239]},{"Token":49},{"And":[[241,49],null]},{"Token":7},{"Token":88},{"Opt":244},{"And":[[243,245],null]},{"Or":[246]},{"Opt":14},{"Call":[146,[[2,12]]]},{"And":[[248,249],null]},{"Or":[250]},{"Call":[148,[[4,251]]]},{"And":[[252],null]},{"Token":57},{"And":[[62,254,49],1]},{"Or":[255]},{"Token":57},{"And":[[257,49],null]},{"Or":[258]},{"Opt":259},{"And":[[62,260],null]},{"Or":[261]},{"And":[[118],null]},{"Token":27},{"And":[[264],null]},{"Or":[263,265]},{"Opt":266},{"Token":18},{"Token":57},{"And":[[269,49],null]},{"Or":[270]},{"Opt":271},{"Token":59},{"And":[[273],null]},"Eof",{"And":[[275],null]},{"Or":[274,276]},{"And":[[267,268,272,277],2]},{"Or":[278]},{"Token":11},{"And":[[280],null]},{"ContextualToken":[4,"union"]},{"And":[[282],null]},{"Or":[281,283]},{"Token":90},{"Opt":31},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,287]]]},{"And":[[288],null]},{"Token":56},{"And":[[290],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,292]]]},{"Token":56},{"And":[[293,294],null]},{"Or":[289,291,295]},{"And":[[284,285,286,296],1]},{"Or":[297]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[299,300,301,49],2]},{"Or":[302]},{"Opt":36},{"And":[[304,49],null]},{"Or":[305]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[146,[[2,19]]]},{"Call":[147,[[3,310]]]},{"And":[[307,308,309,311],1]},{"Or":[312]},{"Token":90},{"Token":51},{"And":[[315,74],null]},{"Call":[146,[[2,17]]]},{"Call":[148,[[4,317]]]},{"And":[[318],null]},{"Call":[146,[[2,16]]]},{"Call":[147,[[3,320]]]},{"And":[[321],null]},{"Or":[316,319,322]},{"Opt":323},{"And":[[314,324],1]},{"Or":[325]},{"Token":13},{"Token":90},{"Token":56},{"And":[[329],null]},{"Call":[147,[[3,1]]]},{"And":[[331],null]},{"Or":[330,332]},{"And":[[327,328,333],1]},{"Or":[334]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[338,49],null]},{"Or":[339]},{"Opt":340},{"And":[[49,341],null]},{"Or":[342]},{"Opt":37},{"And":[[336,337,343,344,23],1]},{"Or":[345]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[141,347,348,349,350,351,352,23],3]},{"Or":[353]},{"Opt":36},{"And":[[141,355],null]},{"Or":[356]},{"Inject":[357,24]},{"And":[[358],null]},{"And":[[28],null]},{"Or":[359,360]},{"WithSkip":[25,361]},{"Rep":362},{"Call":[147,[[3,363]]]},{"And":[[364],null]},{"Or":[365]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[370],null]},{"Token":8},{"And":[[372],null]},{"Token":20},{"And":[[374],null]},{"Token":21},{"And":[[376],null]},{"Token":22},{"And":[[378],null]},{"Token":33},{"And":[[380],null]},{"Token":63},{"And":[[382],null]},{"Token":7},{"And":[[384],null]},{"Token":90},{"Token":80},{"And":[[386,387],null]},{"Or":[388]},{"And":[[389],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[394,49],null]},{"Or":[395]},{"Opt":396},{"Token":56},{"And":[[391,392,393,397,398],1]},{"Or":[399]},{"Token":21},{"And":[[401],null]},{"Token":22},{"And":[[403],null]},{"Or":[402,404]},{"Token":90},{"Token":57},{"Token":51},{"And":[[408,74],null]},{"Or":[409]},{"Opt":410},{"Token":56},{"And":[[405,406,407,49,411,412],1]},{"Or":[413]},{"And":[[143],null]},{"Token":56},{"And":[[144,416],null]},{"Or":[415,417]},{"Rep":30},{"Call":[147,[[3,419]]]},{"And":[[10,420],null]},{"Or":[421]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[146,[[2,35]]]},{"Call":[146,[[2,32]]]},{"And":[[425,426],null]},{"Or":[427]},{"Call":[149,[[5,428]]]},{"And":[[429],null]},{"Or":[430]},{"Token":90},{"Opt":33},{"And":[[432,433],1]},{"Or":[434]},{"Token":57},{"Token":71},{"And":[[437],null]},"Eof",{"And":[[439],null]},{"Token":59},{"And":[[441],null]},{"Token":37},{"And":[[443],null]},{"Token":34},{"And":[[445],null]},{"Or":[442,444,446]},{"Not":447},{"Not":448},{"And":[[449],null]},{"Or":[438,440,450]},{"And":[[34,451],1]},{"Or":[452]},{"Rep":453},{"And":[[436,454],null]},{"Token":85},{"And":[[456],null]},{"And":[[51],null]},{"Or":[457,458]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[463],null]},"Eof",{"And":[[465],null]},{"Token":59},{"Not":467},{"Not":468},{"And":[[469],null]},{"Or":[464,466,470]},{"And":[[462,471],1]},{"Or":[472]},{"Rep":473},{"And":[[461,474],null]},{"Or":[475]},{"Opt":476},{"And":[[460,477],1]},{"Or":[478]},{"Token":10},{"Token":6},{"And":[[481],null]},{"Token":19},{"And":[[483],null]},{"Or":[482,484]},{"Call":[148,[[4,485]]]},{"Opt":486},{"And":[[480,487],null]},{"Or":[488]},{"Token":34},{"Token":59},{"And":[[491],null]},"Eof",{"And":[[493],null]},{"Token":37},{"Not":495},{"Not":496},{"And":[[497],null]},{"Or":[492,494,498]},{"And":[[49,33,499],null]},{"Or":[500]},{"Rep":501},{"And":[[490,502],1]},{"Or":[503]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[506],null]},{"Enter":[1,41]},{"And":[[508],null]},{"Token":58},{"And":[[510,45],null]},{"Or":[511]},{"Token":58},{"Opt":513},{"And":[[514,45],null]},{"Or":[515]},{"Token":58},{"And":[[517,45],null]},{"Or":[518]},{"And":[[41,519],null]},{"Or":[520]},{"Token":5},{"And":[[49,522,49],null]},{"Or":[523]},{"Call":[149,[[5,524]]]},{"Token":58},{"And":[[525,526,45],null]},{"Or":[527]},{"Token":90},{"And":[[529],null]},{"Token":18},{"And":[[531],null]},{"Token":19},{"And":[[533],null]},{"Or":[530,532,534]},{"And":[[46],null]},{"IsIn":3},{"And":[[537,47],null]},{"Or":[536,538]},{"Opt":539},{"And":[[535,540],null]},{"Or":[541]},{"IsIn":3},{"And":[[543],null]},{"IsIn":1},{"Token":58},{"And":[[545,546],null]},{"Or":[544,547]},{"Token":85},{"Call":[146,[[2,549]]]},{"Token":90},{"Token":51},{"And":[[551,552],null]},{"Or":[553]},{"Not":554},{"And":[[555,49],null]},{"Or":[556]},{"Call":[146,[[2,557]]]},{"Token":90},{"Token":51},{"And":[[559,560,49],null]},{"Or":[561]},{"Call":[146,[[2,562]]]},{"And":[[550,558,563],null]},{"Or":[564]},{"Call":[149,[[5,565]]]},{"And":[[548,566],null]},{"Or":[567]},{"Call":[146,[[2,49]]]},{"Call":[148,[[4,569]]]},{"Opt":9},{"And":[[570,571],null]},{"Or":[572]},{"Token":5},{"Token":90},{"And":[[574,575],null]},{"Or":[576]},{"Token":71},{"And":[[578,34],null]},{"Or":[579]},{"Rep":580},{"And":[[50,581],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[60],null]},{"And":[[39],null]},{"Or":[592]},{"And":[[118,49],null]},{"Or":[594]},{"Token":65},{"Token":21},{"And":[[597],null]},{"Token":27},{"And":[[599],null]},{"Or":[598,600]},{"And":[[596,601,49],null]},{"Or":[602]},{"Token":79},{"And":[[604],null]},{"Or":[605]},{"Token":35},{"Token":36},{"And":[[607,608],null]},{"Or":[609]},{"Opt":57},{"And":[[49,611],null]},{"Or":[612]},{"Call":[148,[[4,613]]]},{"And":[[614],null]},{"Or":[615]},{"Token":59},{"Call":[146,[[2,49]]]},{"And":[[617,618],null]},{"Or":[619]},{"Token":80},{"And":[[621],null]},{"Or":[622]},{"Token":56},{"And":[[624,74],null]},{"Or":[625]},{"Opt":626},{"And":[[49,627],null]},{"Or":[628]},{"Call":[150,[[6,629]]]},{"And":[[630],null]},{"Or":[631]},{"Token":8},{"Call":[146,[[2,61]]]},{"Call":[148,[[4,634]]]},{"And":[[633,635,9],1]},{"Or":[636]},{"Token":57},{"And":[[62,638],null]},{"Or":[639]},{"Opt":640},{"And":[[641,49],null]},{"Or":[642]},{"And":[[63],null]},{"And":[[64],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[71],null]},{"And":[[73],null]},{"Token":79},{"And":[[651],null]},{"Or":[652]},{"And":[[65],null]},{"And":[[66],null]},{"Or":[654,655]},{"Opt":656},{"And":[[40,657],null]},{"Or":[658]},{"Call":[146,[[2,62]]]},{"Token":61},{"Token":59},{"Opt":662},{"And":[[661,663],null]},{"Or":[664]},{"Opt":665},{"And":[[660,666],null]},{"Or":[667]},{"Call":[148,[[4,668]]]},{"And":[[669],null]},{"Or":[670]},{"Call":[146,[[2,67]]]},{"Token":61},{"Token":59},{"Opt":674},{"And":[[673,675],null]},{"Or":[676]},{"Opt":677},{"And":[[672,678],null]},{"Or":[679]},{"Call":[147,[[3,680]]]},{"And":[[681],null]},{"Or":[682]},{"Token":57},{"Not":684},{"And":[[68,685],null]},{"Token":90},{"Token":57},{"And":[[687,688,62],2]},{"Or":[686,689]},{"Token":28},{"Opt":691},{"Token":27},{"Opt":693},{"Token":90},{"And":[[692,694,695],null]},{"Or":[696]},{"And":[[78],null]},{"Or":[698]},{"Token":35},{"Token":36},{"And":[[700,701],null]},{"Or":[702]},{"Opt":72},{"And":[[62,704],null]},{"Or":[705]},{"Call":[148,[[4,706]]]},{"And":[[707],null]},{"Or":[708]},{"Token":59},{"Call":[146,[[2,62]]]},{"And":[[710,711],null]},{"Or":[712]},{"Token":75},{"Token":27},{"Opt":715},{"And":[[714,716,62],null]},{"Or":[717]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":721},{"And":[[719,720,722,111],null]},{"Or":[723]},{"IsIn":2},{"Not":76},{"And":[[725,726],null]},{"IsIn":2},{"Not":728},{"And":[[729],null]},{"Or":[727,730]},{"And":[[731,111],null]},{"Or":[732]},{"Token":60},{"Token":90},{"And":[[735],null]},{"Token":87},{"And":[[737],null]},{"Or":[736,738]},{"And":[[734,739],null]},{"Or":[740]},{"Call":[150,[[6,74]]]},{"Token":81},{"Token":5},{"And":[[744,49],null]},{"Or":[745]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[750],null]},{"Token":67},{"And":[[752],null]},{"Token":69},{"And":[[754],null]},{"Or":[751,753,755]},{"Call":[125,[[1,756]]]},{"Token":71},{"And":[[758],null]},{"Token":73},{"And":[[760],null]},{"Or":[759,761]},{"Call":[125,[[1,762]]]},{"ContextualToken":[43,"<<"]},{"And":[[764],null]},{"ContextualToken":[45,">>"]},{"And":[[766],null]},{"Or":[765,767]},{"Call":[125,[[1,768]]]},{"Token":75},{"Token":75},{"Not":771},{"And":[[770,772],null]},{"Or":[773]},{"Call":[125,[[1,774]]]},{"Token":82},{"Call":[125,[[1,776]]]},{"Token":77},{"Token":77},{"Not":779},{"And":[[778,780],null]},{"Or":[781]},{"Call":[125,[[1,782]]]},{"Call":[125,[[1,130]]]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,785]]]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,787]]]},{"Call":[125,[[1,138]]]},{"Token":51},{"And":[[790],null]},{"Token":72},{"And":[[792],null]},{"Token":74},{"And":[[794],null]},{"Token":66},{"And":[[796],null]},{"Token":68},{"And":[[798],null]},{"Token":70},{"And":[[800],null]},{"Token":76},{"And":[[802],null]},{"Token":78},{"And":[[804],null]},{"Token":83},{"And":[[806],null]},{"ContextualToken":[46,">>="]},{"And":[[808],null]},{"ContextualToken":[44,"<<="]},{"And":[[810],null]},{"Or":[791,793,795,797,799,801,803,805,807,809,811]},{"Call":[125,[[1,812]]]},{"And":[[78],null]},{"Token":90},{"And":[[815],null]},{"Token":18},{"And":[[817],null]},{"Token":19},{"And":[[819],null]},{"Token":39},{"And":[[821],null]},{"Token":58},{"And":[[823],null]},{"Token":35},{"And":[[825],null]},{"Token":41},{"And":[[827],null]},{"Token":77},{"And":[[829],null]},{"Token":31},{"And":[[831],null]},{"Token":37},{"And":[[833],null]},{"Token":14},{"And":[[835],null]},{"Token":25},{"And":[[837],null]},{"Token":24},{"And":[[839],null]},{"Token":23},{"And":[[841],null]},{"Token":30},{"And":[[843],null]},{"Token":75},{"And":[[845],null]},{"Token":65},{"And":[[847],null]},{"Token":73},{"And":[[849],null]},{"Token":80},{"And":[[851],null]},{"Token":61},{"And":[[853],null]},{"Token":62},{"And":[[855],null]},{"PrevIs":[158,164,165,166,167,168,171]},{"And":[[857],null]},{"Var":0},{"Exit":[2,859]},{"Exit":[0,860]},{"And":[[861],null]},{"Token":87},{"And":[[863],null]},{"Token":88},{"And":[[865],null]},{"Token":89},{"And":[[867],null]},{"Token":84},{"And":[[869],null]},{"Token":86},{"And":[[871],null]},{"Or":[864,866,868,870,872]},{"Token":90},{"Token":80},{"And":[[874,875],null]},{"Or":[876]},{"Not":877},{"Opt":80},{"And":[[878,40,879],null]},{"Or":[880]},{"IsIn":0},{"Not":882},{"Call":[146,[[2,81]]]},{"Token":61},{"Call":[77,[[0,74]]]},{"And":[[885,886],null]},{"Or":[887]},{"Opt":888},{"And":[[884,889],null]},{"Or":[890]},{"Call":[147,[[3,891]]]},{"And":[[883,892],null]},{"Or":[893]},{"Token":90},{"Token":57},{"And":[[896,74],null]},{"Or":[897]},{"Opt":898},{"And":[[895,899],1]},{"Or":[900]},{"Token":35},{"Token":36},{"And":[[902,903],null]},{"Or":[904]},{"Call":[77,[[0,74]]]},{"Opt":84},{"And":[[906,907],null]},{"Or":[908]},{"Call":[148,[[4,909]]]},{"And":[[910],null]},{"Or":[911]},{"Token":59},{"Call":[77,[[0,74]]]},{"Call":[146,[[2,914]]]},{"And":[[913,915],null]},{"Or":[916]},{"Call":[146,[[2,74]]]},{"Call":[77,[[0,918]]]},{"Call":[150,[[6,919]]]},{"And":[[920],null]},{"Or":[921]},{"Token":26},{"Opt":923},{"Token":77},{"Rep":87},{"Token":77},{"Token":49},{"And":[[928,49,89],null]},{"Call":[77,[[0,74]]]},{"And":[[930],null]},{"Or":[929,931]},{"And":[[924,925,926,927,932],null]},{"Or":[933]},{"Token":59},{"And":[[935],null]},{"Token":77},{"Not":937},{"Not":938},{"And":[[939],null]},{"Or":[936,940]},{"And":[[13,941],1]},{"Token":31},{"Opt":74},{"And":[[943,944],null]},{"Or":[945]},{"Token":33},{"Opt":947},{"Rep":90},{"Opt":74},{"And":[[949,950],null]},{"Or":[951]},{"Call":[147,[[3,952]]]},{"And":[[948,953],null]},{"Or":[954]},{"Call":[77,[[0,955]]]},{"And":[[956],null]},{"Or":[957]},{"And":[[91],null]},{"And":[[95],null]},{"And":[[94],null]},{"And":[[3],null]},{"Token":9},{"Opt":92},{"Opt":93},{"Token":56},{"And":[[963,62,964,965,966],1]},{"Or":[967]},{"Token":57},{"And":[[969,49],null]},{"Or":[970]},{"Token":51},{"And":[[972,74],null]},{"Or":[973]},{"Token":56},{"And":[[975],null]},{"Or":[976]},"Eof",{"Not":978},{"And":[[76,979],null]},{"Token":56},{"And":[[981],null]},{"Or":[980,982]},{"And":[[74,983],null]},{"Or":[984]},{"Enter":[2,985]},{"And":[[986],null]},{"Or":[987]},{"Token":14},{"Token":15},{"And":[[89],null]},{"And":[[96],null]},{"Or":[991,992]},{"And":[[990,993],null]},{"Or":[994]},{"Opt":995},{"And":[[989,98,89,996],1]},{"Or":[997]},{"Opt":102},{"Token":25},{"And":[[999,1000,98,89],2]},{"Or":[1001]},{"Token":9},{"Token":51},{"And":[[1003,62,1004],1]},{"Or":[1005]},{"Opt":1006},{"And":[[1007,99],null]},{"Enter":[0,74]},{"And":[[1009],null]},{"Opt":102},{"Token":24},{"And":[[1011,1012,89],2]},{"Or":[1013]},{"Opt":102},{"Token":23},{"Token":32},{"And":[[1015,1016,62,1017,99,89],2]},{"Or":[1018]},{"Token":85},{"Token":57},{"And":[[1020,1021],null]},{"Token":30},{"Rep":104},{"Call":[147,[[3,1024]]]},{"And":[[1023,99,1025],1]},{"Or":[1026]},{"Token":50},{"Enter":[2,74]},{"Token":59},{"And":[[1030],null]},"Eof",{"And":[[1032],null]},{"And":[[76],null]},{"Or":[1031,1033,1034]},{"And":[[105,1028,1029,1035],1]},{"Or":[1036]},{"Token":77},{"And":[[1038,62],null]},{"Or":[1039]},{"Rep":1040},{"Opt":106},{"And":[[62,1041,1042],null]},{"Token":14},{"And":[[1044,74],null]},{"Or":[1045]},{"And":[[143],null]},{"Or":[1047]},{"And":[[144],null]},{"Or":[1049]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1053},{"And":[[1051,1052,1054,111],null]},{"Or":[1055]},{"And":[[74,1056],null]},{"Or":[1057]},{"IsIn":2},{"Not":76},{"And":[[1059,1060],null]},{"IsIn":2},{"Not":1062},{"And":[[1063],null]},{"Or":[1061,1064]},{"And":[[1065,111],null]},{"Or":[1066]},{"And":[[74,1067],null]},{"Or":[1068]},{"Call":[146,[[2,112]]]},{"Call":[148,[[4,1070]]]},{"Call":[77,[[0,1071]]]},{"And":[[1072],null]},{"And":[[74],null]},{"Or":[1074]},{"Token":60},{"Token":90},{"And":[[1077],null]},{"Token":87},{"And":[[1079],null]},{"Or":[1078,1080]},{"And":[[1076,1081],null]},{"Or":[1082]},{"And":[[74,1083],null]},{"Or":[1084]},{"Call":[150,[[6,74]]]},{"And":[[74,1086],null]},{"Or":[1087]},{"Token":81},{"And":[[74,1089],null]},{"Or":[1090]},{"Token":5},{"And":[[1092,49],null]},{"Or":[1093]},{"And":[[74,1094],null]},{"Or":[1095]},{"And":[[118,74],null]},{"Or":[1097]},{"Token":75},{"Token":85},{"Opt":1100},{"Token":27},{"Opt":1102},{"And":[[1099,1101,1103],null]},{"Token":65},{"And":[[1105,74],null]},{"Or":[1106]},{"Token":73},{"And":[[1108,74],null]},{"Or":[1109]},{"Token":80},{"And":[[1111,74],null]},{"Or":[1112]},{"Token":65},{"And":[[1114],null]},{"Token":67},{"And":[[1116],null]},{"Token":69},{"And":[[1118],null]},{"Or":[1115,1117,1119]},{"Call":[125,[[1,1120]]]},{"And":[[74,1121,74],null]},{"Or":[1122]},{"Token":71},{"And":[[1124],null]},{"Token":73},{"And":[[1126],null]},{"Or":[1125,1127]},{"Call":[125,[[1,1128]]]},{"And":[[74,1129,74],null]},{"Or":[1130]},{"ContextualToken":[43,"<<"]},{"And":[[1132],null]},{"ContextualToken":[45,">>"]},{"And":[[1134],null]},{"Or":[1133,1135]},{"Call":[125,[[1,1136]]]},{"And":[[74,1137,74],null]},{"Or":[1138]},{"IsIn":2},{"Not":76},{"Var":1},{"And":[[1140,1141,1142],null]},{"IsIn":2},{"Not":1144},{"Var":1},{"And":[[1145,1146],null]},{"Token":75},{"Token":75},{"Not":1149},{"And":[[1148,1150],null]},{"Or":[1151]},{"Call":[125,[[1,1152]]]},{"And":[[74,1153,74],null]},{"Or":[1154]},{"Token":82},{"Call":[125,[[1,1156]]]},{"And":[[74,1157,74],null]},{"Or":[1158]},{"Token":77},{"Token":77},{"Not":1161},{"And":[[1160,1162],null]},{"Or":[1163]},{"Call":[125,[[1,1164]]]},{"And":[[74,1165,74],null]},{"Or":[1166]},{"Call":[125,[[1,130]]]},{"And":[[74,1168,74],null]},{"Or":[1169]},{"Token":52},{"And":[[1171],null]},{"Token":53},{"And":[[1173],null]},{"Token":39},{"And":[[1175],null]},{"Token":40},{"And":[[1177],null]},{"Token":55},{"And":[[1179],null]},{"Token":54},{"And":[[1181],null]},{"ContextualToken":[47,"&&"]},{"Call":[125,[[1,1183]]]},{"And":[[74,1184,74],null]},{"Or":[1185]},{"ContextualToken":[48,"||"]},{"Call":[125,[[1,1187]]]},{"And":[[74,1188,74],null]},{"Or":[1189]},{"Call":[125,[[1,138]]]},{"And":[[74,1191,74],null]},{"Or":[1192]},{"And":[[138,74],null]},{"Or":[1194]},{"And":[[74,137],null]},{"Or":[1196]},{"Token":61},{"Not":75},{"And":[[1198,1199],null]},{"Or":[1200]},{"Token":61},{"And":[[1202],null]},{"Token":62},{"And":[[1204],null]},{"Not":75},{"Not":1206},{"Token":37},{"IsIn":0},{"And":[[1208,1209],null]},{"Or":[1210]},{"Not":1211},{"And":[[137,1207,1212],null]},{"Token":51},{"And":[[1214],null]},{"Token":72},{"And":[[1216],null]},{"Token":74},{"And":[[1218],null]},{"Token":66},{"And":[[1220],null]},{"Token":68},{"And":[[1222],null]},{"Token":70},{"And":[[1224],null]},{"Token":76},{"And":[[1226],null]},{"Token":78},{"And":[[1228],null]},{"Token":83},{"And":[[1230],null]},{"ContextualToken":[46,">>="]},{"And":[[1232],null]},{"ContextualToken":[44,"<<="]},{"And":[[1234],null]},{"Or":[1215,1217,1219,1221,1223,1225,1227,1229,1231,1233,1235]},{"Call":[125,[[1,1236]]]},{"And":[[74,1237,74],null]},{"Or":[1238]},{"Token":63},{"Call":[146,[[2,142]]]},{"Call":[150,[[6,1241]]]},{"And":[[1240,1242],null]},{"Or":[1243]},{"Rep":140},{"And":[[1245],null]},{"Token":90},{"Token":51},{"And":[[1248,74],null]},{"Call":[146,[[2,142]]]},{"Call":[148,[[4,1250]]]},{"And":[[1251],null]},{"Or":[1249,1252]},{"Opt":1253},{"And":[[1247,1254],1]},{"Or":[1255]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1259},{"Rep":145},{"Call":[147,[[3,1261]]]},{"And":[[1257,1258,1260,1262],null]},{"Or":[1263]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1267},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1269,1270,1271],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1273,1274,1275],null]},{"Or":[1272,1276]},{"And":[[1265,1266,1268,1277],null]},{"Or":[1278]},{"Token":35},{"And":[[1280],null]},{"Token":36},{"And":[[1282],null]},{"Token":37},{"And":[[1284],null]},{"Token":38},{"And":[[1286],null]},{"Token":41},{"And":[[1288],null]},{"Token":42},{"And":[[1290],null]},{"Or":[1281,1283,1285,1287,1289,1291]},{"Not":1292},"Any",{"And":[[1293,1294],null]},{"Token":35},{"Rep":145},{"Token":36},{"And":[[1296,1297,1298],null]},{"Token":41},{"Rep":145},{"Token":42},{"And":[[1300,1301,1302],null]},{"Token":37},{"Rep":145},{"Token":38},{"And":[[1304,1305,1306],null]},{"Or":[1295,1299,1303,1307]},{"Var":2},"Eof",{"And":[[1310],null]},{"Token":59},{"And":[[1312],null]},{"Or":[1311,1313]},{"And":[[1309,1314],1]},{"Or":[1315]},{"Rep":1316},{"And":[[1317],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[151,[[7,1319],[8,1320],[9,1321]]]},{"And":[[1322],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[151,[[7,1324],[8,1325],[9,1326]]]},{"And":[[1327],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[151,[[7,1329],[8,1330],[9,1331]]]},{"And":[[1332],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[151,[[7,1334],[8,1335],[9,1336]]]},{"And":[[1337],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[152,[[10,1340],[11,1341]]]},{"Var":9},{"Layer":[1342,1343]},{"Var":8},{"And":[[1339,1344,1345],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[152,[[10,1348],[11,1349]]]},{"Var":11},{"And":[[1347,1350,1351],1]},{"Var":11},{"Not":1353},"Any",{"And":[[1354,1355],null]},{"Or":[1356]},{"And":[[1357],null]},{"Or":[1352,1358]},{"Rep":1359},{"And":[[1360],null]}]"##;

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