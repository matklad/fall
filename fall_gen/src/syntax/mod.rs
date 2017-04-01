mod grammar;

use fall_tree::File;
use fall_parse::{self, TreeBuilder};
pub use self::grammar::*;

pub fn parse(text: String) -> File {
    register_node_types();
    fall_parse::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_file
    )
}

fn parse_file(b: &mut TreeBuilder) {
    parse_nodes(b);
    b.skip_until(&[KW_TOKENIZER]);
    parse_tokenizer(b);

    b.parse_many(&parse_syn_rule)
}

fn parse_nodes(b: &mut TreeBuilder) -> bool {
    b.start(NODES_DEF);
    if !b.try_eat(KW_NODES) {
        b.rollback(NODES_DEF);
        return false;
    }
    if b.try_eat(LBRACE) {
        b.parse_many(&|b| b.try_eat(IDENT));
        b.try_eat(RBRACE);
    }
    b.finish(NODES_DEF);
    true
}

fn parse_tokenizer(b: &mut TreeBuilder) -> bool {
    b.start(TOKENIZER_DEF);
    if !b.try_eat(KW_TOKENIZER) {
        b.rollback(TOKENIZER_DEF);
        return false;
    }
    if b.try_eat(LBRACE) {
        b.parse_many(&|b| {
            b.skip_until(&[RBRACE, IDENT]);
            parse_lex_rule(b)
        });
        b.try_eat(RBRACE);
    }
    b.finish(TOKENIZER_DEF);
    true
}

fn parse_lex_rule(b: &mut TreeBuilder) -> bool {
    b.start(LEX_RULE);
    if !b.try_eat(IDENT) {
        b.rollback(LEX_RULE);
        return false
    }
    parse_string(b) && parse_string(b);
    b.finish(LEX_RULE);
    true
}

fn parse_syn_rule(b: &mut TreeBuilder) -> bool {
    b.start(SYN_RULE);
    if !b.try_eat(KW_RULE) {
        b.rollback(SYN_RULE);
        return false;
    }
    if b.try_eat(IDENT) && b.try_eat(LBRACE) {
        parse_syn_rule_body(b);
        b.try_eat(RBRACE);
    }
    b.finish(SYN_RULE);
    true
}

fn parse_syn_rule_body(b: &mut TreeBuilder) {
    loop {
        if b.next_is(RBRACE) || b.next_is(RANGLE) || b.current().is_none() {
            return;
        }
        parse_alt(b);
        if !(b.next_is(RBRACE) || b.next_is(RANGLE)) {
            b.try_eat(PIPE);
        }
    }
}

fn parse_alt(b: &mut TreeBuilder) {
    b.start(ALT);
    b.parse_many(&|b| {
        b.skip_until(&[RBRACE, PIPE, IDENT, LANGLE, LPAREN, RPAREN, RANGLE]);
        if b.current().is_none() || b.next_is(RBRACE) || b.next_is(RPAREN) || b.next_is(PIPE) || b.next_is(RANGLE) {
            false
        } else {
            parse_part(b);
            true
        }
    });
    b.finish(ALT)
}

fn parse_part(b: &mut TreeBuilder) {
    b.start(PART);
    if b.try_eat(IDENT) {
        // NOP
    } else if b.try_eat(LANGLE) {
        if b.try_eat(IDENT) {
            parse_syn_rule_body(b)
        }
        b.skip_until(&[RANGLE]);
        b.try_eat(RANGLE);
    } else {
        unreachable!()
    }

    b.finish(PART);
}


fn parse_string(b: &mut TreeBuilder) -> bool {
    b.start(STRING);
    if b.try_eat(SIMPLE_STRING) || b.try_eat(HASH_STRING) {
        b.finish(STRING);
        true
    } else {
        b.rollback(STRING);
        false
    }
}

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let closing = &"\"########################"[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}
