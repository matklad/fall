extern crate fall_tree;
extern crate lang_fall;

use std::path::Path;

use fall_tree::{TextRange, TextBuf, TextEditBuilder, dump_file_ws, tu};
use fall_tree::test_util::{check_inline_tests, report_diff};

#[test]
fn inline_tests() {
    check_inline_tests(&lang_fall::syntax::lang_fall(), Path::new("syntax/src/fall.fall"), Path::new("tests/inline.txt"))
}

#[test]
fn relex_test() {
    let mut text: TextBuf = "tokenizer { foo 'foo' bar 'bar' }".into();
    let mut file = lang_fall::parse(text.to_string());

    let edit1 = |b: &mut TextEditBuilder| {
        b.insert(tu(21), " ".to_string());
    };

    let edit2 = |b: &mut TextEditBuilder| {
        b.replace(TextRange::from_len(tu(23), tu(3)), "meh".to_string());
    };

    let edits: Vec<fn(&mut TextEditBuilder)> = vec![edit1, edit2];
    for edit in edits {
        let edit = {
            let mut b = TextEditBuilder::new(text.as_text());
            edit(&mut b);
            b.build()
        };
        text = edit.apply(text.as_text());
        eprintln!("text = {}", text);
        file = file.edit(&edit);
        let fresh_file = lang_fall::parse(text.to_string());
        report_diff(&dump_file_ws(&fresh_file), &dump_file_ws(&file));
    }
}

#[test]
fn reparse_test() {
    let mut text: TextBuf = "pub rule foo { bar }  pub rule bar { }".into();
    let mut file = lang_fall::parse(text.to_string());

    let edit1 = |b: &mut TextEditBuilder| {
        b.replace(TextRange::from_len(tu(15), tu(3)), "baz".to_string());
    };

    let edit2 = |b: &mut TextEditBuilder| {
        b.replace(TextRange::from_len(tu(15), tu(3)), "baz".to_string());
    };

    let edits: Vec<fn(&mut TextEditBuilder)> = vec![edit1, edit2];

    for edit in edits {
        let edit = {
            let mut b = TextEditBuilder::new(text.as_text());
            edit(&mut b);
            b.build()
        };
        text = edit.apply(text.as_text());
        file = file.edit(&edit);
        let fresh_file = lang_fall::parse(text.to_string());
        report_diff(&dump_file_ws(&fresh_file), &dump_file_ws(&file));
    }
}
