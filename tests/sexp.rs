extern crate lang;

use lang::sexp;


fn ast(code: &str) -> String {
    sexp::parse(code.to_owned()).dump()
}

fn match_ast(actual: &str, expected: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if actual != expected {
        panic!("Actual:\n{}\nExpected:\n{}\n", actual, expected)
    }
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
      ERROR
        RPAREN ")"
        WHITESPACE "  "
        RPAREN ")"
"#)
}
