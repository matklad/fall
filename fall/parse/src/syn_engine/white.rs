use fall_tree::{NodeType, INode};
use lex_engine::Token;
use super::{BlackNode, BlackIdx};

pub fn into_white(
    text: &str,
    node: BlackNode,
    tokens: &[Token],
    whitespace_binder: fn(ty: NodeType, adjacent_spaces: Vec<(NodeType, &str)>, leading: bool) -> usize
) -> WhiteNode {
    let file_ty = match node {
        BlackNode::Composite { ty: Some(ty), .. } => ty,
        _ => panic!("Bad file node")
    };
    let binder = |ty: Option<NodeType>,
                  adjacent_spaces: Vec<(NodeType, &str)>,
                  leading: bool| -> usize {
        match ty {
            None => 0,
            Some(ty) if ty == file_ty => adjacent_spaces.len(),
            Some(ty) => whitespace_binder(ty, adjacent_spaces, leading)
        }
    };

    Whitespacer::new(text, tokens, &binder).black_to_white(node, (0, tokens.len()))
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

struct Whitespacer<'a> {
    text: &'a str,
    tokens: &'a [Token],
    token_lens: Vec<usize>,
    whitespace_binder: &'a Fn(Option<NodeType>, Vec<(NodeType, &str)>, bool) -> usize,
}

impl<'a> Whitespacer<'a> {
    fn new(
        text: &'a str,
        tokens: &'a [Token],
        whitespace_binder: &'a Fn(Option<NodeType>, Vec<(NodeType, &str)>, bool) -> usize
    ) -> Whitespacer<'a> {
        let mut token_lens = Vec::with_capacity(tokens.len() + 1);
        let mut current = 0;
        token_lens.push(current);
        for t in tokens {
            current += t.len.into();
            token_lens.push(current);
        }

        Whitespacer {
            text,
            tokens,
            token_lens,
            whitespace_binder,
        }
    }

    fn black_to_white(&self, black: BlackNode, cover_range: (usize, usize)) -> WhiteNode {
        if let Some((BlackIdx(l), BlackIdx(r))) = black.token_range() {
            assert!(cover_range.0 <= l && r <= cover_range.1);
        }

        match black {
            BlackNode::Leaf { ty, token_idx: BlackIdx(token_idx) } =>
                return WhiteNode {
                    ty,
                    token_range: (token_idx, token_idx + 1),
                    children: Vec::new(),
                },
            BlackNode::Composite { ty, mut children } => {
                let mut internal_children = Vec::new();
                children.reverse();

                let mut left_edge = cover_range.0;
                let mut first_child = true;
                while let Some(child) = children.pop() {
                    let right_edge = children.last()
                        .and_then(|n| n.token_range())
                        .map(|(_, BlackIdx(i))| i)
                        .unwrap_or(cover_range.1);
                    let child = self.black_to_white(child, (left_edge, right_edge));

                    if !first_child {
                        for i in left_edge..child.token_range.0 {
                            internal_children.push(self.leaf(i))
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


                let leading_space = (cover_range.0..internal_start).into_iter()
                    .map(|token_idx| self.external_token(token_idx));

                let trailing_space = (internal_end..cover_range.1).into_iter()
                    .map(|token_idx| self.external_token(token_idx));

                let external_start = internal_start
                    - (self.whitespace_binder)(ty, leading_space.collect(), true);

                let external_end = internal_end
                    + (self.whitespace_binder)(ty, trailing_space.collect(), false);

                let mut children = Vec::with_capacity(internal_children.len());
                for i in external_start..internal_start {
                    children.push(self.leaf(i))
                }
                children.extend(internal_children);
                for i in internal_end..external_end {
                    children.push(self.leaf(i))
                }
                WhiteNode::new(ty, (external_start, external_end), children)
            }
        }
    }

    fn external_token(&self, token_idx: usize) -> (NodeType, &'a str) {
        let t = self.tokens[token_idx];
        let start = self.token_lens[token_idx];
        (t.ty, &self.text[start .. start + usize::from(t.len)])
    }

    fn leaf(&self, token_idx: usize) -> WhiteNode {
        let ty = self.tokens[token_idx].ty;
        WhiteNode::new(Some(ty), (token_idx, token_idx + 1), Vec::new())
    }
}
