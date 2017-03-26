extern crate regex;
extern crate fall_tree;
use fall_tree::{TextRange, NodeType, File, FileBuilder, NodeBuilder, ERROR, WHITESPACE};

use std::collections::HashSet;

mod tokenizer;
pub use tokenizer::Rule;

use tokenizer::{tokenize, Token};


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
        let node = self.to_prenode(top);

        if let Some(t) = node.children.first() {
            assert!(!self.is_skip(t.ty));
        }
        if let Some(t) = node.children.last() {
            assert!(!self.is_skip(t.ty));
        }

        self.top().children.push(node);
        self.do_skip();
    }

    fn to_prenode(&self, frame: Frame) -> PreNode {
        let range = if frame.children.is_empty() {
            let start = self.tokens.get(frame.start_token).map(|t| t.range.start()).unwrap_or(0);
            TextRange::from_to(start, start)
        } else {
            let first = frame.children.first().unwrap();
            let last = frame.children.last().unwrap();
            TextRange::from_to(first.range.start(), last.range.end())
        };

        PreNode { ty: frame.ty, range: range, children: frame.children }
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
        let root = self.to_prenode(top);
        let mut builder = FileBuilder::new();
        go(&mut builder, None, root);
        return builder.build(self.text);

        fn go(builder: &mut FileBuilder, parent: Option<NodeBuilder>, node: PreNode) {
            let id = builder.node(parent, node.ty, node.range);
            for child in node.children {
                go(builder, Some(id), child)
            }
        }
    }

    fn is_skip(&self, ty: NodeType) -> bool {
        self.skip.contains(&ty)
    }
}

impl ::std::fmt::Debug for TreeBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TreeBuilder {")?;
        let pending = self.pending.iter().map(|f| f.ty.name()).collect::<String>();
        f.write_str(&format!("  pending = {}", pending))?;
        f.write_str("}")?;
        Ok(())
    }
}

pub fn parse(
    text: String,
    file_type: NodeType,
    tokenizer: &[Rule],
    parser: &Fn(&mut TreeBuilder)
) -> File {
    let tokens = tokenize(&text, tokenizer).collect();
    let mut builder = TreeBuilder::new(text, file_type, tokens);
    parser(&mut builder);
    builder.into_file()
}

