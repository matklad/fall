use fall_tree::File;
use fall_parse::{self, TreeBuilder};
use self::grammar::*;

mod grammar;


fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let closing = &"\"########################"[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

pub fn parse(text: String) -> File {
    register_node_types();
    fall_parse::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_weired
    )
}

fn parse_weired(b: &mut TreeBuilder) {
    if b.try_eat(RAW_STRING) {
        return
    }
    b.start(EMPTY);
    b.finish(EMPTY);
    assert!(b.try_eat(ATOM));
    b.start(EMPTY);
    b.finish(EMPTY);
}
