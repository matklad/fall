use std::path::Path;

use file;
use {Language, File, dump_file, dump_file_ws, TextRange, TextUnit, tu};
use difference::Changeset;



pub fn extract_range(input: &str, caret: &str) -> (String, TextRange) {
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
    let range = TextRange::from_to(
        tu(left_offset as u32),
        tu((right_offset - caret.len()) as u32),
    );
    (input, range)
}

pub fn extract_offset(input: &str, caret: &str) -> (String, TextUnit) {
    let left_offset = input.find(caret).expect(
        &format!("No caret ({}) in\n{}\n", caret, input)
    );
    let right_offset = left_offset + caret.len();
    let input = input[..left_offset].to_string() + &input[right_offset..];
    (input, tu(left_offset as u32))
}

pub fn parse_with_caret(lang: &Language, input: &str, caret: &str) -> (File, TextUnit) {
    let (text, offset) = extract_offset(input, caret);
    (lang.parse(text), offset)
}

pub fn parse_with_range(lang: &Language, input: &str, caret: &str) -> (File, TextRange) {
    let (input, range) = extract_range(input, caret);
    let file = lang.parse(input);
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

pub fn check_equal_files(expected: &File, actual: &File) {
    let expected = dump_file_ws(expected);
    let actual = dump_file_ws(actual);
    report_diff(&expected, &actual);
}

pub fn compare_trees(expected: &str, actual: &str) -> Changeset {
    Changeset::new(&expected, &actual, "\n")
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
            let separator = "----------------------------------------";
            for (e, a) in expected.split(separator).zip(actual.split(separator)) {
                if e != a {
                    report_diff(&e, &a)
                }
            }
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


pub fn report_diff(expected: &str, actual: &str) {
    if let Some(diff) = compute_diff(expected, actual) {
        println!("Actual\n{}\n\nExpected:\n{}\n\nDiff:\n{}\n",
                 actual, expected, diff);
        panic!("Mismatch")
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

fn check_file(lang: &Language, source_path: &Path, tree: &Path, rewrite: bool) {
    let source = file::get_text(source_path)
        .expect(&format!("Can't read {}", source_path.display()));

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
