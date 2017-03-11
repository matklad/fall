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

    let status = process::Command::new(generator_path())
            .arg(&grammar_path)
            .status()
            .expect("Failed to execute process");
    assert!(status.success());
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
