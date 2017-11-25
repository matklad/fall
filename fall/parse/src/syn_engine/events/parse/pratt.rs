use PrattTable;
use super::{Parser, TokenSeq, parse_any, parse_expr};

pub(super) fn parse_pratt<'g, 't>(
    p: &mut Parser<'g>,
    table: &'g PrattTable,
    tokens: TokenSeq<'t>,
) -> Option<TokenSeq<'t>> {
    go(p, table, tokens, 0)
}

fn go<'g, 't>(
    p: &mut Parser<'g>,
    table: &'g PrattTable,
    tokens: TokenSeq<'t>,
    min_prior: u32
) -> Option<TokenSeq<'t>> {
//    {
//        let mut tokens = match prefix(ctx, table, tokens) {
//            Some(ts) => ts,
//            _ => return None,
//        };
//
//        'l: loop {
//            for ix in table.infixes(min_prior) {
//                if let Some((op_node, rest)) = parse_exp(ctx, &ix.op, tokens) {
//                    tokens = rest;
//                    let ty = ctx.node_type(ix.ty);
//                    let mut node = ctx.create_composite_node(Some(ty));
//                    ctx.push_child(&mut node, lhs);
//                    ctx.push_child(&mut node, op_node);
//                    if ix.has_rhs {
//                        if let Some((rhs, rest)) = go(ctx, table, tokens, ix.priority + 1) {
//                            tokens = rest;
//                            ctx.push_child(&mut node, rhs);
//                        } else {
//                            let error = ctx.create_error_node();
//                            ctx.push_child(&mut node, error);
//                        }
//                    }
//                    ctx.prev = Some(ty);
//                    lhs = node;
//                    continue 'l;
//                }
//            }
//            break
//        }
//
//        Some((lhs, tokens))
//    }
    unimplemented!()

}
fn prefix<'t, 'p>(
    p: &mut Parser<'p>,
    table: &'p PrattTable,
    tokens: TokenSeq<'t>,
) -> Option<TokenSeq<'t>> {
    if let Some(result) = parse_any(p, table.atoms.iter(), tokens) {
        return Some(result);
    }
    for prefix in table.prefixes.iter() {
        let mark = p.start(prefix.ty);
        if let Some(tokens) = parse_expr(p, &prefix.op, tokens) {
            if let Some(rest) = go(p, table, tokens, prefix.priority) {
                p.prev = Some(p.node_type(prefix.ty));
                return Some(rest);
            }
        }
        p.rollback(mark);
    }
    None
}
