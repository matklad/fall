extern crate fall;

use fall::Node;

pub use generator::Grammar;

mod parser;

mod generator;

use self::parser::grammar::*;

type Error = ();

pub fn parse(text: &str) -> Result<Grammar, Error> {
    let file = parser::parse(text.to_owned());
    let root = file.root();

    let nodes = root.children().single_of_type(NODES_DEF).ok_or(())?;
    let tokenizer = root.children().single_of_type(TOKENIZER_DEF).ok_or(())?;

    let node_types = nodes
        .children().many_of_type(IDENT)
        .map(|n| n.text().to_owned())
        .collect();


    let rules = tokenizer
        .children().many_of_type(RULE)
        .map(|rule| -> Result<(String, String), Error> {
            let id = rule.children().single_of_type(IDENT).ok_or(())?
                .text().to_owned();
            let pat = rule.children().single_of_type(STRING).ok_or(())?
                .text().to_owned();
            Ok((id, pat))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(Grammar { node_types: node_types, tokenizer_rules: rules })
}
