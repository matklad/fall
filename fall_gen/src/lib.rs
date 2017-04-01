extern crate fall_tree;
extern crate fall_parse;

use fall_tree::{dump_file, Node};
use fall_tree::search::{child_of_type, child_of_type_exn, children_of_type};

mod syntax;

use self::syntax::grammar::*;

mod generator;

pub use generator::{Grammar, LexRule, SynRule, Alt, Part};

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
    dump_file(&syntax::parse(text.to_owned()))
}

pub fn parse(text: &str) -> Result<Grammar, Error> {
    let file = syntax::parse(text.to_owned());
    let root = file.root();

    let nodes = child_of_type_exn(root, NODES_DEF);
    let tokenizer = child_of_type_exn(root, TOKENIZER_DEF);
    let syn_rules = children_of_type(root, RULE_DEF);

    let node_types = children_of_type(nodes, IDENT)
        .map(|n| n.text().to_owned())
        .collect();

    let lex_rules = children_of_type(tokenizer, TOKEN_DEF)
        .map(|rule| -> Result<LexRule, Error> {
            let ty = child_of_type(rule, IDENT)
                .ok_or(error("Missing name in rule"))?
                .text().to_owned();
            let mut pats = children_of_type(rule, STRING);
            let re = pats.next()
                .ok_or(error(format!("Missing pattern in rule {:?}", rule.text())))?;
            let f = pats.next().map(|n| {
                let t = n.text();
                t[1..t.len() - 1].to_owned()
            });
            Ok(LexRule { ty: ty, re: re.text().to_owned(), f: f })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let syn_rules = syn_rules.map(parse_syn_rule).collect();

    let g = Grammar {
        node_types: node_types,
        lex_rules: lex_rules,
        syn_rules: syn_rules
    };

    Ok(g)
}


fn parse_syn_rule(rule: Node) -> SynRule {
    SynRule {
        name: child_of_type_exn(rule, IDENT).text().to_string(),
        alts: children_of_type(rule, ALT).map(parse_alt).collect(),
    }
}

fn parse_alt(alt: Node) -> Alt {
    Alt {
        parts: children_of_type(alt, PART)
            .filter(|n| n.text() != "<commit>")
            .map(parse_part).collect(),
        commit: children_of_type(alt, PART).position(|n| n.text() == "<commit>"),
    }
}

fn parse_part(node: Node) -> Part {
    if let Some(id) = child_of_type(node, IDENT) {
        Part::Rule(id.text().to_owned())
    } else {
        Part::Rep(parse_alt(child_of_type_exn(node, ALT)))
    }

}
