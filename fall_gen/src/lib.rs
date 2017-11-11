extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate tera;
extern crate file;

extern crate fall_tree;
extern crate fall_parse;
extern crate lang_fall;

use std::error::Error;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::fs;
use std::process::{Command, Stdio};
use fall_tree::{File, AstNode};


mod util;
mod generate;

pub fn generate(analysis: &lang_fall::Analysis) -> generate::Result<String> {
    generate::generate(analysis)
}

pub struct TestRenderer;

impl TestRenderer {
    pub fn render_one(&mut self, file: &File, test: usize) -> String {
        let file = lang_fall::FallFile::new(file.root());
        let text = match file.tests().nth(test).and_then(|t| t.contents()) {
            None => return String::new(),
            Some(text) => text.to_string()
        };

        match self.render_all(file.node().text().to_string(), Some(text)) {
            Ok(result) => result,
            Err(e) => format!("{}", e),
        }
    }

    pub fn render_all(&mut self, grammar: String, test: Option<String>) -> Result<String, Box<Error>> {
        let file = lang_fall::editor_api::analyse(grammar);
        let parser = match file.analyse(generate) {
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
                    let file = syntax::language().parse(test.to_owned());
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
            let mut output = String::from_utf8(build.stderr).unwrap_or_default();
            output += "\n\n";
            output += &String::from_utf8(build.stdout).unwrap_or_default();
            return Ok(output);
        }

        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(&toml)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let tests: String = match test {
            Some(test) => test,

            None => file.analyse(|a| a.ast().tests()
                .filter_map(|t| t.contents())
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("\n***###***\n")
            )
        };

        {
            let stdin = child.stdin.as_mut().unwrap();
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
