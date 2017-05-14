use std::path::PathBuf;
pub use ediproto::{Direction, Amount, ViewStateReply};

#[derive(Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

type Spans = Vec<(u32, u32, &'static str)>;

#[derive(Debug, Default, Clone)]
pub struct State {
    pub lines: Vec<String>,
    pub cursor: GridPosition,
    pub syntax_tree: String,
    pub spans: Spans,
}

#[derive(Debug)]
pub enum InputEvent {
    Ready,
    MoveCursor(Direction, Amount),
    InsertText(String),
    OpenFile(PathBuf),
}

pub trait Editor: Default {
    fn apply(&mut self, event: InputEvent);
    fn render(&self) -> ViewStateReply;
}
