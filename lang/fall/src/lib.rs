extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate fall_tree;
extern crate fall_parse;

use fall_tree::{AstNode, File};

mod fall;
mod ast_ext;
pub mod editor_api;


pub use self::fall::*;
pub use self::ast_ext::{RefKind, PratKind, CallKind, MethodDescription, Arity, ChildKind};
pub use self::fall::language as lang_fall;

pub fn parse<S: Into<String>>(text: S) -> File {
    language().parse(text.into())
}
pub fn ast(file: &File) -> FallFile {
    FallFile::new(file.root())
}

#[cfg(test)]
mod test_util {
    use fall_tree::{File, TextUnit, TextRange};

    pub fn parse_with_caret(text: &str) -> (File, TextUnit) {
        ::fall_tree::test_util::parse_with_caret(::lang_fall(), text, "^")
    }

    pub fn parse_with_range(text: &str) -> (File, TextRange) {
        ::fall_tree::test_util::parse_with_range(::lang_fall(), text, "^")
    }
}
