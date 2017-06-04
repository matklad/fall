use fall_tree::{File, AstNode, NodeType, Node};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type};
use ::*;


type Spans = Vec<(u32, u32, &'static str)>;

pub fn highlight(file: &File) -> Spans {
    let file = ast(file);
    let mut spans = vec![];
    Visitor(&mut spans)
        .visit_nodes(&[HASH_STRING, SIMPLE_STRING], |spans, node| {
            colorize_node(node, "string", spans)
        })
        .visit_nodes(&[KW_RULE, KW_VERBATIM, KW_TOKENIZER, KW_AST, KW_NODE, KW_CLASS, KW_PUB], |spans, node| {
            colorize_node(node, "keyword", spans)
        })
        .visit_nodes(&[ERROR], |spans, node| {
            spans.push((node.range().start(), node.range().end() + 1, "error"))
        })
        .visit::<LexRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "token", spans))
        .visit::<SynRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<AstNodeDef, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<RefExpr, _>(|spans, ref_| match ref_.resolve() {
            Some(RefKind::Token(_)) => colorize_node(ref_.node(), "token", spans),
            Some(RefKind::RuleReference { .. }) => colorize_node(ref_.node(), "rule", spans),
            None => {}
        })
        .visit::<CallExpr, _>(|spans, call| {
            colorize_child(call.node(), IDENT, "builtin", spans);
            colorize_child(call.node(), LANGLE, "builtin", spans);
            colorize_child(call.node(), RANGLE, "builtin", spans);
        })
        .visit::<Attributes, _>(|spans, attrs| {
            colorize_node(attrs.node(), "meta", spans)
        })
        .walk_recursively_children_first(file.node());
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
