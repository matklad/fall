use fall_tree::{AstNode, File};

mod syntax;
mod ast_ext;

pub use self::syntax::*;
pub use self::ast_ext::{SelectorKind, RefKind};

pub fn parse(text: String) -> File {
    LANG.parse(text)
}

pub fn ast(file: &File) -> FallFile {
    FallFile::new(file.root())
}

