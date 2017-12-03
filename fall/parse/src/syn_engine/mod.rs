use fall_tree::NodeType;
use ::{Expr, ExprRef};

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Start { ty: NodeType, forward_parent: Option<u32> },
    Token { ty: NodeType, n_raw_tokens: u16 },
    End,
}

pub struct Grammar<'g> {
    pub node_types: &'g [NodeType],
    pub rules: &'g [Expr],
    pub start_rule: ExprRef,
}

mod parser;
mod expr;
mod pratt;
pub use self::expr::parse;
mod convert;
pub use self::convert::convert;



