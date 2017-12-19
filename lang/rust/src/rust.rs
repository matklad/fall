use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, TextEdit, TreeBuilder};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LINE_COMMENT: NodeType = NodeType(101);
pub const BLOCK_COMMENT: NodeType = NodeType(102);
pub const UNION: NodeType = NodeType(103);
pub const AS: NodeType = NodeType(104);
pub const CRATE: NodeType = NodeType(105);
pub const EXTERN: NodeType = NodeType(106);
pub const FN: NodeType = NodeType(107);
pub const LET: NodeType = NodeType(108);
pub const PUB: NodeType = NodeType(109);
pub const STRUCT: NodeType = NodeType(110);
pub const USE: NodeType = NodeType(111);
pub const MOD: NodeType = NodeType(112);
pub const IF: NodeType = NodeType(113);
pub const ELSE: NodeType = NodeType(114);
pub const ENUM: NodeType = NodeType(115);
pub const IMPL: NodeType = NodeType(116);
pub const SELF: NodeType = NodeType(117);
pub const SUPER: NodeType = NodeType(118);
pub const TYPE: NodeType = NodeType(119);
pub const CONST: NodeType = NodeType(120);
pub const STATIC: NodeType = NodeType(121);
pub const FOR: NodeType = NodeType(122);
pub const LOOP: NodeType = NodeType(123);
pub const WHILE: NodeType = NodeType(124);
pub const MOVE: NodeType = NodeType(125);
pub const MUT: NodeType = NodeType(126);
pub const REF: NodeType = NodeType(127);
pub const TRAIT: NodeType = NodeType(128);
pub const MATCH: NodeType = NodeType(129);
pub const RETURN: NodeType = NodeType(130);
pub const IN: NodeType = NodeType(131);
pub const UNSAFE: NodeType = NodeType(132);
pub const WHERE: NodeType = NodeType(133);
pub const L_PAREN: NodeType = NodeType(134);
pub const R_PAREN: NodeType = NodeType(135);
pub const L_CURLY: NodeType = NodeType(136);
pub const R_CURLY: NodeType = NodeType(137);
pub const L_ANGLE: NodeType = NodeType(138);
pub const R_ANGLE: NodeType = NodeType(139);
pub const L_BRACK: NodeType = NodeType(140);
pub const R_BRACK: NodeType = NodeType(141);
pub const SHL: NodeType = NodeType(142);
pub const SHL_EQ: NodeType = NodeType(143);
pub const SHR: NodeType = NodeType(144);
pub const SHR_EQ: NodeType = NodeType(145);
pub const AND: NodeType = NodeType(146);
pub const OR: NodeType = NodeType(147);
pub const THIN_ARROW: NodeType = NodeType(148);
pub const FAT_ARROW: NodeType = NodeType(149);
pub const EQ: NodeType = NodeType(150);
pub const EQEQ: NodeType = NodeType(151);
pub const BANGEQ: NodeType = NodeType(152);
pub const GTET: NodeType = NodeType(153);
pub const LTEQ: NodeType = NodeType(154);
pub const SEMI: NodeType = NodeType(155);
pub const COLON: NodeType = NodeType(156);
pub const COLONCOLON: NodeType = NodeType(157);
pub const COMMA: NodeType = NodeType(158);
pub const DOT: NodeType = NodeType(159);
pub const DOTDOT: NodeType = NodeType(160);
pub const DOTDOTDOT: NodeType = NodeType(161);
pub const HASH: NodeType = NodeType(162);
pub const DOLLAR: NodeType = NodeType(163);
pub const STAR: NodeType = NodeType(164);
pub const STAR_EQ: NodeType = NodeType(165);
pub const SLASH: NodeType = NodeType(166);
pub const SLASH_EQ: NodeType = NodeType(167);
pub const PERCENT: NodeType = NodeType(168);
pub const PERCENT_EQ: NodeType = NodeType(169);
pub const PLUS: NodeType = NodeType(170);
pub const PLUS_EQ: NodeType = NodeType(171);
pub const MINUS: NodeType = NodeType(172);
pub const MINUS_EQ: NodeType = NodeType(173);
pub const AMPERSAND: NodeType = NodeType(174);
pub const AMPERSAND_EQ: NodeType = NodeType(175);
pub const PIPE: NodeType = NodeType(176);
pub const PIPE_EQ: NodeType = NodeType(177);
pub const UNDERSCORE: NodeType = NodeType(178);
pub const BANG: NodeType = NodeType(179);
pub const QUESTION: NodeType = NodeType(180);
pub const CARET: NodeType = NodeType(181);
pub const CARET_EQ: NodeType = NodeType(182);
pub const CHAR: NodeType = NodeType(183);
pub const LIFETIME: NodeType = NodeType(184);
pub const BOOL: NodeType = NodeType(185);
pub const NUMBER: NodeType = NodeType(186);
pub const STRING: NodeType = NodeType(187);
pub const RAW_STRING: NodeType = NodeType(188);
pub const IDENT: NodeType = NodeType(189);
pub const FILE: NodeType = NodeType(190);
pub const USE_DECL: NodeType = NodeType(191);
pub const USE_SPEC: NodeType = NodeType(192);
pub const USE_SPEC_ENTRY: NodeType = NodeType(193);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(194);
pub const FN_DEF: NodeType = NodeType(195);
pub const LINKAGE: NodeType = NodeType(196);
pub const VALUE_PARAM: NodeType = NodeType(197);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(198);
pub const SELF_PARAMETER: NodeType = NodeType(199);
pub const STRUCT_DEF: NodeType = NodeType(200);
pub const STRUCT_FIELD: NodeType = NodeType(201);
pub const TUPLE_FIELD: NodeType = NodeType(202);
pub const ENUM_DEF: NodeType = NodeType(203);
pub const ENUM_VARIANT: NodeType = NodeType(204);
pub const MOD_DEF: NodeType = NodeType(205);
pub const IMPL_DEF: NodeType = NodeType(206);
pub const TRAIT_DEF: NodeType = NodeType(207);
pub const MEMBERS: NodeType = NodeType(208);
pub const TYPE_DEF: NodeType = NodeType(209);
pub const CONST_DEF: NodeType = NodeType(210);
pub const MACRO_ITEM: NodeType = NodeType(211);
pub const EXTERN_BLOCK: NodeType = NodeType(212);
pub const TYPE_PARAMETERS: NodeType = NodeType(213);
pub const TYPE_PARAMETER: NodeType = NodeType(214);
pub const TYPE_BOUND: NodeType = NodeType(215);
pub const LIFETIME_PARAMETER: NodeType = NodeType(216);
pub const VISIBILITY: NodeType = NodeType(217);
pub const WHERE_CLAUSE: NodeType = NodeType(218);
pub const PATH: NodeType = NodeType(219);
pub const TYPE_ARGUMENTS: NodeType = NodeType(220);
pub const FN_TRAIT_SUGAR: NodeType = NodeType(221);
pub const ALIAS: NodeType = NodeType(222);
pub const PATH_TYPE: NodeType = NodeType(223);
pub const REFERENCE_TYPE: NodeType = NodeType(224);
pub const PLACEHOLDER_TYPE: NodeType = NodeType(225);
pub const UNIT_TYPE: NodeType = NodeType(226);
pub const PAREN_TYPE: NodeType = NodeType(227);
pub const TUPLE_TYPE: NodeType = NodeType(228);
pub const NEVER_TYPE: NodeType = NodeType(229);
pub const ARRAY_TYPE: NodeType = NodeType(230);
pub const FN_POINTER_TYPE: NodeType = NodeType(231);
pub const WILDCARD_PATTERN: NodeType = NodeType(232);
pub const PATH_PATTERN: NodeType = NodeType(233);
pub const TUPE_STRUCT_PATTERN: NodeType = NodeType(234);
pub const STRUCT_PATTERN: NodeType = NodeType(235);
pub const STRUCT_PATTERN_FIELD: NodeType = NodeType(236);
pub const BINDING_PATTERN: NodeType = NodeType(237);
pub const LITERAL_PATTERN: NodeType = NodeType(238);
pub const UNIT_PATTERN: NodeType = NodeType(239);
pub const PAREN_PATTERN: NodeType = NodeType(240);
pub const TUPLE_PATTERN: NodeType = NodeType(241);
pub const REFERENCE_PATTERN: NodeType = NodeType(242);
pub const EXPR: NodeType = NodeType(243);
pub const LITERAL: NodeType = NodeType(244);
pub const PATH_EXPR: NodeType = NodeType(245);
pub const STRUCT_LITERAL: NodeType = NodeType(246);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(247);
pub const UNIT_EXPR: NodeType = NodeType(248);
pub const PAREN_EXPR: NodeType = NodeType(249);
pub const TUPLE_EXPR: NodeType = NodeType(250);
pub const ARRAY_LITERAL: NodeType = NodeType(251);
pub const LAMBDA_EXPR: NodeType = NodeType(252);
pub const RETURN_EXPR: NodeType = NodeType(253);
pub const BLOCK_EXPR: NodeType = NodeType(254);
pub const LET_STMT: NodeType = NodeType(255);
pub const TYPE_ASCRIPTION: NodeType = NodeType(256);
pub const INITIALIZER: NodeType = NodeType(257);
pub const EMPTY_STMT: NodeType = NodeType(258);
pub const EXPR_STMT: NodeType = NodeType(259);
pub const IF_EXPR: NodeType = NodeType(260);
pub const WHILE_EXPR: NodeType = NodeType(261);
pub const LOOP_EXPR: NodeType = NodeType(262);
pub const FOR_EXPR: NodeType = NodeType(263);
pub const MATCH_EXPR: NodeType = NodeType(264);
pub const MATCH_ARM: NodeType = NodeType(265);
pub const GUARD: NodeType = NodeType(266);
pub const BLOCK_MACRO_EXPR: NodeType = NodeType(267);
pub const LINE_MACRO_EXPR: NodeType = NodeType(268);
pub const METHOD_CALL_EXPR: NodeType = NodeType(269);
pub const CALL_EXPR: NodeType = NodeType(270);
pub const VALUE_ARGUMENT: NodeType = NodeType(271);
pub const FIELD_EXPR: NodeType = NodeType(272);
pub const INDEX_EXPR: NodeType = NodeType(273);
pub const TRY_EXPR: NodeType = NodeType(274);
pub const CAST_EXPR: NodeType = NodeType(275);
pub const REFERENCE_EXPR: NodeType = NodeType(276);
pub const DEREFERENCE_EXPR: NodeType = NodeType(277);
pub const NEGATION_EXPR: NodeType = NodeType(278);
pub const NOT_EXPR: NodeType = NodeType(279);
pub const PRODUCT_EXPR: NodeType = NodeType(280);
pub const SUM_EXPR: NodeType = NodeType(281);
pub const BIT_SHIFT: NodeType = NodeType(282);
pub const BIT_AND: NodeType = NodeType(283);
pub const BIT_XOR: NodeType = NodeType(284);
pub const BIT_OR: NodeType = NodeType(285);
pub const COMPARISON: NodeType = NodeType(286);
pub const LOGICAL_AND: NodeType = NodeType(287);
pub const LOGICAL_OR: NodeType = NodeType(288);
pub const RANGE_EXPR: NodeType = NodeType(289);
pub const ASSIGNMENT_EXPR: NodeType = NodeType(290);
pub const ATTRIBUTE: NodeType = NodeType(291);
pub const ATTR_VALUE: NodeType = NodeType(292);
pub const BLOCK_MACRO: NodeType = NodeType(293);
pub const LINE_MACRO: NodeType = NodeType(294);
pub const TT: NodeType = NodeType(295);


pub fn language() -> &'static Language {
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
        let parser_json = r##"[{"Pub":{"ty":91,"body":150,"replaceable":false}},{"Or":[153]},{"Or":[155,157,159,161,163,165,167,168]},{"Cached":185},{"Pub":{"ty":92,"body":201,"replaceable":false}},{"Pub":{"ty":93,"body":207,"replaceable":false}},{"Pub":{"ty":94,"body":213,"replaceable":false}},{"Pub":{"ty":95,"body":220,"replaceable":false}},{"Pub":{"ty":96,"body":232,"replaceable":false}},{"Or":[234]},{"Pub":{"ty":97,"body":239,"replaceable":false}},{"Or":[245]},{"Pub":{"ty":98,"body":248,"replaceable":false}},{"Pub":{"ty":99,"body":254,"replaceable":false}},{"Pub":{"ty":100,"body":270,"replaceable":false}},{"Pub":{"ty":101,"body":289,"replaceable":false}},{"Pub":{"ty":102,"body":294,"replaceable":false}},{"Pub":{"ty":103,"body":297,"replaceable":false}},{"Pub":{"ty":104,"body":303,"replaceable":false}},{"Pub":{"ty":105,"body":316,"replaceable":false}},{"Pub":{"ty":106,"body":325,"replaceable":false}},{"Pub":{"ty":107,"body":335,"replaceable":false}},{"Pub":{"ty":108,"body":341,"replaceable":false}},{"Pub":{"ty":109,"body":353,"replaceable":false}},{"Or":[354,355,356]},{"Or":[358,360,362,364,366,368,370,375]},{"Pub":{"ty":110,"body":385,"replaceable":false}},{"Pub":{"ty":111,"body":399,"replaceable":false}},{"Pub":{"ty":112,"body":403,"replaceable":false}},{"Pub":{"ty":113,"body":407,"replaceable":false}},{"Or":[408,409]},{"Pub":{"ty":114,"body":416,"replaceable":false}},{"Pub":{"ty":115,"body":420,"replaceable":false}},{"Or":[438]},{"Pub":{"ty":116,"body":442,"replaceable":false}},{"Pub":{"ty":117,"body":462,"replaceable":false}},{"Pub":{"ty":118,"body":468,"replaceable":false}},{"Pub":{"ty":119,"body":483,"replaceable":false}},{"Or":[484]},{"Or":[486]},{"Or":[488]},{"Pratt":{"atoms":[42],"prefixes":[],"infixes":[{"ty":120,"op":491,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":495,"replaceable":false}},{"Pub":{"ty":120,"body":500,"replaceable":false}},{"Or":[513]},{"Pub":{"ty":121,"body":527,"replaceable":false}},{"Pub":{"ty":122,"body":532,"replaceable":false}},{"Pub":{"ty":123,"body":536,"replaceable":false}},{"Or":[537,538,539,540,541,542,543,544]},{"Pub":{"ty":124,"body":546,"replaceable":false}},{"Pub":{"ty":125,"body":553,"replaceable":false}},{"Pub":{"ty":126,"body":556,"replaceable":false}},{"Pub":{"ty":127,"body":560,"replaceable":false}},{"Pub":{"ty":128,"body":566,"replaceable":true}},{"PubReplace":{"ty":129,"body":570}},{"Pub":{"ty":130,"body":573,"replaceable":false}},{"Pub":{"ty":131,"body":582,"replaceable":false}},{"Pub":{"ty":132,"body":587,"replaceable":false}},{"Pub":{"ty":98,"body":593,"replaceable":false}},{"Or":[594,595,596,597,598,599,600]},{"Pub":{"ty":133,"body":603,"replaceable":false}},{"Pub":{"ty":134,"body":609,"replaceable":true}},{"PubReplace":{"ty":135,"body":621}},{"PubReplace":{"ty":136,"body":633}},{"Pub":{"ty":137,"body":640,"replaceable":false}},{"Pub":{"ty":138,"body":647,"replaceable":false}},{"Pub":{"ty":139,"body":649,"replaceable":false}},{"Pub":{"ty":140,"body":653,"replaceable":false}},{"Pub":{"ty":141,"body":659,"replaceable":true}},{"PubReplace":{"ty":142,"body":663}},{"Pub":{"ty":143,"body":668,"replaceable":false}},{"Pratt":{"atoms":[75,76,79,80,82,83,85,86,93,94,97,98,100,104,105,132],"prefixes":[{"ty":177,"op":692,"priority":999},{"ty":178,"op":693,"priority":999},{"ty":179,"op":694,"priority":999},{"ty":180,"op":695,"priority":999},{"ty":190,"op":134,"priority":2}],"infixes":[{"ty":170,"op":674,"priority":999,"has_rhs":false},{"ty":171,"op":108,"priority":999,"has_rhs":false},{"ty":173,"op":682,"priority":999,"has_rhs":false},{"ty":174,"op":683,"priority":999,"has_rhs":false},{"ty":175,"op":684,"priority":999,"has_rhs":false},{"ty":176,"op":687,"priority":999,"has_rhs":false},{"ty":181,"op":703,"priority":11,"has_rhs":true},{"ty":182,"op":709,"priority":10,"has_rhs":true},{"ty":183,"op":715,"priority":9,"has_rhs":true},{"ty":184,"op":721,"priority":8,"has_rhs":true},{"ty":185,"op":723,"priority":7,"has_rhs":true},{"ty":186,"op":729,"priority":6,"has_rhs":true},{"ty":187,"op":730,"priority":5,"has_rhs":true},{"ty":188,"op":732,"priority":4,"has_rhs":true},{"ty":189,"op":734,"priority":3,"has_rhs":true},{"ty":190,"op":735,"priority":2,"has_rhs":true},{"ty":190,"op":133,"priority":2,"has_rhs":false},{"ty":191,"op":759,"priority":1,"has_rhs":true}]}},{"Or":[760,762,764,766,768,770,772,774,776,778,780,782,784,786,788,790,792,794,796,798,800,802]},{"Or":[804]},{"Or":[808]},{"Pub":{"ty":145,"body":819,"replaceable":false}},{"Pub":{"ty":146,"body":827,"replaceable":true}},{"PubReplace":{"ty":147,"body":840}},{"Pub":{"ty":148,"body":847,"replaceable":false}},{"Pub":{"ty":149,"body":851,"replaceable":false}},{"Pub":{"ty":150,"body":858,"replaceable":true}},{"PubReplace":{"ty":151,"body":863}},{"Pub":{"ty":152,"body":868,"replaceable":false}},{"Pub":{"ty":153,"body":880,"replaceable":false}},{"Or":[888]},{"Pub":{"ty":154,"body":892,"replaceable":false}},{"Pub":{"ty":155,"body":904,"replaceable":false}},{"Or":[905,906,907,908]},{"Pub":{"ty":156,"body":914,"replaceable":false}},{"Pub":{"ty":157,"body":917,"replaceable":false}},{"Pub":{"ty":158,"body":920,"replaceable":false}},{"Pub":{"ty":159,"body":923,"replaceable":false}},{"Pub":{"ty":160,"body":934,"replaceable":false}},{"Pub":{"ty":161,"body":944,"replaceable":false}},{"Pub":{"ty":162,"body":948,"replaceable":false}},{"Or":[954]},{"Or":[956]},{"Pub":{"ty":163,"body":960,"replaceable":false}},{"Pub":{"ty":164,"body":965,"replaceable":false}},{"Or":[968]},{"Pub":{"ty":165,"body":973,"replaceable":false}},{"Pub":{"ty":166,"body":982,"replaceable":false}},{"Or":[988]},{"Pub":{"ty":167,"body":991,"replaceable":false}},{"Pub":{"ty":168,"body":993,"replaceable":false}},{"Pub":{"ty":169,"body":995,"replaceable":false}},{"Pub":{"ty":170,"body":1003,"replaceable":false}},{"Pub":{"ty":171,"body":1005,"replaceable":false}},{"Or":[1009]},{"Pub":{"ty":172,"body":1011,"replaceable":false}},{"Pub":{"ty":173,"body":1021,"replaceable":false}},{"Pub":{"ty":174,"body":1024,"replaceable":false}},{"Pub":{"ty":175,"body":1027,"replaceable":false}},{"Pub":{"ty":176,"body":1032,"replaceable":false}},{"Pub":{"ty":177,"body":1039,"replaceable":false}},{"Pub":{"ty":178,"body":1042,"replaceable":false}},{"Pub":{"ty":179,"body":1045,"replaceable":false}},{"Pub":{"ty":180,"body":1048,"replaceable":false}},{"Pub":{"ty":181,"body":1058,"replaceable":false}},{"Pub":{"ty":182,"body":1066,"replaceable":false}},{"Pub":{"ty":183,"body":1074,"replaceable":false}},{"Or":[1078,1082]},{"Pub":{"ty":184,"body":1090,"replaceable":false}},{"Pub":{"ty":185,"body":1094,"replaceable":false}},{"Pub":{"ty":186,"body":1102,"replaceable":false}},{"Pub":{"ty":187,"body":1105,"replaceable":false}},{"Or":[1107,1109,1111,1113,1115,1117]},{"Pub":{"ty":188,"body":1121,"replaceable":false}},{"Pub":{"ty":189,"body":1125,"replaceable":false}},{"Pub":{"ty":190,"body":1128,"replaceable":false}},{"Pub":{"ty":190,"body":1130,"replaceable":false}},{"Pub":{"ty":190,"body":1132,"replaceable":false}},{"Pub":{"ty":190,"body":1136,"replaceable":false}},{"Or":[1138,1140]},{"Or":[1146]},{"Pub":{"ty":191,"body":1172,"replaceable":false}},{"Pub":{"ty":192,"body":1177,"replaceable":false}},{"Or":[1179]},{"Pub":{"ty":193,"body":1189,"replaceable":false}},{"Pub":{"ty":194,"body":1197,"replaceable":false}},{"Pub":{"ty":195,"body":1212,"replaceable":false}},{"Pub":{"ty":196,"body":1241,"replaceable":false}},{"Or":[1251]},{"Or":[1256]},{"Or":[1261]},{"Or":[1266]},{"Or":[1271]},{"Or":[1279]},{"Or":[1294]},{"And":[[1],null]},{"Or":[149]},{"WithSkip":[2,3]},{"Rep":151},{"And":[[152],null]},{"Token":11},{"And":[[154],null]},{"ContextualToken":[4,"union"]},{"And":[[156],null]},{"Token":16},{"And":[[158],null]},{"Token":12},{"And":[[160],null]},{"Token":13},{"And":[[162],null]},{"Token":17},{"And":[[164],null]},{"Token":29},{"And":[[166],null]},{"And":[[25],null]},{"Opt":36},{"And":[[137,169],null]},{"Or":[170]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[172,173,174,175,176,177,178,179,180]},{"Inject":[171,181]},{"And":[[182],null]},{"And":[[28],null]},{"Or":[183,184]},{"Token":12},{"And":[[47],null]},{"Token":58},{"And":[[188,5],null]},{"Or":[189]},{"Opt":190},{"And":[[191],null]},{"Or":[187,192]},{"And":[[38,193],null]},{"Token":58},{"Opt":195},{"And":[[196,5],null]},{"Or":[194,197]},{"Token":56},{"And":[[186,198,199],1]},{"Or":[200]},{"Token":65},{"And":[[202],null]},{"Call":[142,[[2,6]]]},{"Call":[143,[[3,204]]]},{"And":[[205],null]},{"Or":[203,206]},{"Token":18},{"And":[[208],null]},{"Token":90},{"Opt":47},{"And":[[210,211],1]},{"Or":[209,212]},{"Token":7},{"Token":6},{"Token":90},{"Opt":47},{"Token":56},{"And":[[214,215,216,217,218],2]},{"Or":[219]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[86],null]},{"Token":56},{"And":[[228],null]},{"Or":[227,229]},{"And":[[221,222,223,224,11,225,226,230],2]},{"Or":[231]},{"Token":49},{"And":[[233,48],null]},{"Token":7},{"Token":88},{"Opt":236},{"And":[[235,237],null]},{"Or":[238]},{"Opt":14},{"Call":[142,[[2,12]]]},{"And":[[240,241],null]},{"Or":[242]},{"Call":[144,[[4,243]]]},{"And":[[244],null]},{"Token":57},{"And":[[59,246,48],1]},{"Or":[247]},{"Token":57},{"And":[[249,48],null]},{"Or":[250]},{"Opt":251},{"And":[[59,252],null]},{"Or":[253]},{"Token":75},{"Opt":255},{"Token":27},{"Opt":257},{"Token":18},{"Token":57},{"And":[[260,48],null]},{"Or":[261]},{"Opt":262},{"Token":59},{"And":[[264],null]},"Eof",{"And":[[266],null]},{"Or":[265,267]},{"And":[[256,258,259,263,268],3]},{"Or":[269]},{"Token":11},{"And":[[271],null]},{"ContextualToken":[4,"union"]},{"And":[[273],null]},{"Or":[272,274]},{"Token":90},{"Opt":31},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,278]]]},{"And":[[279],null]},{"Token":56},{"And":[[281],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,283]]]},{"Token":56},{"And":[[284,285],null]},{"Or":[280,282,286]},{"And":[[275,276,277,287],1]},{"Or":[288]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[290,291,292,48],2]},{"Or":[293]},{"Opt":36},{"And":[[295,48],null]},{"Or":[296]},{"Token":16},{"Token":90},{"Call":[142,[[2,19]]]},{"Call":[143,[[3,300]]]},{"And":[[298,299,301],1]},{"Or":[302]},{"Token":90},{"Token":51},{"And":[[305,71],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,307]]]},{"And":[[308],null]},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,310]]]},{"And":[[311],null]},{"Or":[306,309,312]},{"Opt":313},{"And":[[304,314],1]},{"Or":[315]},{"Token":13},{"Token":90},{"Token":56},{"And":[[319],null]},{"Call":[143,[[3,1]]]},{"And":[[321],null]},{"Or":[320,322]},{"And":[[317,318,323],1]},{"Or":[324]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[328,48],null]},{"Or":[329]},{"Opt":330},{"And":[[48,331],null]},{"Or":[332]},{"And":[[326,327,333,23],1]},{"Or":[334]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"And":[[137,336,337,338,339,23],3]},{"Or":[340]},{"Opt":36},{"And":[[137,342],null]},{"Or":[343]},{"Inject":[344,24]},{"And":[[345],null]},{"And":[[28],null]},{"Or":[346,347]},{"WithSkip":[25,348]},{"Rep":349},{"Call":[143,[[3,350]]]},{"And":[[351],null]},{"Or":[352]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[357],null]},{"Token":8},{"And":[[359],null]},{"Token":20},{"And":[[361],null]},{"Token":21},{"And":[[363],null]},{"Token":22},{"And":[[365],null]},{"Token":63},{"And":[[367],null]},{"Token":7},{"And":[[369],null]},{"Token":90},{"Token":80},{"And":[[371,372],null]},{"Or":[373]},{"And":[[374],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[379,48],null]},{"Or":[380]},{"Opt":381},{"Token":56},{"And":[[376,377,378,382,383],1]},{"Or":[384]},{"Token":21},{"And":[[386],null]},{"Token":22},{"And":[[388],null]},{"Or":[387,389]},{"Token":90},{"Token":57},{"Token":51},{"And":[[393,71],null]},{"Or":[394]},{"Opt":395},{"Token":56},{"And":[[390,391,392,48,396,397],1]},{"Or":[398]},{"And":[[139],null]},{"Token":56},{"And":[[140,401],null]},{"Or":[400,402]},{"Rep":30},{"Call":[143,[[3,404]]]},{"And":[[10,405],null]},{"Or":[406]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[142,[[2,35]]]},{"Call":[142,[[2,32]]]},{"And":[[410,411],null]},{"Or":[412]},{"Call":[145,[[5,413]]]},{"And":[[414],null]},{"Or":[415]},{"Token":90},{"Opt":33},{"And":[[417,418],1]},{"Or":[419]},{"Token":57},{"Token":71},{"And":[[422],null]},"Eof",{"And":[[424],null]},{"Token":59},{"Not":426},{"Not":427},{"And":[[428],null]},{"Token":37},{"Not":430},{"Not":431},{"And":[[432],null]},{"Or":[423,425,429,433]},{"And":[[34,434],1]},{"Or":[435]},{"Rep":436},{"And":[[421,437],null]},{"Token":85},{"And":[[439],null]},{"And":[[48],null]},{"Or":[440,441]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[446],null]},"Eof",{"And":[[448],null]},{"Token":59},{"Not":450},{"Not":451},{"And":[[452],null]},{"Or":[447,449,453]},{"And":[[445,454],1]},{"Or":[455]},{"Rep":456},{"And":[[444,457],null]},{"Or":[458]},{"Opt":459},{"And":[[443,460],1]},{"Or":[461]},{"Token":10},{"Token":6},{"Call":[144,[[4,464]]]},{"Opt":465},{"And":[[463,466],null]},{"Or":[467]},{"Token":34},{"Token":59},{"And":[[470],null]},"Eof",{"And":[[472],null]},{"Token":37},{"Not":474},{"Not":475},{"And":[[476],null]},{"Or":[471,473,477]},{"And":[[48,33,478],null]},{"Or":[479]},{"Rep":480},{"And":[[469,481],1]},{"Or":[482]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[485],null]},{"Enter":[1,41]},{"And":[[487],null]},{"Token":58},{"And":[[489,44],null]},{"Or":[490]},{"Token":58},{"Opt":492},{"And":[[493,44],null]},{"Or":[494]},{"Token":58},{"And":[[496,44],null]},{"Or":[497]},{"And":[[41,498],null]},{"Or":[499]},{"Token":90},{"And":[[501],null]},{"Token":18},{"And":[[503],null]},{"Token":19},{"And":[[505],null]},{"Or":[502,504,506]},{"And":[[45],null]},{"IsIn":3},{"And":[[509,46],null]},{"Or":[508,510]},{"Opt":511},{"And":[[507,512],null]},{"IsIn":3},{"And":[[514],null]},{"IsIn":1},{"Token":58},{"And":[[516,517],null]},{"Or":[515,518]},{"Token":85},{"Call":[142,[[2,520]]]},{"Call":[142,[[2,48]]]},{"And":[[521,522],null]},{"Or":[523]},{"Call":[145,[[5,524]]]},{"And":[[519,525],null]},{"Or":[526]},{"Call":[142,[[2,48]]]},{"Call":[144,[[4,528]]]},{"Opt":9},{"And":[[529,530],null]},{"Or":[531]},{"Token":5},{"Token":90},{"And":[[533,534],null]},{"Or":[535]},{"And":[[49],null]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[39],null]},{"Or":[545]},{"Token":75},{"Token":85},{"Opt":548},{"Token":27},{"Opt":550},{"And":[[547,549,551,48],null]},{"Or":[552]},{"Token":79},{"And":[[554],null]},{"Or":[555]},{"Token":35},{"Token":36},{"And":[[557,558],null]},{"Or":[559]},{"Opt":54},{"And":[[48,561],null]},{"Or":[562]},{"Call":[144,[[4,563]]]},{"And":[[564],null]},{"Or":[565]},{"Token":59},{"Call":[142,[[2,48]]]},{"And":[[567,568],null]},{"Or":[569]},{"Token":80},{"And":[[571],null]},{"Or":[572]},{"Token":56},{"And":[[574,71],null]},{"Or":[575]},{"Opt":576},{"And":[[48,577],null]},{"Or":[578]},{"Call":[146,[[6,579]]]},{"And":[[580],null]},{"Or":[581]},{"Token":8},{"Call":[142,[[2,58]]]},{"Call":[144,[[4,584]]]},{"And":[[583,585,9],1]},{"Or":[586]},{"Token":57},{"And":[[59,588],null]},{"Or":[589]},{"Opt":590},{"And":[[591,48],null]},{"Or":[592]},{"And":[[60],null]},{"And":[[61],null]},{"And":[[65],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[70],null]},{"Token":79},{"And":[[601],null]},{"Or":[602]},{"And":[[62],null]},{"And":[[63],null]},{"Or":[604,605]},{"Opt":606},{"And":[[40,607],null]},{"Or":[608]},{"Call":[142,[[2,59]]]},{"Token":61},{"Token":59},{"Opt":612},{"And":[[611,613],null]},{"Or":[614]},{"Opt":615},{"And":[[610,616],null]},{"Or":[617]},{"Call":[144,[[4,618]]]},{"And":[[619],null]},{"Or":[620]},{"Call":[142,[[2,64]]]},{"Token":61},{"Token":59},{"Opt":624},{"And":[[623,625],null]},{"Or":[626]},{"Opt":627},{"And":[[622,628],null]},{"Or":[629]},{"Call":[143,[[3,630]]]},{"And":[[631],null]},{"Or":[632]},{"Token":57},{"Not":634},{"And":[[65,635],null]},{"Token":90},{"Token":57},{"And":[[637,638,59],2]},{"Or":[636,639]},{"Token":28},{"Opt":641},{"Token":27},{"Opt":643},{"Token":90},{"And":[[642,644,645],null]},{"Or":[646]},{"And":[[75],null]},{"Or":[648]},{"Token":35},{"Token":36},{"And":[[650,651],null]},{"Or":[652]},{"Opt":69},{"And":[[59,654],null]},{"Or":[655]},{"Call":[144,[[4,656]]]},{"And":[[657],null]},{"Or":[658]},{"Token":59},{"Call":[142,[[2,59]]]},{"And":[[660,661],null]},{"Or":[662]},{"Token":75},{"Token":27},{"Opt":665},{"And":[[664,666,59],null]},{"Or":[667]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":671},{"And":[[669,670,672,108],null]},{"Or":[673]},{"Token":60},{"Token":90},{"And":[[676],null]},{"Token":87},{"And":[[678],null]},{"Or":[677,679]},{"And":[[675,680],null]},{"Or":[681]},{"Call":[146,[[6,71]]]},{"Token":81},{"Token":5},{"And":[[685,48],null]},{"Or":[686]},{"Token":75},{"Token":27},{"Opt":689},{"And":[[688,690],null]},{"Or":[691]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[696],null]},{"Token":67},{"And":[[698],null]},{"Token":69},{"And":[[700],null]},{"Or":[697,699,701]},{"Call":[121,[[1,702]]]},{"Token":71},{"And":[[704],null]},{"Token":73},{"And":[[706],null]},{"Or":[705,707]},{"Call":[121,[[1,708]]]},{"ContextualToken":[43,"<<"]},{"And":[[710],null]},{"ContextualToken":[45,">>"]},{"And":[[712],null]},{"Or":[711,713]},{"Call":[121,[[1,714]]]},{"Token":75},{"Token":75},{"Not":717},{"And":[[716,718],null]},{"Or":[719]},{"Call":[121,[[1,720]]]},{"Token":82},{"Call":[121,[[1,722]]]},{"Token":77},{"Token":77},{"Not":725},{"And":[[724,726],null]},{"Or":[727]},{"Call":[121,[[1,728]]]},{"Call":[121,[[1,126]]]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,731]]]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,733]]]},{"Call":[121,[[1,134]]]},{"Token":51},{"And":[[736],null]},{"Token":72},{"And":[[738],null]},{"Token":74},{"And":[[740],null]},{"Token":66},{"And":[[742],null]},{"Token":68},{"And":[[744],null]},{"Token":70},{"And":[[746],null]},{"Token":76},{"And":[[748],null]},{"Token":78},{"And":[[750],null]},{"Token":83},{"And":[[752],null]},{"ContextualToken":[46,">>="]},{"And":[[754],null]},{"ContextualToken":[44,"<<="]},{"And":[[756],null]},{"Or":[737,739,741,743,745,747,749,751,753,755,757]},{"Call":[121,[[1,758]]]},{"And":[[75],null]},{"Token":90},{"And":[[761],null]},{"Token":18},{"And":[[763],null]},{"Token":19},{"And":[[765],null]},{"Token":39},{"And":[[767],null]},{"Token":58},{"And":[[769],null]},{"Token":35},{"And":[[771],null]},{"Token":41},{"And":[[773],null]},{"Token":77},{"And":[[775],null]},{"Token":31},{"And":[[777],null]},{"Token":37},{"And":[[779],null]},{"Token":14},{"And":[[781],null]},{"Token":25},{"And":[[783],null]},{"Token":24},{"And":[[785],null]},{"Token":23},{"And":[[787],null]},{"Token":30},{"And":[[789],null]},{"Token":75},{"And":[[791],null]},{"Token":65},{"And":[[793],null]},{"Token":73},{"And":[[795],null]},{"Token":80},{"And":[[797],null]},{"Token":61},{"And":[[799],null]},{"Token":62},{"And":[[801],null]},{"PrevIs":[155,161,162,163,164,165,168]},{"And":[[803],null]},{"Var":0},{"Exit":[2,805]},{"Exit":[0,806]},{"And":[[807],null]},{"Token":87},{"And":[[809],null]},{"Token":88},{"And":[[811],null]},{"Token":89},{"And":[[813],null]},{"Token":84},{"And":[[815],null]},{"Token":86},{"And":[[817],null]},{"Or":[810,812,814,816,818]},{"Token":90},{"Token":80},{"And":[[820,821],null]},{"Or":[822]},{"Not":823},{"Opt":77},{"And":[[824,40,825],null]},{"Or":[826]},{"IsIn":0},{"Not":828},{"Call":[142,[[2,78]]]},{"Token":61},{"Call":[74,[[0,71]]]},{"And":[[831,832],null]},{"Or":[833]},{"Opt":834},{"And":[[830,835],null]},{"Or":[836]},{"Call":[143,[[3,837]]]},{"And":[[829,838],null]},{"Or":[839]},{"Token":90},{"Token":57},{"And":[[842,71],null]},{"Or":[843]},{"Opt":844},{"And":[[841,845],1]},{"Or":[846]},{"Token":35},{"Token":36},{"And":[[848,849],null]},{"Or":[850]},{"Call":[74,[[0,71]]]},{"Opt":81},{"And":[[852,853],null]},{"Or":[854]},{"Call":[144,[[4,855]]]},{"And":[[856],null]},{"Or":[857]},{"Token":59},{"Call":[74,[[0,71]]]},{"Call":[142,[[2,860]]]},{"And":[[859,861],null]},{"Or":[862]},{"Call":[142,[[2,71]]]},{"Call":[74,[[0,864]]]},{"Call":[146,[[6,865]]]},{"And":[[866],null]},{"Or":[867]},{"Token":26},{"Opt":869},{"Token":77},{"Rep":84},{"Token":77},{"Token":49},{"And":[[874,48,86],null]},{"Call":[74,[[0,71]]]},{"And":[[876],null]},{"Or":[875,877]},{"And":[[870,871,872,873,878],null]},{"Or":[879]},{"Token":59},{"And":[[881],null]},{"Token":77},{"Not":883},{"Not":884},{"And":[[885],null]},{"Or":[882,886]},{"And":[[13,887],1]},{"Token":31},{"Opt":71},{"And":[[889,890],null]},{"Or":[891]},{"Token":33},{"Opt":893},{"Rep":87},{"Opt":71},{"And":[[895,896],null]},{"Or":[897]},{"Call":[143,[[3,898]]]},{"And":[[894,899],null]},{"Or":[900]},{"Call":[74,[[0,901]]]},{"And":[[902],null]},{"Or":[903]},{"And":[[88],null]},{"And":[[92],null]},{"And":[[91],null]},{"And":[[3],null]},{"Token":9},{"Opt":89},{"Opt":90},{"Token":56},{"And":[[909,59,910,911,912],1]},{"Or":[913]},{"Token":57},{"And":[[915,48],null]},{"Or":[916]},{"Token":51},{"And":[[918,71],null]},{"Or":[919]},{"Token":56},{"And":[[921],null]},{"Or":[922]},"Eof",{"Not":924},{"And":[[73,925],null]},{"Token":56},{"And":[[927],null]},{"Or":[926,928]},{"And":[[71,929],null]},{"Or":[930]},{"Enter":[2,931]},{"And":[[932],null]},{"Or":[933]},{"Token":14},{"Token":15},{"And":[[86],null]},{"And":[[93],null]},{"Or":[937,938]},{"And":[[936,939],null]},{"Or":[940]},{"Opt":941},{"And":[[935,95,86,942],1]},{"Or":[943]},{"Opt":99},{"Token":25},{"And":[[945,946,95,86],2]},{"Or":[947]},{"Token":9},{"Token":51},{"And":[[949,59,950],1]},{"Or":[951]},{"Opt":952},{"And":[[953,96],null]},{"Enter":[0,71]},{"And":[[955],null]},{"Opt":99},{"Token":24},{"And":[[957,958,86],2]},{"Or":[959]},{"Opt":99},{"Token":23},{"Token":32},{"And":[[961,962,59,963,96,86],2]},{"Or":[964]},{"Token":85},{"Token":57},{"And":[[966,967],null]},{"Token":30},{"Rep":101},{"Call":[143,[[3,970]]]},{"And":[[969,96,971],1]},{"Or":[972]},{"Token":50},{"Token":59},{"And":[[975],null]},"Eof",{"And":[[977],null]},{"And":[[73],null]},{"Or":[976,978,979]},{"And":[[102,974,71,980],1]},{"Or":[981]},{"Token":77},{"And":[[983,59],null]},{"Or":[984]},{"Rep":985},{"Opt":103},{"And":[[59,986,987],null]},{"Token":14},{"And":[[989,71],null]},{"Or":[990]},{"And":[[139],null]},{"Or":[992]},{"And":[[140],null]},{"Or":[994]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":998},{"And":[[996,997,999,108],null]},{"Or":[1000]},{"And":[[71,1001],null]},{"Or":[1002]},{"And":[[71,108],null]},{"Or":[1004]},{"Call":[142,[[2,109]]]},{"Call":[144,[[4,1006]]]},{"Call":[74,[[0,1007]]]},{"And":[[1008],null]},{"And":[[71],null]},{"Or":[1010]},{"Token":60},{"Token":90},{"And":[[1013],null]},{"Token":87},{"And":[[1015],null]},{"Or":[1014,1016]},{"And":[[1012,1017],null]},{"Or":[1018]},{"And":[[71,1019],null]},{"Or":[1020]},{"Call":[146,[[6,71]]]},{"And":[[71,1022],null]},{"Or":[1023]},{"Token":81},{"And":[[71,1025],null]},{"Or":[1026]},{"Token":5},{"And":[[1028,48],null]},{"Or":[1029]},{"And":[[71,1030],null]},{"Or":[1031]},{"Token":75},{"Token":27},{"Opt":1034},{"And":[[1033,1035],null]},{"Or":[1036]},{"And":[[1037,71],null]},{"Or":[1038]},{"Token":65},{"And":[[1040,71],null]},{"Or":[1041]},{"Token":73},{"And":[[1043,71],null]},{"Or":[1044]},{"Token":80},{"And":[[1046,71],null]},{"Or":[1047]},{"Token":65},{"And":[[1049],null]},{"Token":67},{"And":[[1051],null]},{"Token":69},{"And":[[1053],null]},{"Or":[1050,1052,1054]},{"Call":[121,[[1,1055]]]},{"And":[[71,1056,71],null]},{"Or":[1057]},{"Token":71},{"And":[[1059],null]},{"Token":73},{"And":[[1061],null]},{"Or":[1060,1062]},{"Call":[121,[[1,1063]]]},{"And":[[71,1064,71],null]},{"Or":[1065]},{"ContextualToken":[43,"<<"]},{"And":[[1067],null]},{"ContextualToken":[45,">>"]},{"And":[[1069],null]},{"Or":[1068,1070]},{"Call":[121,[[1,1071]]]},{"And":[[71,1072,71],null]},{"Or":[1073]},{"IsIn":2},{"Not":73},{"Var":1},{"And":[[1075,1076,1077],null]},{"IsIn":2},{"Not":1079},{"Var":1},{"And":[[1080,1081],null]},{"Token":75},{"Token":75},{"Not":1084},{"And":[[1083,1085],null]},{"Or":[1086]},{"Call":[121,[[1,1087]]]},{"And":[[71,1088,71],null]},{"Or":[1089]},{"Token":82},{"Call":[121,[[1,1091]]]},{"And":[[71,1092,71],null]},{"Or":[1093]},{"Token":77},{"Token":77},{"Not":1096},{"And":[[1095,1097],null]},{"Or":[1098]},{"Call":[121,[[1,1099]]]},{"And":[[71,1100,71],null]},{"Or":[1101]},{"Call":[121,[[1,126]]]},{"And":[[71,1103,71],null]},{"Or":[1104]},{"Token":52},{"And":[[1106],null]},{"Token":53},{"And":[[1108],null]},{"Token":39},{"And":[[1110],null]},{"Token":40},{"And":[[1112],null]},{"Token":55},{"And":[[1114],null]},{"Token":54},{"And":[[1116],null]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,1118]]]},{"And":[[71,1119,71],null]},{"Or":[1120]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,1122]]]},{"And":[[71,1123,71],null]},{"Or":[1124]},{"Call":[121,[[1,134]]]},{"And":[[71,1126,71],null]},{"Or":[1127]},{"And":[[134,71],null]},{"Or":[1129]},{"And":[[71,133],null]},{"Or":[1131]},{"Token":61},{"Not":72},{"And":[[1133,1134],null]},{"Or":[1135]},{"Token":61},{"And":[[1137],null]},{"Token":62},{"And":[[1139],null]},{"Token":37},{"IsIn":0},{"And":[[1141,1142],null]},{"Or":[1143]},{"Not":1144},{"And":[[133,1145],null]},{"Token":51},{"And":[[1147],null]},{"Token":72},{"And":[[1149],null]},{"Token":74},{"And":[[1151],null]},{"Token":66},{"And":[[1153],null]},{"Token":68},{"And":[[1155],null]},{"Token":70},{"And":[[1157],null]},{"Token":76},{"And":[[1159],null]},{"Token":78},{"And":[[1161],null]},{"Token":83},{"And":[[1163],null]},{"ContextualToken":[46,">>="]},{"And":[[1165],null]},{"ContextualToken":[44,"<<="]},{"And":[[1167],null]},{"Or":[1148,1150,1152,1154,1156,1158,1160,1162,1164,1166,1168]},{"Call":[121,[[1,1169]]]},{"And":[[71,1170,71],null]},{"Or":[1171]},{"Token":63},{"Call":[142,[[2,138]]]},{"Call":[146,[[6,1174]]]},{"And":[[1173,1175],null]},{"Or":[1176]},{"Rep":136},{"And":[[1178],null]},{"Token":90},{"Token":51},{"And":[[1181,71],null]},{"Call":[142,[[2,138]]]},{"Call":[144,[[4,1183]]]},{"And":[[1184],null]},{"Or":[1182,1185]},{"Opt":1186},{"And":[[1180,1187],1]},{"Or":[1188]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1192},{"Rep":141},{"Call":[143,[[3,1194]]]},{"And":[[1190,1191,1193,1195],null]},{"Or":[1196]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1200},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1202,1203,1204],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1206,1207,1208],null]},{"Or":[1205,1209]},{"And":[[1198,1199,1201,1210],null]},{"Or":[1211]},{"Token":35},{"And":[[1213],null]},{"Token":36},{"And":[[1215],null]},{"Token":37},{"And":[[1217],null]},{"Token":38},{"And":[[1219],null]},{"Token":41},{"And":[[1221],null]},{"Token":42},{"And":[[1223],null]},{"Or":[1214,1216,1218,1220,1222,1224]},{"Not":1225},"Any",{"And":[[1226,1227],null]},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1229,1230,1231],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1233,1234,1235],null]},{"Token":37},{"Rep":141},{"Token":38},{"And":[[1237,1238,1239],null]},{"Or":[1228,1232,1236,1240]},{"Var":2},"Eof",{"And":[[1243],null]},{"Token":59},{"And":[[1245],null]},{"Or":[1244,1246]},{"And":[[1242,1247],1]},{"Or":[1248]},{"Rep":1249},{"And":[[1250],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[147,[[7,1252],[8,1253],[9,1254]]]},{"And":[[1255],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[147,[[7,1257],[8,1258],[9,1259]]]},{"And":[[1260],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[147,[[7,1262],[8,1263],[9,1264]]]},{"And":[[1265],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[147,[[7,1267],[8,1268],[9,1269]]]},{"And":[[1270],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[148,[[10,1273],[11,1274]]]},{"Var":9},{"Layer":[1275,1276]},{"Var":8},{"And":[[1272,1277,1278],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[148,[[10,1281],[11,1282]]]},{"Var":11},{"And":[[1280,1283,1284],1]},{"Var":11},{"Not":1286},"Any",{"And":[[1287,1288],null]},{"Or":[1289]},{"And":[[1290],null]},{"Or":[1285,1291]},{"Rep":1292},{"And":[[1293],null]}]"##;

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
        static ref LANG: Language = {
            use fall_parse::{ParserDefinition, parse, reparse};
            use std::any::Any;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
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

use self::fall_tree::{AstNode, AstChildren, Node};
use self::fall_tree::search::{child_of_type_exn, child_of_type};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FnDef<'f> { node: Node<'f> }

impl<'f> AstNode<'f> for FnDef<'f> {
    fn wrap(node: Node<'f>) -> Option<Self> {
        if node.ty() == FN_DEF {
            Some(FnDef { node })
        } else {
            None
        }
    }
    fn node(self) -> Node<'f> { self.node }
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