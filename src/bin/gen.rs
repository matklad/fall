use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
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
    let grammar = parse(&input);
    let result = generate(&grammar);
    println!("{}", result);
    Ok(())
}

fn read_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn generate(g: &Grammar) -> String {
    let mut result = String::new();
    result.push_str("use std::sync::{Once, ONCE_INIT};\n");
    result.push_str("use fall::{NodeType, NodeTypeInfo};\n");
    result.push_str("use fall::builder::Rule;\n");
    result.push_str("pub use fall::{ERROR, WHITESPACE};\n\n");
    for (i, t) in g.node_types.iter().enumerate() {
        writeln!(result, "pub const {:10}: NodeType = NodeType({});", scream(t), 100 + i)
            .unwrap();
    }

    result.push_str("\npub fn register_node_types() {\n");
    result.push_str("    static REGISTER: Once = ONCE_INIT;\n");
    result.push_str("    REGISTER.call_once(||{\n");
    for t in g.node_types.iter() {
        writeln!(result, "        {}.register(NodeTypeInfo {{ name: {:?} }});", scream(t), scream(t))
            .unwrap();
    }
    result.push_str("    });\n");
    result.push_str("}\n");

    result.push_str("\npub const TOKENIZER: &'static [Rule] = &[\n");
    for &(ref ty, ref re) in g.tokenizer_rules.iter() {
        result.push_str("    ");
        result.push_str(&format!("Rule {{ ty: {}, re: {} }},", scream(ty), re));
        result.push_str("\n");
    }
    result.push_str("];\n");

    result
}

fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}


struct Grammar {
    node_types: Vec<String>,
    tokenizer_rules: Vec<(String, String)>,
}

fn parse(text: &str) -> Grammar {
    let tokens_start = text.find('{').unwrap();
    let tokens_end = text.find('}').unwrap();
    let tokens = &text[tokens_start + 1..tokens_end].trim();
    let rest = &text[tokens_end + 1..];
    let rules_start = rest.find('{').unwrap();
    let rules_end = rest.rfind('}').unwrap();
    let rules = &rest[rules_start + 1..rules_end].trim();

    Grammar {
        node_types: tokens.split_whitespace().map(|s| s.to_owned()).collect(),
        tokenizer_rules: rules.lines()
            .map(|line| line.trim())
            .map(|line| {
                let words = Vec::from_iter(line.split_whitespace());
                assert_eq!(words.len(), 2);
                (words[0], words[1])
            })
            .map(|(token, rule)| {
                (token.to_owned(), rule.to_owned())
            }).collect(),
    }
}
