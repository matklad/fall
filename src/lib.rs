extern crate regex;
#[macro_use]
extern crate lazy_static;

mod text_range;
mod node_type;
mod node;

pub use text_range::TextRange;
pub use node_type::{NodeType, NodeTypeInfo, ERROR, WHITESPACE};
pub use node::{Node, File};
pub mod builder;
