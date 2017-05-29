extern crate fall_tree;
extern crate lang_rust;

use fall_tree::test_util::check_syntax;
use lang_rust::LANG_RUST;


#[test]
fn opt_pub() {
    check_syntax(&LANG_RUST, "\
struct Foo {}
fn bar() {}
pub struct Baz {}
pub fn quux() {}
", r#"
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
    check_syntax(&LANG_RUST, "fn foo foo", r#"
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
    check_syntax(&LANG_RUST, "foo fn foo(){} bar baz struct S {} quuz", r#"
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
