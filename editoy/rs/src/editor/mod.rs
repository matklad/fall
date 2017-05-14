use std::path::Path;

use file;
use fall_gen::highighting;

use ediproto::{ViewStateReply, Line, StyledText};
use model::{Direction, Amount, GridPosition, State, Editor, InputEvent};

mod utils;

use self::utils::{move_cursor_by, collect_text};


#[derive(Default)]
pub struct EditorImpl {
    state: State,
}

impl Editor for EditorImpl {
    fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::Ready => {}
            InputEvent::MoveCursor(d, a) => do_move_cursor(&mut self.state, d, a),
            InputEvent::InsertText(text) => {
                if text == "\x08" {
                    do_backspace(&mut self.state);
                } else {
                    do_insert(&mut self.state, text);
                }
            }
            InputEvent::OpenFile(path) => do_open_file(&mut self.state, &path)
        }
        let text = collect_text(&self.state);
        let file = ::fall_gen::FallFile::parse(text.clone());
        self.state.syntax_tree = file.tree_to_string();
        self.state.spans = highighting::colorize(text);
    }

    fn render(&self) -> ViewStateReply {
        let mut result = ViewStateReply::new();
        render_lines(&mut result, &self.state.lines, &self.state.spans);
        result.cursorX = self.state.cursor.x as i32;
        result.cursorY = self.state.cursor.y as i32;
        result.syntax_tree = self.state.syntax_tree.clone();
        result
    }
}

fn render_lines(reply: &mut ViewStateReply, lines: &[String], spans: &[(u32, u32, &'static str)]) {
    let mut lines = lines.iter();
    let mut curr_global_offset = 0;
    macro_rules! next_line {
        () => { if let Some(line) = lines.next() { &line } else { return } }
    }
    let mut curr_line: &str = next_line!();

    let mut line_sink = Line::new();
    macro_rules! push_line {
        () => {
            let mut line_done = Line::new();
            ::std::mem::swap(&mut line_done, &mut line_sink);
            reply.mut_lines().push(line_done);
         }
    }

    let mut events = Vec::new();
    for (i, span) in spans.iter().enumerate() {
        events.push((span.0 as usize, i as i32, false, span.2));
        events.push((span.1 as usize, -(i as i32), true, span.2));
    }
    events.sort();
    let mut events = events.into_iter();

    let mut color_stack = vec!["text"];
    macro_rules! push_range {
        ($chunk:expr) => {
            let range = {
                let mut range = StyledText::new();
                range.style = (*color_stack.last().unwrap()).to_owned();
                range.text = $chunk.to_owned();
                range
            };
            line_sink.mut_ranges().push(range);
         }
    }

    while let Some((off, _, is_close, color)) = events.next() {
        let mut l = off - curr_global_offset;
        while l > 0 {
            let effective_l = ::std::cmp::min(l, curr_line.len());
            l -= effective_l;
            let (chunk, new_curr) = curr_line.split_at(effective_l);
            curr_line = new_curr;
            push_range!(chunk);
            if curr_line.is_empty() {
                push_line!();
                curr_line = next_line!();
            }
        }
        if is_close {
            color_stack.pop().unwrap();
        } else {
            color_stack.push(color);
        }
        curr_global_offset = off;
    }

    loop {
        push_range!(curr_line);
        push_line!();
        curr_line = next_line!();
    }
}

fn do_move_cursor(state: &mut State, direction: Direction, amount: Amount) {
    let (am_vert, am_hor): (i32, i32) = match amount {
        Amount::CHAR => (1, 1),
        Amount::PAGE => (10, ::std::i32::MAX / 2),
    };

    let (dx, dy) = match direction {
        Direction::UP => (0i32, -1 * am_vert),
        Direction::LEFT => (-1 * am_hor, 0),
        Direction::DOWN => (0, 1 * am_vert),
        Direction::RIGHT => (1 * am_hor, 0),
    };

    move_cursor_by(state, dx, dy)
}

fn do_backspace(state: &mut State) {
    delete_backwards(state);
    move_cursor_by(state, -1, 0);
}

fn do_insert(state: &mut State, text: String) {
    let dx = text.len();
    insert_text(state, text);
    move_cursor_by(state, dx as i32, 0);
}

fn do_open_file(state: &mut State, path: &Path) {
    let text = file::get_text(path).unwrap();
    state.lines = text.lines().map(|l| l.to_owned()).collect();
}

fn insert_text(state: &mut State, text: String) {
    let line_idx = state.cursor.y as usize;
    if line_idx == state.lines.len() {
        state.lines.push(String::new())
    }
    let (line, insert_point) = match position_in_line(&mut state.lines, state.cursor) {
        Some(p) => p,
        None => return
    };
    let tail = line.split_off(insert_point);
    line.push_str(&text);
    line.push_str(&tail);
}


fn delete_backwards(state: &mut State) {
    let (line, insert_point) = match position_in_line(&mut state.lines, state.cursor) {
        Some(p) => p,
        None => return
    };
    if insert_point == 0 {
        return
    }
    let tail = line.split_off(insert_point - 1);
    line.push_str(&tail[1..]);
}

fn position_in_line<'l>(lines: &'l mut [String], cursor: GridPosition) -> Option<(&'l mut String, usize)> {
    let line = match lines.get_mut(cursor.y as usize) {
        Some(line) => line,
        None => return None
    };
    let insert_point = cursor.x as usize;
    if insert_point > line.len() {
        return None;
    }
    Some((line, insert_point))
}
