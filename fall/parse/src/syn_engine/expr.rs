use fall_tree::{Language, Text, IToken, Event};
use syn_engine::parser::{Parser, Pos};

use ::{NodeTypeRef, Context, Arg, ExprRef, Expr};

use super::Grammar;
use super::pratt::parse_pratt;

pub fn parse(
    grammar: Grammar,
    lang: &Language,
    text: Text,
    tokens: &[IToken],
) -> (Vec<Event>, u64) {
    let is_ws = |t: IToken| lang.node_type_info(t.ty).whitespace_like;
    let (mut parser, pos) = Parser::new(&grammar, &is_ws, text, tokens);

    let start_rule = parser.grammar.start_rule;
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

    parser.done()
}


pub(crate) fn parse_expr(p: &mut Parser, expr: ExprRef, tokens: Pos) -> Option<Pos> {
    p.tick();
    let mark = p.mark();
    let result = parse_expr_inner(p, expr, tokens);
    if result.is_none() {
        p.rollback(mark);
    }
    result
}

fn parse_expr_pred(p: &mut Parser, expr: ExprRef, tokens: Pos) -> Option<Pos> {
    let old_mode = p.predicate_mode;
    p.predicate_mode = true;
    let result = parse_expr(p, expr, tokens);
    p.predicate_mode = old_mode;
    result
}

fn parse_expr_inner(p: &mut Parser, expr: ExprRef, tokens: Pos) -> Option<Pos> {
    let grammar = &*p.grammar;
    match grammar[expr] {
        Expr::Pub { ty, body, replaceable } =>
            parse_pub(p, tokens, ty, body, replaceable),

        Expr::PubReplace { ty, body } =>
            parse_pub_replace(p, tokens, ty, body),

        Expr::Or(ref parts) =>
            parse_or(p, parts, tokens),

        Expr::And(ref parts, commit) =>
            parse_and(p, tokens, &*parts, commit),

        Expr::Token(ty) =>
            parse_token(p, tokens, ty),

        Expr::ContextualToken(ty, ref text) =>
            parse_contextual_token(p, tokens, ty, text),

        Expr::Opt(body) =>
            parse_opt(p, tokens, body),

        Expr::Not(e) =>
            parse_not(p, tokens, e),

        Expr::Eof =>
            parse_eof(p, tokens),

        Expr::Any =>
            p.bump(tokens).map(|(_ty, ts)| ts),

        Expr::Layer(l, e) =>
            parse_layer(p, tokens, l, e),

        Expr::Rep(body) =>
            parse_rep(p, tokens, body),

        Expr::WithSkip(first, body) =>
            parse_with_skip(p, tokens, first, body),

        Expr::Pratt(ref table) =>
            parse_pratt(p, table, tokens),

        Expr::Enter(ctx, e) =>
            parse_enter(p, tokens, ctx, e),

        Expr::Exit(ctx, e) =>
            parse_exit(p, tokens, ctx, e),

        Expr::IsIn(ctx) =>
            parse_is_in(p, tokens, ctx),

        Expr::Call(body, ref args) =>
            parse_call(p, tokens, body, &*args),

        Expr::Var(i) =>
            parse_var(p, tokens, i),

        Expr::PrevIs(ref ts) =>
            parse_prev_is(p, tokens, ts),

        Expr::Inject(prefix, body) =>
            parse_inject(p, tokens, prefix, body),

        Expr::Cached(body) =>
            parse_cached(p, body, tokens),
    }
}


fn parse_pub<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ty_idx: NodeTypeRef, body: ExprRef, replaceable: bool,
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
    ty_idx: NodeTypeRef, body: ExprRef
) -> Option<Pos> {
    let ts = parse_expr(p, body, tokens)?;
    p.replacement = Some(ty_idx);
    Some(ts)
}

pub(crate) fn parse_or<'t, 'g>(
    p: &mut Parser<'g>,
    options: &'g [ExprRef],
    tokens: Pos
) -> Option<(Pos)> {
    options.iter().filter_map(|&opt| parse_expr(p, opt, tokens)).next()
}

fn parse_and<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    parts: &'g [ExprRef], commit: Option<usize>,
) -> Option<Pos> {
    let mut tokens = tokens;
    let mut consumed = 0;

    for &part in parts {
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
    body: ExprRef,
) -> Option<Pos> {
    Some(parse_expr(p, body, tokens).unwrap_or(tokens))
}

fn parse_not<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    e: ExprRef,
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
    l: ExprRef, e: ExprRef,
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
    body: ExprRef,
) -> Option<Pos> {
    let mut tokens = tokens;
    while let Some(ts) = parse_expr(p, body, tokens) {
        tokens = ts;
    }
    Some(tokens)
}

fn parse_with_skip<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    first: ExprRef, body: ExprRef,
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
    ctx: Context, e: ExprRef,
) -> Option<Pos> {
    let idx = ctx.0 as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = true;
    let result = parse_expr(p, e, tokens);
    p.contexts[idx] = old;
    result
}

fn parse_exit<'g>(
    p: &mut Parser<'g>, tokens: Pos,
    ctx: Context, e: ExprRef,
) -> Option<Pos> {
    let idx = ctx.0 as usize;
    let old = p.contexts[idx];
    p.contexts[idx] = false;
    let result = parse_expr(p, e, tokens);
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
    body: ExprRef, args: &'g [(Arg, ExprRef)],
) -> Option<Pos> {
    let old = p.args;
    for &(arg_pos, arg) in args {
        let arg = match p.grammar[arg] {
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
    prefix: ExprRef, body: ExprRef,
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

fn parse_cached<'g>(p: &mut Parser<'g>, expr: ExprRef, pos: Pos) -> Option<Pos> {
    let mark = p.start_cached(expr);
    let result = parse_expr(p, expr, pos)?;
    p.finish_cached(mark);

    Some(result)
}
