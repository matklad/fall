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

pub use crate::text_unit::{TextUnit, tu};
pub use crate::text_range::TextRange;
pub use crate::text_buf::TextBuf;
pub use crate::text::Text;
pub use crate::text_edit::{TextEdit, TextEditOp, TextEditBuilder};
pub use crate::text_slice::TextSuffix;

