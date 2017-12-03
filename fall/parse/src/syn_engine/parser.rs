use std::collections::HashMap;

use fall_tree::{TextUnit, IToken, Text, TextSuffix, NodeType, Event, tu};
use syn_engine::Grammar;
use {NodeTypeRef, ExprRef};

pub(crate) struct Parser<'g> {
    cache: Option<(HashMap<(TextUnit, ExprRef), (u32, u32, u32)>, &'g [Event])>,
    pub grammar: &'g Grammar<'g>,
    text: Text<'g>,
    tokens: &'g [IToken],
    non_ws_indexes: Vec<(TextUnit, usize)>,

    ticks: u64,
    events: Vec<Event>,
    pub replacement: Option<NodeTypeRef>,
    pub predicate_mode: bool,
    pub contexts: [bool; 16],
    pub args: [Option<ExprRef>; 16],
    pub prev: Option<NodeType>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Pos(u32, u32);

impl Pos {
    pub fn next(self) -> Pos {
        Pos(self.0 + 1, self.1)
    }

    pub fn is_empty(self) -> bool {
        self.0 == self.1
    }
}

pub(crate) struct Mark(u32);

impl<'g> Parser<'g> {
    pub fn new(
        cache: Option<(HashMap<(TextUnit, ExprRef), (u32, u32, u32)>, &'g [Event])>,
        grammar: &'g Grammar<'g>,
        is_ws: &Fn(IToken) -> bool,
        text: Text<'g>,
        tokens: &'g [IToken],
    ) -> (Parser<'g>, Pos) {
        let non_ws_indexes = {
            let mut indexes = Vec::new();
            let mut len = tu(0);
            for (i, &t) in tokens.iter().enumerate() {
                if !is_ws(t) {
                    indexes.push((len, i))
                }
                len += t.len
            }
            indexes
        };

        let pos = Pos(0, non_ws_indexes.len() as u32);

        let parser = Parser {
            cache,
            grammar,
            text,
            tokens,
            non_ws_indexes,

            ticks: 0,
            events: Vec::new(),
            replacement: None,
            predicate_mode: false,
            contexts: [false; 16],
            args: [None; 16],
            prev: None,
        };
        (parser, pos)
    }


    pub fn done(self) -> (Vec<Event>, u64) {
        (self.events, self.ticks)
    }

    pub fn tick(&mut self) {
        self.ticks += 1
    }


    pub fn start(&mut self, ty_idx: NodeTypeRef) -> Mark {
        let ty = self[ty_idx];
        self.start_ty(ty)
    }

    pub fn start_error(&mut self) {
        self.start_ty(::fall_tree::ERROR);
    }

    pub fn finish(&mut self) {
        self.event(Event::End)
    }

    pub fn get_from_cache(&mut self, expr: ExprRef, pos: Pos) -> Option<Pos> {
        if let Some((ref cache, events)) = self.cache {
            let text_pos = self.non_ws_indexes[pos.0 as usize].0;
            if let Some(&(start_event, n_events, n_tokens)) = cache.get(&(text_pos, expr)) {
                self.events.extend_from_slice(&events[start_event as usize..(start_event + n_events) as usize]);
                return Some(Pos(pos.0 + n_tokens, pos.1));
            }
        }

        return None;
    }

    pub fn start_cached(&mut self, expr: ExprRef) -> Mark {
        let mark = self.mark();
        self.event(Event::Cached { key: expr.0, n_events: 0 });
        mark
    }

    pub fn finish_cached(&mut self, mark: Mark) {
        if !self.predicate_mode {
            let len = self.events.len() as u32 - mark.0;
            match self.events[mark.0 as usize] {
                Event::Cached { ref mut n_events, .. } => *n_events = len,
                _ => unreachable!(),
            }
        }
    }

    pub fn reopen(&mut self) {
        if !self.predicate_mode {
            match self.events.pop() {
                Some(Event::End) => {}
                _ => unreachable!()
            }
        }
    }

    pub fn mark(&self) -> Mark {
        Mark(self.events.len() as u32)
    }

    pub fn rollback(&mut self, mark: Mark) {
        fn truncate_fast<T>(xs: &mut Vec<T>, len: usize) {
            assert!(len <= xs.len());
            unsafe { xs.set_len(len) }
        }

        truncate_fast(&mut self.events, mark.0 as usize);
    }

    pub fn replace(&mut self, mark: Mark, ty_idx: NodeTypeRef) {
        let ty = self[ty_idx];
        match self.events[mark.0 as usize] {
            Event::Start { ty: ref mut prev, .. } => *prev = ty,
            _ => unreachable!()
        }
    }

    pub fn forward_parent(&mut self, child: Mark, parent: Mark) {
        match self.events[child.0 as usize] {
            Event::Start { ref mut forward_parent, .. } =>
                *forward_parent = Some(parent.0 - child.0),
            _ => unreachable!(),
        }
    }

    pub fn bump(&mut self, pos: Pos) -> Option<(NodeType, Pos)> {
        if pos.is_empty() {
            return None;
        }
        let ty = self[pos].ty;
        self.token(ty, 1);
        Some((ty, pos.next()))
    }

    pub fn bump_by_text(&mut self, tokens: Pos, text: &str, ty_idx: NodeTypeRef) -> Option<Pos> {
        if tokens.is_empty() {
            return None;
        }
        let start = self.non_ws_indexes[tokens.0 as usize].0;
        let current_text = self.text.slice(TextSuffix::from(start));

        if !current_text.starts_with(text) {
            return None;
        }

        let mut leftover = tu(text.len() as u32);
        let mut n_tokens = 0;
        let mut pos = tokens;
        while leftover > tu(0) {
            if pos.is_empty() {
                return None;
            }
            let cur_len = self[pos].len;
            if leftover < cur_len {
                panic!("Textual match split token in half")
            }
            leftover -= cur_len;
            n_tokens += 1;
            pos = pos.next();
        }

        let ty = self[ty_idx];
        self.token(ty, n_tokens);

        Some(pos)
    }

    pub fn cut_suffix(&self, tokens: Pos, suffix: Pos) -> Pos {
        Pos(tokens.0, suffix.0)
    }

    fn start_ty(&mut self, ty: NodeType) -> Mark {
        let mark = Mark(self.events.len() as u32);
        self.event(Event::Start { ty, forward_parent: None });
        mark
    }

    fn token(&mut self, ty: NodeType, n_raw_tokens: u16) {
        self.event(Event::Token { ty, n_raw_tokens });
    }

    fn event(&mut self, event: Event) {
        if !self.predicate_mode {
            self.events.push(event)
        }
    }
}

impl<'g> ::std::ops::Index<Pos> for Parser<'g> {
    type Output = IToken;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.tokens[self.non_ws_indexes[index.0 as usize].1]
    }
}

impl<'g> ::std::ops::Index<NodeTypeRef> for Parser<'g> {
    type Output = NodeType;

    fn index(&self, index: NodeTypeRef) -> &Self::Output {
        &self.grammar.node_types[index.0 as usize]
    }
}
