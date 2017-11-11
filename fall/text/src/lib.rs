extern crate serde;

mod text_unit;
mod text_range;
mod text_buf;
mod text;
mod text_edit;

pub use text_unit::{TextUnit, tu};
pub use text_range::TextRange;
pub use text_buf::TextBuf;
pub use text::Text;
pub use text_edit::{TextEdit, TextEditOp, TextEditBuilder};

