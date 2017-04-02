#[macro_use]
extern crate serde_derive;

extern crate jsonrpc_core;
extern crate jsonrpc_minihttp_server;

use std::collections::HashMap;

use jsonrpc_core::{IoHandler, Value, Params, to_value};
use jsonrpc_minihttp_server::{ServerBuilder};

#[derive(Serialize)]
struct Response {
    spans: Vec<(usize, usize, &'static str)>
}

fn main() {
    let mut io = IoHandler::new();
    io.add_method("colors", |params: Params| {
        let (text, ): (String, ) = params.parse().unwrap();
        let r = to_value(Response { spans: vec![(1, 8, "ident")] }).unwrap();
        Ok(r)
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:9292".parse().unwrap())
        .unwrap();

    println!("Starting");
    server.wait().unwrap();
}
