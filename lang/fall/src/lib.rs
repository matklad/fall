extern crate regex;

extern crate fall_tree;
extern crate fall_parse;

use fall_tree::{AstNode, File};

mod fall;
mod ast_ext;
pub mod editor_api;


pub use self::fall::*;
pub use self::ast_ext::{RefKind, PratKind, CallKind, MethodDescription, Arity, ChildKind};
pub use self::fall::language as lang_fall;

pub fn ast(file: &File) -> FallFile {
    FallFile::new(file.root())
}

