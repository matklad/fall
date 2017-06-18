extern crate fall_tree;
extern crate lang_rust;

use fall_tree::test_util::{check_syntax, check_reparse};
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
    BLOCK_EXPR
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
    BLOCK_EXPR
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
    BLOCK_EXPR
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

#[test]
fn lets() {
    check_syntax(&LANG_RUST, "fn foo() { let a = 1; let b = 2; }", r#"
FILE
  FN_DEF
    FN "fn"
    IDENT "foo"
    LPAREN "("
    RPAREN ")"
    BLOCK_EXPR
      LBRACE "{"
      STMT
        KW_LET "let"
        PATTERN
          IDENT "a"
        EQ "="
        EXPR
          NUMBER "1"
        SEMI ";"
      STMT
        KW_LET "let"
        PATTERN
          IDENT "b"
        EQ "="
        EXPR
          NUMBER "2"
        SEMI ";"
      RBRACE "}"
"#);
}

#[test]
fn check_reparse_in_block_body() {
    check_reparse(
        &LANG_RUST,
        "fn foo() { let a = 1 }",
        "fn foo() { let a = 1; }",
        r#"
FILE
  FN_DEF
    FN "fn"
    IDENT "foo"
    LPAREN "("
    RPAREN ")"
    BLOCK_EXPR
      LBRACE "{"
      STMT
        KW_LET "let"
        PATTERN
          IDENT "a"
        EQ "="
        EXPR
          NUMBER "1"
        SEMI ";"
      RBRACE "}""#,
        "fn foo() { let a = 1; }")
}
