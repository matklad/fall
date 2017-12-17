extern crate itertools;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rental;

extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;
pub extern crate lang_fall_syntax as syntax;

use fall_tree::{AstNode, File};

mod analysis;
pub mod editor_api;


pub use self::analysis::{Analysis, FileWithAnalysis, CallKind, RefKind, PratVariant, PrattOp,
                         MethodKind, Arity, ChildKind};

pub fn parse<S: Into<String>>(text: S) -> File {
    syntax::lang_fall().parse(text.into())
}

pub fn ast(file: &File) -> syntax::FallFile {
    syntax::FallFile::wrap(file.root()).unwrap()
}

mod editor;
pub use self::editor::FALL_EDITOR_SUPPORT;


#[cfg(test)]
mod test_util {
    use fall_tree::{File, TextUnit, TextRange};

    pub fn parse_with_caret(text: &str) -> (File, TextUnit) {
        ::fall_tree::test_util::parse_with_caret(::syntax::lang_fall(), text, "^")
    }

    pub fn parse_with_range(text: &str) -> (File, TextRange) {
        ::fall_tree::test_util::parse_with_range(::syntax::lang_fall(), text, "^")
    }
}
