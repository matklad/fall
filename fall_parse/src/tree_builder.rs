use elapsed::measure_time;

use fall_tree::{Language, NodeType, File, ERROR, WHITESPACE, TextRange,
                FileStats, ImmutableNode, TextUnit};
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
        layer: Option<u32>,
    }
}

impl Node {
    pub fn error() -> Node {
        Self::composite(Some(ERROR))
    }

    pub fn composite(ty: Option<NodeType>) -> Node {
        Node::Composite { ty: ty, children: Vec::new(), layer: None }
    }

    pub fn layer(layer: u32) -> Node {
        Node::Composite { ty: None, children: Vec::new(), layer: Some(layer) }
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
            text: self.text,
            start: self.start + 1,
            non_ws_indexes: &self.non_ws_indexes[1..],
            original_tokens: self.original_tokens,
        };
        (node, rest)
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
    lang: Language,
    text: String,
    tokenizer: &[LexRule],
    parser: &Fn(TokenSequence, &mut FileStats) -> Node
) -> File {
    let mut stats = FileStats::new();
    let (lex_time, owned_tokens) = measure_time(|| tokenize(&text, tokenizer).collect::<Vec<_>>());
    stats.lexing_time = lex_time.duration();
    let non_ws_indexes: Vec<usize> = owned_tokens.iter().enumerate().filter_map(|(i, t)| {
        if t.ty == WHITESPACE {
            None
        } else {
            Some(i)
        }
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
    let pre_node = to_pre_node(node, &owned_tokens);

    let node = pre_node.into_immutable_node();
    File::new(lang, text, stats, node)
}

#[derive(Debug)]
struct PreNode {
    ty: NodeType,
    len: TextUnit,
    children: Vec<PreNode>,
    first: Option<usize>,
    last: Option<usize>
}

impl PreNode {
    fn push_child(&mut self, child: PreNode, tokens: &[Token]) {
        match (self.last, child.first) {
            (Some(l), Some(r)) if l + 1 < r => {
                for idx in l + 1..r {
                    let t = tokens[idx];
                    assert!(t.ty == WHITESPACE, "expected whitespace, got {:?}", t.ty);
                    self.push_child_raw(token_pre_node(idx, t))
                }
            }
            _ => {}
        }
        self.push_child_raw(child)
    }

    fn push_child_raw(&mut self, child: PreNode) {
        self.last = child.last.or(self.last);
        self.first = self.first.or(child.first);
        self.len += child.len;
        self.children.push(child);
    }

    fn into_immutable_node(self) -> ImmutableNode {
        if self.children.is_empty() {
            return ImmutableNode::new_leaf(self.ty, self.len)
        }
        let mut node = ImmutableNode::new(self.ty);
        for child in self.children {
            node.push_child(child.into_immutable_node());
        }
        node
    }
}

fn token_pre_node(idx: usize, t: Token) -> PreNode {
    PreNode {
        ty: t.ty,
        len: t.len,
        children: Vec::new(),
        first: Some(idx),
        last: Some(idx),
    }
}

fn to_pre_node(file_node: Node, tokens: &[Token]) -> PreNode {
    let (ty, children) = match file_node {
        Node::Composite { ty, children , .. } => (ty.unwrap(), children),
        _ => panic!("Root node must be composite")
    };
    let mut result = PreNode {
        ty: ty,
        len: TextUnit::zero(),
        children: Vec::new(),
        first: None,
        last: None,
    };

    for (i, &t) in tokens.iter().enumerate() {
        if t.ty != WHITESPACE {
            break
        }
        result.push_child_raw(token_pre_node(i, t))
    }

    for child in children {
        add_child(&mut result, &child, tokens)
    }
    if let Some(idx) = result.last {
        for idx in idx + 1..tokens.len() {
            let t = tokens[idx];
            assert!(t.ty == WHITESPACE);
            result.push_child_raw(token_pre_node(idx, t))
        }
    }
    result
}

fn add_child(parent: &mut PreNode, node: &Node, tokens: &[Token]) {
    match *node {
        Node::Leaf(_, idx) => {
            parent.push_child(token_pre_node(idx, tokens[idx]), tokens)
        }
        Node::Composite { ty, ref children, layer } => {
            if ty.is_none() {
                for child in children {
                    add_child(parent, child, tokens)
                }
                return
            }
            let ty = ty.unwrap();
            let mut p = PreNode {
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
