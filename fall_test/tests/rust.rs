extern crate fall_test;
extern crate fall_tree;

use fall_test::{rust, match_ast};
use fall_tree::dump_file;


fn ast(code: &str) -> String {
    dump_file(&rust::LANG.parse(code.to_owned()))
}

#[test]
fn opt_pub() {
    match_ast(&ast("\
struct Foo {}
fn bar() {}
pub struct Baz {}
pub fn quux() {}
"), r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    RBRACE "}"
  FN_DEF
    FN "fn"
    IDENT "bar"
    LPAREN "("
    RPAREN ")"
    LBRACE "{"
    RBRACE "}"
  STRUCT_DEF
    KW_PUB "pub"
    STRUCT "struct"
    IDENT "Baz"
    LBRACE "{"
    RBRACE "}"
  FN_DEF
    KW_PUB "pub"
    FN "fn"
    IDENT "quux"
    LPAREN "("
    RPAREN ")"
    LBRACE "{"
    RBRACE "}"
"#)
}

#[test]
fn missing_token() {
    match_ast(&ast("fn foo foo"), r#"
FILE
  FN_DEF
    FN "fn"
    IDENT "foo"
    ERROR ""
  ERROR
    IDENT "foo"
"#);
}

#[test]
fn skipping() {
    match_ast(&ast("foo fn foo(){} bar baz struct S {} quuz"), r#"
FILE
  ERROR
    IDENT "foo"
  FN_DEF
    FN "fn"
    IDENT "foo"
    LPAREN "("
    RPAREN ")"
    LBRACE "{"
    RBRACE "}"
  ERROR
    IDENT "bar"
    IDENT "baz"
  STRUCT_DEF
    STRUCT "struct"
    IDENT "S"
    LBRACE "{"
    RBRACE "}"
  ERROR
    IDENT "quuz""#);
}
