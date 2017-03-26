use fall_tree::File;
use fall_parse::{self, TreeBuilder};
use self::grammar::*;

mod grammar;


pub fn parse(text: String) -> File {
    register_node_types();
    fall_parse::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_rust
    )
}

fn parse_rust(b: &mut TreeBuilder) {
    b.parse_many(&|b| {
        b.skip_until(&[PUB, FN, STRUCT]);
        parse_fn(b) || parse_struct(b)
    });
}

fn parse_fn(b: &mut TreeBuilder) -> bool {
    b.start(FN_DEF);
    b.try_eat(PUB);
    if !b.try_eat(FN) {
        b.rollback(FN_DEF);
        return false;
    }
    b.try_eat(IDENT)
        && b.try_eat(LPAREN) && b.try_eat(RPAREN)
        && b.try_eat(LBRACE) && b.try_eat(RBRACE);
    b.finish(FN_DEF);
    return true;
}

fn parse_struct(b: &mut TreeBuilder) -> bool {
    b.start(STRUCT_DEF);
    b.try_eat(PUB);
    if !b.try_eat(STRUCT) {
        b.rollback(STRUCT_DEF);
        return false;
    }
    b.try_eat(IDENT)
        && b.try_eat(LBRACE) && b.try_eat(RBRACE);
    b.finish(STRUCT_DEF);
    return true;
}
