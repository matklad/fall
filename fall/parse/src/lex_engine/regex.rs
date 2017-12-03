use fall_tree::{Text, ERROR, tu, IToken};

use LexRule;
use super::Lexer;

pub struct RegexLexer {
    rules: Vec<LexRule>,
}

impl RegexLexer {
    pub fn new(rules: Vec<LexRule>) -> RegexLexer {
        RegexLexer { rules }
    }
}

impl Lexer for RegexLexer {
    fn next_token(&self, text: Text) -> IToken {
        let text = text.to_cow();
        let text = text.as_ref();
        let longest_match = self.rules.iter().rev()
            .filter_map(|rule| {
                rule.re.find(text).map(|m| (tu(m.end() as u32), (rule.ty, rule.f)))
            })
            .max_by_key(|&(len, _)| len);

        let first_char_len = tu(text.chars().next().unwrap().len_utf8() as u32);
        let (len, (ty, custom_rule)) = match longest_match {
            Some(m) => m,
            None => return IToken { ty: ERROR, len: first_char_len },
        };

        assert!(len > tu(0), "Empty Token {:?}", ty);

        match custom_rule {
            None => IToken { ty, len },
            Some(f) => if let Some(len) = f(text) {
                IToken { ty, len: tu(len as u32) }
            } else {
                IToken { ty: ERROR, len: first_char_len }
            }
        }
    }
}

#[test]
fn tokenize_longest_first_wins() {
    use fall_tree::NodeType;
    let lexer = RegexLexer::new(vec![
        LexRule::new(NodeType(1), r"\s+", None),
        LexRule::new(NodeType(10), "foo", None),
        LexRule::new(NodeType(11), r"\w+", None),
        LexRule::new(NodeType(12), "foobar", None),
    ]);

    let text: ::fall_tree::TextBuf = "foo foob foobar".into();
    let tokens: Vec<_> = ::lex_engine::lex(&lexer, text.as_slice())
        .into_iter()
        .map(|t| t.ty.0)
        .collect();
    assert_eq!(tokens, vec![10, 1, 11, 1, 11]);
}

