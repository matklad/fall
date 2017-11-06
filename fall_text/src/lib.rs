extern crate serde;

mod text_unit;
mod text_range;
mod text;

pub use self::text_unit::{TextUnit, tu};
pub use self::text_range::TextRange;
pub use self::text::Text;

