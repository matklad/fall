use fall_tree::{NodeType};
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
    Rep(Box<Expr>, Option<Vec<usize>>, Option<Vec<usize>>),
    Opt(Box<Expr>),
    Not(Vec<usize>),
    Layer(Box<Expr>, Box<Expr>),
}

impl<'r> Parser<'r> {
    pub fn new(node_types: &'r [NodeType], rules: &'r [SynRule]) -> Parser<'r> {
        Parser { node_types, rules: rules }
    }

    pub fn parse(&self, tokens: TokenSequence, nf: &mut NodeFactory) -> Node {
        self.parse_exp(&Expr::Rule(0), tokens, nf).unwrap().0
    }

    fn parse_exp<'t>(&self, expr: &Expr, tokens: TokenSequence<'t>, nf: &mut NodeFactory)
                     -> Option<(Node, TokenSequence<'t>)> {
        match *expr {
            Expr::Or(ref parts) => {
                for p in parts.iter() {
                    if let Some(result) = self.parse_exp(p, tokens, nf) {
                        return Some(result)
                    }
                }
                None
            }

            Expr::And(ref parts, commit) => {
                let mut node = nf.create_composite_node(None);
                let commit = commit.unwrap_or(parts.len());
                let mut tokens = tokens;
                for (i, p) in parts.iter().enumerate() {
                    if let Some((n, ts)) = self.parse_exp(p, tokens, nf) {
                        tokens = ts;
                        node.push_child(n);
                    } else {
                        if i < commit {
                            return None
                        }
                        let error_node = nf.create_error_node();
                        node.push_child(error_node);
                        break
                    }
                }
                Some((node, tokens))
            }

            Expr::Rule(id) => {
                let rule = &self.rules[id];
                let ty = rule.ty.map(|ty| self.node_type(ty));
                if let Some((node, ts)) = self.parse_exp(&rule.body, tokens, nf) {
                    let mut result = nf.create_composite_node(ty);
                    result.push_child(node);
                    Some((result, ts))
                } else {
                    None
                }
            }

            Expr::Token(ty) => {
                if let Some(current) = tokens.current() {
                    if self.token_set_contains(&[ty], current) {
                        return Some(nf.create_leaf_node(tokens))
                    }
                }
                None
            }

            Expr::Opt(ref body) => {
                self.parse_exp(&*body, tokens, nf).or_else(|| {
                    Some((nf.create_composite_node(None), tokens))
                })
            }

            Expr::Not(ref ts) => {
                if let Some(current) = tokens.current() {
                    if !self.token_set_contains(ts, current) {
                        return Some(nf.create_leaf_node(tokens))
                    }
                }
                None
            }

            Expr::Layer(_, ref e) => self.parse_exp(e, tokens, nf),

            Expr::Rep(ref body,  _, _) => {
                let mut node = nf.create_composite_node(None);
                let mut tokens = tokens;
                loop {
                    if let Some((n, t)) = self.parse_exp(body, tokens, nf) {
                        node.push_child(n);
                        tokens = t;
                    } else {
                        break;
                    }
                }
                Some((node, tokens))
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

