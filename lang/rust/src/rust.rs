use fall_parse::runtime::*;
use self::fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl, FileStats, INode};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LINE_COMMENT: NodeType = NodeType(101);
pub const UNION: NodeType = NodeType(102);
pub const AS: NodeType = NodeType(103);
pub const CRATE: NodeType = NodeType(104);
pub const EXTERN: NodeType = NodeType(105);
pub const FN: NodeType = NodeType(106);
pub const LET: NodeType = NodeType(107);
pub const PUB: NodeType = NodeType(108);
pub const STRUCT: NodeType = NodeType(109);
pub const USE: NodeType = NodeType(110);
pub const MOD: NodeType = NodeType(111);
pub const IF: NodeType = NodeType(112);
pub const ELSE: NodeType = NodeType(113);
pub const ENUM: NodeType = NodeType(114);
pub const IMPL: NodeType = NodeType(115);
pub const SELF: NodeType = NodeType(116);
pub const SUPER: NodeType = NodeType(117);
pub const TYPE: NodeType = NodeType(118);
pub const CONST: NodeType = NodeType(119);
pub const STATIC: NodeType = NodeType(120);
pub const FOR: NodeType = NodeType(121);
pub const LOOP: NodeType = NodeType(122);
pub const WHILE: NodeType = NodeType(123);
pub const MOVE: NodeType = NodeType(124);
pub const MUT: NodeType = NodeType(125);
pub const REF: NodeType = NodeType(126);
pub const TRAIT: NodeType = NodeType(127);
pub const MATCH: NodeType = NodeType(128);
pub const RETURN: NodeType = NodeType(129);
pub const IN: NodeType = NodeType(130);
pub const L_PAREN: NodeType = NodeType(131);
pub const R_PAREN: NodeType = NodeType(132);
pub const L_CURLY: NodeType = NodeType(133);
pub const R_CURLY: NodeType = NodeType(134);
pub const L_ANGLE: NodeType = NodeType(135);
pub const R_ANGLE: NodeType = NodeType(136);
pub const L_BRACK: NodeType = NodeType(137);
pub const R_BRACK: NodeType = NodeType(138);
pub const SHL: NodeType = NodeType(139);
pub const SHR: NodeType = NodeType(140);
pub const AND: NodeType = NodeType(141);
pub const OR: NodeType = NodeType(142);
pub const THIN_ARROW: NodeType = NodeType(143);
pub const FAT_ARROW: NodeType = NodeType(144);
pub const EQ: NodeType = NodeType(145);
pub const EQEQ: NodeType = NodeType(146);
pub const BANGEQ: NodeType = NodeType(147);
pub const GTET: NodeType = NodeType(148);
pub const LTEQ: NodeType = NodeType(149);
pub const SEMI: NodeType = NodeType(150);
pub const COLON: NodeType = NodeType(151);
pub const COLONCOLON: NodeType = NodeType(152);
pub const COMMA: NodeType = NodeType(153);
pub const DOT: NodeType = NodeType(154);
pub const DOTDOT: NodeType = NodeType(155);
pub const DOTDOTDOT: NodeType = NodeType(156);
pub const HASH: NodeType = NodeType(157);
pub const STAR: NodeType = NodeType(158);
pub const SLASH: NodeType = NodeType(159);
pub const PERCENT: NodeType = NodeType(160);
pub const PLUS: NodeType = NodeType(161);
pub const MINUS: NodeType = NodeType(162);
pub const AMPERSAND: NodeType = NodeType(163);
pub const PIPE: NodeType = NodeType(164);
pub const UNDERSCORE: NodeType = NodeType(165);
pub const BANG: NodeType = NodeType(166);
pub const LIFETIME: NodeType = NodeType(167);
pub const IDENT: NodeType = NodeType(168);
pub const NUMBER: NodeType = NodeType(169);
pub const STRING: NodeType = NodeType(170);
pub const RAW_STRING: NodeType = NodeType(171);
pub const FILE: NodeType = NodeType(172);
pub const USE_DECL: NodeType = NodeType(173);
pub const USE_SPEC: NodeType = NodeType(174);
pub const USE_SPEC_ENTRY: NodeType = NodeType(175);
pub const EXTERN_CRATE_DECL: NodeType = NodeType(176);
pub const FN_DEF: NodeType = NodeType(177);
pub const LINKAGE: NodeType = NodeType(178);
pub const VALUE_PARAM: NodeType = NodeType(179);
pub const LAMBDA_VALUE_PARAM: NodeType = NodeType(180);
pub const SELF_PARAMETER: NodeType = NodeType(181);
pub const STRUCT_DEF: NodeType = NodeType(182);
pub const STRUCT_FIELD: NodeType = NodeType(183);
pub const TUPLE_FIELD: NodeType = NodeType(184);
pub const ENUM_DEF: NodeType = NodeType(185);
pub const ENUM_VARIANT: NodeType = NodeType(186);
pub const MOD_DEF: NodeType = NodeType(187);
pub const IMPL_DEF: NodeType = NodeType(188);
pub const TRAIT_DEF: NodeType = NodeType(189);
pub const MEMBERS: NodeType = NodeType(190);
pub const TYPE_DEF: NodeType = NodeType(191);
pub const CONST_DEF: NodeType = NodeType(192);
pub const EXTERN_BLOCK: NodeType = NodeType(193);
pub const TYPE_PARAMETERS: NodeType = NodeType(194);
pub const TYPE_PARAMETER: NodeType = NodeType(195);
pub const TYPE_BOUND: NodeType = NodeType(196);
pub const LIFETIME_PARAMETER: NodeType = NodeType(197);
pub const VISIBILITY: NodeType = NodeType(198);
pub const PATH: NodeType = NodeType(199);
pub const TYPE_ARGUMENTS: NodeType = NodeType(200);
pub const ALIAS: NodeType = NodeType(201);
pub const PATH_TYPE: NodeType = NodeType(202);
pub const REFERENCE_TYPE: NodeType = NodeType(203);
pub const PLACEHOLDER_TYPE: NodeType = NodeType(204);
pub const UNIT_TYPE: NodeType = NodeType(205);
pub const PAREN_TYPE: NodeType = NodeType(206);
pub const TUPLE_TYPE: NodeType = NodeType(207);
pub const ARRAY_TYPE: NodeType = NodeType(208);
pub const FN_POINTER_TYPE: NodeType = NodeType(209);
pub const WILDCARD_PATTERN: NodeType = NodeType(210);
pub const PATH_PATTERN: NodeType = NodeType(211);
pub const TUPE_STRUCT_PATTERN: NodeType = NodeType(212);
pub const STRUCT_PATTERN: NodeType = NodeType(213);
pub const STRUCT_PATTERN_FIELD: NodeType = NodeType(214);
pub const BINDING_PATTERN: NodeType = NodeType(215);
pub const LITERAL_PATTERN: NodeType = NodeType(216);
pub const UNIT_PATTERN: NodeType = NodeType(217);
pub const PAREN_PATTERN: NodeType = NodeType(218);
pub const TUPLE_PATTERN: NodeType = NodeType(219);
pub const REFERENCE_PATTERN: NodeType = NodeType(220);
pub const EXPR: NodeType = NodeType(221);
pub const LITERAL: NodeType = NodeType(222);
pub const PATH_EXPR: NodeType = NodeType(223);
pub const STRUCT_LITERAL: NodeType = NodeType(224);
pub const STRUCT_LITERAL_FIELD: NodeType = NodeType(225);
pub const UNIT_EXPR: NodeType = NodeType(226);
pub const PAREN_EXPR: NodeType = NodeType(227);
pub const TUPLE_EXPR: NodeType = NodeType(228);
pub const LAMBDA_EXPR: NodeType = NodeType(229);
pub const RETURN_EXPR: NodeType = NodeType(230);
pub const BLOCK_EXPR: NodeType = NodeType(231);
pub const LET_STMT: NodeType = NodeType(232);
pub const EMPTY_STMT: NodeType = NodeType(233);
pub const EXPR_STMT: NodeType = NodeType(234);
pub const IF_EXPR: NodeType = NodeType(235);
pub const WHILE_EXPR: NodeType = NodeType(236);
pub const LOOP_EXPR: NodeType = NodeType(237);
pub const FOR_EXPR: NodeType = NodeType(238);
pub const MATCH_EXPR: NodeType = NodeType(239);
pub const MATCH_ARM: NodeType = NodeType(240);
pub const GUARD: NodeType = NodeType(241);
pub const BLOCK_MACRO_EXPR: NodeType = NodeType(242);
pub const LINE_MACRO_EXPR: NodeType = NodeType(243);
pub const METHOD_CALL_EXPR: NodeType = NodeType(244);
pub const CALL_EXPR: NodeType = NodeType(245);
pub const FIELD_EXPR: NodeType = NodeType(246);
pub const INDEX_EXPR: NodeType = NodeType(247);
pub const VALUE_ARGUMENT: NodeType = NodeType(248);
pub const REFERENCE_EXPR: NodeType = NodeType(249);
pub const DEREFERENCE_EXPR: NodeType = NodeType(250);
pub const NEGATION_EXPR: NodeType = NodeType(251);
pub const PRODUCT_EXPR: NodeType = NodeType(252);
pub const SUM_EXPR: NodeType = NodeType(253);
pub const BIT_SHIFT: NodeType = NodeType(254);
pub const BIT_AND: NodeType = NodeType(255);
pub const BIT_OR: NodeType = NodeType(256);
pub const COMPARISON: NodeType = NodeType(257);
pub const LOGICAL_AND: NodeType = NodeType(258);
pub const LOGICAL_OR: NodeType = NodeType(259);
pub const RANGE_EXPR: NodeType = NodeType(260);
pub const ASSIGNMENT_EXPR: NodeType = NodeType(261);
pub const ATTRIBUTE: NodeType = NodeType(262);
pub const ATTR_VALUE: NodeType = NodeType(263);
pub const BLOCK_MACRO: NodeType = NodeType(264);
pub const LINE_MACRO: NodeType = NodeType(265);
pub const TT: NodeType = NodeType(266);


fn create_parser_definition() -> ::fall_parse::ParserDefinition {
    use fall_parse::LexRule;
    let parser_json = r##"[{"body":{"Pub":{"ty_idx":73,"body":{"Or":[{"And":[[{"Rule":1}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"WithSkip":[{"Rule":2},{"Rule":3}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Token":10}],null]},{"And":[[{"ContextualToken":[3,"union"]}],null]},{"And":[[{"Token":15}],null]},{"And":[[{"Token":11}],null]},{"And":[[{"Token":12}],null]},{"And":[[{"Token":16}],null]},{"And":[[{"Token":28}],null]},{"And":[[{"Rule":25}],null]}]}},{"body":{"Or":[{"And":[[{"Rule":4}],null]},{"And":[[{"Rule":7}],null]},{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":15}],null]},{"And":[[{"Rule":18}],null]},{"And":[[{"Rule":20}],null]},{"And":[[{"Rule":21}],null]},{"And":[[{"Rule":22}],null]},{"And":[[{"Rule":26}],null]},{"And":[[{"Rule":27}],null]},{"And":[[{"Rule":28}],null]}]}},{"body":{"Pub":{"ty_idx":74,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":11},{"Or":[{"And":[[{"Rule":35},{"Or":[{"And":[[{"Rule":43}],null]},{"And":[[{"Opt":{"Or":[{"And":[[{"Token":53},{"Rule":5}],null]}]}}],null]}]}],null]},{"And":[[{"Opt":{"Token":53}},{"Rule":5}],null]}]},{"Token":51}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":75,"body":{"Or":[{"And":[[{"Token":59}],null]},{"And":[[{"Call":[{"Rule":127},[[3,{"Call":[{"Rule":126},[[2,{"Rule":6}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":76,"body":{"Or":[{"And":[[{"Token":69},{"Opt":{"Rule":43}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":77,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":6},{"Token":5},{"Token":69},{"Opt":{"Rule":43}},{"Token":51}],4]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":78,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Opt":{"Rule":10}},{"Token":7},{"Token":69},{"Opt":{"Rule":30}},{"Rule":11},{"Opt":{"Rule":9}},{"Or":[{"And":[[{"Rule":79}],null]},{"And":[[{"Token":51}],null]}]}],4]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":44},{"Rule":44}],null]}]}},{"body":{"Pub":{"ty_idx":79,"body":{"Or":[{"And":[[{"Token":6},{"Opt":{"Token":71}}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Opt":{"Rule":14}},{"Call":[{"Rule":126},[[2,{"Rule":12}]]]}],null]}]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":80,"body":{"Or":[{"And":[[{"Rule":54},{"Token":52},{"Rule":44}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":81,"body":{"Or":[{"And":[[{"Rule":54},{"Opt":{"Or":[{"And":[[{"Token":52},{"Rule":44}],null]}]}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":82,"body":{"Or":[{"And":[[{"Opt":{"Token":64}},{"Opt":{"Token":26}},{"Token":17},{"Opt":{"Or":[{"And":[[{"Token":52},{"Rule":44}],null]}]}},{"Or":[{"And":[[{"Token":54}],null]},{"And":[["Eof"],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":83,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Or":[{"And":[[{"Token":10}],null]},{"And":[[{"ContextualToken":[3,"union"]}],null]}]},{"Token":69},{"Opt":{"Rule":30}},{"Or":[{"And":[[{"Call":[{"Rule":127},[[3,{"Call":[{"Rule":126},[[2,{"Rule":16}]]]}]]]}],null]},{"And":[[{"Token":51}],null]},{"And":[[{"Call":[{"Rule":128},[[4,{"Call":[{"Rule":126},[[2,{"Rule":17}]]]}]]]},{"Token":51}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":84,"body":{"Or":[{"And":[[{"Opt":{"Rule":34}},{"Token":69},{"Token":52},{"Rule":44}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":85,"body":{"Or":[{"And":[[{"Opt":{"Rule":34}},{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":86,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":15},{"Token":69},{"Call":[{"Rule":127},[[3,{"Call":[{"Rule":126},[[2,{"Rule":19}]]]}]]]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":87,"body":{"Or":[{"And":[[{"Token":69},{"Opt":{"Or":[{"And":[[{"Token":46},{"Rule":66}],null]},{"And":[[{"Call":[{"Rule":128},[[4,{"Call":[{"Rule":126},[[2,{"Rule":17}]]]}]]]}],null]},{"And":[[{"Call":[{"Rule":127},[[3,{"Call":[{"Rule":126},[[2,{"Rule":16}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":88,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":12},{"Token":69},{"Or":[{"And":[[{"Token":51}],null]},{"And":[[{"Call":[{"Rule":127},[[3,{"Rule":1}]]]}],null]}]}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":89,"body":{"Or":[{"And":[[{"Rule":121},{"Token":16},{"Opt":{"Rule":30}},{"Or":[{"And":[[{"Rule":44},{"Opt":{"Or":[{"And":[[{"Token":22},{"Rule":44}],null]}]}}],null]}]},{"Rule":23}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":90,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":28},{"Token":69},{"Opt":{"Rule":30}},{"Rule":23}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":91,"body":{"Or":[{"And":[[{"Call":[{"Rule":127},[[3,{"Rep":{"WithSkip":[{"Rule":25},{"Rule":24}]}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":26}],null]}]}},{"body":{"Or":[{"And":[[{"Token":9}],null]},{"And":[[{"Token":7}],null]},{"And":[[{"Token":19}],null]},{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]},{"And":[[{"Token":58}],null]},{"And":[[{"Token":6}],null]}]}},{"body":{"Pub":{"ty_idx":92,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Token":19},{"Token":69},{"Opt":{"Rule":30}},{"Opt":{"Or":[{"And":[[{"Token":46},{"Rule":44}],null]}]}},{"Token":51}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":93,"body":{"Or":[{"And":[[{"Rule":121},{"Opt":{"Rule":34}},{"Or":[{"And":[[{"Token":20}],null]},{"And":[[{"Token":21}],null]}]},{"Token":69},{"Token":52},{"Rule":44},{"Opt":{"Or":[{"And":[[{"Token":46},{"Rule":66}],null]}]}},{"Token":51}],3]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":94,"body":{"Or":[{"And":[[{"Rule":10},{"Call":[{"Rule":127},[[3,{"Rep":{"Rule":29}}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":8}],null]},{"And":[[{"Rule":27}],null]}]}},{"body":{"Pub":{"ty_idx":95,"body":{"Or":[{"And":[[{"Call":[{"Rule":129},[[5,{"Or":[{"And":[[{"Call":[{"Rule":126},[[2,{"Rule":33}]]]},{"Call":[{"Rule":126},[[2,{"Rule":31}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":96,"body":{"Or":[{"And":[[{"Token":69},{"Opt":{"Or":[{"And":[[{"Token":52},{"Rep":{"Or":[{"And":[[{"Rule":32},{"Or":[{"And":[[{"Token":62}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":54}}}],null]}]}],1]}]}}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":97,"body":{"Or":[{"And":[[{"Token":68}],null]},{"And":[[{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":98,"body":{"Or":[{"And":[[{"Token":68},{"Opt":{"Or":[{"And":[[{"Token":52},{"Rep":{"Or":[{"And":[[{"Token":68},{"Or":[{"And":[[{"Token":62}],null]},{"And":[["Eof"],null]},{"And":[[{"Not":{"Not":{"Token":54}}}],null]}]}],1]}]}}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":99,"body":{"Or":[{"And":[[{"Token":9}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":38}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[0,{"Rule":38}]}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[1,{"Rule":38}]}],null]}]}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":100,"body":{"Or":[{"And":[[{"Opt":{"Token":53}},{"Rule":41}],null]}]},"replaceable":false}}}},{"Postfix":{"ty":100,"op":{"Or":[{"And":[[{"Token":53},{"Rule":41}],null]}]}}}]}},{"body":{"Pub":{"ty_idx":100,"body":{"Or":[{"And":[[{"Opt":{"Token":53}},{"Rule":41}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":100,"body":{"Or":[{"And":[[{"Rule":38},{"Or":[{"And":[[{"Token":53},{"Rule":41}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":69}],null]},{"And":[[{"Token":17}],null]},{"And":[[{"Token":18}],null]}]},{"Opt":{"Rule":42}}],null]}]}},{"body":{"Pub":{"ty_idx":101,"body":{"Or":[{"And":[[{"Or":[{"And":[[{"IsIn":0}],null]},{"And":[[{"IsIn":1},{"Token":53}],null]}]},{"Call":[{"Rule":129},[[5,{"Or":[{"And":[[{"Call":[{"Rule":126},[[2,{"Token":68}]]]},{"Call":[{"Rule":126},[[2,{"Rule":44}]]]}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":102,"body":{"Or":[{"And":[[{"Token":4},{"Token":69}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":45}],null]},{"And":[[{"Rule":46}],null]},{"And":[[{"Rule":47}],null]},{"And":[[{"Rule":48}],null]},{"And":[[{"Rule":49}],null]},{"And":[[{"Rule":51}],null]},{"And":[[{"Rule":52}],null]}]}},{"body":{"Pub":{"ty_idx":103,"body":{"Or":[{"And":[[{"Rule":36}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":104,"body":{"Or":[{"And":[[{"Token":64},{"Opt":{"Token":68}},{"Opt":{"Token":26}},{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":105,"body":{"Or":[{"And":[[{"Token":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":106,"body":{"Or":[{"And":[[{"Token":32},{"Token":33}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":107,"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Rule":44},{"Opt":{"Rule":50}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":108,"body":{"Or":[{"And":[[{"Token":54},{"Call":[{"Rule":126},[[2,{"Rule":44}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":109,"body":{"Or":[{"And":[[{"Call":[{"Rule":130},[[6,{"Or":[{"And":[[{"Rule":44},{"Opt":{"Or":[{"And":[[{"Token":51},{"Rule":66}],null]}]}}],null]}]}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":110,"body":{"Or":[{"And":[[{"Token":7},{"Call":[{"Rule":128},[[4,{"Call":[{"Rule":126},[[2,{"Rule":53}]]]}]]]},{"Rule":9}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":80,"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Rule":54},{"Token":52}],null]}]}},{"Rule":44}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":55}],null]},{"And":[[{"Rule":56}],null]},{"And":[[{"Rule":60}],null]},{"And":[[{"Rule":61}],null]},{"And":[[{"Rule":62}],null]},{"And":[[{"Rule":63}],null]},{"And":[[{"Rule":65}],null]}]}},{"body":{"Pub":{"ty_idx":111,"body":{"Or":[{"And":[[{"Token":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":112,"body":{"Or":[{"And":[[{"Rule":37},{"Opt":{"Or":[{"And":[[{"Rule":57}],null]},{"And":[[{"Rule":58}],null]}]}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":113,"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Call":[{"Rule":126},[[2,{"Rule":54}]]]},{"Opt":{"Or":[{"And":[[{"Token":56},{"Opt":{"Token":54}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"PubReplace":{"ty_idx":114,"body":{"Or":[{"And":[[{"Call":[{"Rule":127},[[3,{"Or":[{"And":[[{"Call":[{"Rule":126},[[2,{"Rule":59}]]]},{"Opt":{"Or":[{"And":[[{"Token":56},{"Opt":{"Token":54}}],null]}]}}],null]}]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":115,"body":{"Or":[{"And":[[{"Rule":60},{"Not":{"Token":52}}],null]},{"And":[[{"Token":69},{"Token":52},{"Rule":54}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":116,"body":{"Or":[{"And":[[{"Opt":{"Token":27}},{"Opt":{"Token":26}},{"Token":69}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":117,"body":{"Or":[{"And":[[{"Rule":69}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":118,"body":{"Or":[{"And":[[{"Token":32},{"Token":33}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":119,"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Rule":54},{"Opt":{"Rule":64}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":120,"body":{"Or":[{"And":[[{"Token":54},{"Call":[{"Rule":126},[[2,{"Rule":54}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":121,"body":{"Or":[{"And":[[{"Token":64},{"Opt":{"Token":26}},{"Rule":54}],null]}]},"replaceable":false}}},{"body":{"Pratt":[{"Atom":{"body":{"Pub":{"ty_idx":123,"body":{"Or":[{"And":[[{"Token":70}],null]},{"And":[[{"Token":71}],null]},{"And":[[{"Token":72}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":124,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":69},{"Token":67}],null]}]}},{"Rule":37},{"Opt":{"Rule":71}}],null]}]},"replaceable":true}}}},{"Atom":{"body":{"Pub":{"ty_idx":127,"body":{"Or":[{"And":[[{"Token":32},{"Token":33}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":128,"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Call":[{"Rule":68},[[0,{"Rule":66}]]]},{"Opt":{"Rule":75}}],null]}]}]]]}],null]}]},"replaceable":true}}}},{"Atom":{"body":{"Pub":{"ty_idx":130,"body":{"Or":[{"And":[[{"Opt":{"Token":25}},{"Token":65},{"Rep":{"Rule":77}},{"Token":65},{"Or":[{"And":[[{"Token":44},{"Rule":44},{"Rule":79}],null]},{"And":[[{"Call":[{"Rule":68},[[0,{"Rule":66}]]]}],null]}]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Token":30},{"Opt":{"Rule":66}}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Call":[{"Rule":68},[[0,{"Call":[{"Rule":127},[[3,{"Or":[{"And":[[{"Rep":{"Rule":80}},{"Opt":{"Rule":66}}],null]}]}]]]}]]]}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":136,"body":{"Or":[{"And":[[{"Token":13},{"Rule":86},{"Rule":79},{"Opt":{"Or":[{"And":[[{"Token":14},{"Rule":79}],null]}]}}],1]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":137,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":24},{"Rule":86},{"Rule":79}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":138,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":23},{"Rule":79}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":139,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":22},{"Rule":54},{"Token":31},{"Rule":87},{"Rule":79}],2]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":140,"body":{"Or":[{"And":[[{"Token":29},{"Rule":87},{"Call":[{"Rule":127},[[3,{"Rep":{"Rule":92}}]]]}],1]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":143,"body":{"Or":[{"And":[[{"Rule":123}],null]}]},"replaceable":false}}}},{"Atom":{"body":{"Pub":{"ty_idx":144,"body":{"Or":[{"And":[[{"Rule":124}],null]}]},"replaceable":false}}}},{"Postfix":{"ty":145,"op":{"Or":[{"And":[[{"Token":55},{"Token":69},{"Opt":{"Enter":[1,{"Rule":42}]}},{"Rule":99}],null]}]}}},{"Postfix":{"ty":146,"op":{"Rule":99}}},{"Postfix":{"ty":147,"op":{"Or":[{"And":[[{"Token":55},{"Or":[{"And":[[{"Token":69}],null]},{"And":[[{"Token":70}],null]}]}],null]}]}}},{"Postfix":{"ty":148,"op":{"Call":[{"Rule":130},[[6,{"Rule":66}]]]}}},{"Prefix":{"ty":150,"op":{"Token":64},"priority":999}},{"Prefix":{"ty":151,"op":{"Token":59},"priority":999}},{"Prefix":{"ty":152,"op":{"Token":63},"priority":999}},{"Binary":{"ty":153,"op":{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":59}],null]},{"And":[[{"Token":60}],null]},{"And":[[{"Token":61}],null]}]}]]]},"priority":10}},{"Binary":{"ty":154,"op":{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":62}],null]},{"And":[[{"Token":63}],null]}]}]]]},"priority":9}},{"Binary":{"ty":155,"op":{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"ContextualToken":[40,"<<"]}],null]},{"And":[[{"ContextualToken":[41,">>"]}],null]}]}]]]},"priority":8}},{"Binary":{"ty":156,"op":{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":64},{"Not":{"Token":64}}],null]}]}]]]},"priority":7}},{"Binary":{"ty":157,"op":{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":65},{"Not":{"Token":65}}],null]}]}]]]},"priority":6}},{"Binary":{"ty":158,"op":{"Call":[{"Rule":109},[[1,{"Rule":113}]]]},"priority":5}},{"Binary":{"ty":159,"op":{"Call":[{"Rule":109},[[1,{"ContextualToken":[42,"&&"]}]]]},"priority":4}},{"Binary":{"ty":160,"op":{"Call":[{"Rule":109},[[1,{"ContextualToken":[43,"||"]}]]]},"priority":3}},{"Binary":{"ty":161,"op":{"Call":[{"Rule":109},[[1,{"Rule":118}]]]},"priority":2}},{"Prefix":{"ty":161,"op":{"Rule":118},"priority":2}},{"Binary":{"ty":162,"op":{"Call":[{"Rule":109},[[1,{"Token":46}]]]},"priority":1}}]}},{"body":{"Or":[{"And":[[{"PrevIs":[132,136,137,138,139,140,143]}],null]}]}},{"body":{"Or":[{"And":[[{"Exit":[5,{"Exit":[4,{"Var":0}]}]}],null]}]}},{"body":{"Pub":{"ty_idx":123,"body":{"Or":[{"And":[[{"Token":70}],null]},{"And":[[{"Token":71}],null]},{"And":[[{"Token":72}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":124,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":69},{"Token":67}],null]}]}},{"Rule":37},{"Opt":{"Rule":71}}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":125,"body":{"Or":[{"And":[[{"Not":{"IsIn":5}},{"Call":[{"Rule":127},[[3,{"Call":[{"Rule":126},[[2,{"Rule":72}]]]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":126,"body":{"Or":[{"And":[[{"Token":69},{"Token":52},{"Rule":66}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":127,"body":{"Or":[{"And":[[{"Token":32},{"Token":33}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":128,"body":{"Or":[{"And":[[{"Call":[{"Rule":128},[[4,{"Or":[{"And":[[{"Call":[{"Rule":68},[[0,{"Rule":66}]]]},{"Opt":{"Rule":75}}],null]}]}]]]}],null]}]},"replaceable":true}}},{"body":{"PubReplace":{"ty_idx":129,"body":{"Or":[{"And":[[{"Token":54},{"Call":[{"Rule":126},[[2,{"Call":[{"Rule":68},[[0,{"Rule":66}]]]}]]]}],null]}]}}}},{"body":{"Pub":{"ty_idx":130,"body":{"Or":[{"And":[[{"Opt":{"Token":25}},{"Token":65},{"Rep":{"Rule":77}},{"Token":65},{"Or":[{"And":[[{"Token":44},{"Rule":44},{"Rule":79}],null]},{"And":[[{"Call":[{"Rule":68},[[0,{"Rule":66}]]]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":13},{"Or":[{"And":[[{"Token":54}],null]},{"And":[[{"Not":{"Not":{"Token":65}}}],null]}]}],1]}]}},{"body":{"Pub":{"ty_idx":131,"body":{"Or":[{"And":[[{"Token":30},{"Opt":{"Rule":66}}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":132,"body":{"Or":[{"And":[[{"Call":[{"Rule":68},[[0,{"Call":[{"Rule":127},[[3,{"Or":[{"And":[[{"Rep":{"Rule":80}},{"Opt":{"Rule":66}}],null]}]}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":81}],null]},{"And":[[{"Rule":83}],null]},{"And":[[{"Rule":82}],null]},{"And":[[{"Rule":3}],null]}]}},{"body":{"Pub":{"ty_idx":133,"body":{"Or":[{"And":[[{"Token":8},{"Rule":54},{"Token":46},{"Rule":66},{"Token":51}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":134,"body":{"Or":[{"And":[[{"Token":51}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":135,"body":{"Or":[{"And":[[{"Enter":[4,{"Or":[{"And":[[{"Rule":66},{"Or":[{"And":[[{"Rule":67},{"Not":"Eof"}],null]},{"And":[[{"Token":51}],null]}]}],null]}]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":136,"body":{"Or":[{"And":[[{"Token":13},{"Rule":86},{"Rule":79},{"Opt":{"Or":[{"And":[[{"Token":14},{"Rule":79}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":137,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":24},{"Rule":86},{"Rule":79}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Opt":{"Or":[{"And":[[{"Token":8},{"Rule":54},{"Token":46}],1]}]}},{"Rule":87}],null]}]}},{"body":{"Or":[{"And":[[{"Enter":[5,{"Rule":66}]}],null]}]}},{"body":{"Pub":{"ty_idx":138,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":23},{"Rule":79}],2]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":139,"body":{"Or":[{"And":[[{"Opt":{"Rule":90}},{"Token":22},{"Rule":54},{"Token":31},{"Rule":87},{"Rule":79}],2]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":68},{"Token":52}],null]}]}},{"body":{"Pub":{"ty_idx":140,"body":{"Or":[{"And":[[{"Token":29},{"Rule":87},{"Call":[{"Rule":127},[[3,{"Rep":{"Rule":92}}]]]}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":141,"body":{"Or":[{"And":[[{"Rule":93},{"Token":45},{"Rule":66},{"Or":[{"And":[[{"Token":54}],null]},{"And":[["Eof"],null]},{"And":[[{"Rule":67}],null]}]}],1]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rule":54},{"Rep":{"Or":[{"And":[[{"Token":65},{"Rule":54}],null]}]}},{"Opt":{"Rule":94}}],null]}]}},{"body":{"Pub":{"ty_idx":142,"body":{"Or":[{"And":[[{"Token":13},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":143,"body":{"Or":[{"And":[[{"Rule":123}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":144,"body":{"Or":[{"And":[[{"Rule":124}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":145,"body":{"Or":[{"And":[[{"Rule":66},{"Or":[{"And":[[{"Token":55},{"Token":69},{"Opt":{"Enter":[1,{"Rule":42}]}},{"Rule":99}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":146,"body":{"Or":[{"And":[[{"Rule":66},{"Rule":99}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":68},[[0,{"Call":[{"Rule":128},[[4,{"Call":[{"Rule":126},[[2,{"Rule":102}]]]}]]]}]]]}],null]}]}},{"body":{"Pub":{"ty_idx":147,"body":{"Or":[{"And":[[{"Rule":66},{"Or":[{"And":[[{"Token":55},{"Or":[{"And":[[{"Token":69}],null]},{"And":[[{"Token":70}],null]}]}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":148,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":130},[[6,{"Rule":66}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":149,"body":{"Or":[{"And":[[{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":150,"body":{"Or":[{"And":[[{"Token":64},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":151,"body":{"Or":[{"And":[[{"Token":59},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":152,"body":{"Or":[{"And":[[{"Token":63},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":153,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":59}],null]},{"And":[[{"Token":60}],null]},{"And":[[{"Token":61}],null]}]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":154,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":62}],null]},{"And":[[{"Token":63}],null]}]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":155,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"ContextualToken":[40,"<<"]}],null]},{"And":[[{"ContextualToken":[41,">>"]}],null]}]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"IsIn":4},{"Not":{"Rule":67}},{"Var":1}],null]},{"And":[[{"Not":{"IsIn":4}},{"Var":1}],null]}]}},{"body":{"Pub":{"ty_idx":156,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":64},{"Not":{"Token":64}}],null]}]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":157,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Or":[{"And":[[{"Token":65},{"Not":{"Token":65}}],null]}]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":158,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Rule":113}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Token":47}],null]},{"And":[[{"Token":48}],null]},{"And":[[{"Token":36}],null]},{"And":[[{"Token":37}],null]},{"And":[[{"Token":50}],null]},{"And":[[{"Token":49}],null]}]}},{"body":{"Pub":{"ty_idx":159,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"ContextualToken":[42,"&&"]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":160,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"ContextualToken":[43,"||"]}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":161,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Rule":118}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":161,"body":{"Or":[{"And":[[{"Rule":118},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Or":[{"And":[[{"Token":56}],null]},{"And":[[{"Token":57}],null]}]},{"Not":{"Or":[{"And":[[{"Token":34},{"IsIn":5}],null]}]}}],null]}]}},{"body":{"Pub":{"ty_idx":162,"body":{"Or":[{"And":[[{"Rule":66},{"Call":[{"Rule":109},[[1,{"Token":46}]]]},{"Rule":66}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":163,"body":{"Or":[{"And":[[{"Token":58},{"Call":[{"Rule":130},[[6,{"Call":[{"Rule":126},[[2,{"Rule":122}]]]}]]]}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Rule":120}}],null]}]}},{"body":{"Pub":{"ty_idx":164,"body":{"Or":[{"And":[[{"Token":69},{"Opt":{"Or":[{"And":[[{"Token":46},{"Rule":66}],null]},{"And":[[{"Call":[{"Rule":128},[[4,{"Call":[{"Rule":126},[[2,{"Rule":122}]]]}]]]}],null]}]}}],1]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":165,"body":{"Or":[{"And":[[{"Token":69},{"Token":67},{"Opt":{"Token":69}},{"Call":[{"Rule":127},[[3,{"Rep":{"Rule":125}}]]]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":166,"body":{"Or":[{"And":[[{"Token":69},{"Token":67},{"Opt":{"Token":69}},{"Or":[{"And":[[{"Token":32},{"Rep":{"Rule":125}},{"Token":33}],null]},{"And":[[{"Token":38},{"Rep":{"Rule":125}},{"Token":39}],null]}]}],null]}]},"replaceable":false}}},{"body":{"Pub":{"ty_idx":167,"body":{"Or":[{"And":[[{"Not":{"Or":[{"And":[[{"Token":32}],null]},{"And":[[{"Token":33}],null]},{"And":[[{"Token":34}],null]},{"And":[[{"Token":35}],null]},{"And":[[{"Token":38}],null]},{"And":[[{"Token":39}],null]}]}},"Any"],null]},{"And":[[{"Token":32},{"Rep":{"Rule":125}},{"Token":33}],null]},{"And":[[{"Token":38},{"Rep":{"Rule":125}},{"Token":39}],null]},{"And":[[{"Token":34},{"Rep":{"Rule":125}},{"Token":35}],null]}]},"replaceable":false}}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":2},{"Or":[{"And":[["Eof"],null]},{"And":[[{"Token":54}],null]}]}],1]}]}}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":131},[[7,{"Token":34}],[8,{"Token":35}],[9,{"Var":3}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":131},[[7,{"Token":32}],[8,{"Token":33}],[9,{"Var":4}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":131},[[7,{"Token":36}],[8,{"Token":37}],[9,{"Var":5}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Call":[{"Rule":131},[[7,{"Token":38}],[8,{"Token":39}],[9,{"Var":6}]]]}],null]}]}},{"body":{"Or":[{"And":[[{"Var":7},{"Layer":[{"Call":[{"Rule":132},[[10,{"Var":7}],[11,{"Var":8}]]]},{"Var":9}]},{"Var":8}],1]}]}},{"body":{"Or":[{"And":[[{"Rep":{"Or":[{"And":[[{"Var":10},{"Call":[{"Rule":132},[[10,{"Var":10}],[11,{"Var":11}]]]},{"Var":11}],1]},{"And":[[{"Or":[{"And":[[{"Not":{"Var":11}},"Any"],null]}]}],null]}]}}],null]}]}}]"##;

    ::fall_parse::ParserDefinition {
        node_types: vec![
            ERROR,
            WHITESPACE, LINE_COMMENT, UNION, AS, CRATE, EXTERN, FN, LET, PUB, STRUCT, USE, MOD, IF, ELSE, ENUM, IMPL, SELF, SUPER, TYPE, CONST, STATIC, FOR, LOOP, WHILE, MOVE, MUT, REF, TRAIT, MATCH, RETURN, IN, L_PAREN, R_PAREN, L_CURLY, R_CURLY, L_ANGLE, R_ANGLE, L_BRACK, R_BRACK, SHL, SHR, AND, OR, THIN_ARROW, FAT_ARROW, EQ, EQEQ, BANGEQ, GTET, LTEQ, SEMI, COLON, COLONCOLON, COMMA, DOT, DOTDOT, DOTDOTDOT, HASH, STAR, SLASH, PERCENT, PLUS, MINUS, AMPERSAND, PIPE, UNDERSCORE, BANG, LIFETIME, IDENT, NUMBER, STRING, RAW_STRING, FILE, USE_DECL, USE_SPEC, USE_SPEC_ENTRY, EXTERN_CRATE_DECL, FN_DEF, LINKAGE, VALUE_PARAM, LAMBDA_VALUE_PARAM, SELF_PARAMETER, STRUCT_DEF, STRUCT_FIELD, TUPLE_FIELD, ENUM_DEF, ENUM_VARIANT, MOD_DEF, IMPL_DEF, TRAIT_DEF, MEMBERS, TYPE_DEF, CONST_DEF, EXTERN_BLOCK, TYPE_PARAMETERS, TYPE_PARAMETER, TYPE_BOUND, LIFETIME_PARAMETER, VISIBILITY, PATH, TYPE_ARGUMENTS, ALIAS, PATH_TYPE, REFERENCE_TYPE, PLACEHOLDER_TYPE, UNIT_TYPE, PAREN_TYPE, TUPLE_TYPE, ARRAY_TYPE, FN_POINTER_TYPE, WILDCARD_PATTERN, PATH_PATTERN, TUPE_STRUCT_PATTERN, STRUCT_PATTERN, STRUCT_PATTERN_FIELD, BINDING_PATTERN, LITERAL_PATTERN, UNIT_PATTERN, PAREN_PATTERN, TUPLE_PATTERN, REFERENCE_PATTERN, EXPR, LITERAL, PATH_EXPR, STRUCT_LITERAL, STRUCT_LITERAL_FIELD, UNIT_EXPR, PAREN_EXPR, TUPLE_EXPR, LAMBDA_EXPR, RETURN_EXPR, BLOCK_EXPR, LET_STMT, EMPTY_STMT, EXPR_STMT, IF_EXPR, WHILE_EXPR, LOOP_EXPR, FOR_EXPR, MATCH_EXPR, MATCH_ARM, GUARD, BLOCK_MACRO_EXPR, LINE_MACRO_EXPR, METHOD_CALL_EXPR, CALL_EXPR, FIELD_EXPR, INDEX_EXPR, VALUE_ARGUMENT, REFERENCE_EXPR, DEREFERENCE_EXPR, NEGATION_EXPR, PRODUCT_EXPR, SUM_EXPR, BIT_SHIFT, BIT_AND, BIT_OR, COMPARISON, LOGICAL_AND, LOGICAL_OR, RANGE_EXPR, ASSIGNMENT_EXPR, ATTRIBUTE, ATTR_VALUE, BLOCK_MACRO, LINE_MACRO, TT,
        ],
        lexical_rules: vec![
            LexRule::new(WHITESPACE, "\\s+", None),
            LexRule::new(LINE_COMMENT, "//.*\\n?", None),
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
            LexRule::new(STAR, "\\*", None),
            LexRule::new(SLASH, "/", None),
            LexRule::new(PERCENT, "%", None),
            LexRule::new(PLUS, "\\+", None),
            LexRule::new(MINUS, "\\-", None),
            LexRule::new(AMPERSAND, "\\&", None),
            LexRule::new(PIPE, "\\|", None),
            LexRule::new(UNDERSCORE, "_", None),
            LexRule::new(BANG, "!", None),
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
                        STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                        SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        PERCENT => NodeTypeInfo { name: "PERCENT", whitespace_like: false },
                        PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        AMPERSAND => NodeTypeInfo { name: "AMPERSAND", whitespace_like: false },
                        PIPE => NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        UNDERSCORE => NodeTypeInfo { name: "UNDERSCORE", whitespace_like: false },
                        BANG => NodeTypeInfo { name: "BANG", whitespace_like: false },
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
                        EXTERN_BLOCK => NodeTypeInfo { name: "EXTERN_BLOCK", whitespace_like: false },
                        TYPE_PARAMETERS => NodeTypeInfo { name: "TYPE_PARAMETERS", whitespace_like: false },
                        TYPE_PARAMETER => NodeTypeInfo { name: "TYPE_PARAMETER", whitespace_like: false },
                        TYPE_BOUND => NodeTypeInfo { name: "TYPE_BOUND", whitespace_like: false },
                        LIFETIME_PARAMETER => NodeTypeInfo { name: "LIFETIME_PARAMETER", whitespace_like: false },
                        VISIBILITY => NodeTypeInfo { name: "VISIBILITY", whitespace_like: false },
                        PATH => NodeTypeInfo { name: "PATH", whitespace_like: false },
                        TYPE_ARGUMENTS => NodeTypeInfo { name: "TYPE_ARGUMENTS", whitespace_like: false },
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

