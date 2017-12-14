use std::collections::HashMap;

use fall_tree::{NodeType, TextEdit, TextUnit, TextEditOp, tu, TextRange};
use lex_engine::Token;
use ::{Expr, ExprRef};

pub struct Grammar<'g> {
    pub node_types: &'g [NodeType],
    pub rules: &'g [Expr],
    pub start_rule: ExprRef,
}

impl<'g> ::std::ops::Index<ExprRef> for Grammar<'g> {
    type Output = Expr;

    fn index(&self, index: ExprRef) -> &Self::Output {
        &self.rules[index.0 as usize]
    }
}

mod parser;
mod expr;
mod pratt;

pub(crate) use self::expr::parse;

mod convert;

pub(crate) use self::convert::convert;


#[derive(Copy, Clone, Debug)]
pub(crate) enum Event {
    Start { ty: NodeType, forward_parent: Option<u32> },
    Token { ty: NodeType, n_raw_tokens: u16 },
    End,
    Cached { key: u32, n_events: u32 },
}

pub(crate) fn salvage_segments(
    old_events: &[Event],
    old_tokens: &[Token],
    is_ws: &Fn(&Token) -> bool,
    edit: &TextEdit,
) -> HashMap<(TextUnit, ExprRef), (u32, u32, u32)> {
    let mut result = HashMap::new();

    let mut raw_token_pos = 0;
    let mut text_pos = tu(0);

    let bump = |tok: &mut usize, text: &mut TextUnit| {
        *text += old_tokens[*tok].len;
        *tok += 1;
    };

    let skip_ws = |tok: &mut usize, text: &mut TextUnit| {
        while *tok < old_tokens.len() && is_ws(&old_tokens[*tok]) {
            bump(tok, text);
        }
    };

    let eat_tokens = |tok: &mut usize, text: &mut TextUnit, n: u16| {
        for _ in 0..n {
            bump(tok, text);
        }
        skip_ws(tok, text);
    };

    skip_ws(&mut raw_token_pos, &mut text_pos);

    let mut events = old_events.iter();
    let mut start_event = 0;

    while let Some(event) = events.next() {
        start_event += 1;
        match *event {
            Event::Start { .. } => (),
            Event::End => (),
            Event::Token { n_raw_tokens, .. } =>
                eat_tokens(&mut raw_token_pos, &mut text_pos, n_raw_tokens),

            Event::Cached { key, n_events } => {
                let start = text_pos;
                let mut n_tokens = 0u32;
                for _ in 0..n_events {
                    match *events.next().unwrap() {
                        Event::Token { n_raw_tokens, .. } => {
                            eat_tokens(&mut raw_token_pos, &mut text_pos, n_raw_tokens);
                            n_tokens += n_raw_tokens as u32;
                        }
                        _ => (),
                    }
                }
                let end = text_pos;
                let range = TextRange::from_to(start, end);

                let mut inserted = tu(0);
                for op in edit.ops.iter() {
                    match *op {
                        TextEditOp::Copy(copy) => if range.is_subrange_of(copy) {
                            let fixed_start = inserted + (start - copy.start());
                            result.insert((fixed_start, ExprRef(key)), (start_event, n_events, n_tokens));
                        } else {
                            inserted += copy.len()
                        }
                        TextEditOp::Insert(ref text) => inserted += text.len(),
                    }
                }
                start_event += n_events;
            }
        }
    }

    result
}



