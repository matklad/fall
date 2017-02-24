extern crate fall;

use std::sync::{Once, ONCE_INIT};
use fall::builder::TreeBuilder;

mod grammar;

use grammar::*;

static  REGISTER_TOKENS: Once = ONCE_INIT;

pub fn parse(text: String) -> fall::File {
    REGISTER_TOKENS.call_once(register_node_types);

    fall::builder::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &|builder| {
            parse_file(builder);
        }
    )
}

fn parse_file(b: &mut TreeBuilder) {
    parse_object(b) || parse_array(b);
}

fn parse_object(b: &mut TreeBuilder) -> bool {
    b.start(OBJECT);
    if !b.try_eat(LBRACE) {
        b.rollback(OBJECT);
        return false;
    }

    b.parse_many(&|b| {
        b.skip_until(&[STRING, RBRACE]);
        let r = parse_field(b);
        if r {
            if b.try_eat(COMMA) {
                if b.next_is(RBRACK) {
                    b.error()
                }
            } else {
                if !b.next_is(RBRACK) {
                    b.error();
                }
            }
        }
        r
    });

    b.skip_until(&[RBRACE]);
    if !b.try_eat(RBRACE) {
        b.error();
    }
    b.finish(OBJECT);
    true
}

fn parse_field(b: &mut TreeBuilder) -> bool {
    b.start(FIELD);
    if !b.try_eat(STRING) {
        b.rollback(FIELD);
        return false
    }
    if !(b.try_eat(COLON) && parse_value(b)) {
        b.error();
    }
    b.finish(FIELD);
    true
}

fn parse_array(b: &mut TreeBuilder) -> bool {
    b.start(ARRAY);
    if !b.try_eat(LBRACK) {
        b.rollback(ARRAY);
        return false;
    }

    b.parse_many(&|b| {
        b.skip_until(&[NUMBER, NULL, BOOL, STRING, LBRACE, RBRACK]);
        let r = parse_value(b);
        if r {
            if b.try_eat(COMMA) {
                if b.next_is(RBRACK) {
                    b.error()
                }
            } else {
                if !b.next_is(RBRACK) {
                    b.error();
                }
            }
        }
        r
    });

    b.skip_until(&[RBRACK]);
    if !b.try_eat(RBRACK) {
        b.error();
    }
    b.finish(ARRAY);
    true
}

fn parse_value(b: &mut TreeBuilder) -> bool {
    parse_primitive(b) || parse_object(b) || parse_array(b)
}

fn parse_primitive(b: &mut TreeBuilder) -> bool {
    b.start(PRIMITIVE);
    if b.try_eat(NULL) || b.try_eat(NUMBER) || b.try_eat(STRING) || b.try_eat(BOOL) {
        b.finish(PRIMITIVE);
        return true;
    }
    b.rollback(PRIMITIVE);
    false
}
