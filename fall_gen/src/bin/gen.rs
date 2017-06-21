extern crate clap;
extern crate file;
extern crate fall_gen;
extern crate lang_fall;

use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::fs;

use clap::{App, Arg};

enum Command {
    Generate(PathBuf),
    Examples(PathBuf)
}

fn main() {
    let matches = App::new("Fall parser generator")
        .arg(Arg::with_name("grammar").index(1).required(true))
        .arg(Arg::with_name("--examples"))
        .get_matches();
    let grammar = PathBuf::from(matches.value_of("grammar").unwrap());
    let command = if matches.is_present("examples") {
        Command::Examples(grammar)
    } else {
        Command::Generate(grammar)
    };

    let return_code = if let Err(e) = main_inner(command) {
        writeln!(::std::io::stderr(), "Error occurred: {}", e).unwrap();
        101
    } else {
        0
    };
    std::process::exit(return_code)
}

fn main_inner(command: Command) -> Result<(), Box<Error>> {
    match command {
        Command::Generate(grammar) => {
            let input = file::get_text(&grammar)?;
            let result = grammar_to_parser(input)?;
            file::put_text(grammar.with_extension("rs"), result)?;
        }
        Command::Examples(grammar) => {
            let input = file::get_text(&grammar)?;
            let result = render_examples(input)?;
            file::put_text(grammar.with_extension("txt"), result)?;
        }
    }
    Ok(())
}

fn grammar_to_parser(grammar: String) -> Result<String, Box<Error>> {
    let file = lang_fall::LANG_FALL.parse(grammar);
    let ast = lang_fall::ast(&file);
    let result = fall_gen::generate(ast)?;
    Ok(result)
}


fn render_examples(grammar: String) -> Result<String, Box<Error>> {
    let parser = grammar_to_parser(grammar)?;
    let base_dir = base_directory()?;
    let syntax = base_dir.join("src").join("syntax.rs");
    file::put_text(&syntax, parser)?;
    Ok("Hello".to_owned())
}

fn base_directory() -> Result<PathBuf, Box<Error>> {
    let result = PathBuf::from("./fall-examples");
    fs::create_dir(&result)?;
    Ok(result)
}

