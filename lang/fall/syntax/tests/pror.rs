#[macro_use]
extern crate proptest;
extern crate lang_fall_syntax;
extern crate fall_tree;

use proptest::prelude::prop::collection::vec;

use fall_tree::{TextBuf, TextRange, tu};
use fall_tree::prop::{self, ArbTextEdit};
use fall_tree::test_util::check_equal_files;

use lang_fall_syntax::lang_fall;

#[test]
fn regression1() {
    let text = r##"
        tokenizer { }
    "##;
    let edits = [
        ArbTextEdit {
            ops: vec![
                (false, TextRange::from_to(tu(0), tu(2761050404))),
                (true, TextRange::from_to(tu(0), tu(306783379))),
            ]
        },
        ArbTextEdit { ops: vec![] }
    ];
    incremental_reparse_is_equivalent_to_full_parse(text, &edits);
}

#[test]
fn regression2() {
    let text = r##"
        tokenizer { }

        rule comma_list(el) {
          {el <commit> {<eof> | ','}}*
        }

        // comment
        #[cached]
        pub rule syn_rule {
          attributes?
          'rule' <commit>
          parameters?
          <layer
            { '{' in_braces '}' | '{' {<not file_form_first> <any>}* }
            block_expr
          >
        }

        ast {
         node fall_file {
            tokenizer_def tokenizer_def?
            syn_rules syn_rule*
            verbatim_def verbatim_def?
            ast_def ast_def?
            tests test_def*
          }
        }
    "##;

    let edits = [
        ArbTextEdit {
            ops: vec![
                (false, TextRange::from_to(tu(3166362751), tu(3417163761))),
                (true, TextRange::from_to(tu(135850548), tu(689702778))),
                (true, TextRange::from_to(tu(762853072), tu(2664760731)))
            ]
        },
        ArbTextEdit {
            ops: vec![
                (true, TextRange::from_to(tu(0), tu(0))),
                (true, TextRange::from_to(tu(0), tu(1426128137))),
                (true, TextRange::from_to(tu(1442711022), tu(1459293908)))
            ]
        }
    ];
    incremental_reparse_is_equivalent_to_full_parse(text, &edits);
}

#[test]
fn regression3() {
    let text = r##"
        tokenizer { }

        rule comma_list(el) {
          {el <commit> {<eof> | ','}}*
        }

        // comment
        #[cached]
        pub rule syn_rule {
          attributes?
          'rule' <commit>
          parameters?
          <layer
            { '{' in_braces '}' | '{' {<not file_form_first> <any>}* }
            block_expr
          >
        }

        ast {
         node fall_file {
            tokenizer_def tokenizer_def?
            syn_rules syn_rule*
            verbatim_def verbatim_def?
            ast_def ast_def?
            tests test_def*
          }
        }
    "##;

    let edits = [
        ArbTextEdit {
            ops: vec![
                (true, TextRange::from_to(tu(679252736), tu(1421205723))),
                (true, TextRange::from_to(tu(1786957196), tu(2340809426))),
                (true, TextRange::from_to(tu(2476659974), tu(2591610436)))
            ]
        },
        ArbTextEdit {
            ops: vec![
                (true, TextRange::from_to(tu(0), tu(540847734))),
                (true, TextRange::from_to(tu(2036132644), tu(3149642683)))
            ]
        },
        ArbTextEdit {
            ops: vec![
                (true, TextRange::from_to(tu(0), tu(4129776246))),
                (false, TextRange::from_to(tu(0), tu(1817101548)))
            ]
        }
    ];
    incremental_reparse_is_equivalent_to_full_parse(text, &edits);
}

proptest! {
    #[test]
    fn simple_tokens(ref edits in vec(prop::arb_text_edit(), 2..5)) {
        incremental_reparse_is_equivalent_to_full_parse(r##"
            tokenizer { }
        "##, edits
        )
    }
}

proptest! {
    #[test]
    fn complex(ref edits in vec(prop::arb_text_edit(), 2..5)) {
        incremental_reparse_is_equivalent_to_full_parse(r##"
            tokenizer { }

            rule comma_list(el) {
              {el <commit> {<eof> | ','}}*
            }

            // comment
            #[cached]
            pub rule syn_rule {
              attributes?
              'rule' <commit>
              parameters?
              <layer
                { '{' in_braces '}' | '{' {<not file_form_first> <any>}* }
                block_expr
              >
            }

            ast {
             node fall_file {
                tokenizer_def tokenizer_def?
                syn_rules syn_rule*
                verbatim_def verbatim_def?
                ast_def ast_def?
                tests test_def*
              }
            }
        "##, edits
        )
    }
}

fn incremental_reparse_is_equivalent_to_full_parse(
    initial_text: &str,
    edits: &[ArbTextEdit],
) {
    let initial_text = normalize(initial_text);
    let mut current_text = TextBuf::from(initial_text);
    let mut current_file = lang_fall().parse(current_text.as_text());
    for e in edits {
        let e = e.as_text_edit(current_text.as_text());
        current_text = e.apply(current_text.as_text());

        current_file = lang_fall().reparse(&current_file, &e);
        let fresh_file = lang_fall().parse(current_text.as_text());

        check_equal_files(&fresh_file, &current_file);
    }
}

fn normalize(text: &str) -> String {
    if is_blank(text) {
        return String::new();
    }
    let text = text.trim_right();
    let n_blank_lines = text.lines().take_while(|&line| is_blank(line)).count();
    let indent = text.lines()
        .skip(n_blank_lines)
        .filter(|&line| !is_blank(line))
        .map(|line| line.chars().take_while(|&c| c == ' ').count())
        .min().unwrap();
    let mut result = String::new();
    for line in text.lines().skip(n_blank_lines) {
        if !is_blank(line) {
            result.push_str(&line[indent..]);
        }
        result.push('\n')
    }
    result
}

fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}
