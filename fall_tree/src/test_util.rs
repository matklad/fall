use std::path::Path;

use file;
use {Language, dump_file, dump_file_ws, TextRange, TextUnit};
use edit::TextEdit;
use difference::Changeset;

pub fn check_syntax(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file(&file);
    report_diff(expected_tree, &actual_tree);
}

pub fn check_syntax_ws(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file_ws(&file);
    report_diff(expected_tree, &actual_tree);
}

pub fn compare_trees(expected: &str, actual: &str) -> Changeset {
    Changeset::new(&expected, &actual, "\n")
}

pub fn check_reparse(
    lang: &Language,
    before_input: &str,
    after_input: &str,
    after_edit: &str,
    reparsed: &str
) {
    let before_file = lang.parse(before_input.to_owned());

    let edit = make_edit(before_input, after_input);
    let after_file = before_file.edit(&edit);
    let after_tree = dump_file(&after_file);
    report_diff(after_edit, &after_tree);

    let actual_reparsed = after_file.text().slice(after_file.stats().reparsed_region).to_string();
    report_diff(reparsed, &actual_reparsed);
}


pub fn check_directory(lang: &Language, directory: &Path) {
    let rewrite = ::std::env::var("rewrite_test_data").is_ok();
    for file in ::std::fs::read_dir(directory).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().unwrap_or_default() != "rs" {
            continue
        }
        let tree = path.with_extension("txt");
        check_file(lang, &path, &tree, rewrite);
    }
}


fn make_edit(before: &str, after: &str) -> TextEdit {
    let prefix = {
        before.as_bytes().iter()
            .zip(after.as_bytes())
            .position(|(a, b)| a != b)
            .unwrap()
    };
    let suffix = {
        before.as_bytes().iter().rev()
            .zip(after.as_bytes().iter().rev())
            .position(|(a, b)| a != b)
            .unwrap()
    };
    let delete = TextRange::from_to(
        TextUnit::from_usize(prefix),
        TextUnit::from_usize(before.len() - suffix)
    );
    let insert = after[prefix..after.len() - suffix].to_string();
    TextEdit { delete: delete, insert: insert }
}


fn report_diff(expected: &str, actual: &str) {
    if let Some(diff) = compute_diff(expected, actual) {
        println!("Actual\n{}\n\nExpected:\n{}\n\nDiff:\n{}\n",
                 actual, expected, diff);
        panic!("Mismatched trees!")
    }
}

fn compute_diff(expected: &str, actual: &str) -> Option<Changeset> {
    let actual = actual.trim();
    let expected = expected.trim();
    if expected == actual {
        return None;
    }
    Some(compare_trees(expected, actual))
}

fn check_file(lang: &Language, source: &Path, tree: &Path, rewrite: bool) {
    let source = file::get_text(source)
        .expect(&format!("Can't read {}", source.display()));

    let file = lang.parse(source);
    let actual_tree = dump_file(&file);
    let expected_tree = file::get_text(tree).ok();

    match (rewrite, expected_tree) {
        (true, None) => {
            println!("Creating {}", tree.display());
            file::put_text(tree, actual_tree).unwrap();
        }
        (false, None) => panic!("{} does not exist", tree.display()),
        (true, Some(expected_tree)) => {
            if let Some(_) = compute_diff(&expected_tree, &actual_tree) {
                println!("Rewriting {}", tree.display());
                file::put_text(tree, actual_tree).unwrap();
            }
        }
        (false, Some(expected_tree)) => report_diff(&expected_tree, &actual_tree)
    }
}
