extern crate ediproto;
extern crate futures;
extern crate grpc;
extern crate file;
extern crate env_logger;

mod rpc;
mod model;
mod editor;

pub use model::{Editor, Direction, Amount, GridPosition, InputEvent, ViewState};
use editor::EditorImpl;

use ediproto::EditorServer;
use rpc::EditorServerImpl;

fn main() {
    env_logger::init().unwrap();
    let _server = EditorServer::new(
        "[::]:9292",
        Default::default(),
        EditorServerImpl::<EditorImpl>::default()
    );
    println!("STARTED EDIBACK on 127.0.0.1::9292");
    loop {
        ::std::thread::park()
    }
}
