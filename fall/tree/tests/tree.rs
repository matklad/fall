//extern crate fall_tree;
//extern crate quickcheck;
//
//use fall_tree::{Text, TextBuf, tu, ArbitraryEdit};
//use fall_tree::{Language, LanguageImpl, Token,  Metrics, NodeType, NodeTypeInfo, ERROR};
//
//
//#[test]
//fn retokeization_is_equivalent_to_tokenization() {
//    quickcheck::QuickCheck::new()
//        .max_tests(500)
//        .tests(500)
//        .quickcheck(tokenize as fn(ArbitraryEdit) -> bool);
//}
//
//fn tokenize(edit: ArbitraryEdit) -> bool {
//    let text: TextBuf = "(123 \"foo\" 123) hello".into();
//    let edit = edit.into_text_edit(text.as_slice());
//    let new_text = edit.apply(text.as_slice());
//
//    let lang = Language::new(TestLang);
//
//    let old_file = lang.parse(text);
//    let new_file = lang.parse(new_text.clone());
//    let rep_file = old_file.edit(edit);
//
//    let expected = fall_tree::dump_file_ws(&new_file);
//    let actual = fall_tree::dump_file_ws(&rep_file);
//
//    ::fall_tree::test_util::report_diff(&expected, &actual);
//    true
//}
