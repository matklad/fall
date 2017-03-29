extern crate fall_test;
extern crate fall_tree;

use fall_test::{rust, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&rust::parse(code.to_owned()))
}

#[test]
fn empty_nodes() {
    match_ast(&ast("\
struct Foo {}
fn bar() {}
pub struct Baz {}
pub fn quux() {}
"), r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    WHITESPACE " "
    IDENT "Foo"
    WHITESPACE " "
    LBRACE "{"
    RBRACE "}"
  WHITESPACE "\n"
  FN_DEF
    FN "fn"
    WHITESPACE " "
    IDENT "bar"
    LPAREN "("
    RPAREN ")"
    WHITESPACE " "
    LBRACE "{"
    RBRACE "}"
  WHITESPACE "\n"
  STRUCT_DEF
    PUB "pub"
    WHITESPACE " "
    STRUCT "struct"
    WHITESPACE " "
    IDENT "Baz"
    WHITESPACE " "
    LBRACE "{"
    RBRACE "}"
  WHITESPACE "\n"
  FN_DEF
    PUB "pub"
    WHITESPACE " "
    FN "fn"
    WHITESPACE " "
    IDENT "quux"
    LPAREN "("
    RPAREN ")"
    WHITESPACE " "
    LBRACE "{"
    RBRACE "}"
  WHITESPACE "\n"
"#)
}
