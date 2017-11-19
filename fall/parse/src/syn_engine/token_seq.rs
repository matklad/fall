use fall_tree::{Language, NodeType, tu, IToken, Text, TextUnit, TextSuffix};

#[derive(Copy, Clone, Debug)]
pub struct BlackIdx(pub usize);

pub struct BlackTokens<'a> {
    text: Text<'a>,
    non_ws_indexes: Vec<(TextUnit, BlackIdx)>,
    original_tokens: &'a [IToken],
}

impl<'a> BlackTokens<'a> {
    pub fn new(lang: &Language, text: Text<'a>, tokens: &'a [IToken]) -> BlackTokens<'a> {
        let is_ws = |t: IToken| lang.node_type_info(t.ty).whitespace_like;

        let non_ws_indexes = {
            let mut indexes = Vec::new();
            let mut len = tu(0);
            for (i, &t) in tokens.iter().enumerate() {
                if !is_ws(t) {
                    indexes.push((len, BlackIdx(i)))
                }
                len += t.len
            }
            indexes
        };

        BlackTokens {
            text,
            non_ws_indexes,
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
    text: Text<'a>,
    non_ws_indexes: &'a [(TextUnit, BlackIdx)],
    original_tokens: &'a [IToken],
}


impl<'a> TokenSeq<'a> {
    pub fn current(&self) -> Option<IToken> {
        self.non_ws_indexes.first().map(|&(_, BlackIdx(idx))| {
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
        let (ty, (_, idx)) = (token.ty, self.non_ws_indexes[0]);

        let rest = TokenSeq {
            text: self.text,
            non_ws_indexes: &self.non_ws_indexes[1..],
            original_tokens: self.original_tokens,
        };
        ((ty, idx), rest)
    }

    pub fn bump_by_text(&self, text: &str) -> Option<usize> {
        let current_text = match self.non_ws_indexes.first() {
            None => return None,
            Some(&(start, _)) => self.text.slice(TextSuffix::from(start))
        };

        if !current_text.starts_with(text) {
            return None;
        }

        let mut leftover = tu(text.len() as u32);
        let mut rest = *self;
        let mut n_tokens = 0;

        while leftover > tu(0) {
            let cur_len = rest.current().unwrap().len;
            if leftover < cur_len {
                panic!("Textual match split token in half")
            }
            leftover -= cur_len;
            let (_, rest_) = rest.bump();
            rest = rest_;
            n_tokens += 1;
        }

        Some(n_tokens)
    }
}
