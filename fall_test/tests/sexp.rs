extern crate fall_test;
extern crate fall_tree;

use fall_test::{sexp, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&sexp::parse(code.to_owned()))
}

#[test]
fn empty_file() {
    match_ast(&ast(""), r#"FILE """#);
    match_ast(&ast(" \n "), r#"
FILE
  WHITESPACE " \n "
"#)
}

#[test]
fn trailing_ws() {
    match_ast(&ast(" a "), r#"
FILE
  WHITESPACE " "
  ATOM "a"
  WHITESPACE " "
"#)
}

#[test]
fn simple() {
    match_ast(
        &ast("( hello ( world )  )"), r#"
FILE
  LIST
    LPAREN "("
    WHITESPACE " "
    ATOM "hello"
    WHITESPACE " "
    LIST
      LPAREN "("
      WHITESPACE " "
      ATOM "world"
      WHITESPACE " "
      RPAREN ")"
    WHITESPACE "  "
    RPAREN ")"
"#)
}
