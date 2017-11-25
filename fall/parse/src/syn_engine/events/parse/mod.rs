mod pratt;

use fall_tree::NodeType;
use syn_engine::TokenSeq;
use ::Expr;

use super::{Event, Grammar};

pub fn parse(grammar: Grammar, ts: TokenSeq) -> (Vec<Event>, u64) {
    let start_rule = grammar.start_rule;
    let mut parser = Parser {
        grammar,
        ticks: 0,
        events: Vec::new(),
        replacement: None,
        predicate_mode: false,
        contexts: [false; 16],
        args: [None; 16],
        prev: None,
    };

    let mut leftover = parse_expr(&mut parser, start_rule, ts).unwrap();
    if leftover.try_bump().is_some() {
        parser.reopen();
        parser.start_error();
        while let Some((_, ts)) = parser.try_bump(leftover) {
            leftover = ts;
        }
        parser.finish();
        parser.finish();
    };

    (parser.events, parser.ticks)
}


struct Parser<'g> {
    grammar: Grammar<'g>,

    ticks: u64,
    events: Vec<Event>,
    replacement: Option<usize>,
    predicate_mode: bool,
    contexts: [bool; 16],
    args: [Option<&'g Expr>; 16],
    prev: Option<NodeType>,
}

struct Mark(usize);

impl<'g> Parser<'g> {
    fn mark(&self) -> Mark {
        Mark(self.events.len())
    }

    fn rollback(&mut self, mark: Mark) {
        fn truncate_fast<T>(xs: &mut Vec<T>, len: usize) {
            assert!(len <= xs.len());
            unsafe { xs.set_len(len) }
        }

        truncate_fast(&mut self.events, mark.0);
    }

    fn start(&mut self, ty_idx: usize) -> Mark {
        let ty = self.node_type(ty_idx);
        self.start_ty(ty)
    }

    fn start_ty(&mut self, ty: NodeType) -> Mark {
        let mark = Mark(self.events.len());
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

    fn try_bump<'t>(&mut self, tokens: TokenSeq<'t>) -> Option<(NodeType, TokenSeq<'t>)> {
        tokens.try_bump().map(|(ty, ts)| {
            self.token(ty, 1);
            (ty, ts)
        })
    }

    fn replace(&mut self, mark: Mark, ty_idx: usize) {
        let ty = self.node_type(ty_idx);
        match self.events[mark.0] {
            Event::Start { ty: ref mut prev, .. } => *prev = ty,
            _ => unreachable!()
        }
    }

    fn forward_parent(&mut self, child: Mark, parent: Mark) {
        match self.events[child.0] {
            Event::Start { ref mut forward_parent, .. } =>
                *forward_parent = Some((parent.0 - child.0) as u32),
            _ => unreachable!(),
        }
    }

    fn node_type(&self, ty_idx: usize) -> NodeType {
        self.grammar.node_types[ty_idx]
    }

    fn event(&mut self, event: Event) {
        if !self.predicate_mode {
            self.events.push(event)
        }
    }
}

fn parse_expr<'g, 't>(p: &mut Parser<'g>, expr: &'g Expr, tokens: TokenSeq<'t>) -> Option<TokenSeq<'t>> {
    p.ticks += 1;
    let mark = p.mark();
    let result = parse_expr_inner(p, expr, tokens);
    if result.is_none() {
        p.rollback(mark);
    }
    result
}

fn parse_expr_pred<'t, 'g>(p: &mut Parser<'g>, expr: &'g Expr, tokens: TokenSeq<'t>) -> Option<TokenSeq<'t>> {
    let old_mode = p.predicate_mode;
    p.predicate_mode = true;
    let result = parse_expr(p, expr, tokens);
    p.predicate_mode = old_mode;
    result
}

fn parse_expr_inner<'g, 't>(p: &mut Parser<'g>, expr: &'g Expr, tokens: TokenSeq<'t>) -> Option<TokenSeq<'t>> {
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
            p.try_bump(tokens).map(|(_ty, ts)| ts),

        Expr::Layer(ref l, ref e) =>
            parse_layer(p, tokens, l, e),

        Expr::Rep(ref body) =>
            parse_rep(p, tokens, body),

        Expr::WithSkip(ref first, ref body) =>
            parse_with_skip(p, tokens, first, body),

        Expr::Pratt(ref table) =>
            pratt::parse_pratt(p, table, tokens),

        Expr::Enter(idx, ref e) =>
            parse_enter(p, tokens, idx, e),

        Expr::Exit(idx, ref e) =>
            parse_exit(p, tokens, idx, e),

        Expr::IsIn(idx) =>
            parse_is_in(p, tokens, idx),

        Expr::Call(ref body, ref args) =>
            parse_call(p, tokens, body, &*args),

        Expr::Var(i) =>
            parse_var(p, tokens, i),

        Expr::PrevIs(ref ts) =>
            parse_prev_is(p, tokens, ts),
    }
}


fn parse_pub<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    ty_idx: usize, body: &'g Expr, replaceable: bool,
) -> Option<TokenSeq<'t>> {
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
    p.prev = Some(p.node_type(ty_idx));
    Some(ts)
}

fn parse_pub_replace<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    ty_idx: usize, body: &'g Expr
) -> Option<TokenSeq<'t>> {
    let ts = parse_expr(p, body, tokens)?;
    p.replacement = Some(ty_idx);
    Some(ts)
}

fn parse_or<'t, 'g>(
    p: &mut Parser<'g>,
    options: &'g [Expr],
    tokens: TokenSeq<'t>
) -> Option<(TokenSeq<'t>)> {
    options.iter().filter_map(|opt| parse_expr(p, opt, tokens)).next()
}

fn parse_and<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    parts: &'g [Expr], commit: Option<usize>,
) -> Option<TokenSeq<'t>> {
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

fn parse_token<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    ty_idx: usize,
) -> Option<TokenSeq<'t>> {
    let (ty, ts) = p.try_bump(tokens)?;
    if p.node_type(ty_idx) != ty {
        return None;
    }
    Some(ts)
}

fn parse_contextual_token<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    ty_idx: usize, text: &str,
) -> Option<TokenSeq<'t>> {
    let (n_raw_tokens, ts) = tokens.bump_by_text(text)?;
    let ty = p.node_type(ty_idx);
    p.token(ty, n_raw_tokens as u16);
    Some(ts)
}

fn parse_opt<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    body: &'g Expr,
) -> Option<TokenSeq<'t>> {
    Some(parse_expr(p, body, tokens).unwrap_or(tokens))
}

fn parse_not<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    e: &'g Expr,
) -> Option<TokenSeq<'t>> {
    match parse_expr_pred(p, e, tokens) {
        None => Some(tokens),
        Some(_) => None,
    }
}

fn parse_eof<'g, 't>(
    _: &mut Parser<'g>, tokens: TokenSeq<'t>,
) -> Option<TokenSeq<'t>> {
    match tokens.try_bump() {
        None => Some(tokens),
        Some(_) => None,
    }
}

fn parse_layer<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    l: &'g Expr, e: &'g Expr,
) -> Option<TokenSeq<'t>> {
    let rest = parse_expr_pred(p, l, tokens)?;
    let layer = tokens.cut_suffix(rest);
    let mut leftovers = parse_expr(p, e, layer).unwrap_or(layer);

    if leftovers.try_bump().is_some() {
        p.start_error();
        while let Some((_, ts)) = p.try_bump(leftovers) {
            leftovers = ts;
        }
        p.finish();
    }

    Some(rest)
}

fn parse_rep<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    body: &'g Expr,
) -> Option<TokenSeq<'t>> {
    let mut tokens = tokens;
    while let Some(ts) = parse_expr(p, body, tokens) {
        tokens = ts;
    }
    Some(tokens)
}

fn parse_with_skip<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    first: &'g Expr, body: &'g Expr,
) -> Option<TokenSeq<'t>> {
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
        match p.try_bump(tokens) {
            None => return None,
            Some((_, ts)) => tokens = ts,
        }
    }
}

fn parse_enter<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    idx: u32, e: &'g Expr,
) -> Option<TokenSeq<'t>> {
    let idx = idx as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = true;
    let result = parse_expr(p, &*e, tokens);
    p.contexts[idx] = old;
    result
}

fn parse_exit<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    idx: u32, e: &'g Expr,
) -> Option<TokenSeq<'t>> {
    let idx = idx as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = false;
    let result = parse_expr(p, &*e, tokens);
    p.contexts[idx] = old;
    result
}

fn parse_is_in<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    idx: u32,
) -> Option<TokenSeq<'t>> {
    if p.contexts[idx as usize] { Some(tokens) } else { None }
}

fn parse_call<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    body: &'g Expr, args: &'g [(u32, Expr)],
) -> Option<TokenSeq<'t>> {
    let old = p.args;
    for &(arg_pos, ref arg) in args {
        let arg = match *arg {
            Expr::Var(i) => p.args[i as usize].unwrap(),
            _ => arg
        };

        p.args[arg_pos as usize] = Some(arg);
    }
    let result = parse_expr(p, body, tokens);
    p.args = old;
    result
}

fn parse_var<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    i: u32,
) -> Option<TokenSeq<'t>> {
    let expr = p.args[i as usize].unwrap();
    parse_expr(p, expr, tokens)
}

fn parse_prev_is<'g, 't>(
    p: &mut Parser<'g>, tokens: TokenSeq<'t>,
    ts: &[usize],
) -> Option<TokenSeq<'t>> {
    if let Some(prev) = p.prev {
        for &ty_idx in ts {
            let t = p.node_type(ty_idx);
            if t == prev {
                return Some(tokens);
            }
        }
    }
    None
}
