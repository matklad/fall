use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
use fall_parse::syn;
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_NODES  : NodeType = NodeType(100);
pub const KW_TOKENIZER: NodeType = NodeType(101);
pub const KW_RULE   : NodeType = NodeType(102);
pub const KW_VERBATIM: NodeType = NodeType(103);
pub const EQ        : NodeType = NodeType(104);
pub const PIPE      : NodeType = NodeType(105);
pub const STAR      : NodeType = NodeType(106);
pub const LBRACE    : NodeType = NodeType(107);
pub const RBRACE    : NodeType = NodeType(108);
pub const LANGLE    : NodeType = NodeType(109);
pub const RANGLE    : NodeType = NodeType(110);
pub const LPAREN    : NodeType = NodeType(111);
pub const RPAREN    : NodeType = NodeType(112);
pub const IDENT     : NodeType = NodeType(113);
pub const SIMPLE_STRING: NodeType = NodeType(114);
pub const HASH_STRING: NodeType = NodeType(115);
pub const FILE      : NodeType = NodeType(116);
pub const STRING    : NodeType = NodeType(117);
pub const VERBATIM_DEF: NodeType = NodeType(118);
pub const NODES_DEF : NodeType = NodeType(119);
pub const TOKENIZER_DEF: NodeType = NodeType(120);
pub const LEX_RULE  : NodeType = NodeType(121);
pub const SYN_RULE  : NodeType = NodeType(122);
pub const ALT       : NodeType = NodeType(123);
pub const PART      : NodeType = NodeType(124);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        KW_NODES.register(NodeTypeInfo { name: "KW_NODES" });
        KW_TOKENIZER.register(NodeTypeInfo { name: "KW_TOKENIZER" });
        KW_RULE.register(NodeTypeInfo { name: "KW_RULE" });
        KW_VERBATIM.register(NodeTypeInfo { name: "KW_VERBATIM" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        PIPE.register(NodeTypeInfo { name: "PIPE" });
        STAR.register(NodeTypeInfo { name: "STAR" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LANGLE.register(NodeTypeInfo { name: "LANGLE" });
        RANGLE.register(NodeTypeInfo { name: "RANGLE" });
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        SIMPLE_STRING.register(NodeTypeInfo { name: "SIMPLE_STRING" });
        HASH_STRING.register(NodeTypeInfo { name: "HASH_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        VERBATIM_DEF.register(NodeTypeInfo { name: "VERBATIM_DEF" });
        NODES_DEF.register(NodeTypeInfo { name: "NODES_DEF" });
        TOKENIZER_DEF.register(NodeTypeInfo { name: "TOKENIZER_DEF" });
        LEX_RULE.register(NodeTypeInfo { name: "LEX_RULE" });
        SYN_RULE.register(NodeTypeInfo { name: "SYN_RULE" });
        ALT.register(NodeTypeInfo { name: "ALT" });
        PART.register(NodeTypeInfo { name: "PART" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: "\\s+", f: None },
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: PIPE, re: "\\|", f: None },
    Rule { ty: STAR, re: "\\*", f: None },
    Rule { ty: LBRACE, re: "\\{", f: None },
    Rule { ty: RBRACE, re: "\\}", f: None },
    Rule { ty: LANGLE, re: "<", f: None },
    Rule { ty: RANGLE, re: ">", f: None },
    Rule { ty: LPAREN, re: "\\(", f: None },
    Rule { ty: RPAREN, re: "\\)", f: None },
    Rule { ty: SIMPLE_STRING, re: "(\"([^\"\\\\]|\\\\.)*\")", f: None },
    Rule { ty: HASH_STRING, re: "r#*", f: Some(parse_raw_string) },
    Rule { ty: KW_NODES, re: "nodes", f: None },
    Rule { ty: KW_TOKENIZER, re: "tokenizer", f: None },
    Rule { ty: KW_RULE, re: "rule", f: None },
    Rule { ty: KW_VERBATIM, re: "verbatim", f: None },
    Rule { ty: IDENT, re: "\\w+", f: None },
];

pub const PARSER: &'static [syn::Rule] = &[
    syn::Rule { ty: Some(FILE), alts: &[syn::Alt { parts: &[syn::Part::Rule(1), syn::Part::Rule(2), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(4)], commit: None }), syn::Part::Rule(9)], commit: None }] },
    syn::Rule { ty: Some(NODES_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_NODES), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Token(IDENT)], commit: None }), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: Some(TOKENIZER_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_TOKENIZER), syn::Part::Token(LBRACE), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(3)], commit: None }), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: Some(LEX_RULE), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT), syn::Part::Rule(8), syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(8)], commit: None })], commit: Some(1) }] },
    syn::Rule { ty: Some(SYN_RULE), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_RULE), syn::Part::Token(IDENT), syn::Part::Token(LBRACE), syn::Part::Rule(5), syn::Part::Token(RBRACE)], commit: Some(1) }] },
    syn::Rule { ty: None, alts: &[syn::Alt { parts: &[syn::Part::Opt(syn::Alt { parts: &[syn::Part::Rule(6)], commit: None }), syn::Part::Rep(syn::Alt { parts: &[syn::Part::Token(PIPE), syn::Part::Rule(6)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(ALT), alts: &[syn::Alt { parts: &[syn::Part::Rep(syn::Alt { parts: &[syn::Part::Rule(7)], commit: None })], commit: None }] },
    syn::Rule { ty: Some(PART), alts: &[syn::Alt { parts: &[syn::Part::Token(IDENT)], commit: None }, syn::Alt { parts: &[syn::Part::Token(LANGLE), syn::Part::Token(IDENT), syn::Part::Rule(5), syn::Part::Token(RANGLE)], commit: None }] },
    syn::Rule { ty: Some(STRING), alts: &[syn::Alt { parts: &[syn::Part::Token(SIMPLE_STRING)], commit: None }, syn::Alt { parts: &[syn::Part::Token(HASH_STRING)], commit: None }] },
    syn::Rule { ty: Some(VERBATIM_DEF), alts: &[syn::Alt { parts: &[syn::Part::Token(KW_VERBATIM), syn::Part::Token(HASH_STRING)], commit: None }] },
];

pub fn parse(text: String) -> ::fall_tree::File {
    register_node_types();
    ::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}
