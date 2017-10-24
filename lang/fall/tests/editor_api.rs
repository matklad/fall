extern crate fall_tree;
extern crate lang_fall;

use lang_fall::editor_api;
use fall_tree::{TextUnit, TextRange};

#[test]
fn test_highlighting() {
    let file = parse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"####);

    let spans = editor_api::highlight(&file);
    assert_eq!(
        format!("{:?}", spans),
        r#"[([1; 10), "keyword"), ([20; 26), "string"), ([13; 19), "token"), ([28; 31), "keyword"), ([32; 36), "keyword"), ([43; 46), "rule"), ([37; 40), "rule"), ([49; 53), "keyword"), ([60; 66), "token"), ([54; 57), "rule")]"#
    );
}


#[test]
fn test_extend_selection() {
    let file = parse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"####);
    let offset = TextUnit::from_usize(44);
    let s1 =
        editor_api::extend_selection(&file, TextRange::from_len(offset, TextUnit::zero()))
            .unwrap();
    let s2 = editor_api::extend_selection(&file, s1).unwrap();
    let s3 = editor_api::extend_selection(&file, s2).unwrap();
    assert_eq!(
        format!("{:?}", (s1, s2, s3)),
        "([43; 46), [41; 48), [28; 48))"
    );
}


#[test]
fn test_find_refs() {
    let file = parse(TEXT);
    let usages = editor_api::find_usages(
        &file,
        TextUnit::from_usize(309)
    );

    assert_eq!(usages, vec![TextRange::from_len(
        TextUnit::from_usize(202),
        TextUnit::from_usize(12)
    )]);
}

fn parse(text: &str) -> fall_tree::File {
    lang_fall::lang_fall().parse(text.to_owned())
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
