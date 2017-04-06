extern crate clap;
extern crate fall_gen;

use std::error::Error;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::fs::File;

use clap::{App, Arg};

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

    let result = if file.extension().unwrap() == "ast" {
        fall_gen::gast::generate(&input)
    } else {
        let file = fall_gen::FallFile::parse(input);
        file.generate()
    };

    let mut out_file = File::create(file.with_extension("rs"))?;
    write!(out_file, "{}", result)?;
    Ok(())
}
