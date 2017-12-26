extern crate elapsed;
extern crate file;
extern crate fall_tree;
extern crate lang_rust;

use std::path::PathBuf;

use fall_tree::{TextRange, tu};
use lang_rust::syntax::lang_rust;

fn main() {
    let text = file::get_text(test_data().join("parser.rs_")).unwrap();
    let (total, file) = elapsed::measure_time(|| lang_rust().parse(text));
    let ast_len = fall_tree::dump_file(&file).len();
    let errors = fall_tree::search::descendants_of_type(file.root(), fall_tree::ERROR);
    if let Some(err) = errors.into_iter().next() {
        let err_range = TextRange::from_len(err.range().start(), tu(80));
        let error_text = &file.text().to_string()[err_range];
        let parent = err.parent().unwrap();
        let ctx = parent.parent().unwrap_or(parent);
        eprintln!("\nError in\n----------\n{}\n----------\n{:?}\n----------\n{}\n----------\n\n",
                  ctx.text(),
                  parent,
                  error_text);
    }
    assert!(ast_len > 10000);
    println!("{}\ntotal: {}", file.metrics(), total);
}

fn test_data() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).join("tests/data")
}
