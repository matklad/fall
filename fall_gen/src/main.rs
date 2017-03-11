extern crate clap;
extern crate fall_gen;


use std::error::Error;
use std::io::prelude::*;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;
use std::fs::File;

use clap::{App, Arg};

use fall_gen::Grammar;

fn main() {
    let matches = App::new("Fall parser generator")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();
    let file = Path::new(matches.value_of("file").unwrap());
    let return_code = if let Err(e) = main_inner(file) {
        writeln!(::std::io::stderr(), "Error occurred: {}", e).unwrap();
        101
    } else {
        0
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

//    let result = parse(&input).generate();
    let result = fall_gen::parse(&input)?.generate();

    let mut out_file = File::create(file.with_extension("rs"))?;
    write!(out_file, "{}", result)?;
    Ok(())
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
