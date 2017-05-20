#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate elapsed;
extern crate regex;
extern crate tera;
extern crate fall_tree;
extern crate fall_parse;
use elapsed::ElapsedDuration;
use fall_tree::AstNode;

mod util;
pub mod lang;

mod highighting;
mod generate;

pub fn generate(file: lang::File) -> String {
    generate::generate(file)
}
pub use highighting::colorize;

pub struct FallFile {
    file: fall_tree::File,
}

impl FallFile {
    pub fn parse(text: String) -> Self {
        FallFile { file: lang::LANG.parse(text) }
    }

    pub fn ast(&self) -> lang::File {
        lang::File::new(self.file.root())
    }

    pub fn tree_to_string(&self) -> String {
        fall_tree::dump_file(&self.file)
    }

    pub fn lexing_time(&self) -> ElapsedDuration {
        self.file.lexing_time()
    }
    pub fn parsing_time(&self) -> ElapsedDuration {
        self.file.parsing_time()
    }
}
