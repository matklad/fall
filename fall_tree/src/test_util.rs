use ::{Language, dump_file, dump_file_ws};
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


fn report_diff(expected: &str, actual: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if expected != actual {
        let diff = compare_trees(expected, actual);
        println!("{}", diff);
        panic!("Mismatched trees!")
    }
}
