extern crate fall_tree;
extern crate lang_fall;

use lang_fall::editor_api;
use fall_tree::{TextUnit, TextRange};


#[test]
fn test_find_refs() {
    let file = lang_fall::lang_fall().parse(TEXT.to_owned());
    let usages = editor_api::find_usages(
        &file,
        TextUnit::from_usize(309)
    );

    assert_eq!(usages, vec![TextRange::from_len(
        TextUnit::from_usize(202),
        TextUnit::from_usize(12)
    )]);
}


const TEXT: &str = r#####"
tokenizer {
  #[skip] whitespace r"\s+"

  number r"\d+"
  plus '+'
  minus '-'
  star '*'
  slash '/'
  bang '!'
  lparen '('
  rparen ')'
}

pub rule file { expr }

#[pratt]
rule expr {
  sum_expr | product_expr
  | factorial_expr
  | negate_expr
  | constant_expr | paren_expr
}

#[bin(2)]
pub rule product_expr { expr {'*' | '/'} expr }

#[bin(1)]
pub rule sum_expr { expr {'+' | '-'} expr }

#[atom]
pub rule constant_expr { number }

#[atom]
pub rule paren_expr { '(' expr ')' }

#[postfix]
pub rule factorial_expr { expr '!' }

#[prefix]
pub rule negate_expr { '-' expr }

test r"
  1 + --1! - -2!
"
"#####;
