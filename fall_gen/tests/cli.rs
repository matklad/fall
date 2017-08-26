extern crate fall_tree;
extern crate file;
extern crate tempdir;

use std::process;
use std::env;
use std::path::{PathBuf, Path};

use tempdir::TempDir;

fn generator_path() -> PathBuf {
    let test_exe = env::current_exe().unwrap();
    test_exe.parent().unwrap().parent().unwrap().join("gen")
}

fn check_by_path<T: AsRef<Path>>(grammar_path: T, should_rewrite: bool) {
    let grammar_path = grammar_path.as_ref();
    let generated_path = &grammar_path.with_extension("rs");
    let grammar_text = file::get_text(grammar_path).unwrap();

    let expected = file::get_text(generated_path).unwrap_or_default();

    let generated = {
        let dir = TempDir::new("gen-tests").unwrap();
        let tmp_file = dir.path().join("grammar.fall");
        file::put_text(&tmp_file, grammar_text).unwrap();

        let output = process::Command::new(generator_path())
            .arg(&tmp_file)
            .output()
            .expect("Failed to execute process");

        if !output.status.success() {
            panic!("Generator exited with code {:?}\nERR:\n----\n{}\nOUT:\n----\n{}\n---\n",
                   output.status.code(),
                   std::str::from_utf8(&output.stderr).unwrap(),
                   std::str::from_utf8(&output.stdout).unwrap(),
            )
        } else {
            let err = std::str::from_utf8(&output.stderr).unwrap();
            if !err.trim().is_empty() {
                eprintln!("{}", err);
            }
        }
        file::get_text(tmp_file.with_extension("rs")).unwrap()
    };

    if expected != generated {
        if should_rewrite {
            println!("UPDATING {}", grammar_path.display());
            file::put_text(generated_path, generated)
                .unwrap_or_else(|_| panic!("Failed to write result to {}", generated_path.display()));
            return;
        }
        let difference = fall_tree::test_util::compare_trees(&expected, &generated);
        println!("MISMATCH {}\n{}", grammar_path.display(), difference);
        panic!("Mismatch!")
    }
}

#[test]
fn test_grammars_are_fresh() {
    let bootsrap = ::std::env::var("rewrite").unwrap_or_default() == "bootstrap";
    if bootsrap {
        check_by_path("../lang/fall/src/syntax.fall", true);
        return;
    }

    let rewrite_parsers = ::std::env::var("rewrite").unwrap_or_default() == "parsers";
    if rewrite_parsers {
        check_by_path("../fall_test/src/sexp.fall", true);
        check_by_path("../fall_test/src/weird.fall", true);
        check_by_path("../fall_test/src/arith.fall", true);
        check_by_path("../lang/rust/src/syntax.fall", true);
        check_by_path("../lang/json/src/syntax.fall", true);
        return;
    }

    check_by_path("../fall_test/src/sexp.fall", false);
    check_by_path("../fall_test/src/weird.fall", false);
    check_by_path("../fall_test/src/arith.fall", false);
    check_by_path("../lang/rust/src/syntax.fall", false);
    check_by_path("../lang/json/src/syntax.fall", false);
    check_by_path("../lang/fall/src/syntax.fall", false);
}
