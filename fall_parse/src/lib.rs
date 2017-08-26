extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate elapsed;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

use regex::Regex;
use fall_tree::NodeType;

mod lex;
mod syn;
mod tree_builder;

pub use syn::{SynRule, Expr, Parser, PrattVariant};

pub struct LexRule {
    pub ty: NodeType,
    pub re: Regex,
    pub f: Option<CustomRule>,
}

pub type CustomRule = fn(&str) -> Option<usize>;

impl LexRule {
    pub fn new(ty: NodeType, re: &str, f: Option<CustomRule>) -> LexRule {
        LexRule {
            ty: ty,
            re: Regex::new(&format!("^({})", re)).unwrap(),
            f: f,
        }
    }
}


pub mod runtime {
    pub use serde_json;
    pub use regex;
    pub use fall_tree;
    pub use lazy_static::*;
    pub use tree_builder::parse;
}
