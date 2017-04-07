#[macro_use]
extern crate serde_derive;
extern crate jsonrpc_core;
extern crate jsonrpc_minihttp_server;
extern crate fall_gen;
extern crate fall_tree;

use std::collections::HashSet;

use jsonrpc_core::{IoHandler, Params, to_value};
use jsonrpc_minihttp_server::{ServerBuilder};

use fall_tree::{Node, NodeType, walk_tree, AstNode};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type};
use fall_gen::syntax::*;
use fall_gen::ast::*;
use fall_gen::ast_ext::PartKind;

type Spans = Vec<(u32, u32, &'static str)>;

#[derive(Serialize, Default)]
struct Response {
    spans: Spans
}

fn main() {
    let mut io = IoHandler::new();
    io.add_method("colors", |params: Params| {
        println!("Req");
        let (text, ): (String, ) = params.parse().unwrap();
        let spans = ::std::panic::catch_unwind(|| colorize(text)).unwrap_or_default();
        let r = to_value(Response { spans: spans }).unwrap();
        println!("OK\n");
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
    let file = fall_gen::FallFile::parse(text);
    let ast = file.ast();
    println!("Lexing  = {}", file.lexing_time());
    println!("Parsing = {}", file.parsing_time());
    let mut spans = vec![];
    colorize_tokens(ast.node(), &mut spans);
    let token_names: HashSet<_> = file.ast().tokenizer_def().lex_rules().map(|r| r.node_type()).collect();
    Visitor(&mut spans)
        .visit::<NodesDef, _>(|spans, def| {
            walk_tree(def.node(), |n| {
                if n.ty() == IDENT {
                    let color = if token_names.contains(n.text()) { "token" } else { "rule" };
                    colorize_node(n, color, spans);
                }
            })
        })
        .visit::<LexRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "token", spans))
        .visit::<SynRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<Part, _>(|spans, part| {
            match part.kind() {
                PartKind::Token(_) => colorize_node(part.node(), "token", spans),
                PartKind::RuleReference { .. } => colorize_node(part.node(), "rule", spans),
                PartKind::Call { .. } => {
                    colorize_child(part.node(), IDENT, "builtin", spans);
                    colorize_child(part.node(), LANGLE, "builtin", spans);
                    colorize_child(part.node(), RANGLE, "builtin", spans);
                }
            }
        })
        .walk_recursively_children_first(file.ast().node());
    spans
}

fn colorize_tokens(node: Node, spans: &mut Spans) {
    let keywords = [KW_RULE, KW_VERBATIM, KW_NODES, KW_TOKENIZER];
    walk_tree(node, |node| {
        if keywords.contains(&node.ty()) {
            colorize_node(node, "keyword", spans)
        }
        if [HASH_STRING, SIMPLE_STRING].contains(&node.ty()) {
            colorize_node(node, "string", spans)
        }
    })
}

fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
    spans.push((node.range().start(), node.range().end(), color))
}

fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
    if let Some(child) = child_of_type(node, child) {
        colorize_node(child, color, spans);
    }
}
