use fall_tree::File;
pub use self::grammar::*;

mod grammar;

pub fn parse(text: String) -> File {
    grammar::parse(text)
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let closing = &"\"########################"[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}
