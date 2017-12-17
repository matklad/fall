extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fall_tree;

use fall_tree::{File, Language, dump_file};

pub mod hl;

#[derive(Clone, Copy)]
pub struct EditorSupport {
    pub extension: &'static str,
    pub parse: fn(text: &str) -> File,
    pub syntax_tree: Option<fn(file: &File) -> String>,
    pub highlight: Option<fn(file: &File) -> hl::Highlights>,
}

impl EditorSupport {
    pub fn parse(&self, text: &str) -> File {
        (self.parse)(text)
    }

    pub fn syntax_tree(&self, file: &File) -> Option<String> {
        let f = self.syntax_tree?;
        Some(f(file))
    }

    pub fn highlight(&self, file: &File) -> hl::Highlights {
        match self.highlight {
            None => Vec::new(),
            Some(f) => f(file)
        }
    }
}

pub fn gen_parse(lang: &Language, text: &str) -> File {
    lang.parse(text)
}

pub fn gen_syntax_tree(lang: &Language, file: &File) -> String {
    dump_file(&file)
}
