extern crate clap;
extern crate file;
extern crate fall_gen;
extern crate lang_fall;
extern crate fall_tree;

use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

use clap::{App, Arg};

enum Task {
    Generate(PathBuf),
    Examples(PathBuf)
}

fn main() {
    let matches = App::new("Fall parser generator")
        .arg(Arg::with_name("grammar").index(1).required(true))
        .arg(Arg::with_name("tests").long("tests"))
        .get_matches();
    let grammar = PathBuf::from(matches.value_of("grammar").unwrap());
    let command = if matches.is_present("tests") {
        Task::Examples(grammar)
    } else {
        Task::Generate(grammar)
    };

    let return_code = if let Err(e) = main_inner(command) {
        writeln!(::std::io::stderr(), "Error occurred: {}", e).unwrap();
        101
    } else {
        0
    };
    std::process::exit(return_code)
}

fn main_inner(command: Task) -> Result<(), Box<Error>> {
    let mut renderer = fall_gen::TestRenderer;
    match command {
        Task::Generate(grammar) => {
            let input = file::get_text(&grammar)?;
            let file = lang_fall::lang_fall().parse(input);
            let ast = lang_fall::ast(&file);
            let result = fall_gen::generate(ast)?;
            file::put_text(grammar.with_extension("rs"), result)?;
        }
        Task::Examples(grammar) => {
            let input = file::get_text(&grammar)?;
            let result = renderer.render_all(input, None)?;
            file::put_text(grammar.with_extension("txt"), result)?;
        }
    }
    Ok(())
}
