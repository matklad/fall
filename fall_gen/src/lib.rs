extern crate fall_tree;
extern crate fall_parse;

pub use generator::{Grammar, LexRule};

mod parser;

mod generator;

use self::parser::grammar::*;

#[derive(Debug)]
pub struct Error(String);

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        &self.0
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Syntax error: ")?;
        f.write_str(&self.0)
    }
}

fn error<S: Into<String>>(msg: S) -> Error { Error(msg.into()) }

pub fn debug(text: &str) -> String {
    parser::parse(text.to_owned()).dump()
}

pub fn parse(text: &str) -> Result<Grammar, Error> {
    let file = parser::parse(text.to_owned());
    let root = file.root();

    let nodes = root.children().single_of_type(NODES_DEF)
        .ok_or(error("No `nodes = {}`"))?;

    let tokenizer = root.children().single_of_type(TOKENIZER_DEF)
        .ok_or(error("No `tokenizer = {}`"))?;

    let node_types = nodes
        .children().many_of_type(IDENT)
        .map(|n| n.text().to_owned())
        .collect();

    let rules = tokenizer
        .children().many_of_type(TOKEN_DEF)
        .map(|rule| -> Result<LexRule, Error> {
            let ty = rule.children().single_of_type(IDENT)
                .ok_or(error("Missing name in rule"))?
                .text().to_owned();
            let mut pats = rule.children().many_of_type(STRING);
            let re = pats.next()
                .ok_or(error(format!("Missing pattern in rule {:?}", rule.text())))?;
            let f = pats.next().map(|n| {
                let t = n.text();
                t[1..t.len() - 1].to_owned()
            });
            Ok(LexRule { ty: ty, re: re.text().to_owned(), f: f })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let g = Grammar { node_types: node_types, lex_rules: rules };

    Ok(g)
}
