use elapsed::measure_time;

use fall_tree::{NodeType, ERROR, WHITESPACE, TextRange, FileStats, INode, TextUnit};
use lex::{Token, LexRule, tokenize};

#[derive(Clone, Copy, Debug)]
pub struct TokenSequence<'a> {
    text: &'a str,
    start: usize,
    non_ws_indexes: &'a [usize],
    original_tokens: &'a [Token],
}

#[derive(Debug)]
pub enum Node {
    Leaf(NodeType, usize),
    Composite {
        ty: Option<NodeType>,
        children: Vec<Node>,
    }
}

impl Node {
    pub fn error() -> Node {
        Self::composite(Some(ERROR))
    }

    pub fn composite(ty: Option<NodeType>) -> Node {
        Node::Composite { ty: ty, children: Vec::new() }
    }

    pub fn success<'t>(ts: TokenSequence<'t>) -> (Node, TokenSequence<'t>) {
        (Self::composite(None), ts)
    }
}

impl<'a> TokenSequence<'a> {
    pub fn prefix(&self, suffix: TokenSequence<'a>) -> TokenSequence<'a> {
        TokenSequence {
            text: self.text,
            start: self.start,
            non_ws_indexes: &self.non_ws_indexes[..suffix.start - self.start],
            original_tokens: self.original_tokens
        }
    }

    pub fn current(&self) -> Option<Token> {
        self.non_ws_indexes.first().map(|&idx| {
            self.original_tokens[idx]
        })
    }

    pub fn bump(&self) -> (Node, TokenSequence<'a>) {
        let token = self.current().expect("Can't bump an empty token sequence");
        let node = Node::Leaf(token.ty, self.non_ws_indexes[0]);
        let rest = TokenSequence {
            text: &self.text[token.len.as_u32() as usize..],
            start: self.start + 1,
            non_ws_indexes: &self.non_ws_indexes[1..],
            original_tokens: self.original_tokens,
        };
        (node, rest)
    }

    pub fn bump_by_text(&self, ty: NodeType, text: &str) -> Option<(Node, TokenSequence<'a>)> {
        if let Some(t) = self.current_text() {
            if t == text {
                let node = Node::Leaf(ty, self.non_ws_indexes[0]);
                let (_, rest) = self.bump();
                return Some((node, rest));
            }
        }
        None
    }

    fn current_text(&self) -> Option<&'a str> {
        match self.current() {
            Some(token) => Some(&self.text[..token.len.as_u32() as usize]),
            None => None,
        }
    }
}

impl Node {
    pub fn push_child(&mut self, child: Node) {
        match *self {
            Node::Composite { ref mut children, .. } => children.push(child),
            Node::Leaf(..) => panic!("Can't add children to a leaf node"),
        }
    }

    pub fn debug(&self, tokens: &TokenSequence) -> String {
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

    fn right_idx(&self) -> Option<usize> {
        match *self {
            Node::Leaf(_, idx) => Some(idx),
            Node::Composite { ref children, .. } => children.iter().rev().filter_map(|n| {
                n.right_idx()
            }).next(),
        }
    }

    fn left_idx(&self) -> Option<usize> {
        match *self {
            Node::Leaf(_, idx) => Some(idx),
            Node::Composite { ref children, .. } => match children.first() {
                None => None,
                Some(child) => child.left_idx()
            },
        }
    }
}

pub fn parse(
    text: &str,
    tokenizer: &[LexRule],
    parser: &Fn(TokenSequence, &mut FileStats) -> Node
) -> (FileStats, INode) {
    let mut stats = FileStats::new();
    let (lex_time, owned_tokens) = measure_time(|| tokenize(&text, tokenizer).collect::<Vec<_>>());
    stats.lexing_time = lex_time.duration();
    stats.reparsed_region = TextRange::from_to(TextUnit::zero(), TextUnit::from_usize(text.len()));
    let non_ws_indexes: Vec<usize> = owned_tokens.iter().enumerate().filter_map(|(i, t)| {
        if t.ty == WHITESPACE { None } else { Some(i) }
    }).collect();
    let (parse_time, node) = {
        let tokens = TokenSequence {
            text: &text,
            start: 0,
            non_ws_indexes: &non_ws_indexes,
            original_tokens: &owned_tokens,
        };
        measure_time(|| parser(tokens, &mut stats))
    };
    stats.parsing_time = parse_time.duration();

    let ws_node = to_ws_node(node, &owned_tokens);
    let inode = ws_node.into_inode().unwrap();
    (stats, inode)
}

#[derive(Debug)]
struct WsNode {
    ty: Option<NodeType>,
    len: TextUnit,
    children: Vec<WsNode>,
    first: Option<usize>,
    last: Option<usize>
}

impl WsNode {
    fn push_child(&mut self, child: WsNode, tokens: &[Token]) {
        match (self.last, child.first) {
            (Some(l), Some(r)) if l + 1 < r => {
                for idx in l + 1..r {
                    let t = tokens[idx];
                    assert!(t.ty == WHITESPACE, "expected whitespace, got {:?}", t.ty);
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
            parent.push_child(INode::new_leaf(self.ty.unwrap(), self.len));
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

    fn into_inode(self) -> Result<INode, WsNode> {
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

fn to_ws_node(file_node: Node, tokens: &[Token]) -> WsNode {
    let (ty, children) = match file_node {
        Node::Composite { ty, children, .. } => (ty.unwrap(), children),
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
        if t.ty != WHITESPACE {
            break
        }
        result.push_child_raw(token_ws_node(i, t))
    }

    for child in children {
        add_child(&mut result, &child, tokens)
    }
    if let Some(idx) = result.last {
        for idx in idx + 1..tokens.len() {
            let t = tokens[idx];
            assert!(t.ty == WHITESPACE);
            result.push_child_raw(token_ws_node(idx, t))
        }
    }
    result
}

fn add_child(parent: &mut WsNode, node: &Node, tokens: &[Token]) {
    match *node {
        Node::Leaf(ty, idx) => {
            let mut node = token_ws_node(idx, tokens[idx]);
            node.ty = Some(ty);
            parent.push_child(node, tokens)
        }
        Node::Composite { ty, ref children } => {
            let mut p = WsNode {
                ty: ty,
                len: TextUnit::zero(),
                children: Vec::new(),
                first: None,
                last: None,
            };
            for child in children {
                add_child(&mut p, child, tokens);
            }
            parent.push_child(p, tokens)
        }
    }
}
