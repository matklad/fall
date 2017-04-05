extern crate regex;
extern crate fall_tree;
extern crate fall_parse;

use fall_tree::dump_file;

#[macro_use]
mod util;
pub mod syntax;
mod ast;
mod old_ast;
pub mod gast;

mod generator;
mod gen2;

pub use old_ast::{Grammar, LexRule, SynRule, Alt, Part, LiftError};


pub fn debug(text: &str) -> String {
    dump_file(&syntax::parse(text.to_owned()))
}

pub fn parse(text: &str) -> Result<Grammar, LiftError> {
    let file = syntax::parse(text.to_owned());
    let root = file.root();
    old_ast::lift(root)
}
