extern crate serde;

mod text_unit;
mod text_range;
mod text_buf;
mod text;

pub use self::text_unit::{TextUnit, tu};
pub use self::text_range::TextRange;
pub use self::text_buf::TextBuf;
pub use self::text::Text;

