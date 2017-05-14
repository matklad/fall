use model::{State, CowStr};
use xi_rope::rope::{LinesMetric, BaseMetric};
use std::cmp::{max, min};

pub fn current_line(state: &State) -> CowStr {
    let line_idx = state.cursor.y as usize;
    let line_start = state.text.convert_metrics::<LinesMetric, BaseMetric>(line_idx);
    let line_end = state.text.convert_metrics::<LinesMetric, BaseMetric>(line_idx + 1);
    state.text.lines(line_start, line_end).next().unwrap()
}

pub fn cursor_offset(state: &State) -> usize {
    let line_start = state.text.convert_metrics::<LinesMetric, BaseMetric>(state.cursor.y as usize);
    line_start + state.cursor.x as usize
}

pub fn move_cursor_by(state: &mut State, dx: i32, dy: i32) {
    fn m(p: u32, d: i32, m: usize) -> u32 {
        min(
            max(p as i32 + d, 0) as u32,
            m as u32
        )
    }
    let my = state.text.measure::<LinesMetric>();
    state.cursor.y = m(state.cursor.y, dy, my - 1);
    //XXX: current line may have changed
    let mx = current_line(state).len();
    state.cursor.x = m(state.cursor.x, dx, mx);
}
