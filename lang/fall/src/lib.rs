extern crate itertools;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate rental;

extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;
pub extern crate lang_fall_syntax as syntax;

use fall_tree::{AstNode, File};

mod analysis;


pub use self::analysis::{Analysis, FileWithAnalysis, CallKind, RefKind, PratVariant, PrattOp,
                         MethodKind, Arity, ChildKind};

pub fn parse<S: Into<String>>(text: S) -> File {
    syntax::lang_fall().parse(text.into())
}

pub fn analyse<S: Into<String>>(text: S) -> FileWithAnalysis {
    FileWithAnalysis::new(parse(text))
}

pub fn ast(file: &File) -> syntax::FallFile {
    syntax::FallFile::wrap(file.root()).unwrap()
}

pub mod editor;


#[cfg(test)]
mod test_util {
    use fall_tree::{File, TextUnit};

    pub fn parse_with_caret(text: &str) -> (File, TextUnit) {
        ::fall_tree::test_util::parse_with_caret(::syntax::lang_fall(), text, "^")
    }
}
