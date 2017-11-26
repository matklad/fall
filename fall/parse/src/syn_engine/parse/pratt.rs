use PrattTable;
use super::{Parser, parse_or, parse_expr, Pos};

pub(super) fn parse_pratt<'g>(
    p: &mut Parser<'g>,
    table: &'g PrattTable,
    tokens: Pos,
) -> Option<Pos> {
    pratt_go(p, table, tokens, 0)
}

fn pratt_go<'g>(
    p: &mut Parser<'g>,
    table: &'g PrattTable,
    tokens: Pos,
    min_prior: u32
) -> Option<Pos> {
    let mut lhs = p.mark();
    let mut tokens = match pratt_prefix(p, table, tokens) {
        Some(ts) => ts,
        _ => return None,
    };

    'l: loop {
        for ix in table.infixes(min_prior) {
            let new_lhs = p.mark();
            let mark = p.start(ix.ty);
            if let Some(rest) = parse_expr(p, ix.op, tokens) {
                p.forward_parent(lhs, mark);
                tokens = rest;
                if ix.has_rhs {
                    if let Some(rest) = pratt_go(p, table, tokens, ix.priority + 1) {
                        tokens = rest;
                    } else {
                        p.start_error();
                        p.finish();
                    }
                }
                let ty = p[ix.ty];
                p.prev = Some(ty);
                lhs = new_lhs;
                p.finish();
                continue 'l;
            }
            p.rollback(mark)
        }
        break
    }

    Some(tokens)
}

fn pratt_prefix<'p>(
    p: &mut Parser<'p>,
    table: &'p PrattTable,
    tokens: Pos,
) -> Option<Pos> {
    if let Some(result) = parse_or(p, &table.atoms, tokens) {
        return Some(result);
    }
    for prefix in table.prefixes.iter() {
        let mark = p.start(prefix.ty);
        if let Some(tokens) = parse_expr(p, prefix.op, tokens) {
            if let Some(rest) = pratt_go(p, table, tokens, prefix.priority) {
                p.prev = Some(p[prefix.ty]);
                p.finish();
                return Some(rest);
            }
        }
        p.rollback(mark);
    }
    None
}
