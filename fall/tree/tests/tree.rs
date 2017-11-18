extern crate fall_tree;
extern crate quickcheck;

use fall_tree::{TextUnit, TextRange, Text, TextBuf, tu, ArbitraryEdit};
use fall_tree::{Language, LanguageImpl, IToken, INode, Metrics, NodeType, NodeTypeInfo, ERROR};


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

const TEST_WHITESPACE: NodeType = NodeType(9);
const TEST_FILE: NodeType = NodeType(10);
const TEST_NUMBER: NodeType = NodeType(11);
const TEST_L_PAREN: NodeType = NodeType(12);
const TEST_R_PAREN: NodeType = NodeType(13);
const TEST_STRING: NodeType = NodeType(14);

impl LanguageImpl for TestLang {
    fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't> {
        struct Iter<'t> { text: Text<'t> }
        impl<'t> Iter<'t> {
            fn tok(&mut self, ty: NodeType, len: TextUnit) -> IToken {
                let range = TextRange::from_to(len, self.text.len());
                self.text = self.text.slice(range);
                IToken { ty, len }
            }
        }

        impl<'t> Iterator for Iter<'t> {
            type Item = IToken;

            fn next(&mut self) -> Option<Self::Item> {
                if self.text.is_empty() {
                    return None;
                }

                for &(p, ty) in &[("(", TEST_L_PAREN), (")", TEST_R_PAREN)] {
                    if self.text.starts_with(p) {
                        return Some(self.tok(ty, tu(1)));
                    }
                }

                for &(p, ty) in &[(char::is_numeric as fn(char) -> bool, TEST_NUMBER), (char::is_whitespace, TEST_WHITESPACE)] {
                    let len = self.text.chars().take_while(|&c| p(c)).count();
                    if len > 0 {
                        return Some(self.tok(ty, tu(len as u32)));
                    }
                }
                match self.text.chars().next().unwrap() {
                    '"' => {
                        let len = 1 + self.text.chars().skip(1).take_while(|&c| c != '"').count();
                        Some(self.tok(TEST_STRING, tu(len as u32)))
                    }
                    c => {
                        Some(self.tok(ERROR, tu(c.len_utf8() as u32)))
                    }
                }
            }
        }

        Box::new(Iter { text })
    }

    fn parse(&self, _: Text, tokens: &[IToken], _: &Metrics) -> INode {
        let mut result = INode::new(TEST_FILE);
        for &t in tokens {
            result.push_child(INode::new_leaf(t.ty, t.len));
        }
        result
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
