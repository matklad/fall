use fall_tree::{File};
use fall_parse::{self, TreeBuilder};
use self::grammar::*;

mod grammar;


pub fn parse(text: String) -> File {
    register_node_types();
    fall_parse::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_sexp_list
    )
}

fn parse_sexp_list(b: &mut TreeBuilder) {
    b.parse_many(&|b| {
        b.skip_until(&[ATOM, LPAREN]);
        parse_sexp(b)
    });
}

fn parse_sexp(b: &mut TreeBuilder) -> bool {
    if b.try_eat(ATOM) {
        return true;
    }
    b.start(LIST);
    if b.try_eat(LPAREN) {
        parse_sexp_list(b);
        b.skip_until(&[RPAREN]);
        if !b.try_eat(RPAREN) {
            b.error();
        }
        b.finish(LIST);
        return true;
    } else {
        b.rollback(LIST);
    }
    return false;
}
