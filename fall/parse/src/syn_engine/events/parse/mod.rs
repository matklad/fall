mod pratt;

use fall_tree::NodeType;
use syn_engine::TokenSeq;
use ::Expr;

use super::{Event, Grammar};

pub fn parse(grammar: Grammar, ts: TokenSeq) -> Vec<Event> {
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

    parse_expr(&mut parser, start_rule, ts).unwrap();
    parser.events
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
        self.events.truncate(mark.0)
    }

    fn start(&mut self, ty_idx: usize) -> Mark {
        let ty = self.node_type(ty_idx);
        self.start_ty(ty)
    }

    fn start_ty(&mut self, ty: NodeType) -> Mark {
        let mark = Mark(self.events.len());
        self.event(Event::Start { ty });
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
        Expr::Pub { ty_idx, ref body, replaceable } => {
            if replaceable {
                p.replacement = None;
            }
            let mark = p.start(ty_idx);
            let ts = parse_expr(p, body, tokens)?;
            if let (true, Some(ty)) = (replaceable, p.replacement) {
                p.replace(mark, ty)
            };
            p.finish();
            p.prev = Some(p.node_type(ty_idx));
            Some(ts)
        }

        Expr::PubReplace { ty_idx, ref body } => {
            let ts = parse_expr(p, body, tokens)?;
            p.replacement = Some(ty_idx);
            Some(ts)
        }

        Expr::Or(ref parts) => parse_any(p, parts.iter(), tokens),

        Expr::And(ref parts, commit) => {
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

        Expr::Rule(id) => parse_expr(p, &p.grammar.rules[id].body, tokens),

        Expr::Token(ty_idx) => {
            let (ty, ts) = p.try_bump(tokens)?;
            if p.node_type(ty_idx) != ty {
                return None;
            }
            Some(ts)
        }

        Expr::ContextualToken(ty_idx, ref text) => {
            let (n_raw_tokens, ts) = tokens.bump_by_text(text)?;
            let ty = p.node_type(ty_idx);
            p.token(ty, n_raw_tokens as u16);
            Some(ts)
        }

        Expr::Opt(ref body) => Some(parse_expr(p, body, tokens).unwrap_or(tokens)),

        Expr::Not(ref e) => match parse_expr_pred(p, e, tokens) {
            None => Some(tokens),
            Some(_) => None,
        },

        Expr::Eof => match tokens.try_bump()

            {
                None => Some(tokens),
                Some(_) => None,
            },

        Expr::Any => p.try_bump(tokens).map(|(_ty, ts)| ts),

        Expr::Layer(ref l, ref e) => {
            let rest = parse_expr_pred(p, l, tokens)?;
            let layer = tokens.cut_suffix(rest);
            let mut leftovers = parse_expr(p, e, layer).unwrap_or(layer);

            if leftovers.try_bump().is_some() {
                p.start_error();
                while let Some((_, ts)) = p.try_bump(leftovers) {
                    leftovers = ts;
                }
            }

            Some(rest)
        }

        Expr::Rep(ref body) => {
            let mut tokens = tokens;
            while let Some(ts) = parse_expr(p, body, tokens) {
                tokens = ts;
            }
            Some(tokens)
        }

        Expr::WithSkip(ref first, ref body) => {
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
                    Some((ty, ts)) => {
                        p.token(ty, 1);
                        tokens = ts
                    }
                }
            }
        }

        Expr::Pratt(ref table) => pratt::parse_pratt(p, table, tokens),

        Expr::Enter(idx, ref e) => {
            let idx = idx as usize;
            let old = p.contexts[idx];
            p.contexts[idx] = true;
            let result = parse_expr(p, &*e, tokens);
            p.contexts[idx] = old;
            result
        }

        Expr::Exit(idx, ref e) => {
            let idx = idx as usize;
            let old = p.contexts[idx];
            p.contexts[idx] = false;
            let result = parse_expr(p, &*e, tokens);
            p.contexts[idx] = old;
            result
        }

        Expr::IsIn(idx) => if p.contexts[idx as usize] { Some(tokens) } else { None },

        Expr::Call(ref body, ref args) => {
            let old = p.args;
            for &(arg_pos, ref arg) in args.iter() {
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

        Expr::Var(i) => {
            let expr = p.args[i as usize].unwrap();
            parse_expr(p, expr, tokens)
        }

        Expr::PrevIs(ref ts) => {
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
    }
}


fn parse_any<'t, 'p, I: Iterator<Item=&'p Expr>>(
    p: &mut Parser<'p>,
    options: I,
    tokens: TokenSeq<'t>
) -> Option<(TokenSeq<'t>)> {
    options.filter_map(|opt| parse_expr(p, opt, tokens)).next()
}
