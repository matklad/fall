use fall_tree::{NodeType, FileStats};
use lex::Token;

use tree_builder::{Node, TokenSequence};

pub struct Parser<'r> {
    node_types: &'r [NodeType],
    rules: &'r [SynRule],
    layers: Vec<&'r Expr>,
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

struct Ctx {
    ticks: u64,
    predicate_mode: bool,
}

impl Ctx {
    fn create_composite_node(&mut self, ty: Option<NodeType>) -> Node {
        Node::composite(ty)
    }

    fn create_layer(&mut self, layer: u32) -> Node {
        Node::layer(layer)
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

fn find_layers<'r>(rules: &'r [SynRule]) -> Vec<&'r Expr> {
    let mut result = Vec::new();
    for r in rules {
        go(&r.body, &mut result);
    }
    return result;
    fn go<'r>(expr: &'r Expr, acc: &mut Vec<&'r Expr>) {
        match *expr {
            Expr::Rule(_) | Expr::Token(_) | Expr::Not(_) |
            Expr::NotAhead(_) | Expr::Eof | Expr::SkipUntil(_) => return,
            Expr::Or(ref exprs) | Expr::And(ref exprs, _) => {
                for e in exprs {
                    go(e, acc)
                }
            }
            Expr::Rep(ref e) | Expr::Opt(ref e) => go(e, acc),
            Expr::Layer(..) => acc.push(expr)
        }
    }
}

impl<'r> Parser<'r> {
    pub fn new(node_types: &'r [NodeType], rules: &'r [SynRule]) -> Parser<'r> {
        let layers = find_layers(rules);
        Parser { node_types, rules: rules, layers: layers }
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

    pub fn reparse(&self, parser_id: u32, tokens: TokenSequence, stats: &mut FileStats) -> Option<Node> {
        let expr = self.layers[parser_id as usize];
        let mut ctx = Ctx { ticks: 0, predicate_mode: false };
        if let Some((node, rest)) = self.parse_exp(expr, tokens, &mut ctx) {
            if rest.current().is_some() {
                return None
            }
            stats.parsing_ticks = ctx.ticks;
            return Some(node)
        }
        None
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
                let layer = {
                    let old_mode = ctx.predicate_mode;
                    ctx.predicate_mode = true;
                    let layer = self.parse_exp(l, tokens, ctx);
                    ctx.predicate_mode = old_mode;
                    layer
                };

                if let Some((_, rest)) = layer {
                    let layer_id = self.layers.iter()
                        .position(|&l| l as *const Expr == expr as *const Expr)
                        .unwrap();

                    let mut result = ctx.create_layer(layer_id as u32);
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
                            ctx.push_child(&mut result, p.0);
                            tokens = p.1;
                        }
                    } else {
                        return None
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
