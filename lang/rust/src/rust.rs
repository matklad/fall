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
        let parser_json = r##"[{"Pub":{"ty":91,"body":150,"replaceable":false}},{"Or":[153]},{"Or":[155,157,159,161,163,165,167,168]},{"Cached":185},{"Pub":{"ty":92,"body":201,"replaceable":false}},{"Pub":{"ty":93,"body":207,"replaceable":false}},{"Pub":{"ty":94,"body":213,"replaceable":false}},{"Pub":{"ty":95,"body":220,"replaceable":false}},{"Pub":{"ty":96,"body":232,"replaceable":false}},{"Or":[234]},{"Pub":{"ty":97,"body":239,"replaceable":false}},{"Or":[245]},{"Pub":{"ty":98,"body":248,"replaceable":false}},{"Pub":{"ty":99,"body":254,"replaceable":false}},{"Pub":{"ty":100,"body":270,"replaceable":false}},{"Pub":{"ty":101,"body":289,"replaceable":false}},{"Pub":{"ty":102,"body":294,"replaceable":false}},{"Pub":{"ty":103,"body":297,"replaceable":false}},{"Pub":{"ty":104,"body":303,"replaceable":false}},{"Pub":{"ty":105,"body":316,"replaceable":false}},{"Pub":{"ty":106,"body":325,"replaceable":false}},{"Pub":{"ty":107,"body":335,"replaceable":false}},{"Pub":{"ty":108,"body":341,"replaceable":false}},{"Pub":{"ty":109,"body":353,"replaceable":false}},{"Or":[354,355,356]},{"Or":[358,360,362,364,366,368,370,375]},{"Pub":{"ty":110,"body":385,"replaceable":false}},{"Pub":{"ty":111,"body":399,"replaceable":false}},{"Pub":{"ty":112,"body":403,"replaceable":false}},{"Pub":{"ty":113,"body":407,"replaceable":false}},{"Or":[408,409]},{"Pub":{"ty":114,"body":416,"replaceable":false}},{"Pub":{"ty":115,"body":420,"replaceable":false}},{"Or":[438]},{"Pub":{"ty":116,"body":442,"replaceable":false}},{"Pub":{"ty":117,"body":462,"replaceable":false}},{"Pub":{"ty":118,"body":472,"replaceable":false}},{"Pub":{"ty":119,"body":487,"replaceable":false}},{"Or":[488]},{"Or":[490]},{"Or":[492]},{"Pratt":{"atoms":[42],"prefixes":[],"infixes":[{"ty":120,"op":495,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":499,"replaceable":false}},{"Pub":{"ty":120,"body":504,"replaceable":false}},{"Or":[517]},{"Pub":{"ty":121,"body":531,"replaceable":false}},{"Pub":{"ty":122,"body":536,"replaceable":false}},{"Pub":{"ty":123,"body":540,"replaceable":false}},{"Or":[541,542,543,544,545,546,547,548]},{"Pub":{"ty":124,"body":550,"replaceable":false}},{"Pub":{"ty":125,"body":557,"replaceable":false}},{"Pub":{"ty":126,"body":560,"replaceable":false}},{"Pub":{"ty":127,"body":564,"replaceable":false}},{"Pub":{"ty":128,"body":570,"replaceable":true}},{"PubReplace":{"ty":129,"body":574}},{"Pub":{"ty":130,"body":577,"replaceable":false}},{"Pub":{"ty":131,"body":586,"replaceable":false}},{"Pub":{"ty":132,"body":591,"replaceable":false}},{"Pub":{"ty":98,"body":597,"replaceable":false}},{"Or":[598,599,600,601,602,603,604]},{"Pub":{"ty":133,"body":607,"replaceable":false}},{"Pub":{"ty":134,"body":613,"replaceable":true}},{"PubReplace":{"ty":135,"body":625}},{"PubReplace":{"ty":136,"body":637}},{"Pub":{"ty":137,"body":644,"replaceable":false}},{"Pub":{"ty":138,"body":651,"replaceable":false}},{"Pub":{"ty":139,"body":653,"replaceable":false}},{"Pub":{"ty":140,"body":657,"replaceable":false}},{"Pub":{"ty":141,"body":663,"replaceable":true}},{"PubReplace":{"ty":142,"body":667}},{"Pub":{"ty":143,"body":672,"replaceable":false}},{"Pratt":{"atoms":[75,76,79,80,82,83,85,86,93,94,97,98,100,104,105,132],"prefixes":[{"ty":177,"op":705,"priority":999},{"ty":178,"op":706,"priority":999},{"ty":179,"op":707,"priority":999},{"ty":180,"op":708,"priority":999},{"ty":190,"op":134,"priority":2}],"infixes":[{"ty":170,"op":678,"priority":999,"has_rhs":false},{"ty":171,"op":687,"priority":999,"has_rhs":false},{"ty":173,"op":695,"priority":999,"has_rhs":false},{"ty":174,"op":696,"priority":999,"has_rhs":false},{"ty":175,"op":697,"priority":999,"has_rhs":false},{"ty":176,"op":700,"priority":999,"has_rhs":false},{"ty":181,"op":716,"priority":11,"has_rhs":true},{"ty":182,"op":722,"priority":10,"has_rhs":true},{"ty":183,"op":728,"priority":9,"has_rhs":true},{"ty":184,"op":734,"priority":8,"has_rhs":true},{"ty":185,"op":736,"priority":7,"has_rhs":true},{"ty":186,"op":742,"priority":6,"has_rhs":true},{"ty":187,"op":743,"priority":5,"has_rhs":true},{"ty":188,"op":745,"priority":4,"has_rhs":true},{"ty":189,"op":747,"priority":3,"has_rhs":true},{"ty":190,"op":748,"priority":2,"has_rhs":true},{"ty":190,"op":133,"priority":2,"has_rhs":false},{"ty":191,"op":772,"priority":1,"has_rhs":true}]}},{"Or":[773,775,777,779,781,783,785,787,789,791,793,795,797,799,801,803,805,807,809,811,813,815]},{"Or":[817]},{"Or":[821]},{"Pub":{"ty":145,"body":832,"replaceable":false}},{"Pub":{"ty":146,"body":840,"replaceable":true}},{"PubReplace":{"ty":147,"body":853}},{"Pub":{"ty":148,"body":860,"replaceable":false}},{"Pub":{"ty":149,"body":864,"replaceable":false}},{"Pub":{"ty":150,"body":871,"replaceable":true}},{"PubReplace":{"ty":151,"body":876}},{"Pub":{"ty":152,"body":881,"replaceable":false}},{"Pub":{"ty":153,"body":893,"replaceable":false}},{"Or":[901]},{"Pub":{"ty":154,"body":905,"replaceable":false}},{"Pub":{"ty":155,"body":917,"replaceable":false}},{"Or":[918,919,920,921]},{"Pub":{"ty":156,"body":927,"replaceable":false}},{"Pub":{"ty":157,"body":930,"replaceable":false}},{"Pub":{"ty":158,"body":933,"replaceable":false}},{"Pub":{"ty":159,"body":936,"replaceable":false}},{"Pub":{"ty":160,"body":947,"replaceable":false}},{"Pub":{"ty":161,"body":957,"replaceable":false}},{"Pub":{"ty":162,"body":961,"replaceable":false}},{"Or":[967]},{"Or":[969]},{"Pub":{"ty":163,"body":973,"replaceable":false}},{"Pub":{"ty":164,"body":978,"replaceable":false}},{"Or":[981]},{"Pub":{"ty":165,"body":986,"replaceable":false}},{"Pub":{"ty":166,"body":996,"replaceable":false}},{"Or":[1002]},{"Pub":{"ty":167,"body":1005,"replaceable":false}},{"Pub":{"ty":168,"body":1007,"replaceable":false}},{"Pub":{"ty":169,"body":1009,"replaceable":false}},{"Pub":{"ty":170,"body":1017,"replaceable":false}},{"Pub":{"ty":171,"body":1028,"replaceable":false}},{"Or":[1032]},{"Pub":{"ty":172,"body":1034,"replaceable":false}},{"Pub":{"ty":173,"body":1044,"replaceable":false}},{"Pub":{"ty":174,"body":1047,"replaceable":false}},{"Pub":{"ty":175,"body":1050,"replaceable":false}},{"Pub":{"ty":176,"body":1055,"replaceable":false}},{"Pub":{"ty":177,"body":1062,"replaceable":false}},{"Pub":{"ty":178,"body":1065,"replaceable":false}},{"Pub":{"ty":179,"body":1068,"replaceable":false}},{"Pub":{"ty":180,"body":1071,"replaceable":false}},{"Pub":{"ty":181,"body":1081,"replaceable":false}},{"Pub":{"ty":182,"body":1089,"replaceable":false}},{"Pub":{"ty":183,"body":1097,"replaceable":false}},{"Or":[1101,1105]},{"Pub":{"ty":184,"body":1113,"replaceable":false}},{"Pub":{"ty":185,"body":1117,"replaceable":false}},{"Pub":{"ty":186,"body":1125,"replaceable":false}},{"Pub":{"ty":187,"body":1128,"replaceable":false}},{"Or":[1130,1132,1134,1136,1138,1140]},{"Pub":{"ty":188,"body":1144,"replaceable":false}},{"Pub":{"ty":189,"body":1148,"replaceable":false}},{"Pub":{"ty":190,"body":1151,"replaceable":false}},{"Pub":{"ty":190,"body":1153,"replaceable":false}},{"Pub":{"ty":190,"body":1155,"replaceable":false}},{"Pub":{"ty":190,"body":1159,"replaceable":false}},{"Or":[1161,1163]},{"Or":[1169]},{"Pub":{"ty":191,"body":1195,"replaceable":false}},{"Pub":{"ty":192,"body":1200,"replaceable":false}},{"Or":[1202]},{"Pub":{"ty":193,"body":1212,"replaceable":false}},{"Pub":{"ty":194,"body":1220,"replaceable":false}},{"Pub":{"ty":195,"body":1235,"replaceable":false}},{"Pub":{"ty":196,"body":1264,"replaceable":false}},{"Or":[1274]},{"Or":[1279]},{"Or":[1284]},{"Or":[1289]},{"Or":[1294]},{"Or":[1302]},{"Or":[1317]},{"And":[[1],null]},{"Or":[149]},{"WithSkip":[2,3]},{"Rep":151},{"And":[[152],null]},{"Token":11},{"And":[[154],null]},{"ContextualToken":[4,"union"]},{"And":[[156],null]},{"Token":16},{"And":[[158],null]},{"Token":12},{"And":[[160],null]},{"Token":13},{"And":[[162],null]},{"Token":17},{"And":[[164],null]},{"Token":29},{"And":[[166],null]},{"And":[[25],null]},{"Opt":36},{"And":[[137,169],null]},{"Or":[170]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[172,173,174,175,176,177,178,179,180]},{"Inject":[171,181]},{"And":[[182],null]},{"And":[[28],null]},{"Or":[183,184]},{"Token":12},{"And":[[47],null]},{"Token":58},{"And":[[188,5],null]},{"Or":[189]},{"Opt":190},{"And":[[191],null]},{"Or":[187,192]},{"And":[[38,193],null]},{"Token":58},{"Opt":195},{"And":[[196,5],null]},{"Or":[194,197]},{"Token":56},{"And":[[186,198,199],1]},{"Or":[200]},{"Token":65},{"And":[[202],null]},{"Call":[142,[[2,6]]]},{"Call":[143,[[3,204]]]},{"And":[[205],null]},{"Or":[203,206]},{"Token":18},{"And":[[208],null]},{"Token":90},{"Opt":47},{"And":[[210,211],1]},{"Or":[209,212]},{"Token":7},{"Token":6},{"Token":90},{"Opt":47},{"Token":56},{"And":[[214,215,216,217,218],2]},{"Or":[219]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[86],null]},{"Token":56},{"And":[[228],null]},{"Or":[227,229]},{"And":[[221,222,223,224,11,225,226,230],2]},{"Or":[231]},{"Token":49},{"And":[[233,48],null]},{"Token":7},{"Token":88},{"Opt":236},{"And":[[235,237],null]},{"Or":[238]},{"Opt":14},{"Call":[142,[[2,12]]]},{"And":[[240,241],null]},{"Or":[242]},{"Call":[144,[[4,243]]]},{"And":[[244],null]},{"Token":57},{"And":[[59,246,48],1]},{"Or":[247]},{"Token":57},{"And":[[249,48],null]},{"Or":[250]},{"Opt":251},{"And":[[59,252],null]},{"Or":[253]},{"Token":75},{"Opt":255},{"Token":27},{"Opt":257},{"Token":18},{"Token":57},{"And":[[260,48],null]},{"Or":[261]},{"Opt":262},{"Token":59},{"And":[[264],null]},"Eof",{"And":[[266],null]},{"Or":[265,267]},{"And":[[256,258,259,263,268],3]},{"Or":[269]},{"Token":11},{"And":[[271],null]},{"ContextualToken":[4,"union"]},{"And":[[273],null]},{"Or":[272,274]},{"Token":90},{"Opt":31},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,278]]]},{"And":[[279],null]},{"Token":56},{"And":[[281],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,283]]]},{"Token":56},{"And":[[284,285],null]},{"Or":[280,282,286]},{"And":[[275,276,277,287],1]},{"Or":[288]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[290,291,292,48],2]},{"Or":[293]},{"Opt":36},{"And":[[295,48],null]},{"Or":[296]},{"Token":16},{"Token":90},{"Call":[142,[[2,19]]]},{"Call":[143,[[3,300]]]},{"And":[[298,299,301],1]},{"Or":[302]},{"Token":90},{"Token":51},{"And":[[305,71],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,307]]]},{"And":[[308],null]},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,310]]]},{"And":[[311],null]},{"Or":[306,309,312]},{"Opt":313},{"And":[[304,314],1]},{"Or":[315]},{"Token":13},{"Token":90},{"Token":56},{"And":[[319],null]},{"Call":[143,[[3,1]]]},{"And":[[321],null]},{"Or":[320,322]},{"And":[[317,318,323],1]},{"Or":[324]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[328,48],null]},{"Or":[329]},{"Opt":330},{"And":[[48,331],null]},{"Or":[332]},{"And":[[326,327,333,23],1]},{"Or":[334]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"And":[[137,336,337,338,339,23],3]},{"Or":[340]},{"Opt":36},{"And":[[137,342],null]},{"Or":[343]},{"Inject":[344,24]},{"And":[[345],null]},{"And":[[28],null]},{"Or":[346,347]},{"WithSkip":[25,348]},{"Rep":349},{"Call":[143,[[3,350]]]},{"And":[[351],null]},{"Or":[352]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[357],null]},{"Token":8},{"And":[[359],null]},{"Token":20},{"And":[[361],null]},{"Token":21},{"And":[[363],null]},{"Token":22},{"And":[[365],null]},{"Token":63},{"And":[[367],null]},{"Token":7},{"And":[[369],null]},{"Token":90},{"Token":80},{"And":[[371,372],null]},{"Or":[373]},{"And":[[374],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[379,48],null]},{"Or":[380]},{"Opt":381},{"Token":56},{"And":[[376,377,378,382,383],1]},{"Or":[384]},{"Token":21},{"And":[[386],null]},{"Token":22},{"And":[[388],null]},{"Or":[387,389]},{"Token":90},{"Token":57},{"Token":51},{"And":[[393,71],null]},{"Or":[394]},{"Opt":395},{"Token":56},{"And":[[390,391,392,48,396,397],1]},{"Or":[398]},{"And":[[139],null]},{"Token":56},{"And":[[140,401],null]},{"Or":[400,402]},{"Rep":30},{"Call":[143,[[3,404]]]},{"And":[[10,405],null]},{"Or":[406]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[142,[[2,35]]]},{"Call":[142,[[2,32]]]},{"And":[[410,411],null]},{"Or":[412]},{"Call":[145,[[5,413]]]},{"And":[[414],null]},{"Or":[415]},{"Token":90},{"Opt":33},{"And":[[417,418],1]},{"Or":[419]},{"Token":57},{"Token":71},{"And":[[422],null]},"Eof",{"And":[[424],null]},{"Token":59},{"Not":426},{"Not":427},{"And":[[428],null]},{"Token":37},{"Not":430},{"Not":431},{"And":[[432],null]},{"Or":[423,425,429,433]},{"And":[[34,434],1]},{"Or":[435]},{"Rep":436},{"And":[[421,437],null]},{"Token":85},{"And":[[439],null]},{"And":[[48],null]},{"Or":[440,441]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[446],null]},"Eof",{"And":[[448],null]},{"Token":59},{"Not":450},{"Not":451},{"And":[[452],null]},{"Or":[447,449,453]},{"And":[[445,454],1]},{"Or":[455]},{"Rep":456},{"And":[[444,457],null]},{"Or":[458]},{"Opt":459},{"And":[[443,460],1]},{"Or":[461]},{"Token":10},{"Token":6},{"And":[[464],null]},{"Token":19},{"And":[[466],null]},{"Or":[465,467]},{"Call":[144,[[4,468]]]},{"Opt":469},{"And":[[463,470],null]},{"Or":[471]},{"Token":34},{"Token":59},{"And":[[474],null]},"Eof",{"And":[[476],null]},{"Token":37},{"Not":478},{"Not":479},{"And":[[480],null]},{"Or":[475,477,481]},{"And":[[48,33,482],null]},{"Or":[483]},{"Rep":484},{"And":[[473,485],1]},{"Or":[486]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[489],null]},{"Enter":[1,41]},{"And":[[491],null]},{"Token":58},{"And":[[493,44],null]},{"Or":[494]},{"Token":58},{"Opt":496},{"And":[[497,44],null]},{"Or":[498]},{"Token":58},{"And":[[500,44],null]},{"Or":[501]},{"And":[[41,502],null]},{"Or":[503]},{"Token":90},{"And":[[505],null]},{"Token":18},{"And":[[507],null]},{"Token":19},{"And":[[509],null]},{"Or":[506,508,510]},{"And":[[45],null]},{"IsIn":3},{"And":[[513,46],null]},{"Or":[512,514]},{"Opt":515},{"And":[[511,516],null]},{"IsIn":3},{"And":[[518],null]},{"IsIn":1},{"Token":58},{"And":[[520,521],null]},{"Or":[519,522]},{"Token":85},{"Call":[142,[[2,524]]]},{"Call":[142,[[2,48]]]},{"And":[[525,526],null]},{"Or":[527]},{"Call":[145,[[5,528]]]},{"And":[[523,529],null]},{"Or":[530]},{"Call":[142,[[2,48]]]},{"Call":[144,[[4,532]]]},{"Opt":9},{"And":[[533,534],null]},{"Or":[535]},{"Token":5},{"Token":90},{"And":[[537,538],null]},{"Or":[539]},{"And":[[49],null]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[39],null]},{"Or":[549]},{"Token":75},{"Token":85},{"Opt":552},{"Token":27},{"Opt":554},{"And":[[551,553,555,48],null]},{"Or":[556]},{"Token":79},{"And":[[558],null]},{"Or":[559]},{"Token":35},{"Token":36},{"And":[[561,562],null]},{"Or":[563]},{"Opt":54},{"And":[[48,565],null]},{"Or":[566]},{"Call":[144,[[4,567]]]},{"And":[[568],null]},{"Or":[569]},{"Token":59},{"Call":[142,[[2,48]]]},{"And":[[571,572],null]},{"Or":[573]},{"Token":80},{"And":[[575],null]},{"Or":[576]},{"Token":56},{"And":[[578,71],null]},{"Or":[579]},{"Opt":580},{"And":[[48,581],null]},{"Or":[582]},{"Call":[146,[[6,583]]]},{"And":[[584],null]},{"Or":[585]},{"Token":8},{"Call":[142,[[2,58]]]},{"Call":[144,[[4,588]]]},{"And":[[587,589,9],1]},{"Or":[590]},{"Token":57},{"And":[[59,592],null]},{"Or":[593]},{"Opt":594},{"And":[[595,48],null]},{"Or":[596]},{"And":[[60],null]},{"And":[[61],null]},{"And":[[65],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[70],null]},{"Token":79},{"And":[[605],null]},{"Or":[606]},{"And":[[62],null]},{"And":[[63],null]},{"Or":[608,609]},{"Opt":610},{"And":[[40,611],null]},{"Or":[612]},{"Call":[142,[[2,59]]]},{"Token":61},{"Token":59},{"Opt":616},{"And":[[615,617],null]},{"Or":[618]},{"Opt":619},{"And":[[614,620],null]},{"Or":[621]},{"Call":[144,[[4,622]]]},{"And":[[623],null]},{"Or":[624]},{"Call":[142,[[2,64]]]},{"Token":61},{"Token":59},{"Opt":628},{"And":[[627,629],null]},{"Or":[630]},{"Opt":631},{"And":[[626,632],null]},{"Or":[633]},{"Call":[143,[[3,634]]]},{"And":[[635],null]},{"Or":[636]},{"Token":57},{"Not":638},{"And":[[65,639],null]},{"Token":90},{"Token":57},{"And":[[641,642,59],2]},{"Or":[640,643]},{"Token":28},{"Opt":645},{"Token":27},{"Opt":647},{"Token":90},{"And":[[646,648,649],null]},{"Or":[650]},{"And":[[75],null]},{"Or":[652]},{"Token":35},{"Token":36},{"And":[[654,655],null]},{"Or":[656]},{"Opt":69},{"And":[[59,658],null]},{"Or":[659]},{"Call":[144,[[4,660]]]},{"And":[[661],null]},{"Or":[662]},{"Token":59},{"Call":[142,[[2,59]]]},{"And":[[664,665],null]},{"Or":[666]},{"Token":75},{"Token":27},{"Opt":669},{"And":[[668,670,59],null]},{"Or":[671]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":675},{"And":[[673,674,676,108],null]},{"Or":[677]},{"IsIn":2},{"Not":73},{"And":[[679,680],null]},{"IsIn":2},{"Not":682},{"And":[[683],null]},{"Or":[681,684]},{"And":[[685,108],null]},{"Or":[686]},{"Token":60},{"Token":90},{"And":[[689],null]},{"Token":87},{"And":[[691],null]},{"Or":[690,692]},{"And":[[688,693],null]},{"Or":[694]},{"Call":[146,[[6,71]]]},{"Token":81},{"Token":5},{"And":[[698,48],null]},{"Or":[699]},{"Token":75},{"Token":27},{"Opt":702},{"And":[[701,703],null]},{"Or":[704]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[709],null]},{"Token":67},{"And":[[711],null]},{"Token":69},{"And":[[713],null]},{"Or":[710,712,714]},{"Call":[121,[[1,715]]]},{"Token":71},{"And":[[717],null]},{"Token":73},{"And":[[719],null]},{"Or":[718,720]},{"Call":[121,[[1,721]]]},{"ContextualToken":[43,"<<"]},{"And":[[723],null]},{"ContextualToken":[45,">>"]},{"And":[[725],null]},{"Or":[724,726]},{"Call":[121,[[1,727]]]},{"Token":75},{"Token":75},{"Not":730},{"And":[[729,731],null]},{"Or":[732]},{"Call":[121,[[1,733]]]},{"Token":82},{"Call":[121,[[1,735]]]},{"Token":77},{"Token":77},{"Not":738},{"And":[[737,739],null]},{"Or":[740]},{"Call":[121,[[1,741]]]},{"Call":[121,[[1,126]]]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,744]]]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,746]]]},{"Call":[121,[[1,134]]]},{"Token":51},{"And":[[749],null]},{"Token":72},{"And":[[751],null]},{"Token":74},{"And":[[753],null]},{"Token":66},{"And":[[755],null]},{"Token":68},{"And":[[757],null]},{"Token":70},{"And":[[759],null]},{"Token":76},{"And":[[761],null]},{"Token":78},{"And":[[763],null]},{"Token":83},{"And":[[765],null]},{"ContextualToken":[46,">>="]},{"And":[[767],null]},{"ContextualToken":[44,"<<="]},{"And":[[769],null]},{"Or":[750,752,754,756,758,760,762,764,766,768,770]},{"Call":[121,[[1,771]]]},{"And":[[75],null]},{"Token":90},{"And":[[774],null]},{"Token":18},{"And":[[776],null]},{"Token":19},{"And":[[778],null]},{"Token":39},{"And":[[780],null]},{"Token":58},{"And":[[782],null]},{"Token":35},{"And":[[784],null]},{"Token":41},{"And":[[786],null]},{"Token":77},{"And":[[788],null]},{"Token":31},{"And":[[790],null]},{"Token":37},{"And":[[792],null]},{"Token":14},{"And":[[794],null]},{"Token":25},{"And":[[796],null]},{"Token":24},{"And":[[798],null]},{"Token":23},{"And":[[800],null]},{"Token":30},{"And":[[802],null]},{"Token":75},{"And":[[804],null]},{"Token":65},{"And":[[806],null]},{"Token":73},{"And":[[808],null]},{"Token":80},{"And":[[810],null]},{"Token":61},{"And":[[812],null]},{"Token":62},{"And":[[814],null]},{"PrevIs":[155,161,162,163,164,165,168]},{"And":[[816],null]},{"Var":0},{"Exit":[2,818]},{"Exit":[0,819]},{"And":[[820],null]},{"Token":87},{"And":[[822],null]},{"Token":88},{"And":[[824],null]},{"Token":89},{"And":[[826],null]},{"Token":84},{"And":[[828],null]},{"Token":86},{"And":[[830],null]},{"Or":[823,825,827,829,831]},{"Token":90},{"Token":80},{"And":[[833,834],null]},{"Or":[835]},{"Not":836},{"Opt":77},{"And":[[837,40,838],null]},{"Or":[839]},{"IsIn":0},{"Not":841},{"Call":[142,[[2,78]]]},{"Token":61},{"Call":[74,[[0,71]]]},{"And":[[844,845],null]},{"Or":[846]},{"Opt":847},{"And":[[843,848],null]},{"Or":[849]},{"Call":[143,[[3,850]]]},{"And":[[842,851],null]},{"Or":[852]},{"Token":90},{"Token":57},{"And":[[855,71],null]},{"Or":[856]},{"Opt":857},{"And":[[854,858],1]},{"Or":[859]},{"Token":35},{"Token":36},{"And":[[861,862],null]},{"Or":[863]},{"Call":[74,[[0,71]]]},{"Opt":81},{"And":[[865,866],null]},{"Or":[867]},{"Call":[144,[[4,868]]]},{"And":[[869],null]},{"Or":[870]},{"Token":59},{"Call":[74,[[0,71]]]},{"Call":[142,[[2,873]]]},{"And":[[872,874],null]},{"Or":[875]},{"Call":[142,[[2,71]]]},{"Call":[74,[[0,877]]]},{"Call":[146,[[6,878]]]},{"And":[[879],null]},{"Or":[880]},{"Token":26},{"Opt":882},{"Token":77},{"Rep":84},{"Token":77},{"Token":49},{"And":[[887,48,86],null]},{"Call":[74,[[0,71]]]},{"And":[[889],null]},{"Or":[888,890]},{"And":[[883,884,885,886,891],null]},{"Or":[892]},{"Token":59},{"And":[[894],null]},{"Token":77},{"Not":896},{"Not":897},{"And":[[898],null]},{"Or":[895,899]},{"And":[[13,900],1]},{"Token":31},{"Opt":71},{"And":[[902,903],null]},{"Or":[904]},{"Token":33},{"Opt":906},{"Rep":87},{"Opt":71},{"And":[[908,909],null]},{"Or":[910]},{"Call":[143,[[3,911]]]},{"And":[[907,912],null]},{"Or":[913]},{"Call":[74,[[0,914]]]},{"And":[[915],null]},{"Or":[916]},{"And":[[88],null]},{"And":[[92],null]},{"And":[[91],null]},{"And":[[3],null]},{"Token":9},{"Opt":89},{"Opt":90},{"Token":56},{"And":[[922,59,923,924,925],1]},{"Or":[926]},{"Token":57},{"And":[[928,48],null]},{"Or":[929]},{"Token":51},{"And":[[931,71],null]},{"Or":[932]},{"Token":56},{"And":[[934],null]},{"Or":[935]},"Eof",{"Not":937},{"And":[[73,938],null]},{"Token":56},{"And":[[940],null]},{"Or":[939,941]},{"And":[[71,942],null]},{"Or":[943]},{"Enter":[2,944]},{"And":[[945],null]},{"Or":[946]},{"Token":14},{"Token":15},{"And":[[86],null]},{"And":[[93],null]},{"Or":[950,951]},{"And":[[949,952],null]},{"Or":[953]},{"Opt":954},{"And":[[948,95,86,955],1]},{"Or":[956]},{"Opt":99},{"Token":25},{"And":[[958,959,95,86],2]},{"Or":[960]},{"Token":9},{"Token":51},{"And":[[962,59,963],1]},{"Or":[964]},{"Opt":965},{"And":[[966,96],null]},{"Enter":[0,71]},{"And":[[968],null]},{"Opt":99},{"Token":24},{"And":[[970,971,86],2]},{"Or":[972]},{"Opt":99},{"Token":23},{"Token":32},{"And":[[974,975,59,976,96,86],2]},{"Or":[977]},{"Token":85},{"Token":57},{"And":[[979,980],null]},{"Token":30},{"Rep":101},{"Call":[143,[[3,983]]]},{"And":[[982,96,984],1]},{"Or":[985]},{"Token":50},{"Enter":[2,71]},{"Token":59},{"And":[[989],null]},"Eof",{"And":[[991],null]},{"And":[[73],null]},{"Or":[990,992,993]},{"And":[[102,987,988,994],1]},{"Or":[995]},{"Token":77},{"And":[[997,59],null]},{"Or":[998]},{"Rep":999},{"Opt":103},{"And":[[59,1000,1001],null]},{"Token":14},{"And":[[1003,71],null]},{"Or":[1004]},{"And":[[139],null]},{"Or":[1006]},{"And":[[140],null]},{"Or":[1008]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":1012},{"And":[[1010,1011,1013,108],null]},{"Or":[1014]},{"And":[[71,1015],null]},{"Or":[1016]},{"IsIn":2},{"Not":73},{"And":[[1018,1019],null]},{"IsIn":2},{"Not":1021},{"And":[[1022],null]},{"Or":[1020,1023]},{"And":[[1024,108],null]},{"Or":[1025]},{"And":[[71,1026],null]},{"Or":[1027]},{"Call":[142,[[2,109]]]},{"Call":[144,[[4,1029]]]},{"Call":[74,[[0,1030]]]},{"And":[[1031],null]},{"And":[[71],null]},{"Or":[1033]},{"Token":60},{"Token":90},{"And":[[1036],null]},{"Token":87},{"And":[[1038],null]},{"Or":[1037,1039]},{"And":[[1035,1040],null]},{"Or":[1041]},{"And":[[71,1042],null]},{"Or":[1043]},{"Call":[146,[[6,71]]]},{"And":[[71,1045],null]},{"Or":[1046]},{"Token":81},{"And":[[71,1048],null]},{"Or":[1049]},{"Token":5},{"And":[[1051,48],null]},{"Or":[1052]},{"And":[[71,1053],null]},{"Or":[1054]},{"Token":75},{"Token":27},{"Opt":1057},{"And":[[1056,1058],null]},{"Or":[1059]},{"And":[[1060,71],null]},{"Or":[1061]},{"Token":65},{"And":[[1063,71],null]},{"Or":[1064]},{"Token":73},{"And":[[1066,71],null]},{"Or":[1067]},{"Token":80},{"And":[[1069,71],null]},{"Or":[1070]},{"Token":65},{"And":[[1072],null]},{"Token":67},{"And":[[1074],null]},{"Token":69},{"And":[[1076],null]},{"Or":[1073,1075,1077]},{"Call":[121,[[1,1078]]]},{"And":[[71,1079,71],null]},{"Or":[1080]},{"Token":71},{"And":[[1082],null]},{"Token":73},{"And":[[1084],null]},{"Or":[1083,1085]},{"Call":[121,[[1,1086]]]},{"And":[[71,1087,71],null]},{"Or":[1088]},{"ContextualToken":[43,"<<"]},{"And":[[1090],null]},{"ContextualToken":[45,">>"]},{"And":[[1092],null]},{"Or":[1091,1093]},{"Call":[121,[[1,1094]]]},{"And":[[71,1095,71],null]},{"Or":[1096]},{"IsIn":2},{"Not":73},{"Var":1},{"And":[[1098,1099,1100],null]},{"IsIn":2},{"Not":1102},{"Var":1},{"And":[[1103,1104],null]},{"Token":75},{"Token":75},{"Not":1107},{"And":[[1106,1108],null]},{"Or":[1109]},{"Call":[121,[[1,1110]]]},{"And":[[71,1111,71],null]},{"Or":[1112]},{"Token":82},{"Call":[121,[[1,1114]]]},{"And":[[71,1115,71],null]},{"Or":[1116]},{"Token":77},{"Token":77},{"Not":1119},{"And":[[1118,1120],null]},{"Or":[1121]},{"Call":[121,[[1,1122]]]},{"And":[[71,1123,71],null]},{"Or":[1124]},{"Call":[121,[[1,126]]]},{"And":[[71,1126,71],null]},{"Or":[1127]},{"Token":52},{"And":[[1129],null]},{"Token":53},{"And":[[1131],null]},{"Token":39},{"And":[[1133],null]},{"Token":40},{"And":[[1135],null]},{"Token":55},{"And":[[1137],null]},{"Token":54},{"And":[[1139],null]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,1141]]]},{"And":[[71,1142,71],null]},{"Or":[1143]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,1145]]]},{"And":[[71,1146,71],null]},{"Or":[1147]},{"Call":[121,[[1,134]]]},{"And":[[71,1149,71],null]},{"Or":[1150]},{"And":[[134,71],null]},{"Or":[1152]},{"And":[[71,133],null]},{"Or":[1154]},{"Token":61},{"Not":72},{"And":[[1156,1157],null]},{"Or":[1158]},{"Token":61},{"And":[[1160],null]},{"Token":62},{"And":[[1162],null]},{"Token":37},{"IsIn":0},{"And":[[1164,1165],null]},{"Or":[1166]},{"Not":1167},{"And":[[133,1168],null]},{"Token":51},{"And":[[1170],null]},{"Token":72},{"And":[[1172],null]},{"Token":74},{"And":[[1174],null]},{"Token":66},{"And":[[1176],null]},{"Token":68},{"And":[[1178],null]},{"Token":70},{"And":[[1180],null]},{"Token":76},{"And":[[1182],null]},{"Token":78},{"And":[[1184],null]},{"Token":83},{"And":[[1186],null]},{"ContextualToken":[46,">>="]},{"And":[[1188],null]},{"ContextualToken":[44,"<<="]},{"And":[[1190],null]},{"Or":[1171,1173,1175,1177,1179,1181,1183,1185,1187,1189,1191]},{"Call":[121,[[1,1192]]]},{"And":[[71,1193,71],null]},{"Or":[1194]},{"Token":63},{"Call":[142,[[2,138]]]},{"Call":[146,[[6,1197]]]},{"And":[[1196,1198],null]},{"Or":[1199]},{"Rep":136},{"And":[[1201],null]},{"Token":90},{"Token":51},{"And":[[1204,71],null]},{"Call":[142,[[2,138]]]},{"Call":[144,[[4,1206]]]},{"And":[[1207],null]},{"Or":[1205,1208]},{"Opt":1209},{"And":[[1203,1210],1]},{"Or":[1211]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1215},{"Rep":141},{"Call":[143,[[3,1217]]]},{"And":[[1213,1214,1216,1218],null]},{"Or":[1219]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1223},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1225,1226,1227],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1229,1230,1231],null]},{"Or":[1228,1232]},{"And":[[1221,1222,1224,1233],null]},{"Or":[1234]},{"Token":35},{"And":[[1236],null]},{"Token":36},{"And":[[1238],null]},{"Token":37},{"And":[[1240],null]},{"Token":38},{"And":[[1242],null]},{"Token":41},{"And":[[1244],null]},{"Token":42},{"And":[[1246],null]},{"Or":[1237,1239,1241,1243,1245,1247]},{"Not":1248},"Any",{"And":[[1249,1250],null]},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1252,1253,1254],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1256,1257,1258],null]},{"Token":37},{"Rep":141},{"Token":38},{"And":[[1260,1261,1262],null]},{"Or":[1251,1255,1259,1263]},{"Var":2},"Eof",{"And":[[1266],null]},{"Token":59},{"And":[[1268],null]},{"Or":[1267,1269]},{"And":[[1265,1270],1]},{"Or":[1271]},{"Rep":1272},{"And":[[1273],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[147,[[7,1275],[8,1276],[9,1277]]]},{"And":[[1278],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[147,[[7,1280],[8,1281],[9,1282]]]},{"And":[[1283],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[147,[[7,1285],[8,1286],[9,1287]]]},{"And":[[1288],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[147,[[7,1290],[8,1291],[9,1292]]]},{"And":[[1293],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[148,[[10,1296],[11,1297]]]},{"Var":9},{"Layer":[1298,1299]},{"Var":8},{"And":[[1295,1300,1301],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[148,[[10,1304],[11,1305]]]},{"Var":11},{"And":[[1303,1306,1307],1]},{"Var":11},{"Not":1309},"Any",{"And":[[1310,1311],null]},{"Or":[1312]},{"And":[[1313],null]},{"Or":[1308,1314]},{"Rep":1315},{"And":[[1316],null]}]"##;

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