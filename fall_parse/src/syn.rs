use fall_tree::{NodeType, FileStats};
use lex::Token;

use tree_builder::{Node, TokenSequence};

pub struct Parser<'r> {
    node_types: &'r [NodeType],
    rules: &'r [SynRule],
}

#[derive(Serialize, Deserialize)]
pub struct SynRule {
    pub ty: Option<usize>,
    pub body: Expr,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Or(Vec<Expr>),
    And(Vec<Expr>, Option<usize>),
    Rule(usize),
    Token(usize),
    Rep(Box<Expr>),
    WithSkip(Box<Expr>, Box<Expr>),
    Opt(Box<Expr>),
    Not(Vec<usize>),
    NotAhead(Box<Expr>),
    Eof,
    Layer(Box<Expr>, Box<Expr>),
    Pratt(Vec<PrattVariant>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PrattVariant {
    Atom {
        ty: usize,
        body: Box<Expr>
    },
    Binary {
        ty: usize,
        op: Box<Expr>,
        priority: u32,
    }
}

struct Ctx {
    ticks: u64,
    predicate_mode: bool,
}

impl Ctx {
    fn create_composite_node(&mut self, ty: Option<NodeType>) -> Node {
        Node::composite(ty)
    }

    fn create_error_node(&mut self) -> Node {
        Node::error()
    }

    fn create_leaf_node<'t>(&mut self, tokens: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        tokens.bump()
    }

    fn create_success_node<'t>(&mut self, tokens: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        Node::success(tokens)
    }

    fn push_child(&self, parent: &mut Node, child: Node) {
        if self.predicate_mode {
            return;
        }
        match child {
            // Microoptimization: don't store empty success nodes
            Node::Composite { ty: None, ref children, .. } if children.is_empty() => return,
            _ => {}
        }
        parent.push_child(child)
    }
}

impl<'r> Parser<'r> {
    pub fn new(node_types: &'r [NodeType], rules: &'r [SynRule]) -> Parser<'r> {
        Parser { node_types, rules: rules }
    }

    pub fn parse(&self, tokens: TokenSequence, stats: &mut FileStats) -> Node {
        let mut ctx = Ctx { ticks: 0, predicate_mode: false };
        let (mut file_node, mut leftover) = self
            .parse_exp(&Expr::Rule(0), tokens, &mut ctx)
            .unwrap_or_else(|| {
                let ty = self.node_type(self.rules[0].ty
                    .expect("First rule must be public"));
                (ctx.create_composite_node(Some(ty)), tokens)
            });
        let mut error = ctx.create_error_node();
        let mut skipped = false;
        while leftover.current().is_some() {
            skipped = true;
            let p = ctx.create_leaf_node(leftover);
            leftover = p.1;
            ctx.push_child(&mut error, p.0)
        }
        if skipped {
            ctx.push_child(&mut file_node, error)
        }
        stats.parsing_ticks = ctx.ticks;
        file_node
    }

    fn parse_exp<'t>(&self, expr: &Expr, tokens: TokenSequence<'t>, ctx: &mut Ctx)
                     -> Option<(Node, TokenSequence<'t>)> {
        ctx.ticks += 1;
        match *expr {
            Expr::Or(ref parts) => self.parse_any(parts.iter(), tokens, ctx),

            Expr::And(ref parts, commit) => {
                let mut node = ctx.create_composite_node(None);
                let commit = commit.unwrap_or(parts.len());
                let mut tokens = tokens;
                for (i, p) in parts.iter().enumerate() {
                    if let Some((n, ts)) = self.parse_exp(p, tokens, ctx) {
                        tokens = ts;
                        ctx.push_child(&mut node, n);
                    } else {
                        if i < commit {
                            return None
                        }
                        let error_node = ctx.create_error_node();
                        ctx.push_child(&mut node, error_node);
                        break
                    }
                }
                Some((node, tokens))
            }

            Expr::Rule(id) => {
                let rule = &self.rules[id];
                let ty = rule.ty.map(|ty| self.node_type(ty));
                if let Some((node, ts)) = self.parse_exp(&rule.body, tokens, ctx) {
                    let mut result = ctx.create_composite_node(ty);
                    ctx.push_child(&mut result, node);
                    Some((result, ts))
                } else {
                    None
                }
            }

            Expr::Token(ty) => {
                if let Some(current) = tokens.current() {
                    if self.token_set_contains(&[ty], current) {
                        return Some(ctx.create_leaf_node(tokens))
                    }
                }
                None
            }

            Expr::Opt(ref body) => self.parse_exp(&*body, tokens, ctx).or_else(|| {
                Some(ctx.create_success_node(tokens))
            }),
            Expr::Not(ref ts) => {
                if let Some(current) = tokens.current() {
                    if !self.token_set_contains(ts, current) {
                        return Some(ctx.create_leaf_node(tokens))
                    }
                }
                None
            }

            Expr::NotAhead(ref e) => if self.parse_exp(e, tokens, ctx).is_some() {
                None
            } else {
                Some(ctx.create_success_node(tokens))
            },
            Expr::Eof => if tokens.current().is_none() {
                Some(ctx.create_success_node(tokens))
            } else {
                None
            },
            Expr::Layer(ref l, ref e) => {
                if let Some(rest) = self.parse_exp_pred(l, tokens, ctx) {
                    let mut result = ctx.create_composite_node(None);
                    let layer = tokens.prefix(rest);
                    if let Some((layer_contents, mut leftovers)) = self.parse_exp(e, layer, ctx) {
                        ctx.push_child(&mut result, layer_contents);
                        if leftovers.current().is_some() {
                            let mut error = ctx.create_error_node();
                            while leftovers.current().is_some() {
                                let p = ctx.create_leaf_node(leftovers);
                                ctx.push_child(&mut error, p.0);
                                leftovers = p.1;
                            }
                            ctx.push_child(&mut result, error)
                        }
                    };
                    return Some((result, rest));
                };
                None
            }

            Expr::Rep(ref body) => {
                let mut node = ctx.create_composite_node(None);
                let mut tokens = tokens;
                loop {
                    if let Some((n, t)) = self.parse_exp(body, tokens, ctx) {
                        ctx.push_child(&mut node, n);
                        tokens = t;
                    } else {
                        break;
                    }
                }
                Some((node, tokens))
            }

            Expr::WithSkip(ref first, ref body) => {
                let mut error = ctx.create_error_node();
                let mut result = ctx.create_composite_node(None);
                let mut skipped = false;
                let mut tokens = tokens;
                loop {
                    if tokens.current().is_none() {
                        return None;
                    }
                    if let Some(_) = self.parse_exp_pred(first, tokens, ctx) {
                        if let Some((node, ts)) = self.parse_exp(body, tokens, ctx) {
                            if skipped {
                                ctx.push_child(&mut result, error);
                            }
                            ctx.push_child(&mut result, node);
                            return Some((result, ts));
                        }
                    }

                    skipped = true;
                    let (node, new_tokens) = ctx.create_leaf_node(tokens);
                    tokens = new_tokens;
                    ctx.push_child(&mut error, node);
                }
            }

            Expr::Pratt(ref g) => self.parse_pratt(&*g, tokens, ctx, 0),
        }
    }

    fn parse_any<'t, 'e, I: Iterator<Item=&'e Expr>>(&self, options: I, tokens: TokenSequence<'t>, ctx: &mut Ctx)
                                                     -> Option<(Node, TokenSequence<'t>)> {
        for p in options {
            if let Some(result) = self.parse_exp(p, tokens, ctx) {
                return Some(result)
            }
        }
        None
    }

    fn parse_pratt<'t>(&self, expr_grammar: &[PrattVariant], tokens: TokenSequence<'t>, ctx: &mut Ctx, min_prior: u32)
                       -> Option<(Node, TokenSequence<'t>)> {
        let atoms = expr_grammar.iter().filter_map(|v| {
            match *v {
                PrattVariant::Atom { ty, ref body } => Some((self.node_type(ty), body.as_ref())),
                _ => None
            }
        });

        let (mut lhs, mut tokens) = match atoms.filter_map(|(ty, body)| {
            if let Some((n, rest)) = self.parse_exp(body, tokens, ctx) {
                let mut lhs_node = ctx.create_composite_node(Some(ty));
                ctx.push_child(&mut lhs_node, n);
                return Some((lhs_node, rest));
            }
            None
        }).next() {
            Some(p) => p,
            None => return None
        };

        'outer: loop {
            let bins = expr_grammar.iter().filter_map(|v| {
                match *v {
                    PrattVariant::Binary { ty, ref op, priority } if priority > min_prior => {
                        Some((self.node_type(ty), op.as_ref(), priority))
                    }
                    _ => None
                }
            });

            for (ty, op, p) in bins {
                if let Some((op_node, rest)) = self.parse_exp(op, tokens, ctx) {
                    if let Some((rhs_node, rest)) = self.parse_pratt(expr_grammar, rest, ctx, p) {
                        let mut node = ctx.create_composite_node(Some(ty));
                        ::std::mem::swap(&mut node, &mut lhs);
                        ctx.push_child(&mut lhs, node);
                        ctx.push_child(&mut lhs, op_node);
                        ctx.push_child(&mut lhs, rhs_node);
                        tokens = rest;
                        continue 'outer;
                    }
                }
            }
            break
        }
        Some((lhs, tokens))
    }

    fn parse_exp_pred<'t>(&self, expr: &Expr, tokens: TokenSequence<'t>, ctx: &mut Ctx)
                          -> Option<(TokenSequence<'t>)> {
        let old_mode = ctx.predicate_mode;
        ctx.predicate_mode = true;
        let result = self.parse_exp(expr, tokens, ctx);
        ctx.predicate_mode = old_mode;
        result.map(|(_, ts)| ts)
    }

    fn token_set_contains(&self, ts: &[usize], token: Token) -> bool {
        ts.iter().any(|&t| self.node_type(t) == token.ty)
    }

    fn node_type(&self, idx: usize) -> NodeType {
        self.node_types[idx]
    }
}
