extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;


pub mod editor;
pub mod editor_grpc;

pub use editor::{InputEvent, EventReply, ViewStateRequest, ViewStateReply, Direction, Amount};
pub use editor_grpc::{Editor, EditorServer};
