use fall_tree::Text;
use std::ascii::AsciiExt;

pub fn scream(word: Text) -> String {
    word.to_cow().chars().map(|c| c.to_ascii_uppercase()).collect()
}

pub fn camel(word: Text) -> String {
    word.to_cow()
        .split("_")
        .map(|w| w[..1].to_ascii_uppercase() + &w[1..])
        .collect()
}
