use std::cmp::Ordering;
use fall_tree::{Text, tu, IToken, TextSuffix, TextEditOp, TextEdit};

mod regex;

pub use self::regex::RegexLexer;

pub trait Lexer {
    fn next_token(&self, text: Text) -> IToken;

    fn step(&self, text: &mut Text) -> IToken {
        let t = self.next_token(*text);
        *text = text.slice(TextSuffix::from(t.len));
        t
    }
}

pub fn lex<L: Lexer>(lexer: &L, text: Text) -> Vec<IToken> {
    let mut result = Vec::new();
    let mut text = text;
    while !text.is_empty() {
        let t = lexer.step(&mut text);
        result.push(t);
    }
    result
}

pub fn relex<L: Lexer>(
    lexer: &L,
    old_tokens: &[IToken],
    edit: &TextEdit,
    new_text: Text
) -> (Vec<IToken>, usize)
{
    let mut old_tokens = old_tokens.iter().cloned();
    let mut old_len = tu(0);

    let mut new_tokens: Vec<IToken> = Vec::new();
    let mut new_len = tu(0);

    let mut edit_point = tu(0);
    let mut reused = tu(0);

    for op in edit.ops.iter() {
        match *op {
            TextEditOp::Insert(ref buf) => {
                edit_point += buf.len()
            }
            TextEditOp::Copy(range) => {
                let mut txt = new_text.slice(TextSuffix::from(new_len));
                while new_len < edit_point {
                    let token = lexer.step(&mut txt);
                    new_len += token.len;
                    new_tokens.push(token)
                }

                while old_len < range.start() {
                    old_len += old_tokens.next().unwrap().len;
                }

                loop {
                    let new_consumed = new_len - edit_point;
                    let old_consumed = old_len - range.start();
                    if new_consumed >= range.len() || old_consumed >= range.len() {
                        break
                    }

                    match new_consumed.cmp(&old_consumed) {
                        Ordering::Less => {
                            let token = lexer.step(&mut txt);
                            new_len += token.len;
                            new_tokens.push(token)
                        }
                        Ordering::Equal => {
                            for token in &mut old_tokens {
                                old_len += token.len;
                                if old_len >= range.end() {
                                    break;
                                }
                                reused += token.len;
                                new_len += token.len;
                                new_tokens.push(token);
                            }
                        }
                        Ordering::Greater => {
                            let token = old_tokens.next().unwrap();
                            old_len += token.len;
                        }
                    }
                }

                edit_point += range.len()
            }
        }
    }

    let mut txt = new_text.slice(TextSuffix::from(new_len));
    while !txt.is_empty() {
        new_tokens.push(lexer.step(&mut txt));
    };
    let relexed_region = (new_text.len() - reused).utf8_len();
    (new_tokens, relexed_region)
}

