#[macro_use]
extern crate serde_derive;
extern crate jsonrpc_core;
extern crate jsonrpc_minihttp_server;
extern crate fall_gen;
extern crate fall_tree;

use std::collections::HashSet;

use jsonrpc_core::{IoHandler, Params, to_value};
use jsonrpc_minihttp_server::{ServerBuilder};

use fall_tree::{Node, NodeType};
use fall_tree::search::{child_of_type, children_of_type, child_of_type_exn};
use fall_gen::syntax::*;

type Spans = Vec<(u32, u32, &'static str)>;

#[derive(Serialize)]
struct Response {
    spans: Spans
}

fn main() {
    let mut io = IoHandler::new();
    io.add_method("colors", |params: Params| {
        let (text, ): (String, ) = params.parse().unwrap();
        println!("{}\n", text);
        let spans = colorize(text);
        println!("spans = {:?}\n\n\n", spans);
        let r = to_value(Response { spans: spans }).unwrap();
        Ok(r)
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:9292".parse().unwrap())
        .unwrap();

    println!("Starting");
    server.wait().unwrap();
}


fn colorize(text: String) -> Spans {
    let file = fall_gen::syntax::parse(text);
    let mut result = vec![];
    colorize_file(file.root(), &mut result);
    result
}

fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
    spans.push((node.range().start(), node.range().end(), color))
}

fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
    if let Some(child) = child_of_type(node, child) {
        colorize_node(child, color, spans);
    }
}

fn colorize_file(node: Node, spans: &mut Spans) {
    for &kw_ty in [KW_NODES, KW_TOKENIZER, KW_RULE, KW_VERBATIM].iter() {
        colorize_child(node, kw_ty, "keyword", spans);
    }
    let node_types = child_of_type_exn(node, NODES_DEF);
    colorize_child(node_types, KW_NODES, "keyword", spans);

    let lex_rules = child_of_type_exn(node, TOKENIZER_DEF);
    colorize_child(lex_rules, KW_TOKENIZER, "keyword", spans);

    let tokens: HashSet<&str> = children_of_type(lex_rules, LEX_RULE)
        .map(|n| child_of_type_exn(n, IDENT))
        .map(|n| n.text())
        .collect();

    for ident in children_of_type(node_types, IDENT) {
        if tokens.contains(ident.text()) {
            colorize_node(ident, "token", spans)
        } else {
            colorize_node(ident, "rule", spans)
        }
    }

    for lex_rule in children_of_type(lex_rules, LEX_RULE) {
        colorize_child(lex_rule, IDENT, "token", spans);
        colorize_child(lex_rule, STRING, "string", spans);
    }

    let syn_rules = children_of_type(node, SYN_RULE);
    for rule in syn_rules {
        colorize_child(rule, KW_RULE, "keyword", spans);
        colorize_child(rule, IDENT, "rule", spans);
        for alt in children_of_type(rule, ALT) {
            colorize_alt(alt, &tokens, spans)
        }
    }

    if let Some(verbatim) = child_of_type(node, VERBATIM_DEF) {
        colorize_child(verbatim, HASH_STRING, "string", spans);
        colorize_child(verbatim, KW_VERBATIM, "keyword", spans);
    };

    //    colorize_node_types(node_types);
    //    colorize_lex_rules(lex_rules);
}

fn colorize_alt(node: Node, tokens: &HashSet<&str>, spans: &mut Spans) {
    for part in children_of_type(node, PART) {
        colorize_part(part, tokens, spans)
    }
}

fn colorize_part(node: Node, tokens: &HashSet<&str>, spans: &mut Spans) {
    colorize_child(node, SIMPLE_STRING, "token", spans);
    for ident in children_of_type(node, IDENT) {
        if tokens.contains(ident.text()) {
            colorize_node(ident, "token", spans)
        } else {
            colorize_node(ident, "rule", spans)
        }
    }
    for alt in children_of_type(node, ALT) {
        colorize_alt(alt, tokens, spans)
    }
}
