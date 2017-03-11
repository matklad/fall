extern crate lang;

use lang::{weird, match_ast};


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
