use model::{ViewState, GridPosition};
use std::cmp::{max, min};

pub fn current_line(state: &ViewState) -> Option<&str> {
    match state.lines.get(state.cursor.y as usize) {
        Some(line) => Some(&line),
        None => None
    }
}

pub fn move_cursor_by(state: &mut ViewState, dx: i32, dy: i32) {
    fn m(p: u32, d: i32, m: usize) -> u32 {
        min(
            max(p as i32 + d, 0) as u32,
            m as u32
        )
    }
    let GridPosition { x, y } = state.cursor;
    let mx = current_line(state).unwrap_or("").len();
    let my = state.lines.len();
    state.cursor = GridPosition { x: m(x, dx, mx), y: m(y, dy, my) }
}
