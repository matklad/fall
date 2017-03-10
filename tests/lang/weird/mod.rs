use fall;
use fall::builder::TreeBuilder;
use self::grammar::*;

mod grammar;


pub fn parse(text: String) -> fall::File {
    register_node_types();
    fall::builder::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_weired
    )
}

fn parse_weired(b: &mut TreeBuilder) {
    b.start(EMPTY);
    b.finish(EMPTY);
    assert!(b.try_eat(ATOM));
    b.start(EMPTY);
    b.finish(EMPTY);
}
