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
            ::fall_parse::LexRule::new(STRING, "\"([^\"]|\\\\\")*\"", None),
            ::fall_parse::LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
            ::fall_parse::LexRule::new(IDENT, "(\\p{XID_Start}|_)\\p{XID_Continue}*", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":91,"body":151,"replaceable":false}},{"Or":[154]},{"Or":[156,158,160,162,164,166,168,169]},{"Cached":186},{"Pub":{"ty":92,"body":202,"replaceable":false}},{"Pub":{"ty":93,"body":208,"replaceable":false}},{"Pub":{"ty":94,"body":214,"replaceable":false}},{"Pub":{"ty":95,"body":221,"replaceable":false}},{"Pub":{"ty":96,"body":233,"replaceable":false}},{"Or":[235]},{"Pub":{"ty":97,"body":240,"replaceable":false}},{"Or":[246]},{"Pub":{"ty":98,"body":249,"replaceable":false}},{"Pub":{"ty":99,"body":255,"replaceable":false}},{"Pub":{"ty":100,"body":271,"replaceable":false}},{"Pub":{"ty":101,"body":290,"replaceable":false}},{"Pub":{"ty":102,"body":295,"replaceable":false}},{"Pub":{"ty":103,"body":298,"replaceable":false}},{"Pub":{"ty":104,"body":305,"replaceable":false}},{"Pub":{"ty":105,"body":318,"replaceable":false}},{"Pub":{"ty":106,"body":327,"replaceable":false}},{"Pub":{"ty":107,"body":338,"replaceable":false}},{"Pub":{"ty":108,"body":346,"replaceable":false}},{"Pub":{"ty":109,"body":358,"replaceable":false}},{"Or":[359,360,361]},{"Or":[363,365,367,369,371,373,375,380]},{"Pub":{"ty":110,"body":390,"replaceable":false}},{"Pub":{"ty":111,"body":404,"replaceable":false}},{"Pub":{"ty":112,"body":408,"replaceable":false}},{"Pub":{"ty":113,"body":412,"replaceable":false}},{"Or":[413,414]},{"Pub":{"ty":114,"body":421,"replaceable":false}},{"Pub":{"ty":115,"body":425,"replaceable":false}},{"Or":[445]},{"Pub":{"ty":116,"body":449,"replaceable":false}},{"Pub":{"ty":117,"body":469,"replaceable":false}},{"Pub":{"ty":118,"body":479,"replaceable":false}},{"Pub":{"ty":119,"body":494,"replaceable":false}},{"Or":[495]},{"Or":[497]},{"Or":[499]},{"Pratt":{"atoms":[42,44],"prefixes":[],"infixes":[{"ty":120,"op":502,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":120,"body":506,"replaceable":false}},{"Pub":{"ty":120,"body":511,"replaceable":false}},{"Pub":{"ty":121,"body":518,"replaceable":false}},{"Or":[531]},{"Pub":{"ty":122,"body":545,"replaceable":false}},{"Pub":{"ty":123,"body":550,"replaceable":false}},{"Pub":{"ty":124,"body":554,"replaceable":false}},{"Or":[555,556,557,558,559,560,561,562]},{"Pub":{"ty":125,"body":564,"replaceable":false}},{"Pub":{"ty":126,"body":571,"replaceable":false}},{"Pub":{"ty":127,"body":574,"replaceable":false}},{"Pub":{"ty":128,"body":578,"replaceable":false}},{"Pub":{"ty":129,"body":584,"replaceable":true}},{"PubReplace":{"ty":130,"body":588}},{"Pub":{"ty":131,"body":591,"replaceable":false}},{"Pub":{"ty":132,"body":600,"replaceable":false}},{"Pub":{"ty":133,"body":605,"replaceable":false}},{"Pub":{"ty":98,"body":611,"replaceable":false}},{"Or":[612,613,614,615,616,617,618]},{"Pub":{"ty":134,"body":621,"replaceable":false}},{"Pub":{"ty":135,"body":627,"replaceable":true}},{"PubReplace":{"ty":136,"body":639}},{"PubReplace":{"ty":137,"body":651}},{"Pub":{"ty":138,"body":658,"replaceable":false}},{"Pub":{"ty":139,"body":665,"replaceable":false}},{"Pub":{"ty":140,"body":667,"replaceable":false}},{"Pub":{"ty":141,"body":671,"replaceable":false}},{"Pub":{"ty":142,"body":677,"replaceable":true}},{"PubReplace":{"ty":143,"body":681}},{"Pub":{"ty":144,"body":686,"replaceable":false}},{"Pratt":{"atoms":[76,77,80,81,83,84,86,87,94,95,98,99,101,105,106,133],"prefixes":[{"ty":178,"op":719,"priority":999},{"ty":179,"op":720,"priority":999},{"ty":180,"op":721,"priority":999},{"ty":181,"op":722,"priority":999},{"ty":191,"op":135,"priority":2}],"infixes":[{"ty":171,"op":692,"priority":999,"has_rhs":false},{"ty":172,"op":701,"priority":999,"has_rhs":false},{"ty":174,"op":709,"priority":999,"has_rhs":false},{"ty":175,"op":710,"priority":999,"has_rhs":false},{"ty":176,"op":711,"priority":999,"has_rhs":false},{"ty":177,"op":714,"priority":999,"has_rhs":false},{"ty":182,"op":730,"priority":11,"has_rhs":true},{"ty":183,"op":736,"priority":10,"has_rhs":true},{"ty":184,"op":742,"priority":9,"has_rhs":true},{"ty":185,"op":748,"priority":8,"has_rhs":true},{"ty":186,"op":750,"priority":7,"has_rhs":true},{"ty":187,"op":756,"priority":6,"has_rhs":true},{"ty":188,"op":757,"priority":5,"has_rhs":true},{"ty":189,"op":759,"priority":4,"has_rhs":true},{"ty":190,"op":761,"priority":3,"has_rhs":true},{"ty":191,"op":762,"priority":2,"has_rhs":true},{"ty":191,"op":134,"priority":2,"has_rhs":false},{"ty":192,"op":786,"priority":1,"has_rhs":true}]}},{"Or":[787,789,791,793,795,797,799,801,803,805,807,809,811,813,815,817,819,821,823,825,827,829]},{"Or":[831]},{"Or":[835]},{"Pub":{"ty":146,"body":846,"replaceable":false}},{"Pub":{"ty":147,"body":854,"replaceable":true}},{"PubReplace":{"ty":148,"body":867}},{"Pub":{"ty":149,"body":874,"replaceable":false}},{"Pub":{"ty":150,"body":878,"replaceable":false}},{"Pub":{"ty":151,"body":885,"replaceable":true}},{"PubReplace":{"ty":152,"body":890}},{"Pub":{"ty":153,"body":895,"replaceable":false}},{"Pub":{"ty":154,"body":907,"replaceable":false}},{"Or":[915]},{"Pub":{"ty":155,"body":919,"replaceable":false}},{"Pub":{"ty":156,"body":931,"replaceable":false}},{"Or":[932,933,934,935]},{"Pub":{"ty":157,"body":941,"replaceable":false}},{"Pub":{"ty":158,"body":944,"replaceable":false}},{"Pub":{"ty":159,"body":947,"replaceable":false}},{"Pub":{"ty":160,"body":950,"replaceable":false}},{"Pub":{"ty":161,"body":961,"replaceable":false}},{"Pub":{"ty":162,"body":971,"replaceable":false}},{"Pub":{"ty":163,"body":975,"replaceable":false}},{"Or":[981]},{"Or":[983]},{"Pub":{"ty":164,"body":987,"replaceable":false}},{"Pub":{"ty":165,"body":992,"replaceable":false}},{"Or":[995]},{"Pub":{"ty":166,"body":1000,"replaceable":false}},{"Pub":{"ty":167,"body":1010,"replaceable":false}},{"Or":[1016]},{"Pub":{"ty":168,"body":1019,"replaceable":false}},{"Pub":{"ty":169,"body":1021,"replaceable":false}},{"Pub":{"ty":170,"body":1023,"replaceable":false}},{"Pub":{"ty":171,"body":1031,"replaceable":false}},{"Pub":{"ty":172,"body":1042,"replaceable":false}},{"Or":[1046]},{"Pub":{"ty":173,"body":1048,"replaceable":false}},{"Pub":{"ty":174,"body":1058,"replaceable":false}},{"Pub":{"ty":175,"body":1061,"replaceable":false}},{"Pub":{"ty":176,"body":1064,"replaceable":false}},{"Pub":{"ty":177,"body":1069,"replaceable":false}},{"Pub":{"ty":178,"body":1076,"replaceable":false}},{"Pub":{"ty":179,"body":1079,"replaceable":false}},{"Pub":{"ty":180,"body":1082,"replaceable":false}},{"Pub":{"ty":181,"body":1085,"replaceable":false}},{"Pub":{"ty":182,"body":1095,"replaceable":false}},{"Pub":{"ty":183,"body":1103,"replaceable":false}},{"Pub":{"ty":184,"body":1111,"replaceable":false}},{"Or":[1115,1119]},{"Pub":{"ty":185,"body":1127,"replaceable":false}},{"Pub":{"ty":186,"body":1131,"replaceable":false}},{"Pub":{"ty":187,"body":1139,"replaceable":false}},{"Pub":{"ty":188,"body":1142,"replaceable":false}},{"Or":[1144,1146,1148,1150,1152,1154]},{"Pub":{"ty":189,"body":1158,"replaceable":false}},{"Pub":{"ty":190,"body":1162,"replaceable":false}},{"Pub":{"ty":191,"body":1165,"replaceable":false}},{"Pub":{"ty":191,"body":1167,"replaceable":false}},{"Pub":{"ty":191,"body":1169,"replaceable":false}},{"Pub":{"ty":191,"body":1173,"replaceable":false}},{"Or":[1175,1177]},{"Or":[1183]},{"Pub":{"ty":192,"body":1209,"replaceable":false}},{"Pub":{"ty":193,"body":1214,"replaceable":false}},{"Or":[1216]},{"Pub":{"ty":194,"body":1226,"replaceable":false}},{"Pub":{"ty":195,"body":1234,"replaceable":false}},{"Pub":{"ty":196,"body":1249,"replaceable":false}},{"Pub":{"ty":197,"body":1278,"replaceable":false}},{"Or":[1288]},{"Or":[1293]},{"Or":[1298]},{"Or":[1303]},{"Or":[1308]},{"Or":[1316]},{"Or":[1331]},{"And":[[1],null]},{"Or":[150]},{"WithSkip":[2,3]},{"Rep":152},{"And":[[153],null]},{"Token":11},{"And":[[155],null]},{"ContextualToken":[4,"union"]},{"And":[[157],null]},{"Token":16},{"And":[[159],null]},{"Token":12},{"And":[[161],null]},{"Token":13},{"And":[[163],null]},{"Token":17},{"And":[[165],null]},{"Token":29},{"And":[[167],null]},{"And":[[25],null]},{"Opt":36},{"And":[[138,170],null]},{"Or":[171]},{"And":[[24],null]},{"And":[[4],null]},{"And":[[7],null]},{"And":[[15],null]},{"And":[[18],null]},{"And":[[20],null]},{"And":[[21],null]},{"And":[[22],null]},{"And":[[29],null]},{"Or":[173,174,175,176,177,178,179,180,181]},{"Inject":[172,182]},{"And":[[183],null]},{"And":[[28],null]},{"Or":[184,185]},{"Token":12},{"And":[[48],null]},{"Token":58},{"And":[[189,5],null]},{"Or":[190]},{"Opt":191},{"And":[[192],null]},{"Or":[188,193]},{"And":[[38,194],null]},{"Token":58},{"Opt":196},{"And":[[197,5],null]},{"Or":[195,198]},{"Token":56},{"And":[[187,199,200],1]},{"Or":[201]},{"Token":65},{"And":[[203],null]},{"Call":[143,[[2,6]]]},{"Call":[144,[[3,205]]]},{"And":[[206],null]},{"Or":[204,207]},{"Token":18},{"And":[[209],null]},{"Token":90},{"Opt":48},{"And":[[211,212],1]},{"Or":[210,213]},{"Token":7},{"Token":6},{"Token":90},{"Opt":48},{"Token":56},{"And":[[215,216,217,218,219],2]},{"Or":[220]},{"Opt":10},{"Token":8},{"Token":90},{"Opt":31},{"Opt":9},{"Opt":37},{"And":[[87],null]},{"Token":56},{"And":[[229],null]},{"Or":[228,230]},{"And":[[222,223,224,225,11,226,227,231],2]},{"Or":[232]},{"Token":49},{"And":[[234,49],null]},{"Token":7},{"Token":88},{"Opt":237},{"And":[[236,238],null]},{"Or":[239]},{"Opt":14},{"Call":[143,[[2,12]]]},{"And":[[241,242],null]},{"Or":[243]},{"Call":[145,[[4,244]]]},{"And":[[245],null]},{"Token":57},{"And":[[60,247,49],1]},{"Or":[248]},{"Token":57},{"And":[[250,49],null]},{"Or":[251]},{"Opt":252},{"And":[[60,253],null]},{"Or":[254]},{"Token":75},{"Opt":256},{"Token":27},{"Opt":258},{"Token":18},{"Token":57},{"And":[[261,49],null]},{"Or":[262]},{"Opt":263},{"Token":59},{"And":[[265],null]},"Eof",{"And":[[267],null]},{"Or":[266,268]},{"And":[[257,259,260,264,269],3]},{"Or":[270]},{"Token":11},{"And":[[272],null]},{"ContextualToken":[4,"union"]},{"And":[[274],null]},{"Or":[273,275]},{"Token":90},{"Opt":31},{"Call":[143,[[2,16]]]},{"Call":[144,[[3,279]]]},{"And":[[280],null]},{"Token":56},{"And":[[282],null]},{"Call":[143,[[2,17]]]},{"Call":[145,[[4,284]]]},{"Token":56},{"And":[[285,286],null]},{"Or":[281,283,287]},{"And":[[276,277,278,288],1]},{"Or":[289]},{"Opt":36},{"Token":90},{"Token":57},{"And":[[291,292,293,49],2]},{"Or":[294]},{"Opt":36},{"And":[[296,49],null]},{"Or":[297]},{"Token":16},{"Token":90},{"Opt":31},{"Call":[143,[[2,19]]]},{"Call":[144,[[3,302]]]},{"And":[[299,300,301,303],1]},{"Or":[304]},{"Token":90},{"Token":51},{"And":[[307,72],null]},{"Call":[143,[[2,17]]]},{"Call":[145,[[4,309]]]},{"And":[[310],null]},{"Call":[143,[[2,16]]]},{"Call":[144,[[3,312]]]},{"And":[[313],null]},{"Or":[308,311,314]},{"Opt":315},{"And":[[306,316],1]},{"Or":[317]},{"Token":13},{"Token":90},{"Token":56},{"And":[[321],null]},{"Call":[144,[[3,1]]]},{"And":[[323],null]},{"Or":[322,324]},{"And":[[319,320,325],1]},{"Or":[326]},{"Token":17},{"Opt":31},{"Token":23},{"And":[[330,49],null]},{"Or":[331]},{"Opt":332},{"And":[[49,333],null]},{"Or":[334]},{"Opt":37},{"And":[[328,329,335,336,23],1]},{"Or":[337]},{"Opt":36},{"Token":29},{"Token":90},{"Opt":31},{"Opt":33},{"Opt":37},{"And":[[138,339,340,341,342,343,344,23],3]},{"Or":[345]},{"Opt":36},{"And":[[138,347],null]},{"Or":[348]},{"Inject":[349,24]},{"And":[[350],null]},{"And":[[28],null]},{"Or":[351,352]},{"WithSkip":[25,353]},{"Rep":354},{"Call":[144,[[3,355]]]},{"And":[[356],null]},{"Or":[357]},{"And":[[8],null]},{"And":[[26],null]},{"And":[[27],null]},{"Token":10},{"And":[[362],null]},{"Token":8},{"And":[[364],null]},{"Token":20},{"And":[[366],null]},{"Token":21},{"And":[[368],null]},{"Token":22},{"And":[[370],null]},{"Token":63},{"And":[[372],null]},{"Token":7},{"And":[[374],null]},{"Token":90},{"Token":80},{"And":[[376,377],null]},{"Or":[378]},{"And":[[379],null]},{"Token":20},{"Token":90},{"Opt":31},{"Token":51},{"And":[[384,49],null]},{"Or":[385]},{"Opt":386},{"Token":56},{"And":[[381,382,383,387,388],1]},{"Or":[389]},{"Token":21},{"And":[[391],null]},{"Token":22},{"And":[[393],null]},{"Or":[392,394]},{"Token":90},{"Token":57},{"Token":51},{"And":[[398,72],null]},{"Or":[399]},{"Opt":400},{"Token":56},{"And":[[395,396,397,49,401,402],1]},{"Or":[403]},{"And":[[140],null]},{"Token":56},{"And":[[141,406],null]},{"Or":[405,407]},{"Rep":30},{"Call":[144,[[3,409]]]},{"And":[[10,410],null]},{"Or":[411]},{"And":[[8],null]},{"And":[[27],null]},{"Call":[143,[[2,35]]]},{"Call":[143,[[2,32]]]},{"And":[[415,416],null]},{"Or":[417]},{"Call":[146,[[5,418]]]},{"And":[[419],null]},{"Or":[420]},{"Token":90},{"Opt":33},{"And":[[422,423],1]},{"Or":[424]},{"Token":57},{"Token":71},{"And":[[427],null]},"Eof",{"And":[[429],null]},{"Token":59},{"And":[[431],null]},{"Token":37},{"And":[[433],null]},{"Token":34},{"And":[[435],null]},{"Or":[432,434,436]},{"Not":437},{"Not":438},{"And":[[439],null]},{"Or":[428,430,440]},{"And":[[34,441],1]},{"Or":[442]},{"Rep":443},{"And":[[426,444],null]},{"Token":85},{"And":[[446],null]},{"And":[[49],null]},{"Or":[447,448]},{"Token":85},{"Token":57},{"Token":85},{"Token":71},{"And":[[453],null]},"Eof",{"And":[[455],null]},{"Token":59},{"Not":457},{"Not":458},{"And":[[459],null]},{"Or":[454,456,460]},{"And":[[452,461],1]},{"Or":[462]},{"Rep":463},{"And":[[451,464],null]},{"Or":[465]},{"Opt":466},{"And":[[450,467],1]},{"Or":[468]},{"Token":10},{"Token":6},{"And":[[471],null]},{"Token":19},{"And":[[473],null]},{"Or":[472,474]},{"Call":[145,[[4,475]]]},{"Opt":476},{"And":[[470,477],null]},{"Or":[478]},{"Token":34},{"Token":59},{"And":[[481],null]},"Eof",{"And":[[483],null]},{"Token":37},{"Not":485},{"Not":486},{"And":[[487],null]},{"Or":[482,484,488]},{"And":[[49,33,489],null]},{"Or":[490]},{"Rep":491},{"And":[[480,492],1]},{"Or":[493]},{"And":[[41],null]},{"Enter":[3,41]},{"And":[[496],null]},{"Enter":[1,41]},{"And":[[498],null]},{"Token":58},{"And":[[500,45],null]},{"Or":[501]},{"Token":58},{"Opt":503},{"And":[[504,45],null]},{"Or":[505]},{"Token":58},{"And":[[507,45],null]},{"Or":[508]},{"And":[[41,509],null]},{"Or":[510]},{"Token":5},{"And":[[49,512,49],null]},{"Or":[513]},{"Call":[146,[[5,514]]]},{"Token":58},{"And":[[515,516,45],null]},{"Or":[517]},{"Token":90},{"And":[[519],null]},{"Token":18},{"And":[[521],null]},{"Token":19},{"And":[[523],null]},{"Or":[520,522,524]},{"And":[[46],null]},{"IsIn":3},{"And":[[527,47],null]},{"Or":[526,528]},{"Opt":529},{"And":[[525,530],null]},{"IsIn":3},{"And":[[532],null]},{"IsIn":1},{"Token":58},{"And":[[534,535],null]},{"Or":[533,536]},{"Token":85},{"Call":[143,[[2,538]]]},{"Call":[143,[[2,49]]]},{"And":[[539,540],null]},{"Or":[541]},{"Call":[146,[[5,542]]]},{"And":[[537,543],null]},{"Or":[544]},{"Call":[143,[[2,49]]]},{"Call":[145,[[4,546]]]},{"Opt":9},{"And":[[547,548],null]},{"Or":[549]},{"Token":5},{"Token":90},{"And":[[551,552],null]},{"Or":[553]},{"And":[[50],null]},{"And":[[51],null]},{"And":[[52],null]},{"And":[[53],null]},{"And":[[54],null]},{"And":[[56],null]},{"And":[[57],null]},{"And":[[58],null]},{"And":[[39],null]},{"Or":[563]},{"Token":75},{"Token":85},{"Opt":566},{"Token":27},{"Opt":568},{"And":[[565,567,569,49],null]},{"Or":[570]},{"Token":79},{"And":[[572],null]},{"Or":[573]},{"Token":35},{"Token":36},{"And":[[575,576],null]},{"Or":[577]},{"Opt":55},{"And":[[49,579],null]},{"Or":[580]},{"Call":[145,[[4,581]]]},{"And":[[582],null]},{"Or":[583]},{"Token":59},{"Call":[143,[[2,49]]]},{"And":[[585,586],null]},{"Or":[587]},{"Token":80},{"And":[[589],null]},{"Or":[590]},{"Token":56},{"And":[[592,72],null]},{"Or":[593]},{"Opt":594},{"And":[[49,595],null]},{"Or":[596]},{"Call":[147,[[6,597]]]},{"And":[[598],null]},{"Or":[599]},{"Token":8},{"Call":[143,[[2,59]]]},{"Call":[145,[[4,602]]]},{"And":[[601,603,9],1]},{"Or":[604]},{"Token":57},{"And":[[60,606],null]},{"Or":[607]},{"Opt":608},{"And":[[609,49],null]},{"Or":[610]},{"And":[[61],null]},{"And":[[62],null]},{"And":[[66],null]},{"And":[[67],null]},{"And":[[68],null]},{"And":[[69],null]},{"And":[[71],null]},{"Token":79},{"And":[[619],null]},{"Or":[620]},{"And":[[63],null]},{"And":[[64],null]},{"Or":[622,623]},{"Opt":624},{"And":[[40,625],null]},{"Or":[626]},{"Call":[143,[[2,60]]]},{"Token":61},{"Token":59},{"Opt":630},{"And":[[629,631],null]},{"Or":[632]},{"Opt":633},{"And":[[628,634],null]},{"Or":[635]},{"Call":[145,[[4,636]]]},{"And":[[637],null]},{"Or":[638]},{"Call":[143,[[2,65]]]},{"Token":61},{"Token":59},{"Opt":642},{"And":[[641,643],null]},{"Or":[644]},{"Opt":645},{"And":[[640,646],null]},{"Or":[647]},{"Call":[144,[[3,648]]]},{"And":[[649],null]},{"Or":[650]},{"Token":57},{"Not":652},{"And":[[66,653],null]},{"Token":90},{"Token":57},{"And":[[655,656,60],2]},{"Or":[654,657]},{"Token":28},{"Opt":659},{"Token":27},{"Opt":661},{"Token":90},{"And":[[660,662,663],null]},{"Or":[664]},{"And":[[76],null]},{"Or":[666]},{"Token":35},{"Token":36},{"And":[[668,669],null]},{"Or":[670]},{"Opt":70},{"And":[[60,672],null]},{"Or":[673]},{"Call":[145,[[4,674]]]},{"And":[[675],null]},{"Or":[676]},{"Token":59},{"Call":[143,[[2,60]]]},{"And":[[678,679],null]},{"Or":[680]},{"Token":75},{"Token":27},{"Opt":683},{"And":[[682,684,60],null]},{"Or":[685]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":689},{"And":[[687,688,690,109],null]},{"Or":[691]},{"IsIn":2},{"Not":74},{"And":[[693,694],null]},{"IsIn":2},{"Not":696},{"And":[[697],null]},{"Or":[695,698]},{"And":[[699,109],null]},{"Or":[700]},{"Token":60},{"Token":90},{"And":[[703],null]},{"Token":87},{"And":[[705],null]},{"Or":[704,706]},{"And":[[702,707],null]},{"Or":[708]},{"Call":[147,[[6,72]]]},{"Token":81},{"Token":5},{"And":[[712,49],null]},{"Or":[713]},{"Token":75},{"Token":27},{"Opt":716},{"And":[[715,717],null]},{"Or":[718]},{"Token":65},{"Token":73},{"Token":80},{"Token":65},{"And":[[723],null]},{"Token":67},{"And":[[725],null]},{"Token":69},{"And":[[727],null]},{"Or":[724,726,728]},{"Call":[122,[[1,729]]]},{"Token":71},{"And":[[731],null]},{"Token":73},{"And":[[733],null]},{"Or":[732,734]},{"Call":[122,[[1,735]]]},{"ContextualToken":[43,"<<"]},{"And":[[737],null]},{"ContextualToken":[45,">>"]},{"And":[[739],null]},{"Or":[738,740]},{"Call":[122,[[1,741]]]},{"Token":75},{"Token":75},{"Not":744},{"And":[[743,745],null]},{"Or":[746]},{"Call":[122,[[1,747]]]},{"Token":82},{"Call":[122,[[1,749]]]},{"Token":77},{"Token":77},{"Not":752},{"And":[[751,753],null]},{"Or":[754]},{"Call":[122,[[1,755]]]},{"Call":[122,[[1,127]]]},{"ContextualToken":[47,"&&"]},{"Call":[122,[[1,758]]]},{"ContextualToken":[48,"||"]},{"Call":[122,[[1,760]]]},{"Call":[122,[[1,135]]]},{"Token":51},{"And":[[763],null]},{"Token":72},{"And":[[765],null]},{"Token":74},{"And":[[767],null]},{"Token":66},{"And":[[769],null]},{"Token":68},{"And":[[771],null]},{"Token":70},{"And":[[773],null]},{"Token":76},{"And":[[775],null]},{"Token":78},{"And":[[777],null]},{"Token":83},{"And":[[779],null]},{"ContextualToken":[46,">>="]},{"And":[[781],null]},{"ContextualToken":[44,"<<="]},{"And":[[783],null]},{"Or":[764,766,768,770,772,774,776,778,780,782,784]},{"Call":[122,[[1,785]]]},{"And":[[76],null]},{"Token":90},{"And":[[788],null]},{"Token":18},{"And":[[790],null]},{"Token":19},{"And":[[792],null]},{"Token":39},{"And":[[794],null]},{"Token":58},{"And":[[796],null]},{"Token":35},{"And":[[798],null]},{"Token":41},{"And":[[800],null]},{"Token":77},{"And":[[802],null]},{"Token":31},{"And":[[804],null]},{"Token":37},{"And":[[806],null]},{"Token":14},{"And":[[808],null]},{"Token":25},{"And":[[810],null]},{"Token":24},{"And":[[812],null]},{"Token":23},{"And":[[814],null]},{"Token":30},{"And":[[816],null]},{"Token":75},{"And":[[818],null]},{"Token":65},{"And":[[820],null]},{"Token":73},{"And":[[822],null]},{"Token":80},{"And":[[824],null]},{"Token":61},{"And":[[826],null]},{"Token":62},{"And":[[828],null]},{"PrevIs":[156,162,163,164,165,166,169]},{"And":[[830],null]},{"Var":0},{"Exit":[2,832]},{"Exit":[0,833]},{"And":[[834],null]},{"Token":87},{"And":[[836],null]},{"Token":88},{"And":[[838],null]},{"Token":89},{"And":[[840],null]},{"Token":84},{"And":[[842],null]},{"Token":86},{"And":[[844],null]},{"Or":[837,839,841,843,845]},{"Token":90},{"Token":80},{"And":[[847,848],null]},{"Or":[849]},{"Not":850},{"Opt":78},{"And":[[851,40,852],null]},{"Or":[853]},{"IsIn":0},{"Not":855},{"Call":[143,[[2,79]]]},{"Token":61},{"Call":[75,[[0,72]]]},{"And":[[858,859],null]},{"Or":[860]},{"Opt":861},{"And":[[857,862],null]},{"Or":[863]},{"Call":[144,[[3,864]]]},{"And":[[856,865],null]},{"Or":[866]},{"Token":90},{"Token":57},{"And":[[869,72],null]},{"Or":[870]},{"Opt":871},{"And":[[868,872],1]},{"Or":[873]},{"Token":35},{"Token":36},{"And":[[875,876],null]},{"Or":[877]},{"Call":[75,[[0,72]]]},{"Opt":82},{"And":[[879,880],null]},{"Or":[881]},{"Call":[145,[[4,882]]]},{"And":[[883],null]},{"Or":[884]},{"Token":59},{"Call":[75,[[0,72]]]},{"Call":[143,[[2,887]]]},{"And":[[886,888],null]},{"Or":[889]},{"Call":[143,[[2,72]]]},{"Call":[75,[[0,891]]]},{"Call":[147,[[6,892]]]},{"And":[[893],null]},{"Or":[894]},{"Token":26},{"Opt":896},{"Token":77},{"Rep":85},{"Token":77},{"Token":49},{"And":[[901,49,87],null]},{"Call":[75,[[0,72]]]},{"And":[[903],null]},{"Or":[902,904]},{"And":[[897,898,899,900,905],null]},{"Or":[906]},{"Token":59},{"And":[[908],null]},{"Token":77},{"Not":910},{"Not":911},{"And":[[912],null]},{"Or":[909,913]},{"And":[[13,914],1]},{"Token":31},{"Opt":72},{"And":[[916,917],null]},{"Or":[918]},{"Token":33},{"Opt":920},{"Rep":88},{"Opt":72},{"And":[[922,923],null]},{"Or":[924]},{"Call":[144,[[3,925]]]},{"And":[[921,926],null]},{"Or":[927]},{"Call":[75,[[0,928]]]},{"And":[[929],null]},{"Or":[930]},{"And":[[89],null]},{"And":[[93],null]},{"And":[[92],null]},{"And":[[3],null]},{"Token":9},{"Opt":90},{"Opt":91},{"Token":56},{"And":[[936,60,937,938,939],1]},{"Or":[940]},{"Token":57},{"And":[[942,49],null]},{"Or":[943]},{"Token":51},{"And":[[945,72],null]},{"Or":[946]},{"Token":56},{"And":[[948],null]},{"Or":[949]},"Eof",{"Not":951},{"And":[[74,952],null]},{"Token":56},{"And":[[954],null]},{"Or":[953,955]},{"And":[[72,956],null]},{"Or":[957]},{"Enter":[2,958]},{"And":[[959],null]},{"Or":[960]},{"Token":14},{"Token":15},{"And":[[87],null]},{"And":[[94],null]},{"Or":[964,965]},{"And":[[963,966],null]},{"Or":[967]},{"Opt":968},{"And":[[962,96,87,969],1]},{"Or":[970]},{"Opt":100},{"Token":25},{"And":[[972,973,96,87],2]},{"Or":[974]},{"Token":9},{"Token":51},{"And":[[976,60,977],1]},{"Or":[978]},{"Opt":979},{"And":[[980,97],null]},{"Enter":[0,72]},{"And":[[982],null]},{"Opt":100},{"Token":24},{"And":[[984,985,87],2]},{"Or":[986]},{"Opt":100},{"Token":23},{"Token":32},{"And":[[988,989,60,990,97,87],2]},{"Or":[991]},{"Token":85},{"Token":57},{"And":[[993,994],null]},{"Token":30},{"Rep":102},{"Call":[144,[[3,997]]]},{"And":[[996,97,998],1]},{"Or":[999]},{"Token":50},{"Enter":[2,72]},{"Token":59},{"And":[[1003],null]},"Eof",{"And":[[1005],null]},{"And":[[74],null]},{"Or":[1004,1006,1007]},{"And":[[103,1001,1002,1008],1]},{"Or":[1009]},{"Token":77},{"And":[[1011,60],null]},{"Or":[1012]},{"Rep":1013},{"Opt":104},{"And":[[60,1014,1015],null]},{"Token":14},{"And":[[1017,72],null]},{"Or":[1018]},{"And":[[140],null]},{"Or":[1020]},{"And":[[141],null]},{"Or":[1022]},{"Token":60},{"Token":90},{"Enter":[1,46]},{"Opt":1026},{"And":[[1024,1025,1027,109],null]},{"Or":[1028]},{"And":[[72,1029],null]},{"Or":[1030]},{"IsIn":2},{"Not":74},{"And":[[1032,1033],null]},{"IsIn":2},{"Not":1035},{"And":[[1036],null]},{"Or":[1034,1037]},{"And":[[1038,109],null]},{"Or":[1039]},{"And":[[72,1040],null]},{"Or":[1041]},{"Call":[143,[[2,110]]]},{"Call":[145,[[4,1043]]]},{"Call":[75,[[0,1044]]]},{"And":[[1045],null]},{"And":[[72],null]},{"Or":[1047]},{"Token":60},{"Token":90},{"And":[[1050],null]},{"Token":87},{"And":[[1052],null]},{"Or":[1051,1053]},{"And":[[1049,1054],null]},{"Or":[1055]},{"And":[[72,1056],null]},{"Or":[1057]},{"Call":[147,[[6,72]]]},{"And":[[72,1059],null]},{"Or":[1060]},{"Token":81},{"And":[[72,1062],null]},{"Or":[1063]},{"Token":5},{"And":[[1065,49],null]},{"Or":[1066]},{"And":[[72,1067],null]},{"Or":[1068]},{"Token":75},{"Token":27},{"Opt":1071},{"And":[[1070,1072],null]},{"Or":[1073]},{"And":[[1074,72],null]},{"Or":[1075]},{"Token":65},{"And":[[1077,72],null]},{"Or":[1078]},{"Token":73},{"And":[[1080,72],null]},{"Or":[1081]},{"Token":80},{"And":[[1083,72],null]},{"Or":[1084]},{"Token":65},{"And":[[1086],null]},{"Token":67},{"And":[[1088],null]},{"Token":69},{"And":[[1090],null]},{"Or":[1087,1089,1091]},{"Call":[122,[[1,1092]]]},{"And":[[72,1093,72],null]},{"Or":[1094]},{"Token":71},{"And":[[1096],null]},{"Token":73},{"And":[[1098],null]},{"Or":[1097,1099]},{"Call":[122,[[1,1100]]]},{"And":[[72,1101,72],null]},{"Or":[1102]},{"ContextualToken":[43,"<<"]},{"And":[[1104],null]},{"ContextualToken":[45,">>"]},{"And":[[1106],null]},{"Or":[1105,1107]},{"Call":[122,[[1,1108]]]},{"And":[[72,1109,72],null]},{"Or":[1110]},{"IsIn":2},{"Not":74},{"Var":1},{"And":[[1112,1113,1114],null]},{"IsIn":2},{"Not":1116},{"Var":1},{"And":[[1117,1118],null]},{"Token":75},{"Token":75},{"Not":1121},{"And":[[1120,1122],null]},{"Or":[1123]},{"Call":[122,[[1,1124]]]},{"And":[[72,1125,72],null]},{"Or":[1126]},{"Token":82},{"Call":[122,[[1,1128]]]},{"And":[[72,1129,72],null]},{"Or":[1130]},{"Token":77},{"Token":77},{"Not":1133},{"And":[[1132,1134],null]},{"Or":[1135]},{"Call":[122,[[1,1136]]]},{"And":[[72,1137,72],null]},{"Or":[1138]},{"Call":[122,[[1,127]]]},{"And":[[72,1140,72],null]},{"Or":[1141]},{"Token":52},{"And":[[1143],null]},{"Token":53},{"And":[[1145],null]},{"Token":39},{"And":[[1147],null]},{"Token":40},{"And":[[1149],null]},{"Token":55},{"And":[[1151],null]},{"Token":54},{"And":[[1153],null]},{"ContextualToken":[47,"&&"]},{"Call":[122,[[1,1155]]]},{"And":[[72,1156,72],null]},{"Or":[1157]},{"ContextualToken":[48,"||"]},{"Call":[122,[[1,1159]]]},{"And":[[72,1160,72],null]},{"Or":[1161]},{"Call":[122,[[1,135]]]},{"And":[[72,1163,72],null]},{"Or":[1164]},{"And":[[135,72],null]},{"Or":[1166]},{"And":[[72,134],null]},{"Or":[1168]},{"Token":61},{"Not":73},{"And":[[1170,1171],null]},{"Or":[1172]},{"Token":61},{"And":[[1174],null]},{"Token":62},{"And":[[1176],null]},{"Token":37},{"IsIn":0},{"And":[[1178,1179],null]},{"Or":[1180]},{"Not":1181},{"And":[[134,1182],null]},{"Token":51},{"And":[[1184],null]},{"Token":72},{"And":[[1186],null]},{"Token":74},{"And":[[1188],null]},{"Token":66},{"And":[[1190],null]},{"Token":68},{"And":[[1192],null]},{"Token":70},{"And":[[1194],null]},{"Token":76},{"And":[[1196],null]},{"Token":78},{"And":[[1198],null]},{"Token":83},{"And":[[1200],null]},{"ContextualToken":[46,">>="]},{"And":[[1202],null]},{"ContextualToken":[44,"<<="]},{"And":[[1204],null]},{"Or":[1185,1187,1189,1191,1193,1195,1197,1199,1201,1203,1205]},{"Call":[122,[[1,1206]]]},{"And":[[72,1207,72],null]},{"Or":[1208]},{"Token":63},{"Call":[143,[[2,139]]]},{"Call":[147,[[6,1211]]]},{"And":[[1210,1212],null]},{"Or":[1213]},{"Rep":137},{"And":[[1215],null]},{"Token":90},{"Token":51},{"And":[[1218,72],null]},{"Call":[143,[[2,139]]]},{"Call":[145,[[4,1220]]]},{"And":[[1221],null]},{"Or":[1219,1222]},{"Opt":1223},{"And":[[1217,1224],1]},{"Or":[1225]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1229},{"Rep":142},{"Call":[144,[[3,1231]]]},{"And":[[1227,1228,1230,1232],null]},{"Or":[1233]},{"Token":90},{"Token":80},{"Token":90},{"Opt":1237},{"Token":35},{"Rep":142},{"Token":36},{"And":[[1239,1240,1241],null]},{"Token":41},{"Rep":142},{"Token":42},{"And":[[1243,1244,1245],null]},{"Or":[1242,1246]},{"And":[[1235,1236,1238,1247],null]},{"Or":[1248]},{"Token":35},{"And":[[1250],null]},{"Token":36},{"And":[[1252],null]},{"Token":37},{"And":[[1254],null]},{"Token":38},{"And":[[1256],null]},{"Token":41},{"And":[[1258],null]},{"Token":42},{"And":[[1260],null]},{"Or":[1251,1253,1255,1257,1259,1261]},{"Not":1262},"Any",{"And":[[1263,1264],null]},{"Token":35},{"Rep":142},{"Token":36},{"And":[[1266,1267,1268],null]},{"Token":41},{"Rep":142},{"Token":42},{"And":[[1270,1271,1272],null]},{"Token":37},{"Rep":142},{"Token":38},{"And":[[1274,1275,1276],null]},{"Or":[1265,1269,1273,1277]},{"Var":2},"Eof",{"And":[[1280],null]},{"Token":59},{"And":[[1282],null]},{"Or":[1281,1283]},{"And":[[1279,1284],1]},{"Or":[1285]},{"Rep":1286},{"And":[[1287],null]},{"Token":37},{"Token":38},{"Var":3},{"Call":[148,[[7,1289],[8,1290],[9,1291]]]},{"And":[[1292],null]},{"Token":35},{"Token":36},{"Var":4},{"Call":[148,[[7,1294],[8,1295],[9,1296]]]},{"And":[[1297],null]},{"Token":39},{"Token":40},{"Var":5},{"Call":[148,[[7,1299],[8,1300],[9,1301]]]},{"And":[[1302],null]},{"Token":41},{"Token":42},{"Var":6},{"Call":[148,[[7,1304],[8,1305],[9,1306]]]},{"And":[[1307],null]},{"Var":7},{"Var":7},{"Var":8},{"Call":[149,[[10,1310],[11,1311]]]},{"Var":9},{"Layer":[1312,1313]},{"Var":8},{"And":[[1309,1314,1315],1]},{"Var":10},{"Var":10},{"Var":11},{"Call":[149,[[10,1318],[11,1319]]]},{"Var":11},{"And":[[1317,1320,1321],1]},{"Var":11},{"Not":1323},"Any",{"And":[[1324,1325],null]},{"Or":[1326]},{"And":[[1327],null]},{"Or":[1322,1328]},{"Rep":1329},{"And":[[1330],null]}]"##;

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