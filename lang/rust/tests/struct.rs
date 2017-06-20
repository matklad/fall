extern crate fall_tree;
extern crate lang_rust;

use fall_tree::test_util::check_syntax;
use lang_rust::LANG_RUST;


#[test]
fn empty_struct() {
    check_syntax(&LANG_RUST, "
struct Foo { }
", r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    RBRACE "}"
"#)
}

#[test]
fn field1() {
    check_syntax(&LANG_RUST, "
struct Foo { foo: Bar }
", r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    STRUCT_FIELD
      IDENT "foo"
      COLON ":"
      TYPE
        IDENT "Bar"
    RBRACE "}"
"#)
}

#[test]
fn field2() {
    check_syntax(&LANG_RUST, "
struct Foo { a: A, pub b: B }
", r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    STRUCT_FIELD
      IDENT "a"
      COLON ":"
      TYPE
        IDENT "A"
      COMMA ","
    STRUCT_FIELD
      KW_PUB "pub"
      IDENT "b"
      COLON ":"
      TYPE
        IDENT "B"
    RBRACE "}"
"#)
}

#[test]
fn field3() {
    check_syntax(&LANG_RUST, "
struct Foo { a: A, }
", r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    STRUCT_FIELD
      IDENT "a"
      COLON ":"
      TYPE
        IDENT "A"
      COMMA ","
    RBRACE "}"
"#)
}


#[test]
fn stray_comma() {
    check_syntax(&LANG_RUST, "
struct Foo {, }
", r#"
FILE
  STRUCT_DEF
    STRUCT "struct"
    IDENT "Foo"
    LBRACE "{"
    ERROR
      COMMA ","
    RBRACE "}"
"#)
}
