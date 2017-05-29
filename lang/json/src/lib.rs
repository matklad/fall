extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate fall_tree;
extern crate fall_parse;

mod syntax;

pub use self::syntax::*;
pub use self::syntax::LANG as LANG_JSON;

