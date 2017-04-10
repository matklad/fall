use std::collections::HashSet;

use elapsed::ElapsedDuration;

use fall_tree::{Language, TextRange, NodeType, File, FileBuilder, NodeBuilder, WHITESPACE};
use lex::Token;


pub struct TreeBuilderImpl {
    lang: Language,
    text: String,
    skip: HashSet<NodeType>,
    tokens: Vec<Token>,
    pending: Vec<Frame>,
    current_token: usize,
}

#[derive(Debug)]
struct PreNode {
    ty: NodeType,
    range: TextRange,
    children: Vec<PreNode>,
}

#[derive(Debug)]
struct Frame {
    ty: NodeType,
    children: Vec<PreNode>,
    start_token: usize,
}


impl TreeBuilderImpl {
    pub fn current(&self) -> Option<Token> {
        self.tokens.get(self.current_token).cloned()
    }

    pub fn bump(&mut self) {
        self.bump_raw();
        self.do_skip();
    }

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

    pub fn rollback(&mut self, ty: NodeType) {
        assert!(self.top().ty == ty, "Expected {:?}, got {:?}", self.top().ty, ty);
        let top = self.pending.pop().unwrap();
        self.current_token = top.start_token;
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

    pub fn new(lang: Language,text: String, file_type: NodeType, tokens: Vec<Token>) -> TreeBuilderImpl {
        let skip = &[WHITESPACE];
        let mut result = TreeBuilderImpl {
            lang: lang,
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

    fn bump_raw(&mut self) {
        let t = self.current().expect("EOF reached");
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
            self.bump_raw();
        }
    }

    pub fn into_file(mut self, lex_time: ElapsedDuration, parse_time: ElapsedDuration) -> File {
        while self.current().is_some() {
            self.bump();
        }
        let top = self.pending.pop().unwrap();
        assert!(self.pending.is_empty());
        let root = self.to_prenode(top);
        let mut builder = FileBuilder::new(self.lang, self.text, lex_time, parse_time);
        go(&mut builder, None, root);
        return builder.build();

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

impl ::std::fmt::Debug for TreeBuilderImpl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("TreeBuilder {")?;
        let pending = self.pending.iter().map(|f| format!("{:?}", f.ty)).collect::<String>();
        f.write_str(&format!("  pending = {}", pending))?;
        f.write_str("}")?;
        Ok(())
    }
}
