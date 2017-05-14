use model::State;
use std::cmp::{max, min};

pub fn current_line(state: &State) -> Option<&str> {
    match state.lines.get(state.cursor.y as usize) {
        Some(line) => Some(&line),
        None => None
    }
}

pub fn move_cursor_by(state: &mut State, dx: i32, dy: i32) {
    fn m(p: u32, d: i32, m: usize) -> u32 {
        min(
            max(p as i32 + d, 0) as u32,
            m as u32
        )
    }
    let my = state.lines.len();
    state.cursor.y = m(state.cursor.y, dy, my);
    //XXX: current line may have changed
    let mx = current_line(state).unwrap_or("").len();
    state.cursor.x = m(state.cursor.x, dx, mx);
}

pub fn collect_text(state: &State) -> String {
    let mut result = String::new();
    for line in state.lines.iter() {
        result += line;
    }
    result
}
