use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fmt::Write;
use std::ascii::AsciiExt;

fn main() {
    let return_code = if let Err(e) = main_inner() {
        println!("Error occurred: {}", e);
        101
    } else {
        1
    };

    std::process::exit(return_code)
}

fn main_inner() -> Result<(), Box<Error>> {
    let input = read_stdin()?;
    let result = generate(&input);
    println!("{}", result);
    Ok(())
}

fn read_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn generate(input: &str) -> String {
    let mut result = String::new();
    result.push_str("use fall::{NodeType, NodeTypeInfo};\n");
    result.push_str("pub use fall::{ERROR, WHITESPACE};\n\n");
    for (i, t) in input.split_whitespace().enumerate() {
        writeln!(result, "pub const {:10}: NodeType = NodeType({});", scream(t), 100 + i)
            .unwrap();
    }

    result.push_str("\npub fn register_node_types() {");
    for t in input.split_whitespace() {
        writeln!(result, "    {}.register(NodeTypeInfo {{ name: {:?} }});", scream(t), scream(t))
            .unwrap();
    }

    result.push_str("}\n");

    result
}

fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}
