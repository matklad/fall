extern crate fall_tree;

use fall_tree::{Language, dump_file};

#[derive(Clone, Copy)]
pub struct EditorSupport {
    pub extension: &'static str,
    pub syntax_tree: Option<fn(text: &str) -> String>,
}

impl EditorSupport {
    pub fn syntax_tree(&self, text: &str) -> Option<String> {
        let f = self.syntax_tree?;
        Some(f(text))
    }
}


pub fn gen_syntax_tree(lang: &Language, text: &str) -> String {
    let file = lang.parse(text);
    dump_file(&file)
}
