use fall_tree::{self, AstNode};

mod syntax;
mod ast_ext;

pub use self::syntax::*;
pub use self::ast_ext::{SelectorKind, Expr, RefKind};

pub fn parse(text: String) -> fall_tree::File {
    LANG.parse(text)
}

pub fn ast(file: &fall_tree::File) -> File {
    File::new(file.root())
}

