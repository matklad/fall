use fall_tree::{Text, NodeType, ERROR, TextUnit, tu, IToken, TextRange};

use LexRule;

pub type Token = IToken;

pub fn tokenize2<'f, 'r>(text: Text<'f>, tokenizer: &'r [LexRule]) -> TokenIter2<'f, 'r> {
    TokenIter2 {
        rest: text,
        offset: tu(0),
        rules: tokenizer,
    }
}

pub struct TokenIter2<'t, 'r> {
    rest: Text<'t>,
    offset: TextUnit,
    rules: &'r [LexRule],
}


impl<'t, 'r> Iterator for TokenIter2<'t, 'r> {
    type Item = IToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let longest_match = self.rules.iter().rev()
            .filter_map(|rule| {
                rule.re.find(self.rest.to_cow().as_ref()).map(|m| (m.end(), (rule.ty, rule.f)))
            })
            .max_by_key(|&(len, _)| len);


        let (len, (ty, custom_rule)) = match longest_match {
            Some(m) => m,
            None => return Some(self.bad_char()),
        };

        assert!(len > 0, "Empty Token {:?}", ty);

        let token = match custom_rule {
            None => self.token(ty, len),
            Some(f) => if let Some(len) = f(self.rest.to_cow().as_ref()) {
                self.token(ty, len)
            } else {
                self.bad_char()
            }
        };

        Some(token)
    }
}

impl<'t, 'r> TokenIter2<'t, 'r> {
    fn bad_char(&mut self) -> IToken {
        let char_len = self.rest.to_cow().chars().next().unwrap().len_utf8();
        self.token(ERROR, char_len)
    }

    fn token(&mut self, ty: NodeType, len: usize) -> IToken {
        let len = tu(len as u32);
        self.rest = self.rest.slice(TextRange::from_to(len, self.rest.len()));
        self.offset += len;
        IToken { ty, len }
    }
}


#[test]
fn tokenize_longest_first_wins() {
    let rules = &[
        LexRule::new(NodeType(1), r"\s+", None),
        LexRule::new(NodeType(10), "foo", None),
        LexRule::new(NodeType(11), r"\w+", None),
        LexRule::new(NodeType(12), "foobar", None),
    ];

    let text: ::fall_tree::TextBuf = "foo foob foobar".into();
    let tokens: Vec<_> = tokenize2(text.as_slice(), rules)
        .map(|t| t.ty.0)
        .collect();
    assert_eq!(tokens, vec![10, 1, 11, 1, 11]);
}
