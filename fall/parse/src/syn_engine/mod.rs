mod token_seq;

pub mod events;
/// In the "black" part, we totally skip all whitespace tokens.
mod black;
/// In the "white" part, we reconstruct proper whitespace.
mod white;

pub use self::token_seq::{BlackTokens, TokenSeq, BlackIdx};
pub use self::black::{BlackNode, parse_black};
pub use self::white::into_white;
