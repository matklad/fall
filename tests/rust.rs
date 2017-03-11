extern crate lang;

use lang::{rust, match_ast};


fn ast(code: &str) -> String {
    rust::parse(code.to_owned()).dump_ws()
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
