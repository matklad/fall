use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode, Event, TextEdit};
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
        let parser_json = r##"[{"Pub":{"ty":91,"body":150,"replaceable":false}},{"Or":[153]},{"Or":[155,157,159,161,163,165,167,168]},{"Or":[183,184]},{"Pub":{"ty":92,"body":200,"replaceable":false}},{"Pub":{"ty":93,"body":206,"replaceable":false}},{"Pub":{"ty":94,"body":212,"replaceable":false}},{"Pub":{"ty":95,"body":219,"replaceable":false}},{"Pub":{"ty":96,"body":231,"replaceable":false}},{"Or":[233]},{"Pub":{"ty":97,"body":238,"replaceable":false}},{"Or":[244]},{"Pub":{"ty":98,"body":247,"replaceable":false}},{"Pub":{"ty":99,"body":253,"replaceable":false}},{"Pub":{"ty":100,"body":269,"replaceable":false}},{"Pub":{"ty":101,"body":288,"replaceable":false}},{"Pub":{"ty":102,"body":293,"replaceable":false}},{"Pub":{"ty":103,"body":296,"replaceable":false}},{"Pub":{"ty":104,"body":302,"replaceable":false}},{"Pub":{"ty":105,"body":315,"replaceable":false}},{"Pub":{"ty":106,"body":324,"replaceable":false}},{"Pub":{"ty":107,"body":334,"replaceable":false}},{"Pub":{"ty":108,"body":340,"replaceable":false}},{"Pub":{"ty":109,"body":352,"replaceable":false}},{"Or":[353,354,355]},{"Or":[357,359,361,363,365,367,369,374]},{"Pub":{"ty":110,"body":384,"replaceable":false}},{"Pub":{"ty":111,"body":398,"replaceable":false}},{"Pub":{"ty":112,"body":402,"replaceable":false}},{"Pub":{"ty":113,"body":406,"replaceable":false}},{"Or":[407,408]},{"Pub":{"ty":114,"body":415,"replaceable":false}},{"Pub":{"ty":115,"body":419,"replaceable":false}},{"Or":[437]},{"Pub":{"ty":116,"body":441,"replaceable":false}},{"Pub":{"ty":117,"body":461,"replaceable":false}},{"Pub":{"ty":118,"body":464,"replaceable":false}},{"Pub":{"ty":119,"body":479,"replaceable":false}},{"Or":[480]},{"Or":[482]},{"Or":[484]},{"Pratt":{"atoms":[42],"prefixes":[],"infixes":[{"ty":120,"op":487,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":491,"replaceable":false}},{"Pub":{"ty":120,"body":496,"replaceable":false}},{"Or":[509]},{"Pub":{"ty":121,"body":523,"replaceable":false}},{"Pub":{"ty":122,"body":528,"replaceable":false}},{"Pub":{"ty":123,"body":532,"replaceable":false}},{"Or":[533,534,535,536,537,538,539,540]},{"Pub":{"ty":124,"body":542,"replaceable":false}},{"Pub":{"ty":125,"body":549,"replaceable":false}},{"Pub":{"ty":126,"body":552,"replaceable":false}},{"Pub":{"ty":127,"body":556,"replaceable":false}},{"Pub":{"ty":128,"body":562,"replaceable":true}},{"PubReplace":{"ty":129,"body":566}},{"Pub":{"ty":130,"body":569,"replaceable":false}},{"Pub":{"ty":131,"body":578,"replaceable":false}},{"Pub":{"ty":132,"body":583,"replaceable":false}},{"Pub":{"ty":98,"body":589,"replaceable":false}},{"Or":[590,591,592,593,594,595,596]},{"Pub":{"ty":133,"body":599,"replaceable":false}},{"Pub":{"ty":134,"body":605,"replaceable":true}},{"PubReplace":{"ty":135,"body":617}},{"PubReplace":{"ty":136,"body":629}},{"Pub":{"ty":137,"body":636,"replaceable":false}},{"Pub":{"ty":138,"body":643,"replaceable":false}},{"Pub":{"ty":139,"body":645,"replaceable":false}},{"Pub":{"ty":140,"body":649,"replaceable":false}},{"Pub":{"ty":141,"body":655,"replaceable":true}},{"PubReplace":{"ty":142,"body":659}},{"Pub":{"ty":143,"body":664,"replaceable":false}},{"Pratt":{"atoms":[75,76,79,80,82,83,85,86,93,94,97,98,100,104,105,132],"prefixes":[{"ty":177,"op":688,"priority":999},{"ty":178,"op":689,"priority":999},{"ty":179,"op":690,"priority":999},{"ty":180,"op":691,"priority":999},{"ty":190,"op":134,"priority":2}],"infixes":[{"ty":170,"op":670,"priority":999,"has_rhs":false},{"ty":171,"op":108,"priority":999,"has_rhs":false},{"ty":173,"op":678,"priority":999,"has_rhs":false},{"ty":174,"op":679,"priority":999,"has_rhs":false},{"ty":175,"op":680,"priority":999,"has_rhs":false},{"ty":176,"op":683,"priority":999,"has_rhs":false},{"ty":181,"op":699,"priority":11,"has_rhs":true},{"ty":182,"op":705,"priority":10,"has_rhs":true},{"ty":183,"op":711,"priority":9,"has_rhs":true},{"ty":184,"op":717,"priority":8,"has_rhs":true},{"ty":185,"op":719,"priority":7,"has_rhs":true},{"ty":186,"op":725,"priority":6,"has_rhs":true},{"ty":187,"op":726,"priority":5,"has_rhs":true},{"ty":188,"op":728,"priority":4,"has_rhs":true},{"ty":189,"op":730,"priority":3,"has_rhs":true},{"ty":190,"op":731,"priority":2,"has_rhs":true},{"ty":190,"op":133,"priority":2,"has_rhs":false},{"ty":191,"op":755,"priority":1,"has_rhs":true}]}},{"Or":[756,758,760,762,764,766,768,770,772,774,776,778,780,782,784,786,788,790,792,794,796,798]},{"Or":[800]},{"Or":[804]},{"Pub":{"ty":145,"body":815,"replaceable":false}},{"Pub":{"ty":146,"body":823,"replaceable":true}},{"PubReplace":{"ty":147,"body":836}},{"Pub":{"ty":148,"body":843,"replaceable":false}},{"Pub":{"ty":149,"body":847,"replaceable":false}},{"Pub":{"ty":150,"body":854,"replaceable":true}},{"PubReplace":{"ty":151,"body":859}},{"Pub":{"ty":152,"body":864,"replaceable":false}},{"Pub":{"ty":153,"body":876,"replaceable":false}},{"Or":[884]},{"Pub":{"ty":154,"body":888,"replaceable":false}},{"Pub":{"ty":155,"body":900,"replaceable":false}},{"Or":[901,902,903,904]},{"Pub":{"ty":156,"body":910,"replaceable":false}},{"Pub":{"ty":157,"body":913,"replaceable":false}},{"Pub":{"ty":158,"body":916,"replaceable":false}},{"Pub":{"ty":159,"body":919,"replaceable":false}},{"Pub":{"ty":160,"body":930,"replaceable":false}},{"Pub":{"ty":161,"body":940,"replaceable":false}},{"Pub":{"ty":162,"body":944,"replaceable":false}},{"Or":[950]},{"Or":[952]},{"Pub":{"ty":163,"body":956,"replaceable":false}},{"Pub":{"ty":164,"body":961,"replaceable":false}},{"Or":[964]},{"Pub":{"ty":165,"body":969,"replaceable":false}},{"Pub":{"ty":166,"body":978,"replaceable":false}},{"Or":[984]},{"Pub":{"ty":167,"body":987,"replaceable":false}},{"Pub":{"ty":168,"body":989,"replaceable":false}},{"Pub":{"ty":169,"body":991,"replaceable":false}},{"Pub":{"ty":170,"body":999,"replaceable":false}},{"Pub":{"ty":171,"body":1001,"replaceable":false}},{"Or":[1005]},{"Pub":{"ty":172,"body":1007,"replaceable":false}},{"Pub":{"ty":173,"body":1017,"replaceable":false}},{"Pub":{"ty":174,"body":1020,"replaceable":false}},{"Pub":{"ty":175,"body":1023,"replaceable":false}},{"Pub":{"ty":176,"body":1028,"replaceable":false}},{"Pub":{"ty":177,"body":1035,"replaceable":false}},{"Pub":{"ty":178,"body":1038,"replaceable":false}},{"Pub":{"ty":179,"body":1041,"replaceable":false}},{"Pub":{"ty":180,"body":1044,"replaceable":false}},{"Pub":{"ty":181,"body":1054,"replaceable":false}},{"Pub":{"ty":182,"body":1062,"replaceable":false}},{"Pub":{"ty":183,"body":1070,"replaceable":false}},{"Or":[1074,1078]},{"Pub":{"ty":184,"body":1086,"replaceable":false}},{"Pub":{"ty":185,"body":1090,"replaceable":false}},{"Pub":{"ty":186,"body":1098,"replaceable":false}},{"Pub":{"ty":187,"body":1101,"replaceable":false}},{"Or":[1103,1105,1107,1109,1111,1113]},{"Pub":{"ty":188,"body":1117,"replaceable":false}},{"Pub":{"ty":189,"body":1121,"replaceable":false}},{"Pub":{"ty":190,"body":1124,"replaceable":false}},{"Pub":{"ty":190,"body":1126,"replaceable":false}},{"Pub":{"ty":190,"body":1128,"replaceable":false}},{"Pub":{"ty":190,"body":1132,"replaceable":false}},{"Or":[1134,1136]},{"Or":[1142]},{"Pub":{"ty":191,"body":1168,"replaceable":false}},{"Pub":{"ty":192,"body":1173,"replaceable":false}},{"Or":[1175]},{"Pub":{"ty":193,"body":1185,"replaceable":false}},{"Pub":{"ty":194,"body":1193,"replaceable":false}},{"Pub":{"ty":195,"body":1208,"replaceable":false}},{"Pub":{"ty":196,"body":1237,"replaceable":false}},{"Or":[1247]},{"Or":[1252]},{"Or":[1257]},{"Or":[1262]},{"Or":[1267]},{"Or":[1275]},{"Or":[1290]},{"And":[[1],null]},{"Or":[149]},{"WithSkip":[2,3]},{"Rep":151},{"And":[[152],null]},{"Token":11},{"And":[[154],null]},{"ContextualToken":[4,"union"]},{"And":[[156],null]},{"Token":16},{"And":[[158],null]},{"Token":12},{"And":[[160],null]},{"Token":13},{"And":[[162],null]},{"Token":17},{"And":[[164],null]},{"Token":29},{"And":[[166],null]},{"And":[[25],null]},{"Opt":36},{"And":[[137,169],null]},{"Or":[170]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[172,173,174,175,176,177,178,179,180]},{"Inject":[171,181]},{"And":[[182],null]},{"And":[[28],null]},{"Token":12},{"And":[[47],null]},{"Token":58},{"And":[[187,5],null]},{"Or":[188]},{"Opt":189},{"And":[[190],null]},{"Or":[186,191]},{"And":[[38,192],null]},{"Token":58},{"Opt":194},{"And":[[195,5],null]},{"Or":[193,196]},{"Token":56},{"And":[[185,197,198],1]},{"Or":[199]},{"Token":65},{"And":[[201],null]},{"Call":[142,[[2,6]]]},{"Call":[143,[[3,203]]]},{"And":[[204],null]},{"Or":[202,205]},{"Token":18},{"And":[[207],null]},{"Token":90},{"Opt":47},{"And":[[209,210],1]},{"Or":[208,211]},{"Token":7},{"Token":6},{"Token":90},{"Opt":47},{"Token":56},{"And":[[213,214,215,216,217],2]},{"Or":[218]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[86],null]},{"Token":56},{"And":[[227],null]},{"Or":[226,228]},{"And":[[220,221,222,223,11,224,225,229],2]},{"Or":[230]},{"Token":49},{"And":[[232,48],null]},{"Token":7},{"Token":88},{"Opt":235},{"And":[[234,236],null]},{"Or":[237]},{"Opt":14},{"Call":[142,[[2,12]]]},{"And":[[239,240],null]},{"Or":[241]},{"Call":[144,[[4,242]]]},{"And":[[243],null]},{"Token":57},{"And":[[59,245,48],1]},{"Or":[246]},{"Token":57},{"And":[[248,48],null]},{"Or":[249]},{"Opt":250},{"And":[[59,251],null]},{"Or":[252]},{"Token":75},{"Opt":254},{"Token":27},{"Opt":256},{"Token":18},{"Token":57},{"And":[[259,48],null]},{"Or":[260]},{"Opt":261},{"Token":59},{"And":[[263],null]},"Eof",{"And":[[265],null]},{"Or":[264,266]},{"And":[[255,257,258,262,267],3]},{"Or":[268]},{"Token":11},{"And":[[270],null]},{"ContextualToken":[4,"union"]},{"And":[[272],null]},{"Or":[271,273]},{"Token":90},{"Opt":31},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,277]]]},{"And":[[278],null]},{"Token":56},{"And":[[280],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,282]]]},{"Token":56},{"And":[[283,284],null]},{"Or":[279,281,285]},{"And":[[274,275,276,286],1]},{"Or":[287]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[289,290,291,48],2]},{"Or":[292]},{"Opt":36},{"And":[[294,48],null]},{"Or":[295]},{"Token":16},{"Token":90},{"Call":[142,[[2,19]]]},{"Call":[143,[[3,299]]]},{"And":[[297,298,300],1]},{"Or":[301]},{"Token":90},{"Token":51},{"And":[[304,71],null]},{"Call":[142,[[2,17]]]},{"Call":[144,[[4,306]]]},{"And":[[307],null]},{"Call":[142,[[2,16]]]},{"Call":[143,[[3,309]]]},{"And":[[310],null]},{"Or":[305,308,311]},{"Opt":312},{"And":[[303,313],1]},{"Or":[314]},{"Token":13},{"Token":90},{"Token":56},{"And":[[318],null]},{"Call":[143,[[3,1]]]},{"And":[[320],null]},{"Or":[319,321]},{"And":[[316,317,322],1]},{"Or":[323]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[327,48],null]},{"Or":[328]},{"Opt":329},{"And":[[48,330],null]},{"Or":[331]},{"And":[[325,326,332,23],1]},{"Or":[333]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"And":[[137,335,336,337,338,23],3]},{"Or":[339]},{"Opt":36},{"And":[[137,341],null]},{"Or":[342]},{"Inject":[343,24]},{"And":[[344],null]},{"And":[[28],null]},{"Or":[345,346]},{"WithSkip":[25,347]},{"Rep":348},{"Call":[143,[[3,349]]]},{"And":[[350],null]},{"Or":[351]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[356],null]},{"Token":8},{"And":[[358],null]},{"Token":20},{"And":[[360],null]},{"Token":21},{"And":[[362],null]},{"Token":22},{"And":[[364],null]},{"Token":63},{"And":[[366],null]},{"Token":7},{"And":[[368],null]},{"Token":90},{"Token":80},{"And":[[370,371],null]},{"Or":[372]},{"And":[[373],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[378,48],null]},{"Or":[379]},{"Opt":380},{"Token":56},{"And":[[375,376,377,381,382],1]},{"Or":[383]},{"Token":21},{"And":[[385],null]},{"Token":22},{"And":[[387],null]},{"Or":[386,388]},{"Token":90},{"Token":57},{"Token":51},{"And":[[392,71],null]},{"Or":[393]},{"Opt":394},{"Token":56},{"And":[[389,390,391,48,395,396],1]},{"Or":[397]},{"And":[[139],null]},{"Token":56},{"And":[[140,400],null]},{"Or":[399,401]},{"Rep":30},{"Call":[143,[[3,403]]]},{"And":[[10,404],null]},{"Or":[405]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[142,[[2,35]]]},{"Call":[142,[[2,32]]]},{"And":[[409,410],null]},{"Or":[411]},{"Call":[145,[[5,412]]]},{"And":[[413],null]},{"Or":[414]},{"Token":90},{"Opt":33},{"And":[[416,417],1]},{"Or":[418]},{"Token":57},{"Token":71},{"And":[[421],null]},"Eof",{"And":[[423],null]},{"Token":59},{"Not":425},{"Not":426},{"And":[[427],null]},{"Token":37},{"Not":429},{"Not":430},{"And":[[431],null]},{"Or":[422,424,428,432]},{"And":[[34,433],1]},{"Or":[434]},{"Rep":435},{"And":[[420,436],null]},{"Token":85},{"And":[[438],null]},{"And":[[48],null]},{"Or":[439,440]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[445],null]},"Eof",{"And":[[447],null]},{"Token":59},{"Not":449},{"Not":450},{"And":[[451],null]},{"Or":[446,448,452]},{"And":[[444,453],1]},{"Or":[454]},{"Rep":455},{"And":[[443,456],null]},{"Or":[457]},{"Opt":458},{"And":[[442,459],1]},{"Or":[460]},{"Token":10},{"And":[[462],null]},{"Or":[463]},{"Token":34},{"Token":59},{"And":[[466],null]},"Eof",{"And":[[468],null]},{"Token":37},{"Not":470},{"Not":471},{"And":[[472],null]},{"Or":[467,469,473]},{"And":[[48,33,474],null]},{"Or":[475]},{"Rep":476},{"And":[[465,477],1]},{"Or":[478]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[481],null]},{"Enter":[1,41]},{"And":[[483],null]},{"Token":58},{"And":[[485,44],null]},{"Or":[486]},{"Token":58},{"Opt":488},{"And":[[489,44],null]},{"Or":[490]},{"Token":58},{"And":[[492,44],null]},{"Or":[493]},{"And":[[41,494],null]},{"Or":[495]},{"Token":90},{"And":[[497],null]},{"Token":18},{"And":[[499],null]},{"Token":19},{"And":[[501],null]},{"Or":[498,500,502]},{"And":[[45],null]},{"IsIn":3},{"And":[[505,46],null]},{"Or":[504,506]},{"Opt":507},{"And":[[503,508],null]},{"IsIn":3},{"And":[[510],null]},{"IsIn":1},{"Token":58},{"And":[[512,513],null]},{"Or":[511,514]},{"Token":85},{"Call":[142,[[2,516]]]},{"Call":[142,[[2,48]]]},{"And":[[517,518],null]},{"Or":[519]},{"Call":[145,[[5,520]]]},{"And":[[515,521],null]},{"Or":[522]},{"Call":[142,[[2,48]]]},{"Call":[144,[[4,524]]]},{"Opt":9},{"And":[[525,526],null]},{"Or":[527]},{"Token":5},{"Token":90},{"And":[[529,530],null]},{"Or":[531]},{"And":[[49],null]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[55],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[39],null]},{"Or":[541]},{"Token":75},{"Token":85},{"Opt":544},{"Token":27},{"Opt":546},{"And":[[543,545,547,48],null]},{"Or":[548]},{"Token":79},{"And":[[550],null]},{"Or":[551]},{"Token":35},{"Token":36},{"And":[[553,554],null]},{"Or":[555]},{"Opt":54},{"And":[[48,557],null]},{"Or":[558]},{"Call":[144,[[4,559]]]},{"And":[[560],null]},{"Or":[561]},{"Token":59},{"Call":[142,[[2,48]]]},{"And":[[563,564],null]},{"Or":[565]},{"Token":80},{"And":[[567],null]},{"Or":[568]},{"Token":56},{"And":[[570,71],null]},{"Or":[571]},{"Opt":572},{"And":[[48,573],null]},{"Or":[574]},{"Call":[146,[[6,575]]]},{"And":[[576],null]},{"Or":[577]},{"Token":8},{"Call":[142,[[2,58]]]},{"Call":[144,[[4,580]]]},{"And":[[579,581,9],1]},{"Or":[582]},{"Token":57},{"And":[[59,584],null]},{"Or":[585]},{"Opt":586},{"And":[[587,48],null]},{"Or":[588]},{"And":[[60],null]},{"And":[[61],null]},{"And":[[65],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[70],null]},{"Token":79},{"And":[[597],null]},{"Or":[598]},{"And":[[62],null]},{"And":[[63],null]},{"Or":[600,601]},{"Opt":602},{"And":[[40,603],null]},{"Or":[604]},{"Call":[142,[[2,59]]]},{"Token":61},{"Token":59},{"Opt":608},{"And":[[607,609],null]},{"Or":[610]},{"Opt":611},{"And":[[606,612],null]},{"Or":[613]},{"Call":[144,[[4,614]]]},{"And":[[615],null]},{"Or":[616]},{"Call":[142,[[2,64]]]},{"Token":61},{"Token":59},{"Opt":620},{"And":[[619,621],null]},{"Or":[622]},{"Opt":623},{"And":[[618,624],null]},{"Or":[625]},{"Call":[143,[[3,626]]]},{"And":[[627],null]},{"Or":[628]},{"Token":57},{"Not":630},{"And":[[65,631],null]},{"Token":90},{"Token":57},{"And":[[633,634,59],2]},{"Or":[632,635]},{"Token":28},{"Opt":637},{"Token":27},{"Opt":639},{"Token":90},{"And":[[638,640,641],null]},{"Or":[642]},{"And":[[75],null]},{"Or":[644]},{"Token":35},{"Token":36},{"And":[[646,647],null]},{"Or":[648]},{"Opt":69},{"And":[[59,650],null]},{"Or":[651]},{"Call":[144,[[4,652]]]},{"And":[[653],null]},{"Or":[654]},{"Token":59},{"Call":[142,[[2,59]]]},{"And":[[656,657],null]},{"Or":[658]},{"Token":75},{"Token":27},{"Opt":661},{"And":[[660,662,59],null]},{"Or":[663]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":667},{"And":[[665,666,668,108],null]},{"Or":[669]},{"Token":60},{"Token":90},{"And":[[672],null]},{"Token":87},{"And":[[674],null]},{"Or":[673,675]},{"And":[[671,676],null]},{"Or":[677]},{"Call":[146,[[6,71]]]},{"Token":81},{"Token":5},{"And":[[681,48],null]},{"Or":[682]},{"Token":75},{"Token":27},{"Opt":685},{"And":[[684,686],null]},{"Or":[687]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[692],null]},{"Token":67},{"And":[[694],null]},{"Token":69},{"And":[[696],null]},{"Or":[693,695,697]},{"Call":[121,[[1,698]]]},{"Token":71},{"And":[[700],null]},{"Token":73},{"And":[[702],null]},{"Or":[701,703]},{"Call":[121,[[1,704]]]},{"ContextualToken":[43,"<<"]},{"And":[[706],null]},{"ContextualToken":[45,">>"]},{"And":[[708],null]},{"Or":[707,709]},{"Call":[121,[[1,710]]]},{"Token":75},{"Token":75},{"Not":713},{"And":[[712,714],null]},{"Or":[715]},{"Call":[121,[[1,716]]]},{"Token":82},{"Call":[121,[[1,718]]]},{"Token":77},{"Token":77},{"Not":721},{"And":[[720,722],null]},{"Or":[723]},{"Call":[121,[[1,724]]]},{"Call":[121,[[1,126]]]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,727]]]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,729]]]},{"Call":[121,[[1,134]]]},{"Token":51},{"And":[[732],null]},{"Token":72},{"And":[[734],null]},{"Token":74},{"And":[[736],null]},{"Token":66},{"And":[[738],null]},{"Token":68},{"And":[[740],null]},{"Token":70},{"And":[[742],null]},{"Token":76},{"And":[[744],null]},{"Token":78},{"And":[[746],null]},{"Token":83},{"And":[[748],null]},{"ContextualToken":[46,">>="]},{"And":[[750],null]},{"ContextualToken":[44,"<<="]},{"And":[[752],null]},{"Or":[733,735,737,739,741,743,745,747,749,751,753]},{"Call":[121,[[1,754]]]},{"And":[[75],null]},{"Token":90},{"And":[[757],null]},{"Token":18},{"And":[[759],null]},{"Token":19},{"And":[[761],null]},{"Token":39},{"And":[[763],null]},{"Token":58},{"And":[[765],null]},{"Token":35},{"And":[[767],null]},{"Token":41},{"And":[[769],null]},{"Token":77},{"And":[[771],null]},{"Token":31},{"And":[[773],null]},{"Token":37},{"And":[[775],null]},{"Token":14},{"And":[[777],null]},{"Token":25},{"And":[[779],null]},{"Token":24},{"And":[[781],null]},{"Token":23},{"And":[[783],null]},{"Token":30},{"And":[[785],null]},{"Token":75},{"And":[[787],null]},{"Token":65},{"And":[[789],null]},{"Token":73},{"And":[[791],null]},{"Token":80},{"And":[[793],null]},{"Token":61},{"And":[[795],null]},{"Token":62},{"And":[[797],null]},{"PrevIs":[155,161,162,163,164,165,168]},{"And":[[799],null]},{"Var":0},{"Exit":[2,801]},{"Exit":[0,802]},{"And":[[803],null]},{"Token":87},{"And":[[805],null]},{"Token":88},{"And":[[807],null]},{"Token":89},{"And":[[809],null]},{"Token":84},{"And":[[811],null]},{"Token":86},{"And":[[813],null]},{"Or":[806,808,810,812,814]},{"Token":90},{"Token":80},{"And":[[816,817],null]},{"Or":[818]},{"Not":819},{"Opt":77},{"And":[[820,40,821],null]},{"Or":[822]},{"IsIn":0},{"Not":824},{"Call":[142,[[2,78]]]},{"Token":61},{"Call":[74,[[0,71]]]},{"And":[[827,828],null]},{"Or":[829]},{"Opt":830},{"And":[[826,831],null]},{"Or":[832]},{"Call":[143,[[3,833]]]},{"And":[[825,834],null]},{"Or":[835]},{"Token":90},{"Token":57},{"And":[[838,71],null]},{"Or":[839]},{"Opt":840},{"And":[[837,841],1]},{"Or":[842]},{"Token":35},{"Token":36},{"And":[[844,845],null]},{"Or":[846]},{"Call":[74,[[0,71]]]},{"Opt":81},{"And":[[848,849],null]},{"Or":[850]},{"Call":[144,[[4,851]]]},{"And":[[852],null]},{"Or":[853]},{"Token":59},{"Call":[74,[[0,71]]]},{"Call":[142,[[2,856]]]},{"And":[[855,857],null]},{"Or":[858]},{"Call":[142,[[2,71]]]},{"Call":[74,[[0,860]]]},{"Call":[146,[[6,861]]]},{"And":[[862],null]},{"Or":[863]},{"Token":26},{"Opt":865},{"Token":77},{"Rep":84},{"Token":77},{"Token":49},{"And":[[870,48,86],null]},{"Call":[74,[[0,71]]]},{"And":[[872],null]},{"Or":[871,873]},{"And":[[866,867,868,869,874],null]},{"Or":[875]},{"Token":59},{"And":[[877],null]},{"Token":77},{"Not":879},{"Not":880},{"And":[[881],null]},{"Or":[878,882]},{"And":[[13,883],1]},{"Token":31},{"Opt":71},{"And":[[885,886],null]},{"Or":[887]},{"Token":33},{"Opt":889},{"Rep":87},{"Opt":71},{"And":[[891,892],null]},{"Or":[893]},{"Call":[143,[[3,894]]]},{"And":[[890,895],null]},{"Or":[896]},{"Call":[74,[[0,897]]]},{"And":[[898],null]},{"Or":[899]},{"And":[[88],null]},{"And":[[92],null]},{"And":[[91],null]},{"And":[[3],null]},{"Token":9},{"Opt":89},{"Opt":90},{"Token":56},{"And":[[905,59,906,907,908],1]},{"Or":[909]},{"Token":57},{"And":[[911,48],null]},{"Or":[912]},{"Token":51},{"And":[[914,71],null]},{"Or":[915]},{"Token":56},{"And":[[917],null]},{"Or":[918]},"Eof",{"Not":920},{"And":[[73,921],null]},{"Token":56},{"And":[[923],null]},{"Or":[922,924]},{"And":[[71,925],null]},{"Or":[926]},{"Enter":[2,927]},{"And":[[928],null]},{"Or":[929]},{"Token":14},{"Token":15},{"And":[[86],null]},{"And":[[93],null]},{"Or":[933,934]},{"And":[[932,935],null]},{"Or":[936]},{"Opt":937},{"And":[[931,95,86,938],1]},{"Or":[939]},{"Opt":99},{"Token":25},{"And":[[941,942,95,86],2]},{"Or":[943]},{"Token":9},{"Token":51},{"And":[[945,59,946],1]},{"Or":[947]},{"Opt":948},{"And":[[949,96],null]},{"Enter":[0,71]},{"And":[[951],null]},{"Opt":99},{"Token":24},{"And":[[953,954,86],2]},{"Or":[955]},{"Opt":99},{"Token":23},{"Token":32},{"And":[[957,958,59,959,96,86],2]},{"Or":[960]},{"Token":85},{"Token":57},{"And":[[962,963],null]},{"Token":30},{"Rep":101},{"Call":[143,[[3,966]]]},{"And":[[965,96,967],1]},{"Or":[968]},{"Token":50},{"Token":59},{"And":[[971],null]},"Eof",{"And":[[973],null]},{"And":[[73],null]},{"Or":[972,974,975]},{"And":[[102,970,71,976],1]},{"Or":[977]},{"Token":77},{"And":[[979,59],null]},{"Or":[980]},{"Rep":981},{"Opt":103},{"And":[[59,982,983],null]},{"Token":14},{"And":[[985,71],null]},{"Or":[986]},{"And":[[139],null]},{"Or":[988]},{"And":[[140],null]},{"Or":[990]},{"Token":60},{"Token":90},{"Enter":[1,45]},{"Opt":994},{"And":[[992,993,995,108],null]},{"Or":[996]},{"And":[[71,997],null]},{"Or":[998]},{"And":[[71,108],null]},{"Or":[1000]},{"Call":[142,[[2,109]]]},{"Call":[144,[[4,1002]]]},{"Call":[74,[[0,1003]]]},{"And":[[1004],null]},{"And":[[71],null]},{"Or":[1006]},{"Token":60},{"Token":90},{"And":[[1009],null]},{"Token":87},{"And":[[1011],null]},{"Or":[1010,1012]},{"And":[[1008,1013],null]},{"Or":[1014]},{"And":[[71,1015],null]},{"Or":[1016]},{"Call":[146,[[6,71]]]},{"And":[[71,1018],null]},{"Or":[1019]},{"Token":81},{"And":[[71,1021],null]},{"Or":[1022]},{"Token":5},{"And":[[1024,48],null]},{"Or":[1025]},{"And":[[71,1026],null]},{"Or":[1027]},{"Token":75},{"Token":27},{"Opt":1030},{"And":[[1029,1031],null]},{"Or":[1032]},{"And":[[1033,71],null]},{"Or":[1034]},{"Token":65},{"And":[[1036,71],null]},{"Or":[1037]},{"Token":73},{"And":[[1039,71],null]},{"Or":[1040]},{"Token":80},{"And":[[1042,71],null]},{"Or":[1043]},{"Token":65},{"And":[[1045],null]},{"Token":67},{"And":[[1047],null]},{"Token":69},{"And":[[1049],null]},{"Or":[1046,1048,1050]},{"Call":[121,[[1,1051]]]},{"And":[[71,1052,71],null]},{"Or":[1053]},{"Token":71},{"And":[[1055],null]},{"Token":73},{"And":[[1057],null]},{"Or":[1056,1058]},{"Call":[121,[[1,1059]]]},{"And":[[71,1060,71],null]},{"Or":[1061]},{"ContextualToken":[43,"<<"]},{"And":[[1063],null]},{"ContextualToken":[45,">>"]},{"And":[[1065],null]},{"Or":[1064,1066]},{"Call":[121,[[1,1067]]]},{"And":[[71,1068,71],null]},{"Or":[1069]},{"IsIn":2},{"Not":73},{"Var":1},{"And":[[1071,1072,1073],null]},{"IsIn":2},{"Not":1075},{"Var":1},{"And":[[1076,1077],null]},{"Token":75},{"Token":75},{"Not":1080},{"And":[[1079,1081],null]},{"Or":[1082]},{"Call":[121,[[1,1083]]]},{"And":[[71,1084,71],null]},{"Or":[1085]},{"Token":82},{"Call":[121,[[1,1087]]]},{"And":[[71,1088,71],null]},{"Or":[1089]},{"Token":77},{"Token":77},{"Not":1092},{"And":[[1091,1093],null]},{"Or":[1094]},{"Call":[121,[[1,1095]]]},{"And":[[71,1096,71],null]},{"Or":[1097]},{"Call":[121,[[1,126]]]},{"And":[[71,1099,71],null]},{"Or":[1100]},{"Token":52},{"And":[[1102],null]},{"Token":53},{"And":[[1104],null]},{"Token":39},{"And":[[1106],null]},{"Token":40},{"And":[[1108],null]},{"Token":55},{"And":[[1110],null]},{"Token":54},{"And":[[1112],null]},{"ContextualToken":[47,"&&"]},{"Call":[121,[[1,1114]]]},{"And":[[71,1115,71],null]},{"Or":[1116]},{"ContextualToken":[48,"||"]},{"Call":[121,[[1,1118]]]},{"And":[[71,1119,71],null]},{"Or":[1120]},{"Call":[121,[[1,134]]]},{"And":[[71,1122,71],null]},{"Or":[1123]},{"And":[[134,71],null]},{"Or":[1125]},{"And":[[71,133],null]},{"Or":[1127]},{"Token":61},{"Not":72},{"And":[[1129,1130],null]},{"Or":[1131]},{"Token":61},{"And":[[1133],null]},{"Token":62},{"And":[[1135],null]},{"Token":37},{"IsIn":0},{"And":[[1137,1138],null]},{"Or":[1139]},{"Not":1140},{"And":[[133,1141],null]},{"Token":51},{"And":[[1143],null]},{"Token":72},{"And":[[1145],null]},{"Token":74},{"And":[[1147],null]},{"Token":66},{"And":[[1149],null]},{"Token":68},{"And":[[1151],null]},{"Token":70},{"And":[[1153],null]},{"Token":76},{"And":[[1155],null]},{"Token":78},{"And":[[1157],null]},{"Token":83},{"And":[[1159],null]},{"ContextualToken":[46,">>="]},{"And":[[1161],null]},{"ContextualToken":[44,"<<="]},{"And":[[1163],null]},{"Or":[1144,1146,1148,1150,1152,1154,1156,1158,1160,1162,1164]},{"Call":[121,[[1,1165]]]},{"And":[[71,1166,71],null]},{"Or":[1167]},{"Token":63},{"Call":[142,[[2,138]]]},{"Call":[146,[[6,1170]]]},{"And":[[1169,1171],null]},{"Or":[1172]},{"Rep":136},{"And":[[1174],null]},{"Token":90},{"Token":51},{"And":[[1177,71],null]},{"Call":[142,[[2,138]]]},{"Call":[144,[[4,1179]]]},{"And":[[1180],null]},{"Or":[1178,1181]},{"Opt":1182},{"And":[[1176,1183],1]},{"Or":[1184]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1188},{"Rep":141},{"Call":[143,[[3,1190]]]},{"And":[[1186,1187,1189,1191],null]},{"Or":[1192]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1196},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1198,1199,1200],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1202,1203,1204],null]},{"Or":[1201,1205]},{"And":[[1194,1195,1197,1206],null]},{"Or":[1207]},{"Token":35},{"And":[[1209],null]},{"Token":36},{"And":[[1211],null]},{"Token":37},{"And":[[1213],null]},{"Token":38},{"And":[[1215],null]},{"Token":41},{"And":[[1217],null]},{"Token":42},{"And":[[1219],null]},{"Or":[1210,1212,1214,1216,1218,1220]},{"Not":1221},"Any",{"And":[[1222,1223],null]},{"Token":35},{"Rep":141},{"Token":36},{"And":[[1225,1226,1227],null]},{"Token":41},{"Rep":141},{"Token":42},{"And":[[1229,1230,1231],null]},{"Token":37},{"Rep":141},{"Token":38},{"And":[[1233,1234,1235],null]},{"Or":[1224,1228,1232,1236]},{"Var":2},"Eof",{"And":[[1239],null]},{"Token":59},{"And":[[1241],null]},{"Or":[1240,1242]},{"And":[[1238,1243],1]},{"Or":[1244]},{"Rep":1245},{"And":[[1246],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[147,[[7,1248],[8,1249],[9,1250]]]},{"And":[[1251],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[147,[[7,1253],[8,1254],[9,1255]]]},{"And":[[1256],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[147,[[7,1258],[8,1259],[9,1260]]]},{"And":[[1261],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[147,[[7,1263],[8,1264],[9,1265]]]},{"And":[[1266],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[148,[[10,1269],[11,1270]]]},{"Var":9},{"Layer":[1271,1272]},{"Var":8},{"And":[[1268,1273,1274],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[148,[[10,1277],[11,1278]]]},{"Var":11},{"And":[[1276,1279,1280],1]},{"Var":11},{"Not":1282},"Any",{"And":[[1283,1284],null]},{"Or":[1285]},{"And":[[1286],null]},{"Or":[1281,1287]},{"Rep":1288},{"And":[[1289],null]}]"##;

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
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn lexer(&self) -> &self::fall_tree::Lexer {
                    &self.lexer
                }

                fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> (Vec<Event>, INode) {
                    self.parser_definition.parse(text, tokens, &LANG, metrics)
                }

                fn reparse(&self, old_tokens: &[IToken], old_events: &[Event], edit: &TextEdit, text: Text, tokens: &[IToken], metrics: &Metrics) -> (Vec<Event>, INode) {
                    self.parser_definition.reparse(old_tokens, old_events, edit, text, tokens, &LANG, metrics)
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

