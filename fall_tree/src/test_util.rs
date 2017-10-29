use std::path::Path;

use file;
use {Language, File, dump_file, dump_file_ws, TextRange, TextUnit};
use text_edit::TextEdit;
use difference::Changeset;


pub fn parse_with_caret(lang: &Language, input: &str, caret: &str) -> (File, TextUnit) {
    let offset = input.find(caret).expect(
        &format!("No caret ({}) in\n{}\n", caret, input)
    );
    let input = input[..offset].to_string() + &input[offset + caret.len()..];
    (lang.parse(input), TextUnit::from_usize(offset))
}

pub fn parse_with_range(lang: &Language, input: &str, caret: &str) -> (File, TextRange) {
    let left_offset = input.find(caret).expect(
        &format!("No caret ({}) in\n{}\n", caret, input)
    );
    let mid_offset = left_offset + caret.len();
    let right_offset = mid_offset + input[mid_offset..].find(caret).expect(
        &format!("Only single caret ({}) in \n{}\n", caret, input)
    );
    let input = input[..left_offset].to_string()
        + &input[mid_offset..right_offset]
        + &input[right_offset + caret.len()..];

    let file = lang.parse(input);
    let range = TextRange::from_to(
        TextUnit::from_usize(left_offset),
        TextUnit::from_usize(right_offset - caret.len()),
    );
    (file, range)
}

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

pub fn check_inline_tests(lang: &Language, grammar: &Path, test_data: &Path) {
    let rewrite = ::std::env::var("rewrite_test_data").is_ok();
    let grammar = file::get_text(grammar)
        .unwrap_or_else(|_| panic!("Can't read {}", grammar.display()));

    let tests = collect_tests(&grammar);
    let expected = render_tests(lang, &tests);
    let actual = file::get_text(test_data).unwrap_or(String::new());
    if expected != actual {
        if rewrite {
            file::put_text(test_data, expected).unwrap();
        } else {
            report_diff(&expected, &actual)
        }
    }
}

fn collect_tests(mut grammar: &str) -> Vec<String> {
    let mut result = Vec::new();
    while let Some(pos) = grammar.find("test r") {
        grammar = &grammar[pos + "test r".len()..];
        let n_hashes = grammar.chars().take_while(|&c| c == '#').count();
        grammar = &grammar[n_hashes + 1..];
        if let Some(end) = grammar.find(&"\"################"[..1 + n_hashes]) {
            let example = &grammar[..end].trim();
            result.push(example.to_string())
        }
    }
    result
}

fn render_tests(lang: &Language, tests: &[String]) -> String {
    let mut result = String::new();
    for test in tests {
        result += test;
        result += "\n\n";
        let file = lang.parse(test.to_owned());
        result += &dump_file(&file);
        result += "\n----------------------------------------\n"
    }
    result
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
    TextEdit { delete, insert }
}


pub fn report_diff(expected: &str, actual: &str) {
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
