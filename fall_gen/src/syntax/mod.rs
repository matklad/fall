use fall_tree::File;
pub use self::grammar::*;

mod grammar;

pub fn parse(text: String) -> File {
    grammar::parse(text)
}
