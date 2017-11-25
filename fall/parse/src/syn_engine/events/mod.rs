use fall_tree::NodeType;
use ::{Expr, SynRule};

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Start { ty: NodeType },
    Token { ty: NodeType, n_raw_tokens: u16 },
    End,
}

pub struct Grammar<'g> {
    pub node_types: &'g [NodeType],
    pub rules: &'g [SynRule],
    pub start_rule: &'g Expr,
}

mod parse;
pub use self::parse::parse;
mod convert;
pub use self::convert::convert;



