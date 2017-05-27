extern crate fall_test;
extern crate fall_tree;

use fall_test::{weird, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&weird::LANG.parse(code.to_owned()))
}

#[test]
fn external_rule() {
    match_ast(&ast(r###"_1 r##"f#"o"#o"## "###), r###"
FILE
  T1 "_1"
  WHITESPACE " "
  RAW_STRING "r##\"f#\"o\"#o\"##"
  WHITESPACE " "
"###)
}

#[test]
fn empty_nodes() {
    match_ast(&ast("_2 hello "), r#"
FILE
  T2 "_2"
  EMPTY ""
  WHITESPACE " "
  ATOM "hello"
  EMPTY ""
  WHITESPACE " "
"#)
}


#[test]
fn rollbacks_private_rules() {
    match_ast(&ast("_3 foo bar"), r#"
FILE
  T3 "_3"
  WHITESPACE " "
  PRIVATE_PARTIAL
    FOO "foo"
    WHITESPACE " "
    BAR "bar"
"#);

    match_ast(&ast("_3 foo foo"), r#"
FILE
  T3 "_3"
  WHITESPACE " "
  PRIVATE_PARTIAL
    FOO "foo"
    WHITESPACE " "
    FOO "foo"
"#);
}

#[test]
fn block1() {
    match_ast(&ast("_4 { foo }"), r#"
FILE
  T4 "_4"
  WHITESPACE " "
  BLOCK
    LBRACE "{"
    WHITESPACE " "
    FOO "foo"
    WHITESPACE " "
    RBRACE "}"
"#);
}

#[test]
fn block2() {
    match_ast(&ast("_4 { foo { {foo} foo {bar}} bar }"), r#"
FILE
  T4 "_4"
  WHITESPACE " "
  BLOCK
    LBRACE "{"
    WHITESPACE " "
    FOO "foo"
    WHITESPACE " "
    LBRACE "{"
    WHITESPACE " "
    LBRACE "{"
    FOO "foo"
    RBRACE "}"
    WHITESPACE " "
    FOO "foo"
    WHITESPACE " "
    LBRACE "{"
    BAR "bar"
    RBRACE "}"
    RBRACE "}"
    WHITESPACE " "
    BAR "bar"
    WHITESPACE " "
    RBRACE "}"
"#);
}

#[test]
fn block3() {
    match_ast(&ast("_4 {"), r#"
FILE
  T4 "_4"
  WHITESPACE " "
  BLOCK
    LBRACE "{"
    ERROR ""
"#);
}

#[test]
fn block4() {
    match_ast(&ast("_4 { foo { {} bar"), r#"
FILE
  T4 "_4"
  WHITESPACE " "
  BLOCK
    LBRACE "{"
    WHITESPACE " "
    FOO "foo"
    WHITESPACE " "
    LBRACE "{"
    WHITESPACE " "
    LBRACE "{"
    RBRACE "}"
    WHITESPACE " "
    BAR "bar"
    ERROR ""
    ERROR ""
"#);
}
