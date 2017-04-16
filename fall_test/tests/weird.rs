extern crate fall_test;
extern crate fall_tree;

use fall_test::{weird, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&weird::LANG.parse(code.to_owned()))
}

#[test]
fn empty_nodes() {
    match_ast(&ast("_2 hello "), r#"
FILE
  T2 "_2"
  WHITESPACE " "
  EMPTY ""
  ATOM "hello"
  WHITESPACE " "
  EMPTY ""
"#)
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
