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
            let result = render_testss(input)?;
            file::put_text(grammar.with_extension("txt"), result)?;
        }
    }
    Ok(())
}


fn render_testss(grammar: String) -> Result<String, Box<Error>> {
    let file = lang_fall::LANG_FALL.parse(grammar);
    let ast = lang_fall::ast(&file);
    let parser = match fall_gen::generate(ast) {
        Ok(parser) => parser,
        Err(e) => return Ok(format!("error:\n{}", e))
    };
    let base_dir = base_directory()?;

    let syntax = base_dir.join("src").join("syntax.rs");
    put_text_if_changed(&syntax, &parser)?;

    let toml = base_dir.join("Cargo.toml");
    put_text_if_changed(&toml, &format!(r##"
        [package]
        name = "fall_tests_rendering"
        version = "0.1.0"
        authors = []

        [workspace]

        [dependencies]
        fall_tree = {{ path = "{fall_dir}/fall_tree" }}
        fall_parse = {{ path = "{fall_dir}/fall_parse" }}
    "##, fall_dir = fall_dir().display()))?;

    put_text_if_changed(&base_dir.join("src").join("main.rs"), r##"
        #![allow(warnings)]
        extern crate fall_tree;
        extern crate fall_parse;
        mod syntax;

        use std::io::Read;

        fn main() {
            let mut input = String::new();
            ::std::io::stdin().read_to_string(&mut input).unwrap();
            for test in input.split("\n***###***\n") {
                let file = syntax::LANG.parse(test.to_owned());
                println!("{}\n", fall_tree::dump_file(&file));
            }
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

    let tests: String = ast.tests()
        .filter_map(|t| t.contents())
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join("\n***###***\n");

    {
        let mut stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(tests.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }

    let rendered = child.wait_with_output()?;
    assert!(rendered.status.success());
    let mut result = String::new();
    if !rendered.stderr.is_empty() {
        result += &::std::str::from_utf8(&rendered.stderr).unwrap();
        result += "\n\n";
    }
    result += &::std::str::from_utf8(&rendered.stdout).unwrap();
    Ok(result)
}

fn base_directory() -> Result<PathBuf, Box<Error>> {
    let result = ::std::env::temp_dir().join("fall-tests");
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
            return Ok(());
        }
    }
    file::put_text(path, text)
}
