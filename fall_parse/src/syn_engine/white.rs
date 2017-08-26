use fall_tree::{Language, NodeType, INode};
use lex_engine::Token;
use super::BlackNode;

pub fn into_white(node: BlackNode, lang: &Language, tokens: &[Token]) -> WhiteNode {
    black_to_white(lang, node, tokens, (0, tokens.len()), true)
}


#[derive(Debug)]
pub struct WhiteNode {
    ty: Option<NodeType>,
    token_range: (usize, usize),
    children: Vec<WhiteNode>,
}

impl WhiteNode {
    pub fn new(
        ty: Option<NodeType>,
        token_range: (usize, usize),
        children: Vec<WhiteNode>,
    ) -> WhiteNode {
        if !children.is_empty() {
            let first = children.first().unwrap();
            let last = children.last().unwrap();
            assert_eq!(token_range, (first.token_range.0, last.token_range.1),
                       "child ranges: {:?}", children.iter().map(|n| n.token_range).collect::<Vec<_>>());
        }

        WhiteNode { ty, token_range, children }
    }

    pub fn leaf(ty: Option<NodeType>, token_idx: usize) -> WhiteNode {
        WhiteNode::new(ty, (token_idx, token_idx + 1), Vec::new())
    }

    pub fn into_inode(self, tokens: &[Token]) -> INode {
        let ty = self.ty.unwrap();
        assert_eq!(self.token_range, (0, tokens.len()));
        let mut result = INode::new(ty);
        self.inject_into(&mut result, tokens);
        result
    }

    fn inject_into(self, parent: &mut INode, tokens: &[Token]) {
        if self.children.is_empty() {
            let len = tokens[self.token_range.0..self.token_range.1]
                .iter()
                .map(|t| t.len)
                .sum();
            parent.push_token_part(len);
            return;
        }

        for child in self.children {
            if let Some(ty) = child.ty {
                let mut new_node = INode::new(ty);
                child.inject_into(&mut new_node, tokens);
                parent.push_child(new_node);
            } else {
                child.inject_into(parent, tokens);
            }
        }
    }
}


fn black_to_white(
    lang: &Language,
    black: BlackNode,
    tokens: &[Token],
    cover_range: (usize, usize),
    grow: bool
) -> WhiteNode {
    if let Some(node_range) = black.token_range() {
        assert!(cover_range.0 <= node_range.0 && node_range.1 <= cover_range.1);
    }

    match black {
        BlackNode::Leaf { ty, token_idx } =>
            return WhiteNode::leaf(ty, token_idx),
        BlackNode::Composite { ty, mut children } => {
            let mut internal_children = Vec::new();
            children.reverse();

            let mut left_edge = cover_range.0;
            let mut first_child = true;
            while let Some(child) = children.pop() {
                let right_edge = children.last()
                    .and_then(|n| n.token_range())
                    .unwrap_or(cover_range)
                    .1;
                let child = black_to_white(lang, child, tokens, (left_edge, right_edge), false);

                if !first_child {
                    for i in left_edge..child.token_range.0 {
                        internal_children.push(WhiteNode::leaf(Some(tokens[i].ty), i))
                    }
                }
                first_child = false;
                left_edge = child.token_range.1;
                internal_children.push(child);
            }

            let internal_start = internal_children
                .first()
                .map(|n| n.token_range.0)
                .unwrap_or(cover_range.0);
            let internal_end = internal_children
                .last()
                .map(|n| n.token_range.1)
                .unwrap_or(cover_range.0);

            let (children, range) = if !grow {
                (internal_children, (internal_start, internal_end))
            } else {
                let mut children = Vec::new();
                for i in cover_range.0..internal_start {
                    children.push(WhiteNode::leaf(Some(tokens[i].ty), i))
                }
                children.extend(internal_children);
                for i in internal_end..cover_range.1 {
                    children.push(WhiteNode::leaf(Some(tokens[i].ty), i))
                }
                (children, cover_range)
            };
            WhiteNode::new(ty, range, children)
        }
    }
}
