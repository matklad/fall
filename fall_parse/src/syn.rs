use fall_tree::{NodeType, ERROR};
use lex::Token;
use TreeBuilder;

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
    Rep(Box<Expr>, Option<Vec<usize>>, Option<Vec<usize>>),
    Opt(Box<Expr>),
    Not(Vec<usize>),
    Layer(Box<Expr>, Box<Expr>),
}

impl<'r> Parser<'r> {
    pub fn new(node_types: &'r [NodeType], rules: &'r [SynRule]) -> Parser<'r> {
        Parser { node_types, rules: rules }
    }

    pub fn parse(&self, b: &mut TreeBuilder) {
        self.parse_expr(&Expr::Rule(0), b);
    }

    fn parse_expr(&self, expr: &Expr, b: &mut TreeBuilder) -> bool {
        match *expr {
            Expr::Or(ref parts) => {
                for p in parts.iter() {
                    if self.parse_expr(p, b) {
                        return true;
                    }
                }
                false
            }

            Expr::And(ref parts, commit) => {
                b.start(None);
                let commit = commit.unwrap_or(parts.len());
                for (i, p) in parts.iter().enumerate() {
                    if !self.parse_expr(p, b) {
                        if i < commit {
                            b.rollback(None);
                            return false;
                        } else {
                            b.error();
                            break
                        }
                    }
                }
                b.finish(None);
                true
            }

            Expr::Rule(id) => {
                let rule = &self.rules[id];
                let ty = rule.ty.map(|ty| self.node_type(ty));
                if id != 0 { b.start(ty) }
                if self.parse_expr(&rule.body, b) {
                    if id != 0 { b.finish(ty) };
                    true
                } else {
                    if id != 0 { b.rollback(ty) };
                    false
                }
            }

            Expr::Token(ty) => b.try_eat(self.node_type(ty)),
            Expr::Rep(ref body, ref skip_until, ref stop_at) => {
                'outer: loop {
                    let mut skipped = false;
                    'inner: loop {
                        let current = match b.current() {
                            None => {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'outer
                            }
                            Some(c) => c,
                        };
                        if let Some(ref stop_at) = *stop_at {
                            if stop_at.iter().any(|&it| self.node_type(it) == current.ty) {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'outer
                            }
                        }

                        let skip = match *skip_until {
                            None => {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'inner
                            }
                            Some(ref s) => s,
                        };
                        if skip.iter().any(|&it| self.node_type(it) == current.ty) {
                            if skipped {
                                b.finish(Some(ERROR))
                            }
                            break 'inner
                        } else {
                            if !skipped {
                                b.start(Some(ERROR))
                            }
                            skipped = true;
                            b.bump();
                        }
                    }
                    if !self.parse_expr(&*body, b) {
                        if stop_at.is_none() {
                            break 'outer;
                        } else {
                            b.start(Some(ERROR));
                            b.bump();
                            b.finish(Some(ERROR));
                        }
                    }
                }
                true
            }

            Expr::Opt(ref body) => {
                self.parse_expr(&*body, b);
                true
            }

            Expr::Not(ref ts) => {
                if let Some(current) = b.current() {
                    if self.token_set_contains(ts, current) {
                        false
                    } else {
                        b.bump();
                        true
                    }
                } else {
                    false
                }
            }

            Expr::Layer(_, ref e) => self.parse_expr(e, b)
        }
    }

    fn token_set_contains(&self, ts: &[usize], token: Token) -> bool {
        ts.iter().any(|&t| self.node_type(t) == token.ty)
    }

    fn node_type(&self, idx: usize) -> NodeType {
        self.node_types[idx]
    }
}

