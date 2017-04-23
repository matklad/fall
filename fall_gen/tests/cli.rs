extern crate file;
extern crate tempdir;
extern crate difference;

use std::process;
use std::env;
use std::path::{PathBuf, Path};

use tempdir::TempDir;

const OVERWRITE: bool = true;

fn generator_path() -> PathBuf {
    let test_exe = env::current_exe().unwrap();
    test_exe.parent().unwrap().parent().unwrap().join("gen")
}

fn check_by_path<T: AsRef<Path>>(grammar_path: T) {
    let grammar_path = grammar_path.as_ref();
    let generated_path = &grammar_path.with_extension("rs");
    let grammar_text = file::get_text(grammar_path).unwrap();

    let expected = file::get_text(generated_path).unwrap();

    let generated = {
        let dir = TempDir::new("gen-tests").unwrap();
        let tmp_file = dir.path().join("grammar.fall");
        file::put_text(&tmp_file, grammar_text).unwrap();

        let output = process::Command::new(generator_path())
            .arg(&tmp_file)
            .output()
            .expect("Failed to execute process");

        if !output.status.success() {
            panic!("Generator exited with code {:?}\n----\n{}\n----\n",
                   output.status.code(), std::str::from_utf8(&output.stderr).unwrap())
        }
        file::get_text(tmp_file.with_extension("rs")).unwrap()
    };

    if expected != generated {
        if OVERWRITE {
            println!("UPDATING {}", grammar_path.display());
            file::put_text(generated_path, generated)
                .unwrap_or_else(|_| panic!("Failed to write result to {}", generated_path.display()));
            return
        }
        let difference = difference::Changeset::new(&expected, &generated, "\n");
        println!("MISMATCH {}\n{}", grammar_path.display(), difference);
        panic!("Mismatch!")
    }
}

#[test]
fn test_grammars_are_fresh() {
    check_by_path("../fall_test/src/sexp.fall");
    check_by_path("../fall_test/src/rust.fall");
    check_by_path("../fall_test/src/weird.fall");
    check_by_path("../fall_test/src/json.fall");

    check_by_path("./src/syntax.fall")
}
