extern crate file;
extern crate tempdir;
extern crate difference;

use std::process;
use std::env;
use std::path::PathBuf;

use tempdir::TempDir;

fn generator_path() -> PathBuf {
    let test_exe = env::current_exe().unwrap();
    test_exe.parent().unwrap().parent().unwrap().join("fall-gen")
}

fn do_test(grammar: &str, expected: &str) {
    let dir = TempDir::new("gen-tests").unwrap();
    let grammar_path = dir.path().join("grammar.txt");
    file::put_text(&grammar_path, grammar).unwrap();

    let output = process::Command::new(generator_path())
            .arg(&grammar_path)
            .output()
            .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        panic!("Generator exited with code {:?}", output.status.code())
    }

    let actual = file::get_text(grammar_path.with_extension("rs")).unwrap();

    if expected.trim() != actual.trim() {
        difference::print_diff(&actual, &expected, "\n");
        panic!("Mismatch!")
    }
}

#[test]
fn test_simple_grammar() {
    do_test(r###"
nodes = {
  null bool number string
  lbrace rbrace
  lbrack rbrack
  comma colon
  object array primitive
  field
  file
}

tokenizer = {
  whitespace r"\s+"
  lbrace     r"\{"
  rbrace     r"\}"
  lbrack     r"\["
  rbrack     r"\]"
  colon      r":"
  comma      r","
  string     r#""[^"]*""#
  number     r"\d+"
}
        "###, r###"
use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
pub use fall_tree::{ERROR, WHITESPACE};

pub const NULL      : NodeType = NodeType(100);
pub const BOOL      : NodeType = NodeType(101);
pub const NUMBER    : NodeType = NodeType(102);
pub const STRING    : NodeType = NodeType(103);
pub const LBRACE    : NodeType = NodeType(104);
pub const RBRACE    : NodeType = NodeType(105);
pub const LBRACK    : NodeType = NodeType(106);
pub const RBRACK    : NodeType = NodeType(107);
pub const COMMA     : NodeType = NodeType(108);
pub const COLON     : NodeType = NodeType(109);
pub const OBJECT    : NodeType = NodeType(110);
pub const ARRAY     : NodeType = NodeType(111);
pub const PRIMITIVE : NodeType = NodeType(112);
pub const FIELD     : NodeType = NodeType(113);
pub const FILE      : NodeType = NodeType(114);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        NULL.register(NodeTypeInfo { name: "NULL" });
        BOOL.register(NodeTypeInfo { name: "BOOL" });
        NUMBER.register(NodeTypeInfo { name: "NUMBER" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        LBRACK.register(NodeTypeInfo { name: "LBRACK" });
        RBRACK.register(NodeTypeInfo { name: "RBRACK" });
        COMMA.register(NodeTypeInfo { name: "COMMA" });
        COLON.register(NodeTypeInfo { name: "COLON" });
        OBJECT.register(NodeTypeInfo { name: "OBJECT" });
        ARRAY.register(NodeTypeInfo { name: "ARRAY" });
        PRIMITIVE.register(NodeTypeInfo { name: "PRIMITIVE" });
        FIELD.register(NodeTypeInfo { name: "FIELD" });
        FILE.register(NodeTypeInfo { name: "FILE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: LBRACK, re: r"\[", f: None },
    Rule { ty: RBRACK, re: r"\]", f: None },
    Rule { ty: COLON, re: r":", f: None },
    Rule { ty: COMMA, re: r",", f: None },
    Rule { ty: STRING, re: r#""[^"]*""#, f: None },
    Rule { ty: NUMBER, re: r"\d+", f: None },
];
"###)
}


#[test]
fn test_self_grammar() {
    do_test(r###"
nodes = {
  kw_nodes
  kw_tokenizer

  eq
  lbrace rbrace
  ident
  simple_string
  hash_string

  file
  string
  def_nodes
  def_tokenizer
  rule
}

tokenizer = {
    whitespace r"\s+"
    eq "="
    lbrace r"\{"
    rbrace r"\}"
    simple_string r#"r?"([^"\\]|\\.)*""#
    hash_string "r#+\"" "super::parse_raw_string"
    kw_nodes "nodes"
    kw_tokenizer "tokenizer"
    ident r"\w+"
}
        "###, r###"
use std::sync::{Once, ONCE_INIT};
use fall_tree::{NodeType, NodeTypeInfo};
use fall_parse::Rule;
pub use fall_tree::{ERROR, WHITESPACE};

pub const KW_NODES  : NodeType = NodeType(100);
pub const KW_TOKENIZER: NodeType = NodeType(101);
pub const EQ        : NodeType = NodeType(102);
pub const LBRACE    : NodeType = NodeType(103);
pub const RBRACE    : NodeType = NodeType(104);
pub const IDENT     : NodeType = NodeType(105);
pub const SIMPLE_STRING: NodeType = NodeType(106);
pub const HASH_STRING: NodeType = NodeType(107);
pub const FILE      : NodeType = NodeType(108);
pub const STRING    : NodeType = NodeType(109);
pub const DEF_NODES : NodeType = NodeType(110);
pub const DEF_TOKENIZER: NodeType = NodeType(111);
pub const RULE      : NodeType = NodeType(112);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        KW_NODES.register(NodeTypeInfo { name: "KW_NODES" });
        KW_TOKENIZER.register(NodeTypeInfo { name: "KW_TOKENIZER" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        SIMPLE_STRING.register(NodeTypeInfo { name: "SIMPLE_STRING" });
        HASH_STRING.register(NodeTypeInfo { name: "HASH_STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        DEF_NODES.register(NodeTypeInfo { name: "DEF_NODES" });
        DEF_TOKENIZER.register(NodeTypeInfo { name: "DEF_TOKENIZER" });
        RULE.register(NodeTypeInfo { name: "RULE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+", f: None },
    Rule { ty: EQ, re: "=", f: None },
    Rule { ty: LBRACE, re: r"\{", f: None },
    Rule { ty: RBRACE, re: r"\}", f: None },
    Rule { ty: SIMPLE_STRING, re: r#"r?"([^"\\]|\\.)*""#, f: None },
    Rule { ty: HASH_STRING, re: "r#+\"", f: Some(super::parse_raw_string) },
    Rule { ty: KW_NODES, re: "nodes", f: None },
    Rule { ty: KW_TOKENIZER, re: "tokenizer", f: None },
    Rule { ty: IDENT, re: r"\w+", f: None },
];
"###)
}


#[test]
fn test_grammars_are_fresh() {
    let cwd = ::std::env::current_dir().unwrap();
    let tests = cwd.parent().unwrap().join("fall_test/src");
    for lang in ["sexp", "rust", "weird"].into_iter() {
        let raw = tests.join(lang).join("grammar.txt");
        println!("{:?}", raw);
        let grammar = file::get_text(&raw).unwrap();
        let expected = file::get_text(raw.with_extension("rs")).unwrap();
        do_test(&grammar, &expected);
    }
}
