extern crate clap;
extern crate file;
extern crate fall_gen;
extern crate lang_fall;

use std::error::Error;
use std::io::Write;
use std::path::Path;

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

fn main_inner(path: &Path) -> Result<(), Box<Error>> {
    let input = file::get_text(path)?;

    let file = lang_fall::LANG_FALL.parse(input);
    let ast = lang_fall::ast(&file);
    let result = fall_gen::generate(ast)?;

    file::put_text(path.with_extension("rs"), result)?;
    Ok(())
}
