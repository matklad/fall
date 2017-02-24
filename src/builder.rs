use std::iter::FromIterator;
use std::collections::HashSet;

use {TextRange, NodeType, File};
use regex::Regex;

pub struct Rule {
    pub ty: NodeType,
    pub re: &'static str
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub ty: NodeType,
    pub range: TextRange,
}

pub struct TreeBuilder {
    text: String,
    skip: HashSet<NodeType>,
    tokens: Vec<Token>,
    pending: Vec<Frame>,
    current_token: usize,
}

#[derive(Debug)]
pub struct PreNode {
    pub ty: NodeType,
    pub range: TextRange,
    pub children: Vec<PreNode>,
}

#[derive(Debug)]
struct Frame {
    ty: NodeType,
    children: Vec<PreNode>,
    start_token: usize,
}


impl TreeBuilder {
    pub fn start(&mut self, ty: NodeType) {
        self.pending.push(Frame { ty: ty, children: vec![], start_token: self.current_token })
    }

    pub fn finish(&mut self, ty: NodeType) {
        assert!(self.top().ty == ty, "Expected {:?}, got {:?}", self.top().ty, ty);
        let mut top = self.pending.pop().unwrap();

        while let Some(ty) = top.children.last().map(|n| n.ty) {
            if !self.is_skip(ty) {
                break
            }
            top.children.pop();
            self.current_token -= 1;
        }

        let first_token = self.tokens[top.start_token];
        let last_token = self.tokens[self.current_token - 1];
        assert!(!self.is_skip(first_token.ty) && !self.is_skip(last_token.ty));

        let range = TextRange {
            start: first_token.range.start,
            end: last_token.range.end,
        };
        let node = PreNode { ty: top.ty, range: range, children: top.children };
        self.top().children.push(node);
    }

    pub fn rollback(&mut self, ty: NodeType) {
        assert!(self.top().ty == ty, "Expected {:?}, got {:?}", self.top().ty, ty);
        let top = self.pending.pop().unwrap();
        self.current_token = top.start_token;
    }

    pub fn try_eat(&mut self, ty: NodeType) -> bool {
        let t = match self.current() {
            None => return false,
            Some(t) => t
        };

        if t.ty == ty {
            self.bump();
            self.do_skip();
            true
        } else {
            false
        }
    }

    pub fn skip_until(&mut self, tys: &[NodeType]) {
        self.do_skip();
        self.start(::ERROR);
        let mut skipped = false;
        while let Some(t) = self.current() {
            if tys.contains(&t.ty) {
                break;
            }
            if !self.is_skip(t.ty) {
                skipped = true;
            }
            self.bump();
        }
        if skipped {
            self.finish(::ERROR);
        } else {
            self.rollback(::ERROR);
        }
    }

    pub fn parse_many(&mut self, f: &Fn(&mut TreeBuilder) -> bool) {
        loop {
            if !f(self) {
                break
            }
        }
    }

    pub fn next_is(&mut self, ty: NodeType) -> bool {
        self.do_skip();
        if let Some(t) = self.current() {
            t.ty == ty
        } else {
            false
        }
    }

    pub fn error(&mut self) {}

    fn new(text: String, file_type: NodeType, tokens: Vec<Token>) -> TreeBuilder {
        let skip = &[::WHITESPACE];
        let mut result = TreeBuilder {
            text: text,
            skip: skip.iter().cloned().collect(),
            tokens: tokens,
            pending: vec![Frame { ty: file_type, children: vec![], start_token: 0 }],
            current_token: 0,
        };
        result.do_skip();
        result
    }

    fn top(&mut self) -> &mut Frame {
        self.pending.last_mut().unwrap()
    }

    fn bump(&mut self) {
        let t = self.current().unwrap();
        self.current_token += 1;
        self.top().children.push(PreNode {
            ty: t.ty,
            range: t.range,
            children: vec![],
        })
    }

    fn do_skip(&mut self) {
        while let Some(t) = self.current() {
            if !self.is_skip(t.ty) {
                break;
            }
            self.bump();
        }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.current_token).cloned()
    }

    fn into_file(mut self) -> File {
        while self.current().is_some() {
            self.bump();
        }
        let top = self.pending.pop().unwrap();
        assert!(self.pending.is_empty());
        ::imp::build_file(self.text, top.ty, top.children)
    }

    fn is_skip(&self, ty: NodeType) -> bool {
        self.skip.contains(&ty)
    }
}

pub fn parse(
    text: String,
    file_type: NodeType,
    tokenizer: &[Rule],
    parser: &Fn(&mut TreeBuilder)
) -> File {
    let tokens = tokenize(&text, tokenizer);
    let mut builder = TreeBuilder::new(text, file_type, tokens);
    parser(&mut builder);
    builder.into_file()
}

fn tokenize(text: &str, tokenizer: &[Rule]) -> Vec<Token> {
    let mut result = vec![];

    let rules = Vec::from_iter(
        tokenizer.iter().map(|r| (r.ty, Regex::new(&format!("^{}", r.re)).unwrap()))
    );

    let mut offset = 0;
    let mut rest = text;

    'l: while rest.len() > 0 {
        for &(ty, ref re) in rules.iter() {
            if let Some(m) = re.find(rest) {
                assert!(m.start() == 0);
                assert!(m.end() > 0);
                result.push(Token {
                    ty: ty,
                    range: TextRange { start: offset as u32, end: (offset + m.end()) as u32 },
                });
                offset += m.end();
                rest = &rest[m.end()..];
                continue 'l;
            }
        }
        let bad_char_len = rest.chars().next().unwrap().len_utf8();
        result.push(Token {
            ty: ::ERROR,
            range: TextRange { start: offset as u32, end: (offset + bad_char_len) as u32 }
        });
        offset += bad_char_len;
        rest = &rest[bad_char_len..];
    }

    result
}
