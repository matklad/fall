extern crate lazy_static;


#[macro_use]
extern crate serde_derive;
extern crate elapsed;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

mod lex;
mod syn;
mod tree_builder;

pub use lex::{LexRule};
pub use syn::{SynRule, Expr, Parser, PrattVariant};

pub mod runtime {
    pub use serde_json;
    pub use regex;
    pub use fall_tree;
    pub use lazy_static::*;
    pub use tree_builder::parse;
}
