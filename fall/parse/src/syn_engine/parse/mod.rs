mod pratt;

use fall_tree::{NodeType, Language, Text, TextUnit, IToken, TextSuffix, tu};
use ::{Expr, NodeTypeRef, Context, Arg};

use super::{Event, Grammar};

pub fn parse(
    grammar: Grammar,
    lang: &Language,
    text: Text,
    tokens: &[IToken],
) -> (Vec<Event>, u64) {
    let start_rule = grammar.start_rule;

    let is_ws = |t: IToken| lang.node_type_info(t.ty).whitespace_like;

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

    let mut parser = Parser {
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

    let pos = Pos(0, parser.non_ws_indexes.len() as u32);
    let mut leftover = parse_expr(&mut parser, start_rule, pos).unwrap();
    if !leftover.is_empty() {
        parser.reopen();
        parser.start_error();
        while let Some((_, ts)) = parser.bump(leftover) {
            leftover = ts;
        }
        parser.finish();
        parser.finish();
    };

    (parser.events, parser.ticks)
}


struct Parser<'g> {
    grammar: Grammar<'g>,
    text: Text<'g>,
    tokens: &'g [IToken],
    non_ws_indexes: Vec<(TextUnit, usize)>,

    ticks: u64,
    events: Vec<Event>,
    replacement: Option<NodeTypeRef>,
    predicate_mode: bool,
    contexts: [bool; 16],
    args: [Option<&'g Expr>; 16],
    prev: Option<NodeType>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos(u32, u32);

impl Pos {
    fn next(self) -> Pos {
        Pos(self.0 + 1, self.1)
    }

    fn is_empty(self) -> bool {
        self.0 == self.1
    }
}

struct Mark(u32);

impl<'g> Parser<'g> {
    fn mark(&self) -> Mark {
        Mark(self.events.len() as u32)
    }

    fn rollback(&mut self, mark: Mark) {
        fn truncate_fast<T>(xs: &mut Vec<T>, len: usize) {
            assert!(len <= xs.len());
            unsafe { xs.set_len(len) }
        }

        truncate_fast(&mut self.events, mark.0 as usize);
    }

    fn start(&mut self, ty_idx: NodeTypeRef) -> Mark {
        let ty = self[ty_idx];
        self.start_ty(ty)
    }

    fn start_ty(&mut self, ty: NodeType) -> Mark {
        let mark = Mark(self.events.len() as u32);
        self.event(Event::Start { ty, forward_parent: None });
        mark
    }

    fn start_error(&mut self) {
        self.start_ty(::fall_tree::ERROR);
    }

    fn token(&mut self, ty: NodeType, n_raw_tokens: u16) {
        self.event(Event::Token { ty, n_raw_tokens });
    }

    fn finish(&mut self) {
        self.event(Event::End)
    }

    fn reopen(&mut self) {
        if !self.predicate_mode {
            match self.events.pop() {
                Some(Event::End) => {}
                _ => unreachable!()
            }
        }
    }

    fn bump(&mut self, pos: Pos) -> Option<(NodeType, Pos)> {
        if pos.is_empty() {
            return None;
        }
        let ty = self[pos].ty;
        self.token(ty, 1);
        Some((ty, pos.next()))
    }

    fn bump_by_text(&mut self, tokens: Pos, text: &str, ty_idx: NodeTypeRef) -> Option<Pos> {
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

    fn cut_suffix(&self, tokens: Pos, suffix: Pos) -> Pos {
        Pos(tokens.0, suffix.0)
    }

    fn replace(&mut self, mark: Mark, ty_idx: NodeTypeRef) {
        let ty = self[ty_idx];
        match self.events[mark.0 as usize] {
            Event::Start { ty: ref mut prev, .. } => *prev = ty,
            _ => unreachable!()
        }
    }

    fn forward_parent(&mut self, child: Mark, parent: Mark) {
        match self.events[child.0 as usize] {
            Event::Start { ref mut forward_parent, .. } =>
                *forward_parent = Some(parent.0 - child.0),
            _ => unreachable!(),
        }
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

fn parse_expr<'g>(p: &mut Parser<'g>, expr: &'g Expr, tokens: Pos) -> Option<Pos> {
    p.ticks += 1;
    let mark = p.mark();
    let result = parse_expr_inner(p, expr, tokens);
    if result.is_none() {
        p.rollback(mark);
    }
    result
}

fn parse_expr_pred<'t, 'g>(p: &mut Parser<'g>, expr: &'g Expr, tokens: Pos) -> Option<Pos> {
    let old_mode = p.predicate_mode;
    p.predicate_mode = true;
    let result = parse_expr(p, expr, tokens);
    p.predicate_mode = old_mode;
    result
}

fn parse_expr_inner<'g>(p: &mut Parser<'g>, expr: &'g Expr, tokens: Pos) -> Option<Pos> {
    match *expr {
        Expr::Pub { ty_idx, ref body, replaceable } =>
            parse_pub(p, tokens, ty_idx, body, replaceable),

        Expr::PubReplace { ty_idx, ref body } =>
            parse_pub_replace(p, tokens, ty_idx, body),

        Expr::Or(ref parts) =>
            parse_or(p, parts, tokens),

        Expr::And(ref parts, commit) =>
            parse_and(p, tokens, &*parts, commit),

        Expr::Rule(id) =>
            parse_expr(p, &p.grammar.rules[id].body, tokens),

        Expr::Token(ty_idx) =>
            parse_token(p, tokens, ty_idx),

        Expr::ContextualToken(ty_idx, ref text) =>
            parse_contextual_token(p, tokens, ty_idx, text),

        Expr::Opt(ref body) =>
            parse_opt(p, tokens, body),

        Expr::Not(ref e) =>
            parse_not(p, tokens, e),

        Expr::Eof =>
            parse_eof(p, tokens),

        Expr::Any =>
            p.bump(tokens).map(|(_ty, ts)| ts),

        Expr::Layer(ref l, ref e) =>
            parse_layer(p, tokens, l, e),

        Expr::Rep(ref body) =>
            parse_rep(p, tokens, body),

        Expr::WithSkip(ref first, ref body) =>
            parse_with_skip(p, tokens, first, body),

        Expr::Pratt(ref table) =>
            pratt::parse_pratt(p, table, tokens),

        Expr::Enter(ctx, ref e) =>
            parse_enter(p, tokens, ctx, e),

        Expr::Exit(ctx, ref e) =>
            parse_exit(p, tokens, ctx, e),

        Expr::IsIn(ctx) =>
            parse_is_in(p, tokens, ctx),

        Expr::Call(ref body, ref args) =>
            parse_call(p, tokens, body, &*args),

        Expr::Var(i) =>
            parse_var(p, tokens, i),

        Expr::PrevIs(ref ts) =>
            parse_prev_is(p, tokens, ts),

        Expr::Inject(ref prefix, ref body) =>
            parse_inject(p, tokens, prefix, body),
    }
}


fn parse_pub<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ty_idx: NodeTypeRef, body: &'g Expr, replaceable: bool,
) -> Option<Pos> {
    if replaceable {
        p.replacement = None;
    }
    let mark = p.start(ty_idx);
    let ts = parse_expr(p, body, tokens)?;
    if let (true, Some(ty)) = (replaceable, p.replacement) {
        p.replacement = None;
        p.replace(mark, ty)
    };
    p.finish();
    p.prev = Some(p[ty_idx]);
    Some(ts)
}

fn parse_pub_replace<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ty_idx: NodeTypeRef, body: &'g Expr
) -> Option<Pos> {
    let ts = parse_expr(p, body, tokens)?;
    p.replacement = Some(ty_idx);
    Some(ts)
}

fn parse_or<'t, 'g>(
    p: &mut Parser<'g>,
    options: &'g [Expr],
    tokens: Pos
) -> Option<(Pos)> {
    options.iter().filter_map(|opt| parse_expr(p, opt, tokens)).next()
}

fn parse_and<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    parts: &'g [Expr], commit: Option<usize>,
) -> Option<Pos> {
    let mut tokens = tokens;
    let mut consumed = 0;

    for part in parts {
        if let Some(ts) = parse_expr(p, part, tokens) {
            consumed += 1;
            tokens = ts;
        } else {
            break;
        }
    }
    if consumed < commit.unwrap_or(parts.len()) {
        return None;
    }
    if consumed < parts.len() {
        p.start_error();
        p.finish()
    }
    Some(tokens)
}

fn parse_token<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ty_idx: NodeTypeRef,
) -> Option<Pos> {
    let (ty, ts) = p.bump(tokens)?;
    if p[ty_idx] != ty {
        return None;
    }
    Some(ts)
}

fn parse_contextual_token<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ty_idx: NodeTypeRef, text: &str,
) -> Option<Pos> {
    p.bump_by_text(tokens, text, ty_idx)
}

fn parse_opt<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    body: &'g Expr,
) -> Option<Pos> {
    Some(parse_expr(p, body, tokens).unwrap_or(tokens))
}

fn parse_not<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    e: &'g Expr,
) -> Option<Pos> {
    match parse_expr_pred(p, e, tokens) {
        None => Some(tokens),
        Some(_) => None,
    }
}

fn parse_eof<'g>(
    _: &mut Parser<'g>, tokens: Pos,
) -> Option<Pos> {
    if tokens.is_empty() { Some(tokens) } else { None }
}

fn parse_layer<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    l: &'g Expr, e: &'g Expr,
) -> Option<Pos> {
    let rest = parse_expr_pred(p, l, tokens)?;
    let layer = p.cut_suffix(tokens, rest);
    let mut leftovers = parse_expr(p, e, layer).unwrap_or(layer);

    if !leftovers.is_empty() {
        p.start_error();
        while let Some((_, ts)) = p.bump(leftovers) {
            leftovers = ts;
        }
        p.finish();
    }

    Some(rest)
}

fn parse_rep<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    body: &'g Expr,
) -> Option<Pos> {
    let mut tokens = tokens;
    while let Some(ts) = parse_expr(p, body, tokens) {
        tokens = ts;
    }
    Some(tokens)
}

fn parse_with_skip<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    first: &'g Expr, body: &'g Expr,
) -> Option<Pos> {
    let mut skipped = false;
    let mut tokens = tokens;
    loop {
        if skipped {
            p.finish();
        }
        match parse_expr_pred(p, first, tokens) {
            Some(_) => if let Some(ts) = parse_expr(p, body, tokens) {
                return Some(ts);
            }
            None => {}
        }
        if skipped {
            p.reopen()
        } else {
            p.start_error()
        }
        skipped = true;
        match p.bump(tokens) {
            None => return None,
            Some((_, ts)) => tokens = ts,
        }
    }
}

fn parse_enter<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ctx: Context, e: &'g Expr,
) -> Option<Pos> {
    let idx = ctx.0 as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = true;
    let result = parse_expr(p, &*e, tokens);
    p.contexts[idx] = old;
    result
}

fn parse_exit<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ctx: Context, e: &'g Expr,
) -> Option<Pos> {
    let idx = ctx.0 as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = false;
    let result = parse_expr(p, &*e, tokens);
    p.contexts[idx] = old;
    result
}

fn parse_is_in<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ctx: Context,
) -> Option<Pos> {
    if p.contexts[ctx.0 as usize] { Some(tokens) } else { None }
}

fn parse_call<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    body: &'g Expr, args: &'g [(Arg, Expr)],
) -> Option<Pos> {
    let old = p.args;
    for &(arg_pos, ref arg) in args {
        let arg = match *arg {
            Expr::Var(i) => p.args[i.0 as usize].unwrap(),
            _ => arg
        };

        p.args[arg_pos.0 as usize] = Some(arg);
    }
    let result = parse_expr(p, body, tokens);
    p.args = old;
    result
}

fn parse_var<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    i: Arg,
) -> Option<Pos> {
    let expr = p.args[i.0 as usize].unwrap();
    parse_expr(p, expr, tokens)
}

fn parse_prev_is<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ts: &[NodeTypeRef],
) -> Option<Pos> {
    if let Some(prev) = p.prev {
        for &ty_idx in ts {
            let t = p[ty_idx];
            if t == prev {
                return Some(tokens);
            }
        }
    }
    None
}

fn parse_inject<'g>(
    p: &mut Parser<'g>, pos: Pos,
    prefix: &'g Expr, body: &'g Expr,
) -> Option<Pos> {
    let prefix_mark = p.mark();
    let after_prefix = parse_expr(p, prefix, pos)?;
    let body_mark = p.mark();
    let result = parse_expr(p, body, after_prefix)?;
    if after_prefix != pos {
        p.forward_parent(prefix_mark, body_mark);
    }
    Some(result)
}
