use fall_tree::NodeType;
use ::{Expr, SynRule};

#[derive(Copy, Clone)]
pub enum Event {
    Start { ty: NodeType },
    Token { ty: NodeType, n_raw_tokens: u16 },
    End,
}

pub struct Grammar<'g> {
    node_types: &'g [NodeType],
    rules: &'g [SynRule],
    start_rule: &'g Expr,
}

mod parse;
mod convert;



