extern crate fall_test;
extern crate fall_tree;

use fall_test::{weird, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&weird::parse(code.to_owned()))
}

#[test]
fn empty_nodes() {
    match_ast(&ast(" hello "), r#"
FILE
  WHITESPACE " "
  EMPTY ""
  ATOM "hello"
  WHITESPACE " "
  EMPTY ""
"#)
}

#[test]
fn external_rule() {
    match_ast(&ast(r###" r##"f#"o"#o"## "###), r###"
FILE
  WHITESPACE " "
  RAW_STRING "r##\"f#\"o\"#o\"##"
  WHITESPACE " "
"###)
}
