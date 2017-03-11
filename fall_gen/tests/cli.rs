extern crate tempdir;
extern crate difference;

use std::io::{Write, Read};
use std::fs::File;
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
    {
        let mut f = File::create(&grammar_path).unwrap();
        write!(f, "{}", grammar).unwrap();
    }

    let output = process::Command::new(generator_path())
            .arg(&grammar_path)
            .output()
            .expect("Failed to execute process");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Generator exited with code {:?}", output.status.code())
    }

    let actual = {
        let mut f = File::open(grammar_path.with_extension("rs"))
            .expect("Failed to find output file");
        let mut buff = String::new();
        f.read_to_string(&mut buff).unwrap();
        buff
    };

    if expected.trim() != actual.trim() {
        difference::print_diff(&actual, &expected, "\n");
        panic!("Mismatch!")
    }
}

#[test]
#[ignore]
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
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

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
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: LBRACE, re: r"\{" },
    Rule { ty: RBRACE, re: r"\}" },
    Rule { ty: LBRACK, re: r"\[" },
    Rule { ty: RBRACK, re: r"\]" },
    Rule { ty: COLON, re: r":" },
    Rule { ty: COMMA, re: r"," },
    Rule { ty: STRING, re: r#""[^"]*""# },
    Rule { ty: NUMBER, re: r"\d+" },
];
"###)
}


#[test]
fn test_self_grammar() {
    do_test(r###"
nodes = {
  nodes
  tokenizer_kw

  eq
  lbrace rbrace
  ident
  string

  file
  nodes_def
  tokenizer_def
  rule
}

tokenizer = {
    whitespace r"\s+"
    eq "="
    lbrace r"\{"
    rbrace r"\}"
    string r#"r?"[^"]*""#
    nodes "nodes"
    tokenizer_kw "tokenizer"
    ident r"\w+"
}
        "###, r###"
use std::sync::{Once, ONCE_INIT};
use fall::{NodeType, NodeTypeInfo};
use fall::builder::Rule;
pub use fall::{ERROR, WHITESPACE};

pub const NODES     : NodeType = NodeType(100);
pub const TOKENIZER_KW: NodeType = NodeType(101);
pub const EQ        : NodeType = NodeType(102);
pub const LBRACE    : NodeType = NodeType(103);
pub const RBRACE    : NodeType = NodeType(104);
pub const IDENT     : NodeType = NodeType(105);
pub const STRING    : NodeType = NodeType(106);
pub const FILE      : NodeType = NodeType(107);
pub const NODES_DEF : NodeType = NodeType(108);
pub const TOKENIZER_DEF: NodeType = NodeType(109);
pub const RULE      : NodeType = NodeType(110);

pub fn register_node_types() {
    static REGISTER: Once = ONCE_INIT;
    REGISTER.call_once(||{
        NODES.register(NodeTypeInfo { name: "NODES" });
        TOKENIZER_KW.register(NodeTypeInfo { name: "TOKENIZER_KW" });
        EQ.register(NodeTypeInfo { name: "EQ" });
        LBRACE.register(NodeTypeInfo { name: "LBRACE" });
        RBRACE.register(NodeTypeInfo { name: "RBRACE" });
        IDENT.register(NodeTypeInfo { name: "IDENT" });
        STRING.register(NodeTypeInfo { name: "STRING" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        NODES_DEF.register(NodeTypeInfo { name: "NODES_DEF" });
        TOKENIZER_DEF.register(NodeTypeInfo { name: "TOKENIZER_DEF" });
        RULE.register(NodeTypeInfo { name: "RULE" });
    });
}

pub const TOKENIZER: &'static [Rule] = &[
    Rule { ty: WHITESPACE, re: r"\s+" },
    Rule { ty: EQ, re: "=" },
    Rule { ty: LBRACE, re: r"\{" },
    Rule { ty: RBRACE, re: r"\}" },
    Rule { ty: STRING, re: r#"r?"[^"]*""# },
    Rule { ty: NODES, re: "nodes" },
    Rule { ty: TOKENIZER_KW, re: "tokenizer" },
    Rule { ty: IDENT, re: r"\w+" },
];
"###)
}
