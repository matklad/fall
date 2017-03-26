use regex::Regex;
use fall_tree::{NodeType, TextRange};

pub type CustomRule = fn(&str) -> Option<usize>;

pub struct Rule {
    pub ty: NodeType,
    pub re: &'static str,
    pub f: Option<CustomRule>,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ty: NodeType,
    pub range: TextRange,
}

pub fn tokenize<'t>(text: &'t str, tokenizer: &[Rule]) -> TokenIter<'t> {
    TokenIter {
        rest: text,
        offset: 0,
        rules: tokenizer.iter().map(|r| {
            (r.ty, Regex::new(&format!("^{}", r.re)).unwrap(), r.f)
        }).collect(),
    }
}

pub struct TokenIter<'t> {
    rest: &'t str,
    offset: usize,
    rules: Vec<(NodeType, Regex, Option<CustomRule>)>,
}

impl<'t> Iterator for TokenIter<'t> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let longest_match = self.rules.iter().rev()
            .filter_map(|&(ty, ref re, f)| {
                re.find(self.rest).map(|m| (m.end(), (ty, f)))
            })
            .max_by_key(|&(len, _)| len);


        let (len, (ty, custom_rule)) = match longest_match {
            Some(m) => m,
            None => return Some(self.bad_char()),
        };

        assert!(len > 0);

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

impl<'t> TokenIter<'t> {
    fn bad_char(&mut self) -> Token {
        let char_len = self.rest.chars().next().unwrap().len_utf8();
        self.token(::ERROR, char_len)
    }

    fn token(&mut self, ty: NodeType, len: usize) -> Token {
        let range = TextRange::from_to(self.offset as u32, (self.offset + len) as u32);
        self.rest = &self.rest[len..];
        self.offset += len;
        Token { ty: ty, range: range }
    }
}

#[test]
fn tokenize_longest_first_wins() {
    let rules = &[
        Rule { ty: ::WHITESPACE, re: r"\s+", f: None },
        Rule { ty: NodeType(10), re: "foo", f: None },
        Rule { ty: NodeType(11), re: r"\w+", f: None },
        Rule { ty: NodeType(12), re: "foobar", f: None },
    ];

    let tokens: Vec<_> = tokenize("foo foob foobar", rules)
        .map(|t| t.ty.0)
        .collect();
    assert_eq!(tokens, vec![10, 1, 11, 1, 11]);
}
