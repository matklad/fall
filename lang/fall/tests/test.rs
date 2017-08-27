extern crate fall_tree;
extern crate lang_fall;

use std::path::Path;

use fall_tree::test_util::check_inline_tests;
use lang_fall::LANG_FALL;

#[test]
fn inline_tests() {
    check_inline_tests(&LANG_FALL, Path::new("src/fall.fall"), Path::new("tests/inline.txt"))
}

