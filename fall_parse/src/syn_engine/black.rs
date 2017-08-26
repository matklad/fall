use fall_tree::{TextUnit, TextRange, NodeType};

use super::{TokenSeq, BlackIdx};

/// AST node that includes only non-whitespace (hence, black) tokens.
#[derive(Debug)]
pub enum BlackNode {
    Leaf {
        ty: Option<NodeType>,
        token_idx: BlackIdx
    },
    Composite {
        ty: Option<NodeType>,
        children: Vec<BlackNode>,
    }
}

impl BlackNode {
    pub fn push_child(&mut self, child: BlackNode) {
        match *self {
            BlackNode::Composite { ref mut children, .. } => children.push(child),
            BlackNode::Leaf { .. } => panic!("Can't add children to a leaf node"),
        }
    }

    pub fn debug(&self, tokens: &TokenSeq) -> String {
        let (l, r) = match (self.left_idx(), self.right_idx()) {
            (Some(l), Some(r)) => (l, r),
            _ => return "EMPTY-NODE".to_owned()
        };
        let mut result = String::new();
        let mut start = tokens.original_tokens[..l].iter().map(|t| t.len).sum::<TextUnit>();
        for t in tokens.original_tokens[l..r].iter() {
            result += &tokens.text[TextRange::from_len(start, t.len)];
            start += t.len;
        }
        result
    }

    fn right_idx(&self) -> Option<BlackIdx> {
        match *self {
            BlackNode::Leaf { token_idx, .. } => Some(token_idx),
            BlackNode::Composite { ref children, .. } =>
                children.iter().rev()
                    .filter_map(|n| n.right_idx())
                    .next(),
        }
    }

    fn left_idx(&self) -> Option<BlackIdx> {
        match *self {
            BlackNode::Leaf { token_idx, .. } => Some(token_idx),
            BlackNode::Composite { ref children, .. } =>
                children.first()
                    .and_then(|child| child.left_idx()),
        }
    }
}
