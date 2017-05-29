use ::{Language, dump_file, dump_file_ws};
use difference;

pub fn check_syntax(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file(&file);
    compare_trees(&actual_tree, expected_tree);
}

pub fn check_syntax_ws(lang: &Language, input: &str, expected_tree: &str) {
    let file = lang.parse(input.to_owned());
    let actual_tree = dump_file_ws(&file);
    compare_trees(&actual_tree, expected_tree);
}

fn compare_trees(expected: &str, actual: &str) {
    let actual = actual.trim();
    let expected = expected.trim();
    if actual == expected {
        return;
    }
    let difference = difference::Changeset::new(&expected, &actual, "\n");
    println!("{}", difference);
    panic!("Different trees!")
}


