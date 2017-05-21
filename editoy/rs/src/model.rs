use std::path::PathBuf;
use fall_tree::File;
pub use ediproto::{Direction, Amount, ViewStateReply};
use xi_rope::rope::Rope;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

pub type Spans = Vec<(u32, u32, &'static str)>;
pub type CowStr<'a> = ::std::borrow::Cow<'a, str>;


#[derive(Default)]
pub struct State {
    pub file: Option<File>,
    pub text: Rope,
    pub cursor: GridPosition,
    pub syntax_tree: String,
    pub spans: Spans,

    pub file_path: Option<PathBuf>,
    pub dirty: bool,
}

#[derive(Debug)]
pub enum InputEvent {
    Ready,
    MoveCursor(Direction, Amount),
    InsertText(String),
    OpenFile(PathBuf),
    SaveFile,
}

pub trait Editor: Default {
    fn apply(&mut self, event: InputEvent);
    fn render(&self) -> ViewStateReply;
}
