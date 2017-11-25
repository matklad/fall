use super::Event;
use fall_tree::{NodeType, IToken, INode, Text, TextRange, tu};

pub fn convert(
    text: Text,
    tokens: &[IToken],
    events: &[Event],
    is_whitespace: fn(NodeType) -> bool,
    whitespace_binder: fn(ty: NodeType, adjacent_spaces: &[(NodeType, Text)], leading: bool) -> usize,
) -> INode {
    let (first, rest) = (events[0], &events[1..]);

    let mut len = tu(0);
    let tokens = tokens.iter().map(|&t| {
        let token_text = text.slice(TextRange::from_len(len, t.len));
        len += t.len;
        (t, token_text)
    }).collect::<Vec<_>>();

    match first {
        Event::Start { ty } => {
            let conv = Convertor { is_whitespace, whitespace_binder };
            let conversion = conv.go(ty, &tokens, events);
            assert_eq!(conversion.token_range, (0, tokens.len()));
            assert_eq!(conversion.n_events, events.len());
            conversion.inode
        }
        _ => unreachable!()
    }
}

struct Convertor {
    is_whitespace: fn(NodeType) -> bool,
    whitespace_binder: fn(NodeType, &[(NodeType, Text)], leading: bool) -> usize,
}

struct Conversion {
    token_range: (usize, usize),
    n_events: usize,
    inode: INode,
}

impl Convertor {
    fn collect_tokens_for_binder<'t>(&self, tokens: &[(IToken, Text<'t>)]) -> Vec<(NodeType, Text<'t>)> {
        tokens.iter()
            .take_while(|&&(t, _)| (self.is_whitespace)(t.ty))
            .map(|&(t, text)| (t.ty, text))
            .collect()
    }

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

        for &(t, _) in &tokens[..left_edge] {
            inode.push_child(INode::new_leaf(t.ty, t.len))
        }
        let mut tokens = &tokens[left_edge..];
        let mut right_edge = leading_ws.len();

        let mut events = events;
        let mut n_events = 0;
        loop {
            let (first, rest) = (events[0], &events[1..]);
            n_events += 1;
            events = rest;
            match events[0] {

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

                Event::End => {
                    let trailing_ws = self.collect_tokens_for_binder(tokens);
                    let to_bind = (self.whitespace_binder)(ty, &trailing_ws, false);
                    for &(t, _) in &tokens[right_edge..right_edge + to_bind] {
                        inode.push_child(INode::new_leaf(t.ty, t.len))
                    }
                    right_edge += to_bind;
                    return Conversion {
                        token_range: (left_edge, right_edge),
                        n_events,
                        inode,
                    }
                }

                Event::Token { ty, n_raw_tokens } => {
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

