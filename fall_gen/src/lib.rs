extern crate fall_tree;
extern crate fall_parse;

use fall_tree::dump_file;

mod syntax;
mod ast;

mod generator;

pub use ast::{Grammar, LexRule, SynRule, Alt, Part, LiftError};


pub fn debug(text: &str) -> String {
    dump_file(&syntax::parse(text.to_owned()))
}

pub fn parse(text: &str) -> Result<Grammar, LiftError> {
    let file = syntax::parse(text.to_owned());
    let root = file.root();
    ast::lift(root)
}
