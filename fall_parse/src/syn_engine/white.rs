use fall_tree::{NodeType, INode, TextUnit, Language};
use syn_engine::BlackNode;
use lex_engine::Token;

fn is_ws(lang: &Language, token: Token) -> bool {
    lang.node_type_info(token.ty).whitespace_like
}

#[derive(Debug)]
pub struct WsNode {
    ty: Option<NodeType>,
    len: TextUnit,
    children: Vec<WsNode>,
    first: Option<usize>,
    last: Option<usize>
}

impl WsNode {
    fn push_child(&mut self, lang: &Language, child: WsNode, tokens: &[Token]) {
        match (self.last, child.first) {
            (Some(l), Some(r)) if l + 1 < r => {
                for idx in l + 1..r {
                    let t = tokens[idx];
                    assert!(is_ws(lang, t), "expected whitespace, got {:?}", t.ty);
                    self.push_child_raw(token_ws_node(idx, t))
                }
            }
            _ => {}
        }
        self.push_child_raw(child)
    }

    fn push_child_raw(&mut self, child: WsNode) {
        self.last = child.last.or(self.last);
        self.first = self.first.or(child.first);
        self.len += child.len;
        self.children.push(child);
    }

    fn attach_to_inode(self, parent: &mut INode) {
        if self.children.is_empty() {
            match self.ty {
                Some(ty) => parent.push_child(INode::new_leaf(ty, self.len)),
                None => parent.push_token_part(self.len),
            }
            return;
        }
        match self.into_inode() {
            Ok(node) => parent.push_child(node),
            Err(this) => {
                for child in this.children {
                    child.attach_to_inode(parent);
                }
            }
        }
    }

    pub fn into_inode(self) -> Result<INode, WsNode> {
        if let Some(ty) = self.ty {
            let mut node = INode::new(ty);
            for child in self.children {
                child.attach_to_inode(&mut node);
            }
            Ok(node)
        } else {
            Err(self)
        }
    }
}

fn token_ws_node(idx: usize, t: Token) -> WsNode {
    WsNode {
        ty: Some(t.ty),
        len: t.len,
        children: Vec::new(),
        first: Some(idx),
        last: Some(idx),
    }
}

pub fn to_white_node(lang: &Language, file_node: BlackNode, tokens: &[Token]) -> WsNode {
    let (ty, children) = match file_node {
        BlackNode::Composite { ty, children, .. } => (ty.unwrap(), children),
        _ => panic!("Root node must be composite")
    };
    let mut result = WsNode {
        ty: Some(ty),
        len: TextUnit::zero(),
        children: Vec::new(),
        first: None,
        last: None,
    };

    for (i, &t) in tokens.iter().enumerate() {
        if !is_ws(lang, t) {
            break
        }
        result.push_child_raw(token_ws_node(i, t))
    }

    for child in children {
        add_child(lang, &mut result, &child, tokens)
    }
    if let Some(idx) = result.last {
        for idx in idx + 1..tokens.len() {
            let t = tokens[idx];
            assert!(is_ws(lang, t));
            result.push_child_raw(token_ws_node(idx, t))
        }
    }
    result
}

fn add_child(lang: &Language, parent: &mut WsNode, node: &BlackNode, tokens: &[Token]) {
    match *node {
        BlackNode::Leaf { ty, token_idx } => {
            let mut node = token_ws_node(token_idx, tokens[token_idx]);
            node.ty = ty;
            parent.push_child(lang, node, tokens)
        }
        BlackNode::Composite { ty, ref children } => {
            let mut p = WsNode {
                ty: ty,
                len: TextUnit::zero(),
                children: Vec::new(),
                first: None,
                last: None,
            };
            for child in children {
                add_child(lang, &mut p, child, tokens);
            }
            parent.push_child(lang, p, tokens)
        }
    }
}
