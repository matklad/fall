use fall_tree::NodeType;

use tree_builder::{Node, TokenSequence};

use {SynRule, Expr, PrattVariant};

pub fn parse(
    node_types: &[NodeType],
    rules: &[SynRule],
    tokens: TokenSequence
) -> (Node, u64) {
    Parser { node_types, rules: rules, start_rule: Expr::Rule(0) }.parse(tokens)
}


struct Ctx<'p> {
    ticks: u64,
    predicate_mode: bool,
    contexts: [bool; 16],
    args: [Option<&'p Expr>; 16],
    prev: Option<NodeType>,
    replacement: Option<NodeType>,
}

impl<'p> Ctx<'p> {
    fn create_composite_node(&mut self, ty: Option<NodeType>) -> Node {
        Node::composite(ty)
    }

    fn create_error_node(&mut self) -> Node {
        Node::error()
    }

    fn create_leaf_node<'t>(&mut self, tokens: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        tokens.bump()
    }

    fn create_contextual_leaf_node<'t>(&mut self, tokens: TokenSequence<'t>, ty: NodeType, text: &str)
                                       -> Option<(Node, TokenSequence<'t>)> {
        tokens.bump_by_text(ty, text)
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

struct Parser<'r> {
    node_types: &'r [NodeType],
    rules: &'r [SynRule],
    start_rule: Expr,
}


impl<'r> Parser<'r> {
    fn parse(&self, tokens: TokenSequence) -> (Node, u64) {
        let mut ctx = Ctx {
            ticks: 0,
            predicate_mode: false,
            contexts: [false; 16],
            args: [None; 16],
            prev: None,
            replacement: None,
        };
        let (mut file_node, mut leftover) = self
            .parse_exp(&self.start_rule, tokens, &mut ctx)
            .unwrap_or_else(|| {
                let ty = match self.rules[0].body {
                    Expr::Pub { ty_idx, .. } => self.node_type(ty_idx),
                    _ => panic!("First rule must be public"),
                };
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
        (file_node, ctx.ticks)
    }

    fn parse_exp<'t, 'p>(&'p self, expr: &'p Expr, tokens: TokenSequence<'t>, ctx: &mut Ctx<'p>)
                         -> Option<(Node, TokenSequence<'t>)> {
        ctx.ticks += 1;
        match *expr {
            Expr::Pub { ty_idx, ref body, replaceable } => {
                if replaceable {
                    ctx.replacement = None;
                }
                match self.parse_exp(body, tokens, ctx) {
                    Some((node, ts)) => {
                        let node_type = match (replaceable, ctx.replacement) {
                            (true, Some(ty)) => ty,
                            _ => self.node_type(ty_idx),
                        };
                        let mut result = ctx.create_composite_node(Some(node_type));
                        ctx.push_child(&mut result, node);

                        ctx.prev = Some(node_type);
                        Some((result, ts))
                    }
                    _ => {
                        None
                    }
                }
            }
            Expr::PubReplace { ty_idx, ref body } => {
                match self.parse_exp(body, tokens, ctx) {
                    Some((node, ts)) => {
                        let node_type = self.node_type(ty_idx);
                        ctx.replacement = Some(node_type);
                        Some((node, ts))
                    }
                    _ => {
                        None
                    }
                }
            }

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
                            return None;
                        }
                        let error_node = ctx.create_error_node();
                        ctx.push_child(&mut node, error_node);
                        break
                    }
                }
                Some((node, tokens))
            }

            Expr::Rule(id) => self.parse_exp(&self.rules[id].body, tokens, ctx),

            Expr::Token(ty) => {
                if let Some(current) = tokens.current() {
                    if self.node_type(ty) == current.ty {
                        return Some(ctx.create_leaf_node(tokens));
                    }
                }
                None
            }

            Expr::ContextualToken(ty, ref text) => {
                ctx.create_contextual_leaf_node(tokens, self.node_type(ty), &*text)
            }

            Expr::Opt(ref body) => self.parse_exp(body, tokens, ctx).or_else(|| {
                Some(ctx.create_success_node(tokens))
            }),
            Expr::Not(ref e) => if self.parse_exp(e, tokens, ctx).is_some() {
                None
            } else {
                Some(ctx.create_success_node(tokens))
            },
            Expr::Eof => if tokens.current().is_none() {
                Some(ctx.create_success_node(tokens))
            } else {
                None
            },
            Expr::Any => if tokens.current().is_some() {
                Some(ctx.create_leaf_node(tokens))
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

            Expr::Enter(idx, ref e) => {
                let idx = idx as usize;
                let old = ctx.contexts[idx];
                ctx.contexts[idx] = true;
                let result = self.parse_exp(&*e, tokens, ctx);
                ctx.contexts[idx] = old;
                result
            }

            Expr::Exit(idx, ref e) => {
                let idx = idx as usize;
                let old = ctx.contexts[idx];
                ctx.contexts[idx] = false;
                let result = self.parse_exp(&*e, tokens, ctx);
                ctx.contexts[idx] = old;
                result
            }

            Expr::IsIn(idx) => if ctx.contexts[idx as usize] {
                Some(ctx.create_success_node(tokens))
            } else {
                None
            },

            Expr::Call(ref body, ref args) => {
                let old = ctx.args;
                for &(arg_pos, ref arg) in args.iter() {
                    let arg = match *arg {
                        Expr::Var(i) => ctx.args[i as usize].unwrap(),
                        _ => arg
                    };

                    ctx.args[arg_pos as usize] = Some(arg);
                }
                let result = self.parse_exp(body, tokens, ctx);
                ctx.args = old;
                result
            }

            Expr::Var(i) => {
                let expr = ctx.args[i as usize].unwrap();
                self.parse_exp(expr, tokens, ctx)
            }

            Expr::PrevIs(ref ts) => {
                if let Some(prev) = ctx.prev {
                    for &t in ts {
                        let t = self.node_type(t);
                        if t == prev {
                            return Some(ctx.create_success_node(tokens));
                        }
                    }
                }
                None
            }
        }
    }

    fn parse_any<'t, 'p, I: Iterator<Item=&'p Expr>>(&'p self, options: I, tokens: TokenSequence<'t>, ctx: &mut Ctx<'p>)
                                                     -> Option<(Node, TokenSequence<'t>)> {
        for p in options {
            if let Some(result) = self.parse_exp(p, tokens, ctx) {
                return Some(result);
            }
        }
        None
    }

    fn parse_pratt<'t, 'p>(&'p self, expr_grammar: &'p [PrattVariant], tokens: TokenSequence<'t>, ctx: &mut Ctx<'p>, min_prior: u32)
                           -> Option<(Node, TokenSequence<'t>)> {
        let (mut lhs, mut tokens) = match self.parse_pratt_prefix(expr_grammar, tokens, ctx) {
            Some(p) => p,
            None => return None,
        };

        'postfix: loop {
            let postfix = expr_grammar.iter().filter_map(|v| {
                match *v {
                    PrattVariant::Postfix { ty, ref op } => {
                        Some((self.node_type(ty), op.as_ref()))
                    }
                    _ => None
                }
            });
            for (ty, op) in postfix {
                if let Some((op_node, rest)) = self.parse_exp(op, tokens, ctx) {
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
                        ctx.prev = Some(ty);
                        continue 'bin;
                    }
                }
            }
            break;
        }
        Some((lhs, tokens))
    }

    fn parse_pratt_prefix<'t, 'p>(&'p self, expr_grammar: &'p [PrattVariant], tokens: TokenSequence<'t>, ctx: &mut Ctx<'p>)
                                  -> Option<(Node, TokenSequence<'t>)> {
        let prefix = expr_grammar.iter().filter_map(|v| {
            match *v {
                PrattVariant::Prefix { ty, ref op } => Some((self.node_type(ty), op.as_ref())),
                _ => None
            }
        });

        for (ty, op) in prefix {
            if let Some((op_node, rest)) = self.parse_exp(op, tokens, ctx) {
                let mut node = ctx.create_composite_node(Some(ty));
                ctx.push_child(&mut node, op_node);
                if let Some((expr, rest)) = self.parse_pratt(expr_grammar, rest, ctx, 999) {
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
        self.parse_any(atoms, tokens, ctx)
    }

    fn parse_exp_pred<'t, 'p>(&'p self, expr: &'p Expr, tokens: TokenSequence<'t>, ctx: &mut Ctx<'p>)
                              -> Option<(TokenSequence<'t>)> {
        let old_mode = ctx.predicate_mode;
        ctx.predicate_mode = true;
        let result = self.parse_exp(expr, tokens, ctx);
        ctx.predicate_mode = old_mode;
        result.map(|(_, ts)| ts)
    }

    fn node_type(&self, idx: usize) -> NodeType {
        self.node_types[idx]
    }
}