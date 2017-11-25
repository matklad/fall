use super::Event;
use fall_tree::{NodeType, IToken, INode, Text, TextRange, tu};

pub fn convert(
    text: Text,
    tokens: &[IToken],
    events: &[Event],
    is_whitespace: &Fn(NodeType) -> bool,
    whitespace_binder: &Fn(NodeType, &[(NodeType, Text)], bool) -> usize,
) -> INode {
    let (first, rest) = (events[0], &events[1..]);

    let mut len = tu(0);
    let tokens = tokens.iter().map(|&t| {
        let token_text = text.slice(TextRange::from_len(len, t.len));
        len += t.len;
        (t, token_text)
    }).collect::<Vec<_>>();

    let conv = Convertor { is_whitespace, whitespace_binder };
    match first {
        Event::Start { ty } => {
            let conversion = conv.go(ty, &tokens, rest);
            assert_eq!(conversion.token_range, (0, tokens.len()));
            assert_eq!(conversion.n_events, rest.len());
            conversion.inode
        }
        _ => unreachable!()
    }
}

struct Convertor<'a> {
    is_whitespace: &'a Fn(NodeType) -> bool,
    whitespace_binder: &'a Fn(NodeType, &[(NodeType, Text)], bool) -> usize,
}

struct Conversion {
    token_range: (usize, usize),
    n_events: usize,
    inode: INode,
}

impl<'a> Convertor<'a> {
    fn go(
        &self,
        ty: NodeType,
        tokens: &[(IToken, Text)],
        events: &[Event],
    ) -> Conversion {
        let leading_ws = self.collect_tokens_for_binder(tokens);

        let mut inode = INode::new(ty);
        let left_edge = leading_ws.len();
        let left_edge = left_edge - (self.whitespace_binder)(ty, &leading_ws, true);

        for &(t, _) in &tokens[left_edge..leading_ws.len()] {
            inode.push_child(INode::new_leaf(t.ty, t.len))
        }
        let tokens = &tokens[leading_ws.len()..];
        let (mut right_edge, n_events) = self.fill(&mut inode, tokens, events);
        let tokens = &tokens[right_edge..];
        right_edge += leading_ws.len();

        let trailing_ws = self.collect_tokens_for_binder(tokens);
        let to_bind = (self.whitespace_binder)(ty, &trailing_ws, false);
        for &(t, _) in &tokens[..to_bind] {
            inode.push_child(INode::new_leaf(t.ty, t.len))
        }
        right_edge += to_bind;
        Conversion {
            token_range: (left_edge, right_edge),
            n_events,
            inode,
        }
    }

    fn collect_tokens_for_binder<'t>(&self, tokens: &[(IToken, Text<'t>)]) -> Vec<(NodeType, Text<'t>)> {
        tokens.iter()
            .take_while(|&&(t, _)| (self.is_whitespace)(t.ty))
            .map(|&(t, text)| (t.ty, text))
            .collect()
    }

    fn fill(
        &self,
        inode: &mut INode,
        tokens: &[(IToken, Text)],
        events: &[Event],
    ) -> (usize, usize) {
        let mut tokens = tokens;
        let mut right_edge = 0;
        let mut events = events;
        let mut n_events = 0;
        loop {
            let (first, rest) = (events[0], &events[1..]);
            n_events += 1;
            events = rest;
            match first {
                Event::Start { ty } => {
                    let Conversion { token_range, n_events: child_events, inode: child } = self.go(ty, tokens, events);
                    for i in 0..token_range.0 {
                        let t = tokens[i].0;
                        inode.push_child(INode::new_leaf(t.ty, t.len))
                    }
                    inode.push_child(child);
                    tokens = &tokens[token_range.1..];
                    right_edge += token_range.1;
                    events = &events[child_events..];
                    n_events += child_events;
                }

                Event::End => return (right_edge, n_events),

                Event::Token { ty, n_raw_tokens } => {
                    let non_white = tokens.iter().take_while(|&&(t, _)| (self.is_whitespace)(t.ty)).count();
                    for i in 0..non_white {
                        let t = tokens[i].0;
                        inode.push_child(INode::new_leaf(t.ty, t.len))
                    }
                    tokens = &tokens[non_white..];
                    right_edge += non_white;

                    let n_raw_tokens = n_raw_tokens as usize;
                    let mut token = INode::new(ty);
                    for &(t, _) in &tokens[..n_raw_tokens] {
                        right_edge += 1;
                        token.push_token_part(t.len)
                    }
                    inode.push_child(token);
                    tokens = &tokens[n_raw_tokens..];
                }
            }
        }
    }
}
