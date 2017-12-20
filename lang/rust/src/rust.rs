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
pub const PLACEHOLDER_TYPE: rt::NodeType = rt::NodeType(227);
pub const UNIT_TYPE: rt::NodeType = rt::NodeType(228);
pub const PAREN_TYPE: rt::NodeType = rt::NodeType(229);
pub const TUPLE_TYPE: rt::NodeType = rt::NodeType(230);
pub const NEVER_TYPE: rt::NodeType = rt::NodeType(231);
pub const ARRAY_TYPE: rt::NodeType = rt::NodeType(232);
pub const FN_POINTER_TYPE: rt::NodeType = rt::NodeType(233);
pub const WILDCARD_PATTERN: rt::NodeType = rt::NodeType(234);
pub const PATH_PATTERN: rt::NodeType = rt::NodeType(235);
pub const TUPE_STRUCT_PATTERN: rt::NodeType = rt::NodeType(236);
pub const STRUCT_PATTERN: rt::NodeType = rt::NodeType(237);
pub const STRUCT_PATTERN_FIELD: rt::NodeType = rt::NodeType(238);
pub const BINDING_PATTERN: rt::NodeType = rt::NodeType(239);
pub const LITERAL_PATTERN: rt::NodeType = rt::NodeType(240);
pub const UNIT_PATTERN: rt::NodeType = rt::NodeType(241);
pub const PAREN_PATTERN: rt::NodeType = rt::NodeType(242);
pub const TUPLE_PATTERN: rt::NodeType = rt::NodeType(243);
pub const REFERENCE_PATTERN: rt::NodeType = rt::NodeType(244);
pub const EXPR: rt::NodeType = rt::NodeType(245);
pub const LITERAL: rt::NodeType = rt::NodeType(246);
pub const PATH_EXPR: rt::NodeType = rt::NodeType(247);
pub const STRUCT_LITERAL: rt::NodeType = rt::NodeType(248);
pub const STRUCT_LITERAL_FIELD: rt::NodeType = rt::NodeType(249);
pub const UNIT_EXPR: rt::NodeType = rt::NodeType(250);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(251);
pub const TUPLE_EXPR: rt::NodeType = rt::NodeType(252);
pub const ARRAY_LITERAL: rt::NodeType = rt::NodeType(253);
pub const LAMBDA_EXPR: rt::NodeType = rt::NodeType(254);
pub const RETURN_EXPR: rt::NodeType = rt::NodeType(255);
pub const BLOCK_EXPR: rt::NodeType = rt::NodeType(256);
pub const LET_STMT: rt::NodeType = rt::NodeType(257);
pub const TYPE_ASCRIPTION: rt::NodeType = rt::NodeType(258);
pub const INITIALIZER: rt::NodeType = rt::NodeType(259);
pub const EMPTY_STMT: rt::NodeType = rt::NodeType(260);
pub const EXPR_STMT: rt::NodeType = rt::NodeType(261);
pub const IF_EXPR: rt::NodeType = rt::NodeType(262);
pub const WHILE_EXPR: rt::NodeType = rt::NodeType(263);
pub const LOOP_EXPR: rt::NodeType = rt::NodeType(264);
pub const FOR_EXPR: rt::NodeType = rt::NodeType(265);
pub const MATCH_EXPR: rt::NodeType = rt::NodeType(266);
pub const MATCH_ARM: rt::NodeType = rt::NodeType(267);
pub const GUARD: rt::NodeType = rt::NodeType(268);
pub const BLOCK_MACRO_EXPR: rt::NodeType = rt::NodeType(269);
pub const LINE_MACRO_EXPR: rt::NodeType = rt::NodeType(270);
pub const METHOD_CALL_EXPR: rt::NodeType = rt::NodeType(271);
pub const CALL_EXPR: rt::NodeType = rt::NodeType(272);
pub const VALUE_ARGUMENT: rt::NodeType = rt::NodeType(273);
pub const FIELD_EXPR: rt::NodeType = rt::NodeType(274);
pub const INDEX_EXPR: rt::NodeType = rt::NodeType(275);
pub const TRY_EXPR: rt::NodeType = rt::NodeType(276);
pub const CAST_EXPR: rt::NodeType = rt::NodeType(277);
pub const REFERENCE_EXPR: rt::NodeType = rt::NodeType(278);
pub const DEREFERENCE_EXPR: rt::NodeType = rt::NodeType(279);
pub const NEGATION_EXPR: rt::NodeType = rt::NodeType(280);
pub const NOT_EXPR: rt::NodeType = rt::NodeType(281);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(282);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(283);
pub const BIT_SHIFT: rt::NodeType = rt::NodeType(284);
pub const BIT_AND: rt::NodeType = rt::NodeType(285);
pub const BIT_XOR: rt::NodeType = rt::NodeType(286);
pub const BIT_OR: rt::NodeType = rt::NodeType(287);
pub const COMPARISON: rt::NodeType = rt::NodeType(288);
pub const LOGICAL_AND: rt::NodeType = rt::NodeType(289);
pub const LOGICAL_OR: rt::NodeType = rt::NodeType(290);
pub const RANGE_EXPR: rt::NodeType = rt::NodeType(291);
pub const ASSIGNMENT_EXPR: rt::NodeType = rt::NodeType(292);
pub const ATTRIBUTE: rt::NodeType = rt::NodeType(293);
pub const ATTR_VALUE: rt::NodeType = rt::NodeType(294);
pub const BLOCK_MACRO: rt::NodeType = rt::NodeType(295);
pub const LINE_MACRO: rt::NodeType = rt::NodeType(296);
pub const TT: rt::NodeType = rt::NodeType(297);


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
        let parser_json = r##"[{"Pub":{"ty":91,"body":153,"replaceable":false}},{"Or":[156]},{"Or":[158,160,162,164,166,168,170,171]},{"Cached":188},{"Pub":{"ty":92,"body":204,"replaceable":false}},{"Pub":{"ty":93,"body":210,"replaceable":false}},{"Pub":{"ty":94,"body":216,"replaceable":false}},{"Pub":{"ty":95,"body":223,"replaceable":false}},{"Pub":{"ty":96,"body":235,"replaceable":false}},{"Or":[237]},{"Pub":{"ty":97,"body":242,"replaceable":false}},{"Or":[248]},{"Pub":{"ty":98,"body":251,"replaceable":false}},{"Pub":{"ty":99,"body":257,"replaceable":false}},{"Pub":{"ty":100,"body":270,"replaceable":false}},{"Pub":{"ty":101,"body":289,"replaceable":false}},{"Pub":{"ty":102,"body":294,"replaceable":false}},{"Pub":{"ty":103,"body":297,"replaceable":false}},{"Pub":{"ty":104,"body":304,"replaceable":false}},{"Pub":{"ty":105,"body":317,"replaceable":false}},{"Pub":{"ty":106,"body":326,"replaceable":false}},{"Pub":{"ty":107,"body":337,"replaceable":false}},{"Pub":{"ty":108,"body":345,"replaceable":false}},{"Pub":{"ty":109,"body":357,"replaceable":false}},{"Or":[358,359,360]},{"Or":[362,364,366,368,370,372,374,379]},{"Pub":{"ty":110,"body":389,"replaceable":false}},{"Pub":{"ty":111,"body":403,"replaceable":false}},{"Pub":{"ty":112,"body":407,"replaceable":false}},{"Pub":{"ty":113,"body":411,"replaceable":false}},{"Or":[412,413]},{"Pub":{"ty":114,"body":420,"replaceable":false}},{"Pub":{"ty":115,"body":424,"replaceable":false}},{"Or":[444]},{"Pub":{"ty":116,"body":448,"replaceable":false}},{"Pub":{"ty":117,"body":468,"replaceable":false}},{"Pub":{"ty":118,"body":478,"replaceable":false}},{"Pub":{"ty":119,"body":493,"replaceable":false}},{"Or":[494]},{"Or":[496]},{"Or":[498]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":501,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":505,"replaceable":false}},{"Pub":{"ty":120,"body":510,"replaceable":false}},{"Pub":{"ty":121,"body":517,"replaceable":false}},{"Pub":{"ty":122,"body":531,"replaceable":false}},{"Pub":{"ty":123,"body":557,"replaceable":false}},{"Pub":{"ty":124,"body":562,"replaceable":false}},{"Pub":{"ty":125,"body":566,"replaceable":false}},{"Or":[571]},{"Or":[572,573,574,575,576,577,578,579]},{"Pub":{"ty":126,"body":581,"replaceable":false}},{"Pub":{"ty":127,"body":583,"replaceable":false}},{"Pub":{"ty":128,"body":586,"replaceable":false}},{"Pub":{"ty":129,"body":590,"replaceable":false}},{"Pub":{"ty":130,"body":596,"replaceable":true}},{"PubReplace":{"ty":131,"body":600}},{"Pub":{"ty":132,"body":603,"replaceable":false}},{"Pub":{"ty":133,"body":612,"replaceable":false}},{"Pub":{"ty":134,"body":617,"replaceable":false}},{"Pub":{"ty":98,"body":623,"replaceable":false}},{"Or":[624,625,626,627,628,629,630]},{"Pub":{"ty":135,"body":633,"replaceable":false}},{"Pub":{"ty":136,"body":639,"replaceable":true}},{"PubReplace":{"ty":137,"body":651}},{"PubReplace":{"ty":138,"body":663}},{"Pub":{"ty":139,"body":670,"replaceable":false}},{"Pub":{"ty":140,"body":677,"replaceable":false}},{"Pub":{"ty":141,"body":679,"replaceable":false}},{"Pub":{"ty":142,"body":683,"replaceable":false}},{"Pub":{"ty":143,"body":689,"replaceable":true}},{"PubReplace":{"ty":144,"body":693}},{"Pub":{"ty":145,"body":698,"replaceable":false}},{"Pratt":{"atoms":[77,78,81,82,84,85,87,88,95,96,99,100,102,106,107,135],"prefixes":[{"ty":179,"op":117,"priority":999},{"ty":180,"op":727,"priority":999},{"ty":181,"op":728,"priority":999},{"ty":182,"op":729,"priority":999},{"ty":192,"op":137,"priority":2}],"infixes":[{"ty":172,"op":704,"priority":999,"has_rhs":false},{"ty":173,"op":713,"priority":999,"has_rhs":false},{"ty":175,"op":721,"priority":999,"has_rhs":false},{"ty":176,"op":722,"priority":999,"has_rhs":false},{"ty":177,"op":723,"priority":999,"has_rhs":false},{"ty":178,"op":726,"priority":999,"has_rhs":false},{"ty":183,"op":737,"priority":11,"has_rhs":true},{"ty":184,"op":743,"priority":10,"has_rhs":true},{"ty":185,"op":749,"priority":9,"has_rhs":true},{"ty":186,"op":755,"priority":8,"has_rhs":true},{"ty":187,"op":757,"priority":7,"has_rhs":true},{"ty":188,"op":763,"priority":6,"has_rhs":true},{"ty":189,"op":764,"priority":5,"has_rhs":true},{"ty":190,"op":766,"priority":4,"has_rhs":true},{"ty":191,"op":768,"priority":3,"has_rhs":true},{"ty":192,"op":769,"priority":2,"has_rhs":true},{"ty":192,"op":136,"priority":2,"has_rhs":false},{"ty":193,"op":793,"priority":1,"has_rhs":true}]}},{"Or":[794,796,798,800,802,804,806,808,810,812,814,816,818,820,822,824,826,828,830,832,834,836]},{"Or":[838]},{"Or":[842]},{"Pub":{"ty":147,"body":853,"replaceable":false}},{"Pub":{"ty":148,"body":861,"replaceable":true}},{"PubReplace":{"ty":149,"body":874}},{"Pub":{"ty":150,"body":881,"replaceable":false}},{"Pub":{"ty":151,"body":885,"replaceable":false}},{"Pub":{"ty":152,"body":892,"replaceable":true}},{"PubReplace":{"ty":153,"body":897}},{"Pub":{"ty":154,"body":902,"replaceable":false}},{"Pub":{"ty":155,"body":914,"replaceable":false}},{"Or":[922]},{"Pub":{"ty":156,"body":926,"replaceable":false}},{"Pub":{"ty":157,"body":938,"replaceable":false}},{"Or":[939,940,941,942]},{"Pub":{"ty":158,"body":948,"replaceable":false}},{"Pub":{"ty":159,"body":951,"replaceable":false}},{"Pub":{"ty":160,"body":954,"replaceable":false}},{"Pub":{"ty":161,"body":957,"replaceable":false}},{"Pub":{"ty":162,"body":968,"replaceable":false}},{"Pub":{"ty":163,"body":978,"replaceable":false}},{"Pub":{"ty":164,"body":982,"replaceable":false}},{"Or":[988]},{"Or":[990]},{"Pub":{"ty":165,"body":994,"replaceable":false}},{"Pub":{"ty":166,"body":999,"replaceable":false}},{"Or":[1002]},{"Pub":{"ty":167,"body":1007,"replaceable":false}},{"Pub":{"ty":168,"body":1017,"replaceable":false}},{"Or":[1023]},{"Pub":{"ty":169,"body":1026,"replaceable":false}},{"Pub":{"ty":170,"body":1028,"replaceable":false}},{"Pub":{"ty":171,"body":1030,"replaceable":false}},{"Pub":{"ty":172,"body":1038,"replaceable":false}},{"Pub":{"ty":173,"body":1049,"replaceable":false}},{"Or":[1053]},{"Pub":{"ty":174,"body":1055,"replaceable":false}},{"Pub":{"ty":175,"body":1065,"replaceable":false}},{"Pub":{"ty":176,"body":1068,"replaceable":false}},{"Pub":{"ty":177,"body":1071,"replaceable":false}},{"Pub":{"ty":178,"body":1076,"replaceable":false}},{"Pub":{"ty":179,"body":1078,"replaceable":false}},{"Or":[1084]},{"Pub":{"ty":180,"body":1087,"replaceable":false}},{"Pub":{"ty":181,"body":1090,"replaceable":false}},{"Pub":{"ty":182,"body":1093,"replaceable":false}},{"Pub":{"ty":183,"body":1103,"replaceable":false}},{"Pub":{"ty":184,"body":1111,"replaceable":false}},{"Pub":{"ty":185,"body":1119,"replaceable":false}},{"Or":[1123,1127]},{"Pub":{"ty":186,"body":1135,"replaceable":false}},{"Pub":{"ty":187,"body":1139,"replaceable":false}},{"Pub":{"ty":188,"body":1147,"replaceable":false}},{"Pub":{"ty":189,"body":1150,"replaceable":false}},{"Or":[1152,1154,1156,1158,1160,1162]},{"Pub":{"ty":190,"body":1166,"replaceable":false}},{"Pub":{"ty":191,"body":1170,"replaceable":false}},{"Pub":{"ty":192,"body":1173,"replaceable":false}},{"Pub":{"ty":192,"body":1175,"replaceable":false}},{"Pub":{"ty":192,"body":1177,"replaceable":false}},{"Pub":{"ty":192,"body":1181,"replaceable":false}},{"Or":[1183,1185]},{"Or":[1193]},{"Pub":{"ty":193,"body":1219,"replaceable":false}},{"Pub":{"ty":194,"body":1224,"replaceable":false}},{"Or":[1226]},{"Pub":{"ty":195,"body":1236,"replaceable":false}},{"Pub":{"ty":196,"body":1244,"replaceable":false}},{"Pub":{"ty":197,"body":1259,"replaceable":false}},{"Pub":{"ty":198,"body":1288,"replaceable":false}},{"Or":[1298]},{"Or":[1303]},{"Or":[1308]},{"Or":[1313]},{"Or":[1318]},{"Or":[1326]},{"Or":[1341]},{"And":[[1],null]},{"Or":[152]},{"WithSkip":[2,3]},{"Rep":154},{"And":[[155],null]},{"Token":11},{"And":[[157],null]},{"ContextualToken":[4,"union"]},{"And":[[159],null]},{"Token":16},{"And":[[161],null]},{"Token":12},{"And":[[163],null]},{"Token":13},{"And":[[165],null]},{"Token":17},{"And":[[167],null]},{"Token":29},{"And":[[169],null]},{"And":[[25],null]},{"Opt":36},{"And":[[140,172],null]},{"Or":[173]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[175,176,177,178,179,180,181,182,183]},{"Inject":[174,184]},{"And":[[185],null]},{"And":[[28],null]},{"Or":[186,187]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[191,5],null]},{"Or":[192]},{"Opt":193},{"And":[[194],null]},{"Or":[190,195]},{"And":[[38,196],null]},{"Token":58},{"Opt":198},{"And":[[199,5],null]},{"Or":[197,200]},{"Token":56},{"And":[[189,201,202],1]},{"Or":[203]},{"Token":65},{"And":[[205],null]},{"Call":[145,[[2,6]]]},{"Call":[146,[[3,207]]]},{"And":[[208],null]},{"Or":[206,209]},{"Token":18},{"And":[[211],null]},{"Token":90},{"Opt":48},{"And":[[213,214],1]},{"Or":[212,215]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[217,218,219,220,221],2]},{"Or":[222]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[88],null]},{"Token":56},{"And":[[231],null]},{"Or":[230,232]},{"And":[[224,225,226,227,11,228,229,233],2]},{"Or":[234]},{"Token":49},{"And":[[236,49],null]},{"Token":7},{"Token":88},{"Opt":239},{"And":[[238,240],null]},{"Or":[241]},{"Opt":14},{"Call":[145,[[2,12]]]},{"And":[[243,244],null]},{"Or":[245]},{"Call":[147,[[4,246]]]},{"And":[[247],null]},{"Token":57},{"And":[[61,249,49],1]},{"Or":[250]},{"Token":57},{"And":[[252,49],null]},{"Or":[253]},{"Opt":254},{"And":[[61,255],null]},{"Or":[256]},{"Opt":117},{"Token":18},{"Token":57},{"And":[[260,49],null]},{"Or":[261]},{"Opt":262},{"Token":59},{"And":[[264],null]},"Eof",{"And":[[266],null]},{"Or":[265,267]},{"And":[[258,259,263,268],2]},{"Or":[269]},{"Token":11},{"And":[[271],null]},{"ContextualToken":[4,"union"]},{"And":[[273],null]},{"Or":[272,274]},{"Token":90},{"Opt":31},{"Call":[145,[[2,16]]]},{"Call":[146,[[3,278]]]},{"And":[[279],null]},{"Token":56},{"And":[[281],null]},{"Call":[145,[[2,17]]]},{"Call":[147,[[4,283]]]},{"Token":56},{"And":[[284,285],null]},{"Or":[280,282,286]},{"And":[[275,276,277,287],1]},{"Or":[288]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[290,291,292,49],2]},{"Or":[293]},{"Opt":36},{"And":[[295,49],null]},{"Or":[296]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[145,[[2,19]]]},{"Call":[146,[[3,301]]]},{"And":[[298,299,300,302],1]},{"Or":[303]},{"Token":90},{"Token":51},{"And":[[306,73],null]},{"Call":[145,[[2,17]]]},{"Call":[147,[[4,308]]]},{"And":[[309],null]},{"Call":[145,[[2,16]]]},{"Call":[146,[[3,311]]]},{"And":[[312],null]},{"Or":[307,310,313]},{"Opt":314},{"And":[[305,315],1]},{"Or":[316]},{"Token":13},{"Token":90},{"Token":56},{"And":[[320],null]},{"Call":[146,[[3,1]]]},{"And":[[322],null]},{"Or":[321,323]},{"And":[[318,319,324],1]},{"Or":[325]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[329,49],null]},{"Or":[330]},{"Opt":331},{"And":[[49,332],null]},{"Or":[333]},{"Opt":37},{"And":[[327,328,334,335,23],1]},{"Or":[336]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[140,338,339,340,341,342,343,23],3]},{"Or":[344]},{"Opt":36},{"And":[[140,346],null]},{"Or":[347]},{"Inject":[348,24]},{"And":[[349],null]},{"And":[[28],null]},{"Or":[350,351]},{"WithSkip":[25,352]},{"Rep":353},{"Call":[146,[[3,354]]]},{"And":[[355],null]},{"Or":[356]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[361],null]},{"Token":8},{"And":[[363],null]},{"Token":20},{"And":[[365],null]},{"Token":21},{"And":[[367],null]},{"Token":22},{"And":[[369],null]},{"Token":63},{"And":[[371],null]},{"Token":7},{"And":[[373],null]},{"Token":90},{"Token":80},{"And":[[375,376],null]},{"Or":[377]},{"And":[[378],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[383,49],null]},{"Or":[384]},{"Opt":385},{"Token":56},{"And":[[380,381,382,386,387],1]},{"Or":[388]},{"Token":21},{"And":[[390],null]},{"Token":22},{"And":[[392],null]},{"Or":[391,393]},{"Token":90},{"Token":57},{"Token":51},{"And":[[397,73],null]},{"Or":[398]},{"Opt":399},{"Token":56},{"And":[[394,395,396,49,400,401],1]},{"Or":[402]},{"And":[[142],null]},{"Token":56},{"And":[[143,405],null]},{"Or":[404,406]},{"Rep":30},{"Call":[146,[[3,408]]]},{"And":[[10,409],null]},{"Or":[410]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[145,[[2,35]]]},{"Call":[145,[[2,32]]]},{"And":[[414,415],null]},{"Or":[416]},{"Call":[148,[[5,417]]]},{"And":[[418],null]},{"Or":[419]},{"Token":90},{"Opt":33},{"And":[[421,422],1]},{"Or":[423]},{"Token":57},{"Token":71},{"And":[[426],null]},"Eof",{"And":[[428],null]},{"Token":59},{"And":[[430],null]},{"Token":37},{"And":[[432],null]},{"Token":34},{"And":[[434],null]},{"Or":[431,433,435]},{"Not":436},{"Not":437},{"And":[[438],null]},{"Or":[427,429,439]},{"And":[[34,440],1]},{"Or":[441]},{"Rep":442},{"And":[[425,443],null]},{"Token":85},{"And":[[445],null]},{"And":[[51],null]},{"Or":[446,447]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[452],null]},"Eof",{"And":[[454],null]},{"Token":59},{"Not":456},{"Not":457},{"And":[[458],null]},{"Or":[453,455,459]},{"And":[[451,460],1]},{"Or":[461]},{"Rep":462},{"And":[[450,463],null]},{"Or":[464]},{"Opt":465},{"And":[[449,466],1]},{"Or":[467]},{"Token":10},{"Token":6},{"And":[[470],null]},{"Token":19},{"And":[[472],null]},{"Or":[471,473]},{"Call":[147,[[4,474]]]},{"Opt":475},{"And":[[469,476],null]},{"Or":[477]},{"Token":34},{"Token":59},{"And":[[480],null]},"Eof",{"And":[[482],null]},{"Token":37},{"Not":484},{"Not":485},{"And":[[486],null]},{"Or":[481,483,487]},{"And":[[49,33,488],null]},{"Or":[489]},{"Rep":490},{"And":[[479,491],1]},{"Or":[492]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[495],null]},{"Enter":[1,41]},{"And":[[497],null]},{"Token":58},{"And":[[499,45],null]},{"Or":[500]},{"Token":58},{"Opt":502},{"And":[[503,45],null]},{"Or":[504]},{"Token":58},{"And":[[506,45],null]},{"Or":[507]},{"And":[[41,508],null]},{"Or":[509]},{"Token":5},{"And":[[49,511,49],null]},{"Or":[512]},{"Call":[148,[[5,513]]]},{"Token":58},{"And":[[514,515,45],null]},{"Or":[516]},{"Token":90},{"And":[[518],null]},{"Token":18},{"And":[[520],null]},{"Token":19},{"And":[[522],null]},{"Or":[519,521,523]},{"And":[[46],null]},{"IsIn":3},{"And":[[526,47],null]},{"Or":[525,527]},{"Opt":528},{"And":[[524,529],null]},{"Or":[530]},{"IsIn":3},{"And":[[532],null]},{"IsIn":1},{"Token":58},{"And":[[534,535],null]},{"Or":[533,536]},{"Token":85},{"Call":[145,[[2,538]]]},{"Token":90},{"Token":51},{"And":[[540,541],null]},{"Or":[542]},{"Not":543},{"And":[[544,49],null]},{"Or":[545]},{"Call":[145,[[2,546]]]},{"Token":90},{"Token":51},{"And":[[548,549,49],null]},{"Or":[550]},{"Call":[145,[[2,551]]]},{"And":[[539,547,552],null]},{"Or":[553]},{"Call":[148,[[5,554]]]},{"And":[[537,555],null]},{"Or":[556]},{"Call":[145,[[2,49]]]},{"Call":[147,[[4,558]]]},{"Opt":9},{"And":[[559,560],null]},{"Or":[561]},{"Token":5},{"Token":90},{"And":[[563,564],null]},{"Or":[565]},{"Token":71},{"And":[[567,34],null]},{"Or":[568]},{"Rep":569},{"And":[[50,570],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[55],null]},{"And":[[57],null]},{"And":[[58],null]},{"And":[[59],null]},{"And":[[39],null]},{"Or":[580]},{"And":[[117,49],null]},{"Or":[582]},{"Token":79},{"And":[[584],null]},{"Or":[585]},{"Token":35},{"Token":36},{"And":[[587,588],null]},{"Or":[589]},{"Opt":56},{"And":[[49,591],null]},{"Or":[592]},{"Call":[147,[[4,593]]]},{"And":[[594],null]},{"Or":[595]},{"Token":59},{"Call":[145,[[2,49]]]},{"And":[[597,598],null]},{"Or":[599]},{"Token":80},{"And":[[601],null]},{"Or":[602]},{"Token":56},{"And":[[604,73],null]},{"Or":[605]},{"Opt":606},{"And":[[49,607],null]},{"Or":[608]},{"Call":[149,[[6,609]]]},{"And":[[610],null]},{"Or":[611]},{"Token":8},{"Call":[145,[[2,60]]]},{"Call":[147,[[4,614]]]},{"And":[[613,615,9],1]},{"Or":[616]},{"Token":57},{"And":[[61,618],null]},{"Or":[619]},{"Opt":620},{"And":[[621,49],null]},{"Or":[622]},{"And":[[62],null]},{"And":[[63],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[70],null]},{"And":[[72],null]},{"Token":79},{"And":[[631],null]},{"Or":[632]},{"And":[[64],null]},{"And":[[65],null]},{"Or":[634,635]},{"Opt":636},{"And":[[40,637],null]},{"Or":[638]},{"Call":[145,[[2,61]]]},{"Token":61},{"Token":59},{"Opt":642},{"And":[[641,643],null]},{"Or":[644]},{"Opt":645},{"And":[[640,646],null]},{"Or":[647]},{"Call":[147,[[4,648]]]},{"And":[[649],null]},{"Or":[650]},{"Call":[145,[[2,66]]]},{"Token":61},{"Token":59},{"Opt":654},{"And":[[653,655],null]},{"Or":[656]},{"Opt":657},{"And":[[652,658],null]},{"Or":[659]},{"Call":[146,[[3,660]]]},{"And":[[661],null]},{"Or":[662]},{"Token":57},{"Not":664},{"And":[[67,665],null]},{"Token":90},{"Token":57},{"And":[[667,668,61],2]},{"Or":[666,669]},{"Token":28},{"Opt":671},{"Token":27},{"Opt":673},{"Token":90},{"And":[[672,674,675],null]},{"Or":[676]},{"And":[[77],null]},{"Or":[678]},{"Token":35},{"Token":36},{"And":[[680,681],null]},{"Or":[682]},{"Opt":71},{"And":[[61,684],null]},{"Or":[685]},{"Call":[147,[[4,686]]]},{"And":[[687],null]},{"Or":[688]},{"Token":59},{"Call":[145,[[2,61]]]},{"And":[[690,691],null]},{"Or":[692]},{"Token":75},{"Token":27},{"Opt":695},{"And":[[694,696,61],null]},{"Or":[697]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":701},{"And":[[699,700,702,110],null]},{"Or":[703]},{"IsIn":2},{"Not":75},{"And":[[705,706],null]},{"IsIn":2},{"Not":708},{"And":[[709],null]},{"Or":[707,710]},{"And":[[711,110],null]},{"Or":[712]},{"Token":60},{"Token":90},{"And":[[715],null]},{"Token":87},{"And":[[717],null]},{"Or":[716,718]},{"And":[[714,719],null]},{"Or":[720]},{"Call":[149,[[6,73]]]},{"Token":81},{"Token":5},{"And":[[724,49],null]},{"Or":[725]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[730],null]},{"Token":67},{"And":[[732],null]},{"Token":69},{"And":[[734],null]},{"Or":[731,733,735]},{"Call":[124,[[1,736]]]},{"Token":71},{"And":[[738],null]},{"Token":73},{"And":[[740],null]},{"Or":[739,741]},{"Call":[124,[[1,742]]]},{"ContextualToken":[43,"<<"]},{"And":[[744],null]},{"ContextualToken":[45,">>"]},{"And":[[746],null]},{"Or":[745,747]},{"Call":[124,[[1,748]]]},{"Token":75},{"Token":75},{"Not":751},{"And":[[750,752],null]},{"Or":[753]},{"Call":[124,[[1,754]]]},{"Token":82},{"Call":[124,[[1,756]]]},{"Token":77},{"Token":77},{"Not":759},{"And":[[758,760],null]},{"Or":[761]},{"Call":[124,[[1,762]]]},{"Call":[124,[[1,129]]]},{"ContextualToken":[47,"&&"]},{"Call":[124,[[1,765]]]},{"ContextualToken":[48,"||"]},{"Call":[124,[[1,767]]]},{"Call":[124,[[1,137]]]},{"Token":51},{"And":[[770],null]},{"Token":72},{"And":[[772],null]},{"Token":74},{"And":[[774],null]},{"Token":66},{"And":[[776],null]},{"Token":68},{"And":[[778],null]},{"Token":70},{"And":[[780],null]},{"Token":76},{"And":[[782],null]},{"Token":78},{"And":[[784],null]},{"Token":83},{"And":[[786],null]},{"ContextualToken":[46,">>="]},{"And":[[788],null]},{"ContextualToken":[44,"<<="]},{"And":[[790],null]},{"Or":[771,773,775,777,779,781,783,785,787,789,791]},{"Call":[124,[[1,792]]]},{"And":[[77],null]},{"Token":90},{"And":[[795],null]},{"Token":18},{"And":[[797],null]},{"Token":19},{"And":[[799],null]},{"Token":39},{"And":[[801],null]},{"Token":58},{"And":[[803],null]},{"Token":35},{"And":[[805],null]},{"Token":41},{"And":[[807],null]},{"Token":77},{"And":[[809],null]},{"Token":31},{"And":[[811],null]},{"Token":37},{"And":[[813],null]},{"Token":14},{"And":[[815],null]},{"Token":25},{"And":[[817],null]},{"Token":24},{"And":[[819],null]},{"Token":23},{"And":[[821],null]},{"Token":30},{"And":[[823],null]},{"Token":75},{"And":[[825],null]},{"Token":65},{"And":[[827],null]},{"Token":73},{"And":[[829],null]},{"Token":80},{"And":[[831],null]},{"Token":61},{"And":[[833],null]},{"Token":62},{"And":[[835],null]},{"PrevIs":[157,163,164,165,166,167,170]},{"And":[[837],null]},{"Var":0},{"Exit":[2,839]},{"Exit":[0,840]},{"And":[[841],null]},{"Token":87},{"And":[[843],null]},{"Token":88},{"And":[[845],null]},{"Token":89},{"And":[[847],null]},{"Token":84},{"And":[[849],null]},{"Token":86},{"And":[[851],null]},{"Or":[844,846,848,850,852]},{"Token":90},{"Token":80},{"And":[[854,855],null]},{"Or":[856]},{"Not":857},{"Opt":79},{"And":[[858,40,859],null]},{"Or":[860]},{"IsIn":0},{"Not":862},{"Call":[145,[[2,80]]]},{"Token":61},{"Call":[76,[[0,73]]]},{"And":[[865,866],null]},{"Or":[867]},{"Opt":868},{"And":[[864,869],null]},{"Or":[870]},{"Call":[146,[[3,871]]]},{"And":[[863,872],null]},{"Or":[873]},{"Token":90},{"Token":57},{"And":[[876,73],null]},{"Or":[877]},{"Opt":878},{"And":[[875,879],1]},{"Or":[880]},{"Token":35},{"Token":36},{"And":[[882,883],null]},{"Or":[884]},{"Call":[76,[[0,73]]]},{"Opt":83},{"And":[[886,887],null]},{"Or":[888]},{"Call":[147,[[4,889]]]},{"And":[[890],null]},{"Or":[891]},{"Token":59},{"Call":[76,[[0,73]]]},{"Call":[145,[[2,894]]]},{"And":[[893,895],null]},{"Or":[896]},{"Call":[145,[[2,73]]]},{"Call":[76,[[0,898]]]},{"Call":[149,[[6,899]]]},{"And":[[900],null]},{"Or":[901]},{"Token":26},{"Opt":903},{"Token":77},{"Rep":86},{"Token":77},{"Token":49},{"And":[[908,49,88],null]},{"Call":[76,[[0,73]]]},{"And":[[910],null]},{"Or":[909,911]},{"And":[[904,905,906,907,912],null]},{"Or":[913]},{"Token":59},{"And":[[915],null]},{"Token":77},{"Not":917},{"Not":918},{"And":[[919],null]},{"Or":[916,920]},{"And":[[13,921],1]},{"Token":31},{"Opt":73},{"And":[[923,924],null]},{"Or":[925]},{"Token":33},{"Opt":927},{"Rep":89},{"Opt":73},{"And":[[929,930],null]},{"Or":[931]},{"Call":[146,[[3,932]]]},{"And":[[928,933],null]},{"Or":[934]},{"Call":[76,[[0,935]]]},{"And":[[936],null]},{"Or":[937]},{"And":[[90],null]},{"And":[[94],null]},{"And":[[93],null]},{"And":[[3],null]},{"Token":9},{"Opt":91},{"Opt":92},{"Token":56},{"And":[[943,61,944,945,946],1]},{"Or":[947]},{"Token":57},{"And":[[949,49],null]},{"Or":[950]},{"Token":51},{"And":[[952,73],null]},{"Or":[953]},{"Token":56},{"And":[[955],null]},{"Or":[956]},"Eof",{"Not":958},{"And":[[75,959],null]},{"Token":56},{"And":[[961],null]},{"Or":[960,962]},{"And":[[73,963],null]},{"Or":[964]},{"Enter":[2,965]},{"And":[[966],null]},{"Or":[967]},{"Token":14},{"Token":15},{"And":[[88],null]},{"And":[[95],null]},{"Or":[971,972]},{"And":[[970,973],null]},{"Or":[974]},{"Opt":975},{"And":[[969,97,88,976],1]},{"Or":[977]},{"Opt":101},{"Token":25},{"And":[[979,980,97,88],2]},{"Or":[981]},{"Token":9},{"Token":51},{"And":[[983,61,984],1]},{"Or":[985]},{"Opt":986},{"And":[[987,98],null]},{"Enter":[0,73]},{"And":[[989],null]},{"Opt":101},{"Token":24},{"And":[[991,992,88],2]},{"Or":[993]},{"Opt":101},{"Token":23},{"Token":32},{"And":[[995,996,61,997,98,88],2]},{"Or":[998]},{"Token":85},{"Token":57},{"And":[[1000,1001],null]},{"Token":30},{"Rep":103},{"Call":[146,[[3,1004]]]},{"And":[[1003,98,1005],1]},{"Or":[1006]},{"Token":50},{"Enter":[2,73]},{"Token":59},{"And":[[1010],null]},"Eof",{"And":[[1012],null]},{"And":[[75],null]},{"Or":[1011,1013,1014]},{"And":[[104,1008,1009,1015],1]},{"Or":[1016]},{"Token":77},{"And":[[1018,61],null]},{"Or":[1019]},{"Rep":1020},{"Opt":105},{"And":[[61,1021,1022],null]},{"Token":14},{"And":[[1024,73],null]},{"Or":[1025]},{"And":[[142],null]},{"Or":[1027]},{"And":[[143],null]},{"Or":[1029]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1033},{"And":[[1031,1032,1034,110],null]},{"Or":[1035]},{"And":[[73,1036],null]},{"Or":[1037]},{"IsIn":2},{"Not":75},{"And":[[1039,1040],null]},{"IsIn":2},{"Not":1042},{"And":[[1043],null]},{"Or":[1041,1044]},{"And":[[1045,110],null]},{"Or":[1046]},{"And":[[73,1047],null]},{"Or":[1048]},{"Call":[145,[[2,111]]]},{"Call":[147,[[4,1050]]]},{"Call":[76,[[0,1051]]]},{"And":[[1052],null]},{"And":[[73],null]},{"Or":[1054]},{"Token":60},{"Token":90},{"And":[[1057],null]},{"Token":87},{"And":[[1059],null]},{"Or":[1058,1060]},{"And":[[1056,1061],null]},{"Or":[1062]},{"And":[[73,1063],null]},{"Or":[1064]},{"Call":[149,[[6,73]]]},{"And":[[73,1066],null]},{"Or":[1067]},{"Token":81},{"And":[[73,1069],null]},{"Or":[1070]},{"Token":5},{"And":[[1072,49],null]},{"Or":[1073]},{"And":[[73,1074],null]},{"Or":[1075]},{"And":[[117,73],null]},{"Or":[1077]},{"Token":75},{"Token":85},{"Opt":1080},{"Token":27},{"Opt":1082},{"And":[[1079,1081,1083],null]},{"Token":65},{"And":[[1085,73],null]},{"Or":[1086]},{"Token":73},{"And":[[1088,73],null]},{"Or":[1089]},{"Token":80},{"And":[[1091,73],null]},{"Or":[1092]},{"Token":65},{"And":[[1094],null]},{"Token":67},{"And":[[1096],null]},{"Token":69},{"And":[[1098],null]},{"Or":[1095,1097,1099]},{"Call":[124,[[1,1100]]]},{"And":[[73,1101,73],null]},{"Or":[1102]},{"Token":71},{"And":[[1104],null]},{"Token":73},{"And":[[1106],null]},{"Or":[1105,1107]},{"Call":[124,[[1,1108]]]},{"And":[[73,1109,73],null]},{"Or":[1110]},{"ContextualToken":[43,"<<"]},{"And":[[1112],null]},{"ContextualToken":[45,">>"]},{"And":[[1114],null]},{"Or":[1113,1115]},{"Call":[124,[[1,1116]]]},{"And":[[73,1117,73],null]},{"Or":[1118]},{"IsIn":2},{"Not":75},{"Var":1},{"And":[[1120,1121,1122],null]},{"IsIn":2},{"Not":1124},{"Var":1},{"And":[[1125,1126],null]},{"Token":75},{"Token":75},{"Not":1129},{"And":[[1128,1130],null]},{"Or":[1131]},{"Call":[124,[[1,1132]]]},{"And":[[73,1133,73],null]},{"Or":[1134]},{"Token":82},{"Call":[124,[[1,1136]]]},{"And":[[73,1137,73],null]},{"Or":[1138]},{"Token":77},{"Token":77},{"Not":1141},{"And":[[1140,1142],null]},{"Or":[1143]},{"Call":[124,[[1,1144]]]},{"And":[[73,1145,73],null]},{"Or":[1146]},{"Call":[124,[[1,129]]]},{"And":[[73,1148,73],null]},{"Or":[1149]},{"Token":52},{"And":[[1151],null]},{"Token":53},{"And":[[1153],null]},{"Token":39},{"And":[[1155],null]},{"Token":40},{"And":[[1157],null]},{"Token":55},{"And":[[1159],null]},{"Token":54},{"And":[[1161],null]},{"ContextualToken":[47,"&&"]},{"Call":[124,[[1,1163]]]},{"And":[[73,1164,73],null]},{"Or":[1165]},{"ContextualToken":[48,"||"]},{"Call":[124,[[1,1167]]]},{"And":[[73,1168,73],null]},{"Or":[1169]},{"Call":[124,[[1,137]]]},{"And":[[73,1171,73],null]},{"Or":[1172]},{"And":[[137,73],null]},{"Or":[1174]},{"And":[[73,136],null]},{"Or":[1176]},{"Token":61},{"Not":74},{"And":[[1178,1179],null]},{"Or":[1180]},{"Token":61},{"And":[[1182],null]},{"Token":62},{"And":[[1184],null]},{"Not":74},{"Not":1186},{"Token":37},{"IsIn":0},{"And":[[1188,1189],null]},{"Or":[1190]},{"Not":1191},{"And":[[136,1187,1192],null]},{"Token":51},{"And":[[1194],null]},{"Token":72},{"And":[[1196],null]},{"Token":74},{"And":[[1198],null]},{"Token":66},{"And":[[1200],null]},{"Token":68},{"And":[[1202],null]},{"Token":70},{"And":[[1204],null]},{"Token":76},{"And":[[1206],null]},{"Token":78},{"And":[[1208],null]},{"Token":83},{"And":[[1210],null]},{"ContextualToken":[46,">>="]},{"And":[[1212],null]},{"ContextualToken":[44,"<<="]},{"And":[[1214],null]},{"Or":[1195,1197,1199,1201,1203,1205,1207,1209,1211,1213,1215]},{"Call":[124,[[1,1216]]]},{"And":[[73,1217,73],null]},{"Or":[1218]},{"Token":63},{"Call":[145,[[2,141]]]},{"Call":[149,[[6,1221]]]},{"And":[[1220,1222],null]},{"Or":[1223]},{"Rep":139},{"And":[[1225],null]},{"Token":90},{"Token":51},{"And":[[1228,73],null]},{"Call":[145,[[2,141]]]},{"Call":[147,[[4,1230]]]},{"And":[[1231],null]},{"Or":[1229,1232]},{"Opt":1233},{"And":[[1227,1234],1]},{"Or":[1235]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1239},{"Rep":144},{"Call":[146,[[3,1241]]]},{"And":[[1237,1238,1240,1242],null]},{"Or":[1243]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1247},{"Token":35},{"Rep":144},{"Token":36},{"And":[[1249,1250,1251],null]},{"Token":41},{"Rep":144},{"Token":42},{"And":[[1253,1254,1255],null]},{"Or":[1252,1256]},{"And":[[1245,1246,1248,1257],null]},{"Or":[1258]},{"Token":35},{"And":[[1260],null]},{"Token":36},{"And":[[1262],null]},{"Token":37},{"And":[[1264],null]},{"Token":38},{"And":[[1266],null]},{"Token":41},{"And":[[1268],null]},{"Token":42},{"And":[[1270],null]},{"Or":[1261,1263,1265,1267,1269,1271]},{"Not":1272},"Any",{"And":[[1273,1274],null]},{"Token":35},{"Rep":144},{"Token":36},{"And":[[1276,1277,1278],null]},{"Token":41},{"Rep":144},{"Token":42},{"And":[[1280,1281,1282],null]},{"Token":37},{"Rep":144},{"Token":38},{"And":[[1284,1285,1286],null]},{"Or":[1275,1279,1283,1287]},{"Var":2},"Eof",{"And":[[1290],null]},{"Token":59},{"And":[[1292],null]},{"Or":[1291,1293]},{"And":[[1289,1294],1]},{"Or":[1295]},{"Rep":1296},{"And":[[1297],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[150,[[7,1299],[8,1300],[9,1301]]]},{"And":[[1302],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[150,[[7,1304],[8,1305],[9,1306]]]},{"And":[[1307],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[150,[[7,1309],[8,1310],[9,1311]]]},{"And":[[1312],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[150,[[7,1314],[8,1315],[9,1316]]]},{"And":[[1317],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[151,[[10,1320],[11,1321]]]},{"Var":9},{"Layer":[1322,1323]},{"Var":8},{"And":[[1319,1324,1325],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[151,[[10,1328],[11,1329]]]},{"Var":11},{"And":[[1327,1330,1331],1]},{"Var":11},{"Not":1333},"Any",{"And":[[1334,1335],null]},{"Or":[1336]},{"And":[[1337],null]},{"Or":[1332,1338]},{"Rep":1339},{"And":[[1340],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHL_EQ, SHR, SHR_EQ, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, STAR_EQ, SLASH, SLASH_EQ, PERCENT, PERCENT_EQ, PLUS, PLUS_EQ, MINUS, MINUS_EQ, AMPERSAND, AMPERSAND_EQ, PIPE, PIPE_EQ, UNDERSCORE, BANG, QUESTION, CARET, CARET_EQ, CHAR, LIFETIME, BOOL, NUMBER, STRING, RAW_STRING, IDENT, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TRAIT_PROJECTION_PATH, PATH_SEGMENT, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, NEVER_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, TYPE_ASCRIPTION, INITIALIZER, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, VALUE_ARGUMENT, FIELD_EXPR, INDEX_EXPR, TRY_EXPR, CAST_EXPR, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_XOR, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
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