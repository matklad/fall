use super::Event;
use fall_tree::{NodeType, IToken, INode, Text, TextRange, tu};

pub fn convert(
    text: Text,
    tokens: &[IToken],
    events: &[Event],
    is_whitespace: &Fn(NodeType) -> bool,
    whitespace_binder: &Fn(NodeType, &[(NodeType, Text)], bool) -> usize,
) -> INode {
    let events = reshuffle_events(events);
    let (first, rest) = (events[0], &events[1..]);

    let mut len = tu(0);
    let tokens = tokens.iter().map(|&t| {
        let token_text = text.slice(TextRange::from_len(len, t.len));
        len += t.len;
        (t, token_text)
    }).collect::<Vec<_>>();

    let conv = Convertor { is_whitespace, whitespace_binder };
    match first {
        Event::Start { ty, forward_parent: _ } => {
            let conversion = conv.go(ty, &tokens, rest);
            assert_eq!(conversion.token_range, (0, tokens.len()));
            assert_eq!(conversion.n_events, rest.len());
            conversion.inode
        }
        _ => unreachable!()
    }
}

fn reshuffle_events(events: &[Event]) -> Vec<Event> {
    let mut result = Vec::with_capacity(events.len());
    let mut holes = Vec::new();
    let mut forward_parents = Vec::new();

    for (i, &e) in events.iter().enumerate() {
        if holes.last() == Some(&i) {
            holes.pop();
            continue
        }
        match e {
            Event::Start { forward_parent: Some(_), .. } => {
                forward_parents.clear();
                let mut idx = i;
                loop {
                    let (ty, fwd) = match events[idx] {
                        Event::Start { ty, forward_parent } => (ty, forward_parent),
                        _ => unreachable!(),
                    };
                    forward_parents.push((idx, ty));
                    if let Some(fwd) = fwd {
                        idx += fwd as usize;
                    } else {
                        break;
                    }
                }
                for &(idx, ty) in forward_parents.iter().into_iter().rev() {
                    result.push(Event::Start { ty, forward_parent: None });
                    holes.push(idx);
                }
                holes.pop();
            }
            _ => result.push(e)
        }
    }
    result
}


struct Convertor<'a> {
    is_whitespace: &'a Fn(NodeType) -> bool,
    whitespace_binder: &'a Fn(NodeType, &[(NodeType, Text)], bool) -> usize,
}

#[derive(Debug)]
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
        let left_wd = leading_ws.len() - (self.whitespace_binder)(ty, &leading_ws, true);
        let tokens = &tokens[left_wd..];

        let mut inode = INode::new(ty);
        let (n_tokens, n_events) = self.fill(&mut inode, tokens, events);
        let tokens = &tokens[n_tokens..];

        let trailing_ws = self.collect_tokens_for_binder(tokens);
        let right_ws = (self.whitespace_binder)(ty, &trailing_ws, false);
        for &(t, _) in &tokens[..right_ws] {
            inode.push_child(INode::new_leaf(t.ty, t.len))
        }

        let right_edge = left_wd + n_tokens + right_ws;
        return Conversion {
            token_range: (left_wd, right_edge),
            n_events,
            inode,
        };
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
        let mut n_tokens = 0;
        let mut events = events;
        let mut n_events = 0;
        loop {
            let (first, rest) = (events[0], &events[1..]);
            n_events += 1;
            events = rest;
            match first {
                Event::Start { ty, forward_parent: _ } => {
                    let Conversion { token_range, n_events: child_events, inode: child } =
                        self.go(ty, tokens, events);
                    for i in 0..token_range.0 {
                        let t = tokens[i].0;
                        inode.push_child(INode::new_leaf(t.ty, t.len))
                    }
                    inode.push_child(child);
                    tokens = &tokens[token_range.1..];
                    n_tokens += token_range.1;
                    events = &events[child_events..];
                    n_events += child_events;
                }

                Event::End => return (n_tokens, n_events),

                Event::Token { ty, n_raw_tokens } => {
                    let non_white = tokens.iter().take_while(|&&(t, _)| (self.is_whitespace)(t.ty)).count();
                    for i in 0..non_white {
                        let t = tokens[i].0;
                        inode.push_child(INode::new_leaf(t.ty, t.len))
                    }
                    tokens = &tokens[non_white..];
                    n_tokens += non_white;

                    let n_raw_tokens = n_raw_tokens as usize;
                    let mut token = INode::new(ty);
                    for &(t, _) in &tokens[..n_raw_tokens] {
                        n_tokens += 1;
                        token.push_token_part(t.len)
                    }
                    inode.push_child(token);
                    tokens = &tokens[n_raw_tokens..];
                }
            }
        }
    }
}
