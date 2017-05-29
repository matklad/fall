extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate tera;

extern crate fall_tree;
extern crate fall_parse;
extern crate lang_fall;

mod util;
mod generate;

pub fn generate(file: lang_fall::FallFile) -> String {
    generate::generate(file)
}
