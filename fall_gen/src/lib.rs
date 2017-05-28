extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate elapsed;
extern crate regex;
extern crate tera;
extern crate fall_tree;
extern crate fall_parse;

mod util;
pub mod lang;

mod highighting;
mod generate;

pub fn generate(file: lang::FallFile) -> String {
    generate::generate(file)
}
pub use highighting::colorize;
