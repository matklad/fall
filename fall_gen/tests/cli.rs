extern crate file;
extern crate tempdir;
extern crate difference;

use std::process;
use std::env;
use std::path::{PathBuf, Path};

use tempdir::TempDir;

fn generator_path() -> PathBuf {
    let test_exe = env::current_exe().unwrap();
    test_exe.parent().unwrap().parent().unwrap().join("fall-gen")
}

fn check_by_path<T: AsRef<Path>>(grammar: T) {
    let grammar = grammar.as_ref();
    let input = file::get_text(grammar).unwrap();
    let expected = file::get_text(grammar.with_extension("rs")).unwrap();
    do_test(&input, &expected);
}

fn do_test(grammar: &str, expected: &str) {
    let dir = TempDir::new("gen-tests").unwrap();
    let grammar_path = dir.path().join("grammar.fall");
    file::put_text(&grammar_path, grammar).unwrap();

    let output = process::Command::new(generator_path())
        .arg(&grammar_path)
        .output()
        .expect("Failed to execute process");

    if !output.status.success() {
        panic!("Generator exited with code {:?}\n----\n{}\n----\n",
               output.status.code(), std::str::from_utf8(&output.stderr).unwrap())
    }

    let actual = file::get_text(grammar_path.with_extension("rs")).unwrap();

    if expected.trim() != actual.trim() {
        difference::print_diff(&actual, &expected, "\n");
        panic!("Mismatch!")
    }
}

#[test]
fn test_grammars_are_fresh() {
    check_by_path("../fall_test/src/sexp.txt");
    check_by_path("../fall_test/src/rust.txt");
    check_by_path("../fall_test/src/weird/grammar.fall");
    check_by_path("../fall_test/src/json.txt");
    check_by_path("./src/syntax.txt");
}
