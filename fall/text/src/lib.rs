extern crate serde;
extern crate rand;
extern crate itertools;
extern crate proptest;

mod text_unit;
mod text_range;
mod text_buf;
mod text;
mod text_edit;
mod text_slice;
pub mod prop;

pub use text_unit::{TextUnit, tu};
pub use text_range::TextRange;
pub use text_buf::TextBuf;
pub use text::Text;
pub use text_edit::{TextEdit, TextEditOp, TextEditBuilder};
pub use text_slice::TextSuffix;

