use fall_tree::{Language, NodeType, TextUnit};

use ::lex_engine::Token;

pub type BlackIdx = usize;

pub struct BlackTokens<'a> {
    text: &'a str,
    non_ws_indexes: Vec<BlackIdx>,
    original_tokens: &'a [Token],
}

impl<'a> BlackTokens<'a> {
    pub fn new(lang: &Language, text: &'a str, tokens: &'a [Token]) -> BlackTokens<'a> {
        let is_ws = |t: Token| lang.node_type_info(t.ty).whitespace_like;

        let non_ws_indexes = tokens.iter().enumerate()
            .filter_map(|(i, &t)| if is_ws(t) { None } else { Some(i) })
            .collect();

        let ws_len = tokens.iter()
            .take_while(|&&t| is_ws(t))
            .map(|t| t.len.as_u32() as usize)
            .sum();

        BlackTokens {
            text: &text[ws_len..],
            non_ws_indexes: non_ws_indexes,
            original_tokens: tokens,
        }
    }

    pub fn seq(&'a self) -> TokenSeq<'a> {
        TokenSeq {
            text: self.text,
            non_ws_indexes: &self.non_ws_indexes,
            original_tokens: self.original_tokens,
        }
    }
}

/// An iterator over slice of tokens,
/// capable of skipping over insignificant trivia
/// like whitespace or comments.
#[derive(Clone, Copy, Debug)]
pub struct TokenSeq<'a> {
    pub text: &'a str,
    pub non_ws_indexes: &'a [BlackIdx],
    pub original_tokens: &'a [Token],
}


impl<'a> TokenSeq<'a> {
    pub fn current(&self) -> Option<Token> {
        self.non_ws_indexes.first().map(|&idx| {
            self.original_tokens[idx]
        })
    }

    pub fn cut_suffix(&self, suffix: TokenSeq<'a>) -> TokenSeq<'a> {
        let end = self.non_ws_indexes.len() - suffix.non_ws_indexes.len();
        TokenSeq {
            text: self.text,
            non_ws_indexes: &self.non_ws_indexes[..end],
            original_tokens: self.original_tokens
        }
    }

    pub fn bump(&self) -> ((NodeType, BlackIdx), TokenSeq<'a>) {
        let token = self.current().expect("Can't bump an empty token sequence");
        let (ty, idx) = (token.ty, self.non_ws_indexes[0]);

        let text_len = {
            let next_idx = self.non_ws_indexes.get(1).map(|&i| i).unwrap_or(self.original_tokens.len());
            self.original_tokens[self.non_ws_indexes[0] .. next_idx]
                .iter()
                .map(|t| t.len)
                .sum::<TextUnit>()
                .as_u32() as usize
        };

        let rest = TokenSeq {
            text: &self.text[text_len..],
            non_ws_indexes: &self.non_ws_indexes[1..],
            original_tokens: self.original_tokens,
        };
        ((ty, idx), rest)
    }

    pub fn bump_by_text(&self, text: &str) -> Option<(&'a [BlackIdx], TokenSeq<'a>)> {
        if !self.text.starts_with(text) {
            return None;
        }

        let mut leftover = TextUnit::from_usize(text.len());
        let mut rest = *self;
        let mut n_tokens = 0;

        while leftover > TextUnit::zero() {
            let cur_len = rest.current().unwrap().len;
            if leftover < cur_len {
                panic!("Textual match split token in half")
            }
            leftover -= cur_len;
            let (_, rest_) = rest.bump();
            rest = rest_;
            n_tokens += 1;
        }

        Some((&self.non_ws_indexes[..n_tokens], rest))
    }
}
