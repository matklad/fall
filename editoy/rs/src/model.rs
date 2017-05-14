use std::path::PathBuf;
pub use ediproto::{Direction, Amount, ViewStateReply};
use xi_rope::rope::Rope;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

pub type Spans = Vec<(u32, u32, &'static str)>;
pub type CowStr<'a> = ::std::borrow::Cow<'a, str>;


#[derive(Default, Clone)]
pub struct State {
    pub text: Rope,
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
