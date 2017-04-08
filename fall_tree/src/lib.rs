#[macro_use]
extern crate lazy_static;
extern crate elapsed;

mod text_range;
mod node_type;
mod node;
mod ast;
mod util;
mod lang;
pub mod visitor;
pub mod search;

pub use text_range::TextRange;
pub use node_type::{NodeType, NodeTypeInfo, ERROR, WHITESPACE};
pub use node::{File, Node, FileBuilder, NodeBuilder};
pub use lang::{Language, LanguageImpl};
pub use ast::{AstNode, AstChildren};
pub use util::{dump_file, dump_file_ws, walk_tree};
