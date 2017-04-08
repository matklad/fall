#[macro_use]
extern crate serde_derive;
extern crate jsonrpc_core;
extern crate jsonrpc_minihttp_server;
extern crate fall_gen;
extern crate fall_tree;
extern crate elapsed;

use std::collections::HashSet;

use jsonrpc_core::{IoHandler, Params, to_value};
use jsonrpc_minihttp_server::{ServerBuilder};

use elapsed::measure_time;

use fall_tree::{Node, NodeType, walk_tree, AstNode  };
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type};
use fall_gen::syntax::*;
use fall_gen::ast_ext::PartKind;

type Spans = Vec<(u32, u32, &'static str)>;


fn main() {
    let mut io = IoHandler::new();
    io.add_method("colors", |params: Params| {
        #[derive(Serialize, Default)]
        struct Response { spans: Spans }

        println!("colors");
        let (text, ): (String, ) = params.parse().unwrap();
        let spans = ::std::panic::catch_unwind(|| colorize(text)).unwrap_or_default();
        let r = to_value(Response { spans: spans }).unwrap();
        println!("OK\n");
        Ok(r)
    });
    io.add_method("tree", |params: Params| {
        #[derive(Serialize, Default)]
        struct Response { tree: String }

        println!("tree");
        let (text, ): (String, ) = params.parse().unwrap();
        let file = fall_gen::FallFile::parse(text);
        let r = to_value(Response { tree: file.tree_to_string() }).unwrap();
        println!("OK\n");
        Ok(r)
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:9292".parse().unwrap())
        .unwrap();

    println!("Starting server on 127.0.0.1:9292");
    server.wait().unwrap();
}


fn colorize(text: String) -> Spans {
    let file = fall_gen::FallFile::parse(text);
    let (elapsed, spans) = measure_time(|| {
        let mut spans = vec![];
        let token_names: HashSet<_> = file.ast().tokenizer_def().lex_rules().map(|r| r.node_type()).collect();
        Visitor(&mut spans)
            .visit_nodes(&[HASH_STRING, SIMPLE_STRING], |spans, node| {
                colorize_node(node, "string", spans)
            })
            .visit_nodes(&[KW_RULE, KW_VERBATIM, KW_NODES, KW_TOKENIZER], |spans, node| {
                colorize_node(node, "keyword", spans)
            })
            .visit::<NodesDef, _>(|spans, def| {
                walk_tree(def.node(), |n| if n.ty() == IDENT {
                    let color = if token_names.contains(n.text()) { "token" } else { "rule" };
                    colorize_node(n, color, spans);
                })
            })
            .visit::<LexRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "token", spans))
            .visit::<SynRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
            .visit::<Part, _>(|spans, part| match part.kind() {
                PartKind::Token(_) => colorize_node(part.node(), "token", spans),
                PartKind::RuleReference { .. } => colorize_node(part.node(), "rule", spans),
                PartKind::Call { .. } => {
                    colorize_child(part.node(), IDENT, "builtin", spans);
                    colorize_child(part.node(), LANGLE, "builtin", spans);
                    colorize_child(part.node(), RANGLE, "builtin", spans);
                }
            })
            .walk_recursively_children_first(file.ast().node());
        spans
    });
    println!("Lexing   = {}", file.lexing_time());
    println!("Parsing  = {}", file.parsing_time());
    println!("Coloring = {}", elapsed);

    spans
}

fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
    spans.push((node.range().start(), node.range().end(), color))
}

fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
    if let Some(child) = child_of_type(node, child) {
        colorize_node(child, color, spans);
    }
}
