extern crate fall_test;

use fall_test::{weird, match_ast};


fn ast(code: &str) -> String {
    weird::parse(code.to_owned()).dump_ws()
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
