extern crate fall_tree;
extern crate lang_rust;

use std::path::{Path, PathBuf};

use fall_tree::test_util::{check_syntax, check_reparse, check_directory, check_inline_tests};
use lang_rust::LANG_RUST;

#[test]
fn inline_tests() {
    check_inline_tests(&LANG_RUST, Path::new("src/syntax.fall"), Path::new("tests/inline.txt"))
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
    L_PAREN "("
    R_PAREN ")"
    BLOCK_EXPR
      L_CURLY "{"
      R_CURLY "}"
  ERROR
    IDENT "bar"
    IDENT "baz"
  STRUCT_DEF
    STRUCT "struct"
    IDENT "S"
    L_CURLY "{"
    R_CURLY "}"
  ERROR
    IDENT "quuz""#);
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
    L_PAREN "("
    R_PAREN ")"
    BLOCK_EXPR
      L_CURLY "{"
      LET_STMT
        LET "let"
        PATTERN
          IDENT "a"
        EQ "="
        LITERAL
          NUMBER "1"
        SEMI ";"
      R_CURLY "}""#,
        "fn foo() { let a = 1; }")
}


#[test]
fn check_by_data() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let test_data_path = PathBuf::from(dir).join("tests").join("data");
    check_directory(&LANG_RUST, &test_data_path)
}
