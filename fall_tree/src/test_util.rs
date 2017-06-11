use ::{Language, dump_file, dump_file_ws, TextRange, TextUnit};
use difference::Changeset;

pub fn check_syntax(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file(&file);
    report_diff(expected_tree, &actual_tree);
}

pub fn check_reparse(
    lang: &Language,
    before_input: &str,
    after_input: &str,
    before_edit: &str,
    after_edit: &str
) {
    let before_file = lang.parse(before_input.to_owned());
    let before_tree = dump_file(&before_file);
    report_diff(before_edit, &before_tree);

    let (range, new_text) = make_edit(before_input, after_input);
    let after_file = before_file.edit(range, new_text);
    let after_tree = dump_file(&after_file);
    report_diff(after_edit, &after_tree)
}

fn make_edit(before: &str, after: &str) -> (TextRange, String) {
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
    let range = TextRange::from_to(
        TextUnit::from_usize(prefix),
        TextUnit::from_usize(before.len() - suffix)
    );
    let edit = after[prefix..after.len() - suffix].to_string();
    (range, edit)
}

pub fn check_syntax_ws(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file_ws(&file);
    report_diff(expected_tree, &actual_tree);
}

pub fn compare_trees(expected: &str, actual: &str) -> Changeset {
    Changeset::new(&expected, &actual, "\n")
}


fn report_diff(expected: &str, actual: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if expected != actual {
        let diff = compare_trees(expected, actual);
        println!("Actual\n{}\n\nExpected:\n{}\n\nDiff:\n{}\n",
                 actual, expected, diff);
        panic!("Mismatched trees!")
    }
}
