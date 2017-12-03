extern crate fall_tree;
extern crate quickcheck;

use fall_tree::{Text, TextBuf, tu, ArbitraryEdit};
use fall_tree::{Language, LanguageImpl, IToken, INode, Metrics, NodeType, NodeTypeInfo, ERROR, Event};


#[test]
fn retokeization_is_equivalent_to_tokenization() {
    quickcheck::QuickCheck::new()
        .max_tests(500)
        .tests(500)
        .quickcheck(tokenize as fn(ArbitraryEdit) -> bool);
}

fn tokenize(edit: ArbitraryEdit) -> bool {
    let text: TextBuf = "(123 \"foo\" 123) hello".into();
    let edit = edit.into_text_edit(text.as_slice());
    let new_text = edit.apply(text.as_slice());

    let lang = Language::new(TestLang);

    let old_file = lang.parse(text);
    let new_file = lang.parse(new_text.clone());
    let rep_file = old_file.edit(edit);

    let expected = fall_tree::dump_file_ws(&new_file);
    let actual = fall_tree::dump_file_ws(&rep_file);

    ::fall_tree::test_util::report_diff(&expected, &actual);
    true
}

struct TestLang;

impl fall_tree::Lexer for TestLang {
    fn next_token(&self, text: Text) -> IToken {
        for &(p, ty) in &[("(", TEST_L_PAREN), (")", TEST_R_PAREN)] {
            if text.starts_with(p) {
                return IToken { ty, len: tu(1) };
            }
        }

        for &(p, ty) in &[(char::is_numeric as fn(char) -> bool, TEST_NUMBER), (char::is_whitespace, TEST_WHITESPACE)] {
            let len = text.chars().take_while(|&c| p(c)).count();
            if len > 0 {
                return IToken { ty, len: tu(len as u32) };
            }
        }
        match text.chars().next().unwrap() {
            '"' => {
                let len = 1 + text.chars().skip(1).take_while(|&c| c != '"').count();
                IToken { ty: TEST_STRING, len: tu(len as u32) }
            }
            c => {
                IToken { ty: ERROR, len: tu(c.len_utf8() as u32) }
            }
        }
    }
}

const TEST_WHITESPACE: NodeType = NodeType(9);
const TEST_FILE: NodeType = NodeType(10);
const TEST_NUMBER: NodeType = NodeType(11);
const TEST_L_PAREN: NodeType = NodeType(12);
const TEST_R_PAREN: NodeType = NodeType(13);
const TEST_STRING: NodeType = NodeType(14);

impl LanguageImpl for TestLang {
    fn lexer(&self) -> &fall_tree::Lexer {
        &TestLang
    }

    fn parse(&self, _: Text, tokens: &[IToken], _: &Metrics) -> (Vec<Event>, INode) {
        let mut result = INode::new(TEST_FILE);
        for &t in tokens {
            result.push_child(INode::new_leaf(t.ty, t.len));
        }
        (Vec::new(), result)
    }

    fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
        match ty {
            TEST_WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
            TEST_FILE => NodeTypeInfo { name: "TEST_FILE", whitespace_like: false },
            TEST_NUMBER => NodeTypeInfo { name: "TEST_NUMBER", whitespace_like: false },
            TEST_L_PAREN => NodeTypeInfo { name: "TEST_L_PAREN", whitespace_like: false },
            TEST_R_PAREN => NodeTypeInfo { name: "TEST_R_PAREN", whitespace_like: false },
            TEST_STRING => NodeTypeInfo { name: "TEST_STRING", whitespace_like: false },
            ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
            _ => unreachable!(),
        }
    }
}
