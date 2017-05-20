use fall_tree::{NodeType, ERROR};
use TreeBuilder;

pub struct Parser<'r> {
    rules: &'r [SynRule],
}

pub struct SynRule {
    pub ty: Option<NodeType>,
    pub body: Expr,
}


pub enum Expr {
    Or(&'static [Expr]),
    And(&'static [Expr], Option<usize>),
    Rule(usize),
    Token(NodeType),
    Rep(&'static Expr, Option<&'static [NodeType]>, Option<&'static [NodeType]>),
    Opt(&'static Expr),
}

impl<'r> Parser<'r> {
    pub fn new(rules: &[SynRule]) -> Parser {
        Parser { rules: rules }
    }

    pub fn parse(&self, b: &mut TreeBuilder) {
        self.parse_expr(&Expr::Rule(0), b);
    }

    fn parse_expr(&self, expr: &Expr, b: &mut TreeBuilder) -> bool {
        match *expr {
            Expr::Or(parts) => {
                for p in parts.iter() {
                    if self.parse_expr(p, b) {
                        return true;
                    }
                }
                false
            }

            Expr::And(parts, commit) => {
                let commit = commit.unwrap_or(parts.len());
                for (i, p) in parts.iter().enumerate() {
                    if !self.parse_expr(p, b) {
                        if i < commit {
                            return false;
                        } else {
                            b.error();
                            break
                        }
                    }
                }
                true
            }

            Expr::Rule(id) => {
                let rule = &self.rules[id];
                if id != 0 { b.start(rule.ty) }
                if self.parse_expr(&rule.body, b) {
                    if id != 0 { b.finish(rule.ty) };
                    true
                } else {
                    if id != 0 { b.rollback(rule.ty) };
                    false
                }
            }

            Expr::Token(ty) => b.try_eat(ty),
            Expr::Rep(body, skip_until, _) => {
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
                        let skip = match skip_until {
                            None => {
                                if skipped {
                                    b.finish(Some(ERROR))
                                }
                                break 'inner
                            }
                            Some(s) => s,
                        };
                        if skip.iter().any(|&it| it == current.ty) {
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
                    if !self.parse_expr(body, b) {
                        break 'outer;
                    }
                }
                true
            }
            Expr::Opt(body) => {
                self.parse_expr(body, b);
                true
            }
        }
    }
}

