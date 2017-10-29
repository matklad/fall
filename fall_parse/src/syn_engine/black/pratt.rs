use PrattVariant;
use super::{Ctx, TokenSeq, BlackNode, parse_exp, parse_any};

pub (super) fn parse_pratt<'t, 'p>(
    ctx: &mut Ctx<'p>,
    expr_grammar: &'p [PrattVariant],
    tokens: TokenSeq<'t>,
) -> Option<(BlackNode, TokenSeq<'t>)> {
    with_prior(ctx, expr_grammar, tokens, 0)
}

fn with_prior<'t, 'p>(
    ctx: &mut Ctx<'p>,
    expr_grammar: &'p [PrattVariant],
    tokens: TokenSeq<'t>,
    min_prior: u32
) -> Option<(BlackNode, TokenSeq<'t>)> {
    let (mut lhs, mut tokens) = match parse_pratt_prefix(ctx, expr_grammar, tokens) {
        Some(p) => p,
        None => return None,
    };

    'postfix: loop {
        let postfix = expr_grammar.iter().filter_map(|v| {
            match *v {
                PrattVariant::Postfix { ty, ref op } => {
                    Some((ty, op.as_ref()))
                }
                _ => None
            }
        });
        for (ty, op) in postfix {
            let ty = ctx.node_type(ty);
            if let Some((op_node, rest)) = parse_exp(ctx, op, tokens) {
                let mut node = ctx.create_composite_node(Some(ty));
                ::std::mem::swap(&mut node, &mut lhs);
                ctx.push_child(&mut lhs, node);
                ctx.push_child(&mut lhs, op_node);
                tokens = rest;
                ctx.prev = Some(ty);
                continue 'postfix;
            }
        }
        break;
    }

    'bin: loop {
        let bins = expr_grammar.iter().filter_map(|v| {
            match *v {
                PrattVariant::Binary { ty, ref op, priority } if priority > min_prior => {
                    Some((ty, op.as_ref(), priority))
                }
                _ => None
            }
        });

        for (ty, op, p) in bins {
            let ty = ctx.node_type(ty);
            if let Some((op_node, rest)) = parse_exp(ctx, op, tokens) {
                if let Some((rhs_node, rest)) = with_prior(ctx, expr_grammar, rest, p) {
                    let mut node = ctx.create_composite_node(Some(ty));
                    ::std::mem::swap(&mut node, &mut lhs);
                    ctx.push_child(&mut lhs, node);
                    ctx.push_child(&mut lhs, op_node);
                    ctx.push_child(&mut lhs, rhs_node);
                    tokens = rest;
                    ctx.prev = Some(ty);
                    continue 'bin;
                }
            }
        }
        break;
    }
    Some((lhs, tokens))
}

fn parse_pratt_prefix<'t, 'p>(ctx: &mut Ctx<'p>, expr_grammar: &'p [PrattVariant], tokens: TokenSeq<'t>)
                              -> Option<(BlackNode, TokenSeq<'t>)> {
    let prefix = expr_grammar.iter().filter_map(|v| {
        match *v {
            PrattVariant::Prefix { ty, ref op, priority } => Some((ty, op.as_ref(), priority)),
            _ => None
        }
    });

    for (ty, op, priority) in prefix {
        let ty = ctx.node_type(ty);
        if let Some((op_node, rest)) = parse_exp(ctx, op, tokens) {
            let mut node = ctx.create_composite_node(Some(ty));
            ctx.push_child(&mut node, op_node);
            if let Some((expr, rest)) = with_prior(ctx, expr_grammar, rest, priority) {
                ctx.push_child(&mut node, expr);
                ctx.prev = Some(ty);
                return Some((node, rest));
            }
        }
    }

    let atoms = expr_grammar.iter().filter_map(|v| {
        match *v {
            PrattVariant::Atom { ref body } => Some(body.as_ref()),
            _ => None
        }
    });
    parse_any(ctx, atoms, tokens)
}
