use std::path::Path;

use file;
use {Editor, InputEvent, ViewState, GridPosition, Direction, Amount};

mod utils;

use self::utils::move_cursor_by;


//#[derive(Default)]
pub struct EditorImpl {
    state: ViewState,
}

impl Default for EditorImpl {
    fn default() -> Self {
        EditorImpl {
            state: ViewState {
                lines: vec!["Hello".to_owned(); 32],
                cursor: Default::default(),
            }
        }
    }
}

impl Editor for EditorImpl {
    fn apply(&mut self, event: InputEvent) -> ViewState {
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
        self.state.clone()
    }
}

fn do_move_cursor(state: &mut ViewState, direction: Direction, amount: Amount) {
    let (am_vert, am_hor): (i32, i32) = match amount {
        Amount::Char => (1, 1),
        Amount::Page => (10, ::std::i32::MAX / 2),
    };

    let (dx, dy) = match direction {
        Direction::UP => (0i32, -1 * am_vert),
        Direction::LEFT => (-1 * am_hor, 0),
        Direction::DOWN => (0, 1 * am_vert),
        Direction::RIGHT => (1 * am_hor, 0),
    };

    move_cursor_by(state, dx, dy)
}

fn do_backspace(state: &mut ViewState) {
    delete_backwards(state);
    move_cursor_by(state, -1, 0);
}

fn do_insert(state: &mut ViewState, text: String) {
    let dx = text.len();
    insert_text(state, text);
    move_cursor_by(state, dx as i32, 0);
}

fn do_open_file(state: &mut ViewState, path: &Path) {
    let text = file::get_text(path).unwrap();
    state.lines = text.lines().map(|l| l.to_owned()).collect();
}

fn insert_text(state: &mut ViewState, text: String) {
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


fn delete_backwards(state: &mut ViewState) {
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
