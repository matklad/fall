use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
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
pub const SHR: NodeType = NodeType(143);
pub const AND: NodeType = NodeType(144);
pub const OR: NodeType = NodeType(145);
pub const THIN_ARROW: NodeType = NodeType(146);
pub const FAT_ARROW: NodeType = NodeType(147);
pub const EQ: NodeType = NodeType(148);
pub const EQEQ: NodeType = NodeType(149);
pub const BANGEQ: NodeType = NodeType(150);
pub const GTET: NodeType = NodeType(151);
pub const LTEQ: NodeType = NodeType(152);
pub const SEMI: NodeType = NodeType(153);
pub const COLON: NodeType = NodeType(154);
pub const COLONCOLON: NodeType = NodeType(155);
pub const COMMA: NodeType = NodeType(156);
pub const DOT: NodeType = NodeType(157);
pub const DOTDOT: NodeType = NodeType(158);
pub const DOTDOTDOT: NodeType = NodeType(159);
pub const HASH: NodeType = NodeType(160);
pub const DOLLAR: NodeType = NodeType(161);
pub const STAR: NodeType = NodeType(162);
pub const SLASH: NodeType = NodeType(163);
pub const PERCENT: NodeType = NodeType(164);
pub const PLUS: NodeType = NodeType(165);
pub const MINUS: NodeType = NodeType(166);
pub const AMPERSAND: NodeType = NodeType(167);
pub const PIPE: NodeType = NodeType(168);
pub const UNDERSCORE: NodeType = NodeType(169);
pub const BANG: NodeType = NodeType(170);
pub const CHAR: NodeType = NodeType(171);
pub const LIFETIME: NodeType = NodeType(172);
pub const IDENT: NodeType = NodeType(173);
pub const NUMBER: NodeType = NodeType(174);
pub const STRING: NodeType = NodeType(175);
pub const RAW_STRING: NodeType = NodeType(176);
pub const FILE: NodeType = NodeType(177);
pub const USE_DECL: NodeType = NodeType(178);
pub const USE_SPEC: NodeType = NodeType(179);
pub const USE_SPEC_ENTRY: NodeType = NodeType(180);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(181);
pub const FN_DEF: NodeType = NodeType(182);
pub const LINKAGE: NodeType = NodeType(183);
pub const VALUE_PARAM: NodeType = NodeType(184);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(185);
pub const SELF_PARAMETER: NodeType = NodeType(186);
pub const STRUCT_DEF: NodeType = NodeType(187);
pub const STRUCT_FIELD: NodeType = NodeType(188);
pub const TUPLE_FIELD: NodeType = NodeType(189);
pub const ENUM_DEF: NodeType = NodeType(190);
pub const ENUM_VARIANT: NodeType = NodeType(191);
pub const MOD_DEF: NodeType = NodeType(192);
pub const IMPL_DEF: NodeType = NodeType(193);
pub const TRAIT_DEF: NodeType = NodeType(194);
pub const MEMBERS: NodeType = NodeType(195);
pub const TYPE_DEF: NodeType = NodeType(196);
pub const CONST_DEF: NodeType = NodeType(197);
pub const MACRO_ITEM: NodeType = NodeType(198);
pub const EXTERN_BLOCK: NodeType = NodeType(199);
pub const TYPE_PARAMETERS: NodeType = NodeType(200);
pub const TYPE_PARAMETER: NodeType = NodeType(201);
pub const TYPE_BOUND: NodeType = NodeType(202);
pub const LIFETIME_PARAMETER: NodeType = NodeType(203);
pub const VISIBILITY: NodeType = NodeType(204);
pub const WHERE_CLAUSE: NodeType = NodeType(205);
pub const PATH: NodeType = NodeType(206);
pub const TYPE_ARGUMENTS: NodeType = NodeType(207);
pub const FN_TRAIT_SUGAR: NodeType = NodeType(208);
pub const ALIAS: NodeType = NodeType(209);
pub const PATH_TYPE: NodeType = NodeType(210);
pub const REFERENCE_TYPE: NodeType = NodeType(211);
pub const PLACEHOLDER_TYPE: NodeType = NodeType(212);
pub const UNIT_TYPE: NodeType = NodeType(213);
pub const PAREN_TYPE: NodeType = NodeType(214);
pub const TUPLE_TYPE: NodeType = NodeType(215);
pub const ARRAY_TYPE: NodeType = NodeType(216);
pub const FN_POINTER_TYPE: NodeType = NodeType(217);
pub const WILDCARD_PATTERN: NodeType = NodeType(218);
pub const PATH_PATTERN: NodeType = NodeType(219);
pub const TUPE_STRUCT_PATTERN: NodeType = NodeType(220);
pub const STRUCT_PATTERN: NodeType = NodeType(221);
pub const STRUCT_PATTERN_FIELD: NodeType = NodeType(222);
pub const BINDING_PATTERN: NodeType = NodeType(223);
pub const LITERAL_PATTERN: NodeType = NodeType(224);
pub const UNIT_PATTERN: NodeType = NodeType(225);
pub const PAREN_PATTERN: NodeType = NodeType(226);
pub const TUPLE_PATTERN: NodeType = NodeType(227);
pub const REFERENCE_PATTERN: NodeType = NodeType(228);
pub const EXPR: NodeType = NodeType(229);
pub const LITERAL: NodeType = NodeType(230);
pub const PATH_EXPR: NodeType = NodeType(231);
pub const STRUCT_LITERAL: NodeType = NodeType(232);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(233);
pub const UNIT_EXPR: NodeType = NodeType(234);
pub const PAREN_EXPR: NodeType = NodeType(235);
pub const TUPLE_EXPR: NodeType = NodeType(236);
pub const ARRAY_LITERAL: NodeType = NodeType(237);
pub const LAMBDA_EXPR: NodeType = NodeType(238);
pub const RETURN_EXPR: NodeType = NodeType(239);
pub const BLOCK_EXPR: NodeType = NodeType(240);
pub const LET_STMT: NodeType = NodeType(241);
pub const EMPTY_STMT: NodeType = NodeType(242);
pub const EXPR_STMT: NodeType = NodeType(243);
pub const IF_EXPR: NodeType = NodeType(244);
pub const WHILE_EXPR: NodeType = NodeType(245);
pub const LOOP_EXPR: NodeType = NodeType(246);
pub const FOR_EXPR: NodeType = NodeType(247);
pub const MATCH_EXPR: NodeType = NodeType(248);
pub const MATCH_ARM: NodeType = NodeType(249);
pub const GUARD: NodeType = NodeType(250);
pub const BLOCK_MACRO_EXPR: NodeType = NodeType(251);
pub const LINE_MACRO_EXPR: NodeType = NodeType(252);
pub const METHOD_CALL_EXPR: NodeType = NodeType(253);
pub const CALL_EXPR: NodeType = NodeType(254);
pub const FIELD_EXPR: NodeType = NodeType(255);
pub const INDEX_EXPR: NodeType = NodeType(256);
pub const VALUE_ARGUMENT: NodeType = NodeType(257);
pub const REFERENCE_EXPR: NodeType = NodeType(258);
pub const DEREFERENCE_EXPR: NodeType = NodeType(259);
pub const NEGATION_EXPR: NodeType = NodeType(260);
pub const NOT_EXPR: NodeType = NodeType(261);
pub const PRODUCT_EXPR: NodeType = NodeType(262);
pub const SUM_EXPR: NodeType = NodeType(263);
pub const BIT_SHIFT: NodeType = NodeType(264);
pub const BIT_AND: NodeType = NodeType(265);
pub const BIT_OR: NodeType = NodeType(266);
pub const COMPARISON: NodeType = NodeType(267);
pub const LOGICAL_AND: NodeType = NodeType(268);
pub const LOGICAL_OR: NodeType = NodeType(269);
pub const RANGE_EXPR: NodeType = NodeType(270);
pub const ASSIGNMENT_EXPR: NodeType = NodeType(271);
pub const ATTRIBUTE: NodeType = NodeType(272);
pub const ATTR_VALUE: NodeType = NodeType(273);
pub const BLOCK_MACRO: NodeType = NodeType(274);
pub const LINE_MACRO: NodeType = NodeType(275);
pub const TT: NodeType = NodeType(276);


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"[{"body":{"Pub":{"ty_idx":78,"body":{"Or":[{"And":[[{"Rule":1}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":11}],null]},{"And":[[{"ContextualToken":[4,"union"]}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":12}],null]},{"And":[[{"Token":13}],null]},{"And":[[{"Token":17}],null]},{"And":[[{"Token":29}],null]},{"And":[[{"Rule":25}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":24}],null]},{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":15}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":20}],null]},{"And":[[{"Rule":21}],null]},{"And":[[{"Rule":22}],null]},{"And":[[{"Rule":29}],null]}]}},{"body":{"Pub":{"ty_idx":79,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":12},{"Or":[{"And":[[{"Rule":38},{"Or":[{"And":[[{"Rule":47}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":56},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":56}},{"Rule":5}],null]}]},{"Token":54}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":80,"body":{"Or":[{"And":[[{"Token":63}],null]},{"And":[[{"Call":[{"Rule":133},[[3,{"Call":[{"Rule":132},[[2,{"Rule":6}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":81,"body":{"Or":[{"And":[[{"Token":18}],null]},{"And":[[{"Token":74},{"Opt":{"Rule":47}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":82,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":7},{"Token":6},{"Token":74},{"Opt":{"Rule":47}},{"Token":54}],4]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":83,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Opt":{"Rule":10}},{"Token":8},{"Token":74},{"Opt":{"Rule":31}},{"Rule":11},{"Opt":{"Rule":9}},{"Opt":{"Rule":37}},{"Or":[{"And":[[{"Rule":84}],null]},{"And":[[{"Token":54}],null]}]}],4]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":47},{"Rule":48}],null]}]}},{"body":{"Pub":{"ty_idx":84,"body":{"Or":[{"And":[[{"Token":7},{"Opt":{"Token":76}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Opt":{"Rule":14}},{"Call":[{"Rule":132},[[2,{"Rule":12}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":85,"body":{"Or":[{"And":[[{"Rule":58},{"Token":55},{"Rule":48}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":86,"body":{"Or":[{"And":[[{"Rule":58},{"Opt":{"Or":[{"And":[[{"Token":55},{"Rule":48}],null]}]}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":87,"body":{"Or":[{"And":[[{"Opt":{"Token":68}},{"Opt":{"Token":27}},{"Token":18},{"Opt":{"Or":[{"And":[[{"Token":55},{"Rule":48}],null]}]}},{"Or":[{"And":[[{"Token":57}],null]},{"And":[["Eof"],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":88,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Or":[{"And":[[{"Token":11}],null]},{"And":[[{"ContextualToken":[4,"union"]}],null]}]},{"Token":74},{"Opt":{"Rule":31}},{"Or":[{"And":[[{"Call":[{"Rule":133},[[3,{"Call":[{"Rule":132},[[2,{"Rule":16}]]]}]]]}],null]},{"And":[[{"Token":54}],null]},{"And":[[{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":17}]]]}]]]},{"Token":54}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":89,"body":{"Or":[{"And":[[{"Opt":{"Rule":36}},{"Token":74},{"Token":55},{"Rule":48}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":90,"body":{"Or":[{"And":[[{"Opt":{"Rule":36}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":91,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":16},{"Token":74},{"Call":[{"Rule":133},[[3,{"Call":[{"Rule":132},[[2,{"Rule":19}]]]}]]]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":92,"body":{"Or":[{"And":[[{"Token":74},{"Opt":{"Or":[{"And":[[{"Token":49},{"Rule":70}],null]},{"And":[[{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":17}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":133},[[3,{"Call":[{"Rule":132},[[2,{"Rule":16}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":93,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":13},{"Token":74},{"Or":[{"And":[[{"Token":54}],null]},{"And":[[{"Call":[{"Rule":133},[[3,{"Rule":1}]]]}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":94,"body":{"Or":[{"And":[[{"Rule":127},{"Token":17},{"Opt":{"Rule":31}},{"Or":[{"And":[[{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":23},{"Rule":48}],null]}]}}],null]}]},{"Rule":23}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":95,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":29},{"Token":74},{"Opt":{"Rule":31}},{"Rule":23}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":96,"body":{"Or":[{"And":[[{"Call":[{"Rule":133},[[3,{"Rep":{"WithSkip":[{"Rule":25},{"Rule":24}]}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]},{"And":[[{"Rule":28}],null]}]}},{"body":{"Or":[{"And":[[{"Token":10}],null]},{"And":[[{"Token":8}],null]},{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]},{"And":[[{"Token":61}],null]},{"And":[[{"Token":7}],null]},{"And":[[{"Or":[{"And":[[{"Token":74},{"Token":71}],null]}]}],null]}]}},{"body":{"Pub":{"ty_idx":97,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Token":20},{"Token":74},{"Opt":{"Rule":31}},{"Opt":{"Or":[{"And":[[{"Token":49},{"Rule":48}],null]}]}},{"Token":54}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":98,"body":{"Or":[{"And":[[{"Rule":127},{"Opt":{"Rule":36}},{"Or":[{"And":[[{"Token":21}],null]},{"And":[[{"Token":22}],null]}]},{"Token":74},{"Token":55},{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":49},{"Rule":70}],null]}]}},{"Token":54}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":99,"body":{"Or":[{"And":[[{"Rule":129}],null]},{"And":[[{"Rule":130},{"Token":54}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":100,"body":{"Or":[{"And":[[{"Rule":10},{"Call":[{"Rule":133},[[3,{"Rep":{"Rule":30}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":27}],null]}]}},{"body":{"Pub":{"ty_idx":101,"body":{"Or":[{"And":[[{"Call":[{"Rule":135},[[5,{"Or":[{"And":[[{"Call":[{"Rule":132},[[2,{"Rule":35}]]]},{"Call":[{"Rule":132},[[2,{"Rule":32}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":102,"body":{"Or":[{"And":[[{"Token":74},{"Opt":{"Rule":33}}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":55},{"Rep":{"Or":[{"And":[[{"Rule":34},{"Or":[{"And":[[{"Token":66}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":57}}}],null]},{"And":[[{"Not":{"Not":{"Token":37}}}],null]}]}],1]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":103,"body":{"Or":[{"And":[[{"Token":73}],null]},{"And":[[{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":104,"body":{"Or":[{"And":[[{"Token":73},{"Opt":{"Or":[{"And":[[{"Token":55},{"Rep":{"Or":[{"And":[[{"Token":73},{"Or":[{"And":[[{"Token":66}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":57}}}],null]}]}],1]}]}}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":105,"body":{"Or":[{"And":[[{"Token":10}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":106,"body":{"Or":[{"And":[[{"Token":34},{"Rep":{"Or":[{"And":[[{"Rule":48},{"Rule":33},{"Or":[{"And":[[{"Token":57}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":37}}}],null]}]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":41}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":41}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":41}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":107,"body":{"Or":[{"And":[[{"Opt":{"Token":56}},{"Rule":44}],null]}]},"replaceable":false}}}},{"Postfix":{"ty":107,"op":{"Or":[{"And":[[{"Token":56},{"Rule":44}],null]}]}}}]}},{"body":{"Pub":{"ty_idx":107,"body":{"Or":[{"And":[[{"Opt":{"Token":56}},{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":107,"body":{"Or":[{"And":[[{"Rule":41},{"Or":[{"And":[[{"Token":56},{"Rule":44}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":74}],null]},{"And":[[{"Token":18}],null]},{"And":[[{"Token":19}],null]}]},{"Opt":{"Or":[{"And":[[{"Rule":45}],null]},{"And":[[{"IsIn":0},{"Rule":46}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":108,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":56}],null]}]},{"Call":[{"Rule":135},[[5,{"Or":[{"And":[[{"Call":[{"Rule":132},[[2,{"Token":73}]]]},{"Call":[{"Rule":132},[[2,{"Rule":48}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":109,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":48}]]]}]]]},{"Opt":{"Rule":9}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":110,"body":{"Or":[{"And":[[{"Token":5},{"Token":74}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":49}],null]},{"And":[[{"Rule":50}],null]},{"And":[[{"Rule":51}],null]},{"And":[[{"Rule":52}],null]},{"And":[[{"Rule":53}],null]},{"And":[[{"Rule":55}],null]},{"And":[[{"Rule":56}],null]}]}},{"body":{"Pub":{"ty_idx":111,"body":{"Or":[{"And":[[{"Rule":39}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":112,"body":{"Or":[{"And":[[{"Token":68},{"Opt":{"Token":73}},{"Opt":{"Token":27}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":113,"body":{"Or":[{"And":[[{"Token":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":114,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":115,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Rule":48},{"Opt":{"Rule":54}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":116,"body":{"Or":[{"And":[[{"Token":57},{"Call":[{"Rule":132},[[2,{"Rule":48}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":117,"body":{"Or":[{"And":[[{"Call":[{"Rule":136},[[6,{"Or":[{"And":[[{"Rule":48},{"Opt":{"Or":[{"And":[[{"Token":54},{"Rule":70}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":118,"body":{"Or":[{"And":[[{"Token":8},{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":57}]]]}]]]},{"Rule":9}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":85,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Rule":58},{"Token":55}],null]}]}},{"Rule":48}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":59}],null]},{"And":[[{"Rule":60}],null]},{"And":[[{"Rule":64}],null]},{"And":[[{"Rule":65}],null]},{"And":[[{"Rule":66}],null]},{"And":[[{"Rule":67}],null]},{"And":[[{"Rule":69}],null]}]}},{"body":{"Pub":{"ty_idx":119,"body":{"Or":[{"And":[[{"Token":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":120,"body":{"Or":[{"And":[[{"Rule":40},{"Opt":{"Or":[{"And":[[{"Rule":61}],null]},{"And":[[{"Rule":62}],null]}]}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":121,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Call":[{"Rule":132},[[2,{"Rule":58}]]]},{"Opt":{"Or":[{"And":[[{"Token":59},{"Opt":{"Token":57}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"PubReplace":{"ty_idx":122,"body":{"Or":[{"And":[[{"Call":[{"Rule":133},[[3,{"Or":[{"And":[[{"Call":[{"Rule":132},[[2,{"Rule":63}]]]},{"Opt":{"Or":[{"And":[[{"Token":59},{"Opt":{"Token":57}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":123,"body":{"Or":[{"And":[[{"Rule":64},{"Not":{"Token":55}}],null]},{"And":[[{"Token":74},{"Token":55},{"Rule":58}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":124,"body":{"Or":[{"And":[[{"Opt":{"Token":28}},{"Opt":{"Token":27}},{"Token":74}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":125,"body":{"Or":[{"And":[[{"Rule":73}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":126,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":127,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Rule":58},{"Opt":{"Rule":68}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":128,"body":{"Or":[{"And":[[{"Token":57},{"Call":[{"Rule":132},[[2,{"Rule":58}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":129,"body":{"Or":[{"And":[[{"Token":68},{"Opt":{"Token":27}},{"Rule":58}],null]}]},"replaceable":false}}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Token":75}],null]},{"And":[[{"Token":76}],null]},{"And":[[{"Token":77}],null]},{"And":[[{"Token":72}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":74},{"Token":71}],null]}]}},{"Rule":40},{"Opt":{"Rule":75}}],null]}]},"replaceable":true}}}},{"Atom":{"body":{"Pub":{"ty_idx":135,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":136,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Call":[{"Rule":72},[[0,{"Rule":70}]]]},{"Opt":{"Rule":79}}],null]}]}]]]}],null]}]},"replaceable":true}}}},{"Atom":{"body":{"Pub":{"ty_idx":138,"body":{"Or":[{"And":[[{"Call":[{"Rule":136},[[6,{"Call":[{"Rule":72},[[0,{"Call":[{"Rule":132},[[2,{"Rule":70}]]]}]]]}]]]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":139,"body":{"Or":[{"And":[[{"Opt":{"Token":26}},{"Token":69},{"Rep":{"Rule":82}},{"Token":69},{"Or":[{"And":[[{"Token":47},{"Rule":48},{"Rule":84}],null]},{"And":[[{"Call":[{"Rule":72},[[0,{"Rule":70}]]]}],null]}]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":140,"body":{"Or":[{"And":[[{"Token":31},{"Opt":{"Rule":70}}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":141,"body":{"Or":[{"And":[[{"Call":[{"Rule":72},[[0,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Call":[{"Rule":133},[[3,{"Or":[{"And":[[{"Rep":{"Rule":85}},{"Opt":{"Rule":70}}],null]}]}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":145,"body":{"Or":[{"And":[[{"Token":14},{"Rule":91},{"Rule":84},{"Opt":{"Or":[{"And":[[{"Token":15},{"Or":[{"And":[[{"Rule":84}],null]},{"And":[[{"Rule":89}],null]}]}],null]}]}}],1]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":146,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":25},{"Rule":91},{"Rule":84}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":147,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":24},{"Rule":84}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":148,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":23},{"Rule":58},{"Token":32},{"Rule":92},{"Rule":84}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":149,"body":{"Or":[{"And":[[{"Token":30},{"Rule":92},{"Call":[{"Rule":133},[[3,{"Rep":{"Rule":97}}]]]}],1]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":152,"body":{"Or":[{"And":[[{"Rule":129}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":153,"body":{"Or":[{"And":[[{"Rule":130}],null]}]},"replaceable":false}}}},{"Postfix":{"ty":154,"op":{"Or":[{"And":[[{"Token":58},{"Token":74},{"Opt":{"Enter":[1,{"Rule":45}]}},{"Rule":104}],null]}]}}},{"Postfix":{"ty":155,"op":{"Rule":104}}},{"Postfix":{"ty":156,"op":{"Or":[{"And":[[{"Token":58},{"Or":[{"And":[[{"Token":74}],null]},{"And":[[{"Token":75}],null]}]}],null]}]}}},{"Postfix":{"ty":157,"op":{"Call":[{"Rule":136},[[6,{"Rule":70}]]]}}},{"Prefix":{"ty":159,"op":{"Or":[{"And":[[{"Token":68},{"Opt":{"Token":27}}],null]}]},"priority":999}},{"Prefix":{"ty":160,"op":{"Token":63},"priority":999}},{"Prefix":{"ty":161,"op":{"Token":67},"priority":999}},{"Prefix":{"ty":162,"op":{"Token":71},"priority":999}},{"Binary":{"ty":163,"op":{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":63}],null]},{"And":[[{"Token":64}],null]},{"And":[[{"Token":65}],null]}]}]]]},"priority":10}},{"Binary":{"ty":164,"op":{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":66}],null]},{"And":[[{"Token":67}],null]}]}]]]},"priority":9}},{"Binary":{"ty":165,"op":{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"ContextualToken":[43,"<<"]}],null]},{"And":[[{"ContextualToken":[44,">>"]}],null]}]}]]]},"priority":8}},{"Binary":{"ty":166,"op":{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":68},{"Not":{"Token":68}}],null]}]}]]]},"priority":7}},{"Binary":{"ty":167,"op":{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":69},{"Not":{"Token":69}}],null]}]}]]]},"priority":6}},{"Binary":{"ty":168,"op":{"Call":[{"Rule":115},[[1,{"Rule":119}]]]},"priority":5}},{"Binary":{"ty":169,"op":{"Call":[{"Rule":115},[[1,{"ContextualToken":[45,"&&"]}]]]},"priority":4}},{"Binary":{"ty":170,"op":{"Call":[{"Rule":115},[[1,{"ContextualToken":[46,"||"]}]]]},"priority":3}},{"Binary":{"ty":171,"op":{"Call":[{"Rule":115},[[1,{"Rule":124}]]]},"priority":2}},{"Prefix":{"ty":171,"op":{"Rule":124},"priority":2}},{"Binary":{"ty":172,"op":{"Call":[{"Rule":115},[[1,{"Token":49}]]]},"priority":1}}]}},{"body":{"Or":[{"And":[[{"PrevIs":[141,145,146,147,148,149,152]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[6,{"Exit":[5,{"Var":0}]}]}],null]}]}},{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Token":75}],null]},{"And":[[{"Token":76}],null]},{"And":[[{"Token":77}],null]},{"And":[[{"Token":72}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":74},{"Token":71}],null]}]}},{"Rule":40},{"Opt":{"Rule":75}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":133,"body":{"Or":[{"And":[[{"Not":{"IsIn":6}},{"Call":[{"Rule":133},[[3,{"Or":[{"And":[[{"Call":[{"Rule":132},[[2,{"Rule":76}]]]},{"Opt":{"Or":[{"And":[[{"Token":59},{"Call":[{"Rule":72},[[0,{"Rule":70}]]]}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":134,"body":{"Or":[{"And":[[{"Token":74},{"Opt":{"Or":[{"And":[[{"Token":55},{"Rule":70}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":135,"body":{"Or":[{"And":[[{"Token":35},{"Token":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":136,"body":{"Or":[{"And":[[{"Call":[{"Rule":134},[[4,{"Or":[{"And":[[{"Call":[{"Rule":72},[[0,{"Rule":70}]]]},{"Opt":{"Rule":79}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":137,"body":{"Or":[{"And":[[{"Token":57},{"Call":[{"Rule":132},[[2,{"Call":[{"Rule":72},[[0,{"Rule":70}]]]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":138,"body":{"Or":[{"And":[[{"Call":[{"Rule":136},[[6,{"Call":[{"Rule":72},[[0,{"Call":[{"Rule":132},[[2,{"Rule":70}]]]}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":139,"body":{"Or":[{"And":[[{"Opt":{"Token":26}},{"Token":69},{"Rep":{"Rule":82}},{"Token":69},{"Or":[{"And":[[{"Token":47},{"Rule":48},{"Rule":84}],null]},{"And":[[{"Call":[{"Rule":72},[[0,{"Rule":70}]]]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":13},{"Or":[{"And":[[{"Token":57}],null]},{"And":[[{"Not":{"Not":{"Token":69}}}],null]}]}],1]}]}},{"body":{"Pub":{"ty_idx":140,"body":{"Or":[{"And":[[{"Token":31},{"Opt":{"Rule":70}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":141,"body":{"Or":[{"And":[[{"Call":[{"Rule":72},[[0,{"Or":[{"And":[[{"Opt":{"Token":33}},{"Call":[{"Rule":133},[[3,{"Or":[{"And":[[{"Rep":{"Rule":85}},{"Opt":{"Rule":70}}],null]}]}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":86}],null]},{"And":[[{"Rule":88}],null]},{"And":[[{"Rule":87}],null]},{"And":[[{"Rule":3}],null]}]}},{"body":{"Pub":{"ty_idx":142,"body":{"Or":[{"And":[[{"Token":9},{"Rule":58},{"Token":49},{"Rule":70},{"Token":54}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":143,"body":{"Or":[{"And":[[{"Token":54}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":144,"body":{"Or":[{"And":[[{"Enter":[5,{"Or":[{"And":[[{"Rule":70},{"Or":[{"And":[[{"Rule":71},{"Not":"Eof"}],null]},{"And":[[{"Token":54}],null]}]}],null]}]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":145,"body":{"Or":[{"And":[[{"Token":14},{"Rule":91},{"Rule":84},{"Opt":{"Or":[{"And":[[{"Token":15},{"Or":[{"And":[[{"Rule":84}],null]},{"And":[[{"Rule":89}],null]}]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":146,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":25},{"Rule":91},{"Rule":84}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":9},{"Rule":58},{"Token":49}],1]}]}},{"Rule":92}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[6,{"Rule":70}]}],null]}]}},{"body":{"Pub":{"ty_idx":147,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":24},{"Rule":84}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":148,"body":{"Or":[{"And":[[{"Opt":{"Rule":95}},{"Token":23},{"Rule":58},{"Token":32},{"Rule":92},{"Rule":84}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":73},{"Token":55}],null]}]}},{"body":{"Pub":{"ty_idx":149,"body":{"Or":[{"And":[[{"Token":30},{"Rule":92},{"Call":[{"Rule":133},[[3,{"Rep":{"Rule":97}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":150,"body":{"Or":[{"And":[[{"Rule":98},{"Token":48},{"Rule":70},{"Or":[{"And":[[{"Token":57}],null]},{"And":[["Eof"],null]},{"And":[[{"Rule":71}],null]}]}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":58},{"Rep":{"Or":[{"And":[[{"Token":69},{"Rule":58}],null]}]}},{"Opt":{"Rule":99}}],null]}]}},{"body":{"Pub":{"ty_idx":151,"body":{"Or":[{"And":[[{"Token":14},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":152,"body":{"Or":[{"And":[[{"Rule":129}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":153,"body":{"Or":[{"And":[[{"Rule":130}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":154,"body":{"Or":[{"And":[[{"Rule":70},{"Or":[{"And":[[{"Token":58},{"Token":74},{"Opt":{"Enter":[1,{"Rule":45}]}},{"Rule":104}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":155,"body":{"Or":[{"And":[[{"Rule":70},{"Rule":104}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":72},[[0,{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":107}]]]}]]]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":156,"body":{"Or":[{"And":[[{"Rule":70},{"Or":[{"And":[[{"Token":58},{"Or":[{"And":[[{"Token":74}],null]},{"And":[[{"Token":75}],null]}]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":157,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":136},[[6,{"Rule":70}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":158,"body":{"Or":[{"And":[[{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":159,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":68},{"Opt":{"Token":27}}],null]}]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":160,"body":{"Or":[{"And":[[{"Token":63},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":161,"body":{"Or":[{"And":[[{"Token":67},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":162,"body":{"Or":[{"And":[[{"Token":71},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":163,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":63}],null]},{"And":[[{"Token":64}],null]},{"And":[[{"Token":65}],null]}]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":164,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":66}],null]},{"And":[[{"Token":67}],null]}]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":165,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"ContextualToken":[43,"<<"]}],null]},{"And":[[{"ContextualToken":[44,">>"]}],null]}]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"IsIn":5},{"Not":{"Rule":71}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":5}},{"Var":1}],null]}]}},{"body":{"Pub":{"ty_idx":166,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":68},{"Not":{"Token":68}}],null]}]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":167,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Or":[{"And":[[{"Token":69},{"Not":{"Token":69}}],null]}]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":168,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Rule":119}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":50}],null]},{"And":[[{"Token":51}],null]},{"And":[[{"Token":39}],null]},{"And":[[{"Token":40}],null]},{"And":[[{"Token":53}],null]},{"And":[[{"Token":52}],null]}]}},{"body":{"Pub":{"ty_idx":169,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"ContextualToken":[45,"&&"]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":170,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"ContextualToken":[46,"||"]}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":171,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Rule":124}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":171,"body":{"Or":[{"And":[[{"Rule":124},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":59}],null]},{"And":[[{"Token":60}],null]}]},{"Not":{"Or":[{"And":[[{"Token":37},{"IsIn":6}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":172,"body":{"Or":[{"And":[[{"Rule":70},{"Call":[{"Rule":115},[[1,{"Token":49}]]]},{"Rule":70}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":173,"body":{"Or":[{"And":[[{"Token":61},{"Call":[{"Rule":136},[[6,{"Call":[{"Rule":132},[[2,{"Rule":128}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":126}}],null]}]}},{"body":{"Pub":{"ty_idx":174,"body":{"Or":[{"And":[[{"Token":74},{"Opt":{"Or":[{"And":[[{"Token":49},{"Rule":70}],null]},{"And":[[{"Call":[{"Rule":134},[[4,{"Call":[{"Rule":132},[[2,{"Rule":128}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":175,"body":{"Or":[{"And":[[{"Token":74},{"Token":71},{"Opt":{"Token":74}},{"Call":[{"Rule":133},[[3,{"Rep":{"Rule":131}}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":176,"body":{"Or":[{"And":[[{"Token":74},{"Token":71},{"Opt":{"Token":74}},{"Or":[{"And":[[{"Token":35},{"Rep":{"Rule":131}},{"Token":36}],null]},{"And":[[{"Token":41},{"Rep":{"Rule":131}},{"Token":42}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":177,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":35}],null]},{"And":[[{"Token":36}],null]},{"And":[[{"Token":37}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":41}],null]},{"And":[[{"Token":42}],null]}]}},"Any"],null]},{"And":[[{"Token":35},{"Rep":{"Rule":131}},{"Token":36}],null]},{"And":[[{"Token":41},{"Rep":{"Rule":131}},{"Token":42}],null]},{"And":[[{"Token":37},{"Rep":{"Rule":131}},{"Token":38}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":57}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":137},[[7,{"Token":37}],[8,{"Token":38}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":137},[[7,{"Token":35}],[8,{"Token":36}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":137},[[7,{"Token":39}],[8,{"Token":40}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":137},[[7,{"Token":41}],[8,{"Token":42}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":138},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":138},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            WHITESPACE, LINE_COMMENT, BLOCK_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, UNSAFE, WHERE, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHR, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, DOLLAR, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, BANG, CHAR, LIFETIME, IDENT, NUMBER, STRING, RAW_STRING, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, MACRO_ITEM, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, WHERE_CLAUSE, PATH, TYPE_ARGUMENTS, FN_TRAIT_SUGAR, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, ARRAY_LITERAL, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, FIELD_EXPR, INDEX_EXPR, VALUE_ARGUMENT, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, NOT_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
        ],
        lexical_rules: vec![
            LexRule::new(WHITESPACE, "\\s+", None),
            LexRule::new(LINE_COMMENT, "//.*\\n?", None),
            LexRule::new(BLOCK_COMMENT, "/\\*", Some(parse_block_comment)),
            LexRule::new(AS, "as", None),
            LexRule::new(CRATE, "crate", None),
            LexRule::new(EXTERN, "extern", None),
            LexRule::new(FN, "fn", None),
            LexRule::new(LET, "let", None),
            LexRule::new(PUB, "pub", None),
            LexRule::new(STRUCT, "struct", None),
            LexRule::new(USE, "use", None),
            LexRule::new(MOD, "mod", None),
            LexRule::new(IF, "if", None),
            LexRule::new(ELSE, "else", None),
            LexRule::new(ENUM, "enum", None),
            LexRule::new(IMPL, "impl", None),
            LexRule::new(SELF, "self", None),
            LexRule::new(SUPER, "super", None),
            LexRule::new(TYPE, "type", None),
            LexRule::new(CONST, "const", None),
            LexRule::new(STATIC, "static", None),
            LexRule::new(FOR, "for", None),
            LexRule::new(LOOP, "loop", None),
            LexRule::new(WHILE, "while", None),
            LexRule::new(MOVE, "move", None),
            LexRule::new(MUT, "mut", None),
            LexRule::new(REF, "ref", None),
            LexRule::new(TRAIT, "trait", None),
            LexRule::new(MATCH, "match", None),
            LexRule::new(RETURN, "return", None),
            LexRule::new(IN, "in", None),
            LexRule::new(UNSAFE, "unsafe", None),
            LexRule::new(WHERE, "where", None),
            LexRule::new(L_PAREN, "\\(", None),
            LexRule::new(R_PAREN, "\\)", None),
            LexRule::new(L_CURLY, "\\{", None),
            LexRule::new(R_CURLY, "\\}", None),
            LexRule::new(L_ANGLE, "<", None),
            LexRule::new(R_ANGLE, ">", None),
            LexRule::new(L_BRACK, "\\[", None),
            LexRule::new(R_BRACK, "\\]", None),
            LexRule::new(THIN_ARROW, "\\->", None),
            LexRule::new(FAT_ARROW, "=>", None),
            LexRule::new(EQ, "=", None),
            LexRule::new(EQEQ, "==", None),
            LexRule::new(BANGEQ, "!=", None),
            LexRule::new(GTET, ">=", None),
            LexRule::new(LTEQ, "<=", None),
            LexRule::new(SEMI, ";", None),
            LexRule::new(COLON, ":", None),
            LexRule::new(COLONCOLON, "::", None),
            LexRule::new(COMMA, ",", None),
            LexRule::new(DOT, "\\.", None),
            LexRule::new(DOTDOT, "\\.\\.", None),
            LexRule::new(DOTDOTDOT, "\\.\\.\\.", None),
            LexRule::new(HASH, "\\#", None),
            LexRule::new(DOLLAR, "\\$", None),
            LexRule::new(STAR, "\\*", None),
            LexRule::new(SLASH, "/", None),
            LexRule::new(PERCENT, "%", None),
            LexRule::new(PLUS, "\\+", None),
            LexRule::new(MINUS, "\\-", None),
            LexRule::new(AMPERSAND, "\\&", None),
            LexRule::new(PIPE, "\\|", None),
            LexRule::new(UNDERSCORE, "_", None),
            LexRule::new(BANG, "!", None),
            LexRule::new(CHAR, "\'(\\s*|\\S*)\'", None),
            LexRule::new(LIFETIME, "\'\\p{XID_Continue}*", None),
            LexRule::new(IDENT, "\\p{XID_Start}\\w*", None),
            LexRule::new(NUMBER, "\\d+", None),
            LexRule::new(STRING, "\"([^\"]|\\\\\")*\"", None),
            LexRule::new(RAW_STRING, "r#*\"", Some(parse_raw_string)),
        ],
        syntactical_rules: serde_json::from_str(parser_json).unwrap(),
        whitespace_binder: whitespace_binder,
        .. Default::default()
    }
}

pub fn language() -> &'static Language {
    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition };
            impl LanguageImpl for Impl {
                fn parse(&self, text: &str) -> (FileStats, INode) {
                    self.parser_definition.parse(text, &LANG)
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
                        SHR => NodeTypeInfo { name: "SHR", whitespace_like: false },
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
                        SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                        PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        AMPERSAND => NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                        PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        UNDERSCORE => NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                        BANG => NodeTypeInfo { name: "BANG", whitespace_like: false },
                        CHAR => NodeTypeInfo { name: "CHAR", whitespace_like: false },
                        LIFETIME => NodeTypeInfo { name: "LIFETIME", whitespace_like: false },
                        IDENT => NodeTypeInfo { name: "IDENT", whitespace_like: false },
                        NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                        RAW_STRING => NodeTypeInfo { name: "RAW_STRING", whitespace_like: false },
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
                        FIELD_EXPR => NodeTypeInfo { name: "FIELD_EXPR", whitespace_like: false },
                        INDEX_EXPR => NodeTypeInfo { name: "INDEX_EXPR", whitespace_like: false },
                        VALUE_ARGUMENT => NodeTypeInfo { name: "VALUE_ARGUMENT", whitespace_like: false },
                        REFERENCE_EXPR => NodeTypeInfo { name: "REFERENCE_EXPR", whitespace_like: false },
                        DEREFERENCE_EXPR => NodeTypeInfo { name: "DEREFERENCE_EXPR", whitespace_like: false },
                        NEGATION_EXPR => NodeTypeInfo { name: "NEGATION_EXPR", whitespace_like: false },
                        NOT_EXPR => NodeTypeInfo { name: "NOT_EXPR", whitespace_like: false },
                        PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                        SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                        BIT_SHIFT => NodeTypeInfo { name: "BIT_SHIFT", whitespace_like: false },
                        BIT_AND => NodeTypeInfo { name: "BIT_AND", whitespace_like: false },
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

            Language::new(Impl { parser_definition: create_parser_definition() })
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

