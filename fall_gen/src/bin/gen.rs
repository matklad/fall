extern crate clap;
extern crate file;
extern crate fall_gen;
extern crate lang_fall;
extern crate fall_tree;

use std::error::Error;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::fs;
use std::process::{Command, Stdio};

use clap::{App, Arg};

enum Task {
    Generate(PathBuf),
    Examples(PathBuf)
}

fn main() {
    let matches = App::new("Fall parser generator")
        .arg(Arg::with_name("grammar").index(1).required(true))
        .arg(Arg::with_name("examples").long("examples"))
        .get_matches();
    let grammar = PathBuf::from(matches.value_of("grammar").unwrap());
    let command = if matches.is_present("examples") {
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
    match command {
        Task::Generate(grammar) => {
            let input = file::get_text(&grammar)?;
            let file = lang_fall::LANG_FALL.parse(input);
            let ast = lang_fall::ast(&file);
            let result = fall_gen::generate(ast)?;
            file::put_text(grammar.with_extension("rs"), result)?;
        }
        Task::Examples(grammar) => {
            let input = file::get_text(&grammar)?;
            let result = render_examples(input)?;
            file::put_text(grammar.with_extension("txt"), result)?;
        }
    }
    Ok(())
}


fn render_examples(grammar: String) -> Result<String, Box<Error>> {
    let file = lang_fall::LANG_FALL.parse(grammar);
    let ast = lang_fall::ast(&file);
    let parser = fall_gen::generate(ast)?;
    let example = match ast.examples().next() {
        Some(ex) => ex.contents().to_string(),
        None => return Ok("".to_owned())
    };

    let base_dir = base_directory()?;
    let syntax = base_dir.join("src").join("syntax.rs");
    file::put_text(&syntax, parser)?;
    let toml = base_dir.join("Cargo.toml");
    put_text_if_changed(&toml, format!(r##"
        [package]
        name = "fall_examples"
        version = "0.1.0"
        authors = []

        [workspace]

        [dependencies]
        regex = "0.2"
        serde_json = "1.*"
        lazy_static = "0.2"

        fall_tree = {{ path = "{fall_dir}/fall_tree" }}
        fall_parse = {{ path = "{fall_dir}/fall_parse" }}
    "##, fall_dir = fall_dir().display()))?;
    put_text_if_changed(&base_dir.join("src").join("main.rs"), r##"
        extern crate regex;
        extern crate fall_tree;
        extern crate fall_parse;
        #[macro_use]
        extern crate lazy_static;
        extern crate serde_json;
        mod syntax;

        use std::io::Read;

        fn main() {
            let mut input = String::new();
            ::std::io::stdin().read_to_string(&mut input).unwrap();
            let file = syntax::LANG.parse(input);
            println!("{}", fall_tree::dump_file(&file));
        }
    "##)?;

    let build = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(&toml)
        .spawn()?
        .wait_with_output()?;

    if !build.status.success() {
        return Ok(String::from_utf8(build.stderr).unwrap());
    }

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(&toml)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(example.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }

    let rendered = child.wait_with_output()?;
    assert!(rendered.status.success());
    let out = String::from_utf8(rendered.stdout).unwrap();
    Ok(out)
}

fn base_directory() -> Result<PathBuf, Box<Error>> {
    let result = ::std::env::temp_dir().join("fall-examples");
    fs::create_dir_all(&result)?;
    fs::create_dir_all(&result.join("src"))?;
    Ok(result)
}

fn fall_dir() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir).parent().unwrap().to_owned()
}

fn put_text_if_changed(path: &Path, text: &str) -> ::std::io::Result<()> {
    if path.exists() {
        let old_text = file::get_text(path)?;
        if old_text == text {
            return Ok(())
        }
    }
    file::put_text(path, text)
}
