use PrattTable;
use super::{Ctx, TokenSeq, BlackNode, parse_exp, parse_any};

pub (super) fn parse_pratt2<'t, 'p>(
    ctx: &mut Ctx<'p>,
    table: &'p PrattTable,
    tokens: TokenSeq<'t>,
) -> Option<(BlackNode, TokenSeq<'t>)> {
    go(ctx, table, tokens, 0)
}

fn go<'t, 'p>(
    ctx: &mut Ctx<'p>,
    table: &'p PrattTable,
    tokens: TokenSeq<'t>,
    min_prior: u32
) -> Option<(BlackNode, TokenSeq<'t>)> {
    let (mut lhs, mut tokens) = match prefix(ctx, table, tokens) {
        Some(x) => x,
        _ => return None,
    };

    'l: loop {
        for ix in table.infixes(min_prior) {
            if let Some((op_node, rest)) = parse_exp(ctx, &ix.op, tokens) {
                tokens = rest;
                let ty = ctx.node_type(ix.ty);
                let mut node = ctx.create_composite_node(Some(ty));
                ctx.push_child(&mut node, lhs);
                ctx.push_child(&mut node, op_node);
                if ix.has_rhs {
                    if let Some((rhs, rest)) = go(ctx, table, tokens, ix.priority + 1) {
                        tokens = rest;
                        ctx.push_child(&mut node, rhs);
                    }
                }
                ctx.prev = Some(ty);
                lhs = node;
                continue 'l;
            }
        }
        break
    }

    Some((lhs, tokens))
}

fn prefix<'t, 'p>(
    ctx: &mut Ctx<'p>,
    table: &'p PrattTable,
    tokens: TokenSeq<'t>,
) -> Option<(BlackNode, TokenSeq<'t>)> {
    if let Some(result) = parse_any(ctx, table.atoms.iter(), tokens) {
        return Some(result);
    }
    for p in table.prefixes.iter() {
        if let Some((op_node, tokens)) = parse_exp(ctx, &p.op, tokens) {
            let ty = ctx.node_type(p.ty);
            let mut node = ctx.create_composite_node(Some(ty));
            ctx.push_child(&mut node, op_node);
            if let Some((expr, rest)) = go(ctx, table, tokens, p.priority) {
                ctx.push_child(&mut node, expr);
                ctx.prev = Some(ty);
                return Some((node, rest));
            }
        }
    }
    None
}
