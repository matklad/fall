use {PrattVariant, Expr};
use super::{Ctx, TokenSeq, BlackNode, parse_exp, parse_any};

pub (super) fn parse_pratt<'t, 'p>(
    ctx: &mut Ctx<'p>,
    expr_grammar: &'p [PrattVariant],
    tokens: TokenSeq<'t>,
) -> Option<(BlackNode, TokenSeq<'t>)> {
    let table = PrattTable::new(expr_grammar);
    go(ctx, &table, tokens, 0)
}

struct PrattTable<'p> {
    atoms: Vec<&'p Expr>,
    prefixes: Vec<Prefix<'p>>,
    infixes: Vec<Infix<'p>>
}

#[derive(Copy, Clone)]
struct Prefix<'p> {
    ty: usize,
    op: &'p Expr,
    priority: u32,
}

#[derive(Copy, Clone)]
struct Infix<'p> {
    ty: usize,
    op: &'p Expr,
    priority: u32,
    has_rhs: bool,
}


impl<'p> PrattTable<'p> {
    fn new(grammar: &'p [PrattVariant]) -> PrattTable {
        let mut result = PrattTable {
            atoms: Vec::new(),
            prefixes: Vec::new(),
            infixes: Vec::new(),
        };

        for v in grammar {
            match *v {
                PrattVariant::Atom { ref body } =>
                    result.atoms.push(&*body),
                PrattVariant::Prefix { ty, ref op, priority } =>
                    result.prefixes.push(Prefix { ty, op: &*op, priority }),
                PrattVariant::Binary { ty, ref op, priority } =>
                    result.infixes.push(Infix { ty, op: &*op, priority, has_rhs: true }),
                PrattVariant::Postfix { ty, ref op } =>
                    result.infixes.push(Infix { ty, op: &*op, priority: 999, has_rhs: false }),
            }
        }
        result
    }

    fn infixes<'a>(&'a self, min_prior: u32) -> Box<Iterator<Item=Infix<'p>> + 'a> {
        Box::new(
            self.infixes.iter().map(|&ix| ix)
                .filter(move |ix| ix.priority >= min_prior)
        )
    }
}

fn go<'t, 'p>(
    ctx: &mut Ctx<'p>,
    table: &PrattTable<'p>,
    tokens: TokenSeq<'t>,
    min_prior: u32
) -> Option<(BlackNode, TokenSeq<'t>)> {
    let (mut lhs, mut tokens) = match prefix(ctx, table, tokens) {
        Some(x) => x,
        _ => return None,
    };

    'l: loop {
        for ix in table.infixes(min_prior) {
            if let Some((op_node, rest)) = parse_exp(ctx, ix.op, tokens) {
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
    table: &PrattTable<'p>,
    tokens: TokenSeq<'t>,
) -> Option<(BlackNode, TokenSeq<'t>)> {
    if let Some(result) = parse_any(ctx, table.atoms.iter().map(|&e| e), tokens) {
        return Some(result);
    }
    for &Prefix { ty, priority, op } in table.prefixes.iter() {
        if let Some((op_node, tokens)) = parse_exp(ctx, op, tokens) {
            let ty = ctx.node_type(ty);
            let mut node = ctx.create_composite_node(Some(ty));
            ctx.push_child(&mut node, op_node);
            if let Some((expr, rest)) = go(ctx, table, tokens, priority) {
                ctx.push_child(&mut node, expr);
                ctx.prev = Some(ty);
                return Some((node, rest));
            }
        }
    }
    None
}
