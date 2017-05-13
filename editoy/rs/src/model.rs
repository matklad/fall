use std::path::PathBuf;
pub use ediproto::Direction;
pub use ediproto::Amount;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, Clone)]
pub struct ViewState {
    pub lines: Vec<String>,
    pub cursor: GridPosition,
}

#[derive(Debug)]
pub enum InputEvent {
    Ready,
    MoveCursor(Direction, Amount),
    InsertText(String),
    OpenFile(PathBuf),
}

pub trait Editor: Default {
    fn apply(&mut self, event: InputEvent) -> ViewState;
}
