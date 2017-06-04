use regex::Regex;
use fall_tree::{NodeType, ERROR, TextUnit};

pub type CustomRule = fn(&str) -> Option<usize>;

pub struct LexRule {
    pub ty: NodeType,
    pub re: Regex,
    pub f: Option<CustomRule>,
}

impl LexRule {
    pub fn new(ty: NodeType, re: &str, f: Option<CustomRule>) -> LexRule {
        LexRule {
            ty: ty,
            re: Regex::new(&format!("^({})", re)).unwrap(),
            f: f,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ty: NodeType,
    pub len: TextUnit,
}

pub fn tokenize<'t, 'r>(text: &'t str, tokenizer: &'r [LexRule]) -> TokenIter<'t, 'r> {
    TokenIter {
        rest: text,
        offset: 0,
        rules: tokenizer,
    }
}

pub struct TokenIter<'t, 'r> {
    rest: &'t str,
    offset: usize,
    rules: &'r [LexRule],
}

impl<'t, 'r> Iterator for TokenIter<'t, 'r> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let longest_match = self.rules.iter().rev()
            .filter_map(|rule| {
                rule.re.find(self.rest).map(|m| (m.end(), (rule.ty, rule.f)))
            })
            .max_by_key(|&(len, _)| len);


        let (len, (ty, custom_rule)) = match longest_match {
            Some(m) => m,
            None => return Some(self.bad_char()),
        };

        assert!(len > 0, "Empty Token {:?}", ty);

        let token = match custom_rule {
            None => self.token(ty, len),
            Some(f) => if let Some(len) = f(self.rest) {
                self.token(ty, len)
            } else {
                self.bad_char()
            }
        };

        Some(token)
    }
}

impl<'t, 'r> TokenIter<'t, 'r> {
    fn bad_char(&mut self) -> Token {
        let char_len = self.rest.chars().next().unwrap().len_utf8();
        self.token(ERROR, char_len)
    }

    fn token(&mut self, ty: NodeType, len: usize) -> Token {
        self.rest = &self.rest[len..];
        self.offset += len;
        Token { ty: ty, len: TextUnit::from_usize(len) }
    }
}

#[test]
fn tokenize_longest_first_wins() {
    let rules = &[
        LexRule::new(::fall_tree::WHITESPACE, r"\s+", None),
        LexRule::new(NodeType(10), "foo", None),
        LexRule::new(NodeType(11), r"\w+", None),
        LexRule::new(NodeType(12), "foobar", None),
    ];

    let tokens: Vec<_> = tokenize("foo foob foobar", rules)
        .map(|t| t.ty.0)
        .collect();
    assert_eq!(tokens, vec![10, 1, 11, 1, 11]);
}
