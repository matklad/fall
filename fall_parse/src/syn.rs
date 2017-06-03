use fall_tree::{NodeType, FileStats};
use lex::Token;

use tree_builder::{Node, TokenSequence, NodeFactory};

pub struct Parser<'r> {
    node_types: &'r [NodeType],
    rules: &'r [SynRule],
}

#[derive(Serialize, Deserialize)]
pub struct SynRule {
    pub ty: Option<usize>,
    pub body: Expr,
}

#[derive(Serialize, Deserialize)]
pub enum Expr {
    Or(Vec<Expr>),
    And(Vec<Expr>, Option<usize>),
    Rule(usize),
    Token(usize),
    Rep(Box<Expr>),
    Opt(Box<Expr>),
    Not(Vec<usize>),
    NotAhead(Box<Expr>),
    Eof,
    Layer(Box<Expr>, Box<Expr>),
    SkipUntil(Vec<usize>),
}

struct Ctx<'p> {
    ticks: u64,
    node_factory: &'p mut NodeFactory,
}

impl<'p> Ctx<'p> {
    fn create_composite_node(&mut self, ty: Option<NodeType>) -> Node {
        self.node_factory.create_composite_node(ty)
    }

    fn create_error_node(&mut self) -> Node {
        self.node_factory.create_error_node()
    }

    fn create_leaf_node<'t>(&mut self, tokens: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        self.node_factory.create_leaf_node(tokens)
    }

    fn create_success_node<'t>(&mut self, tokens: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        self.node_factory.create_success_node(tokens)
    }
}

impl<'r> Parser<'r> {
    pub fn new(node_types: &'r [NodeType], rules: &'r [SynRule]) -> Parser<'r> {
        Parser { node_types, rules: rules }
    }

    pub fn parse(&self, tokens: TokenSequence, nf: &mut NodeFactory, stats: &mut FileStats) -> Node {
        let mut ctx = Ctx { ticks: 0, node_factory: nf };
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
            error.push_child(p.0)
        }
        if skipped {
            file_node.push_child(error)
        }
        stats.parsing_ticks = ctx.ticks;
        file_node
    }

    fn parse_exp<'t>(&self, expr: &Expr, tokens: TokenSequence<'t>, ctx: &mut Ctx)
                     -> Option<(Node, TokenSequence<'t>)> {
        ctx.ticks += 1;
        match *expr {
            Expr::Or(ref parts) => {
                for p in parts.iter() {
                    if let Some(result) = self.parse_exp(p, tokens, ctx) {
                        return Some(result)
                    }
                }
                None
            }

            Expr::And(ref parts, commit) => {
                let mut node = ctx.create_composite_node(None);
                let commit = commit.unwrap_or(parts.len());
                let mut tokens = tokens;
                for (i, p) in parts.iter().enumerate() {
                    if let Some((n, ts)) = self.parse_exp(p, tokens, ctx) {
                        tokens = ts;
                        node.push_child(n);
                    } else {
                        if i < commit {
                            return None
                        }
                        let error_node = ctx.create_error_node();
                        node.push_child(error_node);
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
                    result.push_child(node);
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
                if let Some((_, rest)) = self.parse_exp(l, tokens, ctx) {
                    let mut result = ctx.create_composite_node(None);
                    let layer = tokens.prefix(rest);
                    if let Some((layer_contents, mut leftovers)) = self.parse_exp(e, layer, ctx) {
                        result.push_child(layer_contents);
                        if leftovers.current().is_some() {
                            let mut error = ctx.create_error_node();
                            while leftovers.current().is_some() {
                                let p = ctx.create_leaf_node(leftovers);
                                error.push_child(p.0);
                                leftovers = p.1;
                            }
                            result.push_child(error)
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
                        node.push_child(n);
                        tokens = t;
                    } else {
                        break;
                    }
                }
                Some((node, tokens))
            }

            Expr::SkipUntil(ref ts) => {
                let mut result = ctx.create_error_node();
                let mut skipped = false;
                let mut tokens = tokens;
                loop {
                    if let Some(t) = tokens.current() {
                        if self.token_set_contains(ts, t) {
                            break;
                        } else {
                            skipped = true;
                            let p = ctx.create_leaf_node(tokens);
                            result.push_child(p.0);
                            tokens = p.1;
                        }
                    } else {
                        break
                    }
                }

                if !skipped {
                    return Some(ctx.create_success_node(tokens));
                }

                Some((result, tokens))
            }
        }
    }

    fn token_set_contains(&self, ts: &[usize], token: Token) -> bool {
        ts.iter().any(|&t| self.node_type(t) == token.ty)
    }

    fn node_type(&self, idx: usize) -> NodeType {
        self.node_types[idx]
    }
}

