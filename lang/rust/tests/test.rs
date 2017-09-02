extern crate elapsed;
extern crate file;
extern crate fall_tree;
extern crate lang_rust;

use std::path::{Path, PathBuf};

use fall_tree::test_util::{check_syntax_ws, check_syntax, check_directory, check_inline_tests};
use lang_rust::LANG_RUST;

#[test]
fn inline_tests() {
    check_inline_tests(&LANG_RUST, Path::new("src/rust.fall"), Path::new("tests/inline.txt"))
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
fn check_by_data() {
    check_directory(&LANG_RUST, &test_data())
}


#[test]
fn comments_attachment() {
    check_syntax_ws(&LANG_RUST, r#"
/// Doc comment attached
struct A;

// Simple comment attached

// But only if there are no blank lines
struct B;
    "#, r#"
FILE
  WHITESPACE "\n"
  STRUCT_DEF
    LINE_COMMENT "/// Doc comment attached\n"
    STRUCT "struct"
    WHITESPACE " "
    IDENT "A"
    SEMI ";"
  WHITESPACE "\n\n"
  LINE_COMMENT "// Simple comment attached\n"
  WHITESPACE "\n"
  STRUCT_DEF
    LINE_COMMENT "// But only if there are no blank lines\n"
    STRUCT "struct"
    WHITESPACE " "
    IDENT "B"
    SEMI ";"
  WHITESPACE "\n    ""#)
}


#[test]
fn performance_test() {
    if !::std::env::var("slow_tests").is_ok() {
        return;
    }
    let text = file::get_text(test_data().join("parser.rs_")).unwrap();
    let thread = ::std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let (total, file) = elapsed::measure_time(|| LANG_RUST.parse(text));
            let ast_len = fall_tree::dump_file(&file).len();
            assert!(ast_len > 10000);
            println!("{}\ntotal: {}", file.stats(), total);
        })
        .unwrap();
    thread.join().unwrap()
}

fn test_data() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).join("tests/data")
}
