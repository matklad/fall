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

#[macro_use]
mod util;
pub mod syntax;
pub mod ast_ext;

mod generate;

pub struct FallFile {
    file: fall_tree::File,
}

impl FallFile {
    pub fn parse(text: String) -> Self {
        FallFile { file: syntax::LANG.parse(text) }
    }

    pub fn ast(&self) -> syntax::File {
        syntax::File::new(self.file.root())
    }

    pub fn tree_to_string(&self) -> String {
        fall_tree::dump_file(&self.file)
    }

    pub fn generate(&self) -> String {
        generate::generate(self.ast())
    }

    pub fn lexing_time(&self) -> ElapsedDuration {
        self.file.lexing_time()
    }
    pub fn parsing_time(&self) -> ElapsedDuration {
        self.file.parsing_time()
    }
}
