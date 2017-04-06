extern crate regex;
extern crate fall_tree;
extern crate fall_parse;

use fall_tree::AstNode;

#[macro_use]
mod util;
pub mod syntax;
pub mod ast;
pub mod gast;

mod generate;

pub struct FallFile {
    file: fall_tree::File,
}

impl FallFile {
    pub fn parse(text: String) -> Self {
        FallFile { file: syntax::parse(text) }
    }

    pub fn generate(&self) -> String {
        ast::File::new(self.file.root()).generate()
    }
}
