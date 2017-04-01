use fall_tree::Node;
use fall_tree::search::{child_of_type, child_of_type_exn, children_of_type};

use syntax::*;

#[derive(Debug)]
pub struct Grammar {
    pub node_types: Vec<String>,
    pub lex_rules: Vec<LexRule>,
    pub syn_rules: Vec<SynRule>,
}

#[derive(Debug)]
pub struct LexRule {
    pub ty: String,
    pub re: String,
    pub f: Option<String>,
}

#[derive(Debug)]
pub struct SynRule {
    pub name: String,
    pub alts: Vec<Alt>,
}

#[derive(Debug)]
pub struct Alt {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub enum Part {
    Rule(String),
    Call(String, Vec<Alt>)
}


#[derive(Debug)]
pub struct LiftError(String);

impl ::std::error::Error for LiftError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl ::std::fmt::Display for LiftError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Syntax error: ")?;
        f.write_str(&self.0)
    }
}

fn error<S: Into<String>>(msg: S) -> LiftError { LiftError(msg.into()) }

type Result<T> = ::std::result::Result<T, LiftError>;


pub fn lift(node: Node) -> Result<Grammar> {
    let node_types = child_of_type_exn(node, NODES_DEF);
    let lex_rules = child_of_type_exn(node, TOKENIZER_DEF);
    let syn_rules = children_of_type(node, SYN_RULE);

    let g = Grammar {
        node_types: lift_node_types(node_types)?,
        lex_rules: lift_lex_rules(lex_rules)?,
        syn_rules: syn_rules.map(lift_syn_rule).collect::<::std::result::Result<Vec<_>, _>>()?
    };

    Ok(g)
}

fn lift_node_types(node: Node) -> Result<Vec<String>> {
    Ok(children_of_type(node, IDENT)
        .map(node_text)
        .collect())
}

fn lift_lex_rules(node: Node) -> Result<Vec<LexRule>> {
    children_of_type(node, LEX_RULE)
        .map(lift_lex_rule)
        .collect()
}

fn lift_lex_rule(node: Node) -> Result<LexRule> {
    let ty = node_text(
        child_of_type(node, IDENT).ok_or(error("Missing name in rule"))?
    );

    let mut pats = children_of_type(node, STRING);
    let re = node_text(
        pats.next().ok_or(error(format!("Missing pattern in rule {:?}", node.text())))?
    );
    let f = pats.next().map(|n| {
        let t = n.text();
        assert!(t.starts_with("\"") && t.ends_with("\""));
        t[1..t.len() - 1].to_owned()
    });
    assert!(pats.next().is_none());
    Ok(LexRule { ty: ty, re: re, f: f })
}

fn lift_syn_rule(node: Node) -> Result<SynRule> {
    Ok(SynRule {
        name: node_text(child_of_type_exn(node, IDENT)),
        alts: children_of_type(node, ALT).map(lift_alt).collect(),
    })
}

fn lift_alt(node: Node) -> Alt {
    Alt { parts: children_of_type(node, PART).map(lift_part).collect() }
}

fn lift_part(node: Node) -> Part {
    let mut children = node.children();
    let first_child = children.next().unwrap();
    if first_child.ty() == IDENT {
        assert!(children.next().is_none());
        return Part::Rule(node_text(first_child));
    }
    assert!(first_child.ty() == LANGLE);
    let op = node_text(child_of_type_exn(node, IDENT));
    Part::Call(op, children_of_type(node, ALT).map(lift_alt).collect())
}

fn node_text(node: Node) -> String {
    node.text().to_owned()
}
