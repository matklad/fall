use std::ascii::AsciiExt;

pub fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}

pub fn camel(word: &str) -> String {
    word.split("_")
        .map(|w| w[..1].to_ascii_uppercase() + &w[1..])
        .collect()
}
