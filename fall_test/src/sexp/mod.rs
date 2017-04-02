use fall_tree::File;

mod grammar;

pub fn parse(text: String) -> File {
    grammar::parse(text)
}
