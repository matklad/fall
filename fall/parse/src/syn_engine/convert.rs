use fall_tree::{NodeType, Text, TextRange, TextUnit, tu};
use lex_engine::Token;
use syn_engine::Event;

pub trait TB {
    fn start_internal(&mut self, ty: NodeType);
    fn leaf(&mut self, ty: NodeType, len: TextUnit);
    fn finish_internal(&mut self);
}

pub(crate) fn convert(
    text: Text,
    tokens: &[Token],
    events: &[Event],
    is_whitespace: &Fn(NodeType) -> bool,
    whitespace_binder: &Fn(NodeType, &[(NodeType, Text)], bool) -> usize,
    builder: &mut TB,
) {
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
            conv.go(ty, &tokens, rest, builder);
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
    right_edge: usize,
    n_events: usize,
}

impl<'a> Convertor<'a> {
    fn go(
        &self,
        ty: NodeType,
        tokens: &[(Token, Text)],
        events: &[Event],
        builder: &mut TB,
    ) -> Conversion {
        builder.start_internal(ty);
        let (n_tokens, n_events) = self.fill(tokens, events, builder);
        let tokens = &tokens[n_tokens..];

        let trailing_ws = self.collect_tokens_for_binder(tokens);
        let right_ws = (self.whitespace_binder)(ty, &trailing_ws, false);
        for &(t, _) in &tokens[..right_ws] {
            builder.leaf(t.ty, t.len);
        }
        builder.finish_internal();

        let right_edge = n_tokens + right_ws;
        return Conversion {
            right_edge,
            n_events,
        };
    }

    fn collect_tokens_for_binder<'t>(&self, tokens: &[(Token, Text<'t>)]) -> Vec<(NodeType, Text<'t>)> {
        tokens.iter()
            .take_while(|&&(t, _)| (self.is_whitespace)(t.ty))
            .map(|&(t, text)| (t.ty, text))
            .collect()
    }

    fn fill(
        &self,
        tokens: &[(Token, Text)],
        events: &[Event],
        builder: &mut TB,
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
                    let leading_ws = self.collect_tokens_for_binder(tokens);
                    let left_wd = leading_ws.len() - (self.whitespace_binder)(ty, &leading_ws, true);
                    for i in 0..left_wd {
                        let t = tokens[i].0;
                        builder.leaf(t.ty, t.len);
                    }
                    tokens = &tokens[left_wd..];
                    n_tokens += left_wd;

                    let Conversion { right_edge, n_events: child_events } =
                        self.go(ty, tokens, events, builder);
                    tokens = &tokens[right_edge..];
                    n_tokens += right_edge;
                    events = &events[child_events..];
                    n_events += child_events;
                }

                Event::End => return (n_tokens, n_events),

                Event::Token { ty, n_raw_tokens } => {
                    let non_white = tokens.iter().take_while(|&&(t, _)| (self.is_whitespace)(t.ty)).count();
                    for i in 0..non_white {
                        let t = tokens[i].0;
                        builder.leaf(t.ty, t.len);
                    }
                    tokens = &tokens[non_white..];
                    n_tokens += non_white;

                    let n_raw_tokens = n_raw_tokens as usize;
                    let mut len = tu(0);
                    for &(t, _) in &tokens[..n_raw_tokens] {
                        n_tokens += 1;
                        len += t.len;
                    }
                    builder.leaf(ty, len);
                    tokens = &tokens[n_raw_tokens..];
                }

                Event::Cached { .. } => {}
            }
        }
    }
}
