extern crate fall_tree;
extern crate lang_bnf;

use std::path::Path;

#[test]
fn inline_tests() {
    fall_tree::test_util::check_inline_tests(
        &lang_bnf::lang_bnf(),
        Path::new("src/bnf.fall"),
        Path::new("tests/inline.txt")
    )
}
