use std::path::PathBuf;
use xi_rope::tree::Cursor;
use elapsed::ElapsedDuration;

use file;
use fall_tree::{Node, WHITESPACE, TextOffset, File, Language};
use fall_tree::search::{find_leaf_at_offset, ancestors};
use lang_fall::{LANG_FALL, highlight};

use ediproto::{ViewStateReply, Line, StyledText};
use model::{Direction, Amount, State, Editor, InputEvent, CowStr};

mod utils;

use self::utils::{move_cursor_by, cursor_offset};

#[derive(Default)]
pub struct EditorImpl {
    state: State,
}

impl Editor for EditorImpl {
    fn apply(&mut self, event: InputEvent) {
        let mut text_changed = false;
        match event {
            InputEvent::Ready => {}
            InputEvent::MoveCursor(d, a) => do_move_cursor(&mut self.state, d, a),
            InputEvent::InsertText(text) => {
                text_changed = true;
                if text == "\x08" {
                    do_backspace(&mut self.state);
                } else {
                    do_insert(&mut self.state, text);
                }
            }
            InputEvent::OpenFile(path) => {
                text_changed = true;
                do_open_file(&mut self.state, path);
            }
            InputEvent::SaveFile => do_save_file(&mut self.state),
        }
        let text: String = self.state.text.clone().into();
        if text_changed || self.state.file.is_none() {
            let file = LANG_FALL.parse(text.clone());
            self.state.file = Some(file);
        }

        modify_state(&mut self.state,
                     |state| {
                         let file = state.file.as_ref().unwrap();
                         highlight(file)
                     },
                     |state, spans| state.spans = spans
        );

        modify_state(&mut self.state,
                     |state| {
                         let mut off = cursor_offset(state);
                         let mut cursor = Cursor::new(&state.text, off);
                         while let Some(c) = cursor.next_codepoint() {
                             if !(c == ' ' || c == '\n') {
                                 break
                             }
                             off += c.len_utf8()
                         }
                         let file = state.file.as_ref().unwrap();
                         context(&file, off)
                     },
                     |state, syntax_tree| state.syntax_tree = syntax_tree
        );
    }

    fn render(&self) -> ViewStateReply {
        let mut result = ViewStateReply::new();
        render_lines(&mut result, self.state.text.lines_raw(0, self.state.text.len()), &self.state.spans);
        result.cursorX = self.state.cursor.x as i32;
        result.cursorY = self.state.cursor.y as i32;
        result.syntax_tree = self.state.syntax_tree.clone();
        if let Some(ref file) = self.state.file {
            result.lexing_time_nanos = ElapsedDuration::new(file.stats().lexing_time).nanos() as i64;
            result.parsing_time_nanos = ElapsedDuration::new(file.stats().parsing_time).nanos() as i64;
        }
        if let Some(ref path) = self.state.file_path {
            result.file = path.display().to_string();
        }
        result.dirty = self.state.dirty;
        result
    }
}

fn render_lines<'a, L: Iterator<Item=CowStr<'a>>>(reply: &mut ViewStateReply, mut lines: L, spans: &[(u32, u32, &'static str)]) {
    let mut curr_global_offset = 0;

    let mut line_off;
    let mut curr_line: CowStr;
    macro_rules! next_line {
        () => {
            if let Some(line) = lines.next() {
                curr_line = line;
                line_off = 0;
                let _ = line_off;
            } else {
                return
            }
        }
    }
    next_line!();

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
            let effective_l = ::std::cmp::min(l, curr_line.len() - line_off);
            l -= effective_l;
            push_range!(&curr_line[line_off .. line_off + effective_l]);
            line_off += effective_l;
            if line_off == curr_line.len() {
                push_line!();
                next_line!();
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
        push_range!(curr_line.as_ref());
        push_line!();
        next_line!();
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
    let off = cursor_offset(state);
    if off == 0 { return }
    state.text.edit_str(off - 1, off, "");
    state.dirty = true;
    move_cursor_by(state, -1, 0);
}

fn do_insert(state: &mut State, text: String) {
    let dx = text.len();
    let off = cursor_offset(state);
    state.text.edit_str(off, off, &text);
    state.dirty = true;
    move_cursor_by(state, dx as i32, 0);
}

fn do_open_file(state: &mut State, path: PathBuf) {
    let text = file::get_text(&path).unwrap();
    state.text = From::from(text);
    state.dirty = false;
    state.file_path = Some(path)
}

fn do_save_file(state: &mut State) {
    if !state.dirty {
        return
    }
    if let Some(ref path) = state.file_path {
        file::put_text(path, String::from(state.text.clone()))
            .unwrap();
        state.dirty = false
    }
}

fn context(file: &File, offset: usize) -> String {

    fn go(lang: &Language, path: &[Node], level: usize, buff: &mut String) {
        assert!(path.len() > 0);
        if path.len() == 1 {
            return
        }
        let node = path[0];
        let next = path[1];
        for child in node.children() {
            if child.ty() == WHITESPACE {
                continue
            }

            let name = lang.node_type_info(child.ty()).name;
            for _ in 0..level {
                buff.push_str("  ");
            }
            if child == next {
                buff.push_str(">");
            } else {
                buff.push_str(" ");
            }
            buff.push_str(name);

            buff.push_str("\n");

            if child == next {
                go(lang, &path[1..], level + 1, buff);
            }
        }
    }
    let root = file.root();
    let mut result = String::new();
    let offset = TextOffset::in_range(root.range(), offset)
        .expect("Broken offset");
    let leaf = if let Some(leaf) = find_leaf_at_offset(root, offset).right_biased() {
        leaf
    } else {
        return String::new()
    };
    let mut path: Vec<Node> = ancestors(leaf).collect();
    path.reverse();
    go(&file.language(), &path, 0, &mut result);
    result
}

fn modify_state<R, F, C>(state: &mut State, f: F, commit: C)
    where F: FnOnce(&State) -> R + ::std::panic::UnwindSafe,
          C: FnOnce(&mut State, R) {
    let changes = {
        let s: &State = &*state;
        let s = ::std::panic::AssertUnwindSafe(s);
        match ::std::panic::catch_unwind(move || f(*s)) {
            Ok(changes) => changes,
            Err(_) => return
        }
    };
    commit(state, changes)
}
