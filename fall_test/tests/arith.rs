extern crate fall_test;
extern crate fall_tree;

use fall_test::{arith, match_ast};
use fall_tree::dump_file;


fn ast(code: &str) -> String {
    dump_file(&arith::LANG.parse(code.to_owned()))
}

#[test]
fn simple() {
    match_ast(&ast("1 + 2"), r#"
FILE
  SUM_EXPR
    CONSTANT_EXPR
      NUMBER "1"
    PLUS "+"
    CONSTANT_EXPR
      NUMBER "2"
"#);
    match_ast(&ast("1 * 2"), r#"
FILE
  PRODUCT_EXPR
    CONSTANT_EXPR
      NUMBER "1"
    STAR "*"
    CONSTANT_EXPR
      NUMBER "2"
"#);
}

#[test]
fn associativity() {
    match_ast(&ast("1 + 2 + 3"), r#"
  FILE
  SUM_EXPR
    SUM_EXPR
      CONSTANT_EXPR
        NUMBER "1"
      PLUS "+"
      CONSTANT_EXPR
        NUMBER "2"
    PLUS "+"
    CONSTANT_EXPR
      NUMBER "3""#);
}

#[test]
fn precedence() {
    match_ast(&ast("1 + 2 * 3 + 4"), r#"
FILE
  SUM_EXPR
    SUM_EXPR
      CONSTANT_EXPR
        NUMBER "1"
      PLUS "+"
      PRODUCT_EXPR
        CONSTANT_EXPR
          NUMBER "2"
        STAR "*"
        CONSTANT_EXPR
          NUMBER "3"
    PLUS "+"
    CONSTANT_EXPR
      NUMBER "4"
"#);
}

#[test]
fn complex() {
    match_ast(&ast("1 * (2 + 3) * 4"), r#"
FILE
  PRODUCT_EXPR
    PRODUCT_EXPR
      CONSTANT_EXPR
        NUMBER "1"
      STAR "*"
      PAREN_EXPR
        LPAREN "("
        SUM_EXPR
          CONSTANT_EXPR
            NUMBER "2"
          PLUS "+"
          CONSTANT_EXPR
            NUMBER "3"
        RPAREN ")"
    STAR "*"
    CONSTANT_EXPR
      NUMBER "4"
"#);
}

#[test]
fn postfix() {
    match_ast(&ast("1! + 2!! + 3*4! * (5)!"), r#"
FILE
  SUM_EXPR
    SUM_EXPR
      FACTORIAL_EXPR
        CONSTANT_EXPR
          NUMBER "1"
        BANG "!"
      PLUS "+"
      FACTORIAL_EXPR
        FACTORIAL_EXPR
          CONSTANT_EXPR
            NUMBER "2"
          BANG "!"
        BANG "!"
    PLUS "+"
    PRODUCT_EXPR
      PRODUCT_EXPR
        CONSTANT_EXPR
          NUMBER "3"
        STAR "*"
        FACTORIAL_EXPR
          CONSTANT_EXPR
            NUMBER "4"
          BANG "!"
      STAR "*"
      FACTORIAL_EXPR
        PAREN_EXPR
          LPAREN "("
          CONSTANT_EXPR
            NUMBER "5"
          RPAREN ")"
        BANG "!"
"#)
}
