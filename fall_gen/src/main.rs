extern crate clap;

use clap::{App, Arg};

use std::error::Error;
use std::io::prelude::*;
use std::io::Write;
use std::fmt::Write as FmtWrite;
use std::iter::FromIterator;
use std::ascii::AsciiExt;
use std::path::Path;
use std::fs::File;

fn main() {
    let mathces = App::new("Fall parser generator")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let file = Path::new(mathces.value_of("file").unwrap());

    let return_code = if let Err(e) = main_inner(file) {
        println!("Error occurred: {}", e);
        101
    } else {
        1
    };

    std::process::exit(return_code)
}

fn main_inner(file: &Path) -> Result<(), Box<Error>> {
    let input = {
        let mut file = File::open(file)?;
        let mut buff = String::new();
        file.read_to_string(&mut buff)?;
        buff
    };

    let grammar = parse(&input);
    let result = generate(&grammar);

    let mut out_file = File::create(file.with_extension("rs"))?;
    write!(out_file, "{}", result)?;
    Ok(())
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
