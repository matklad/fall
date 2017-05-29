extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate fall_tree;
extern crate fall_parse;

use fall_tree::{AstNode, File};

mod syntax;
mod ast_ext;
mod highighting;

pub use self::syntax::*;
pub use self::ast_ext::{SelectorKind, RefKind};
pub use self::syntax::LANG as LANG_FALL;
pub use highighting::highlight;

pub fn ast(file: &File) -> FallFile {
    FallFile::new(file.root())
}

