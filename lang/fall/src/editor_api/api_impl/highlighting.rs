use fall_tree::{TextUnit, TextRange, File, Node, NodeType};
use fall_tree::search::child_of_type;
use fall_tree::visitor::{Visitor, NodeVisitor};

use ::*;

type Spans = Vec<(TextRange, &'static str)>;

pub fn highlight(file: &File) -> Spans {
    let file = ast(file);
    return Visitor(Vec::new())
        .visit_nodes(&[EOL_COMMENT], |spans, node| {
            colorize_node(node, "comment", spans)
        })
        .visit_nodes(&[HASH_STRING, SIMPLE_STRING], |spans, node| {
            colorize_node(node, "string", spans)
        })
        .visit_nodes(&[RULE, VERBATIM, TOKENIZER, AST, NODE, CLASS, PUB, TEST], |spans, node| {
            colorize_node(node, "keyword", spans)
        })
        .visit_nodes(&[ERROR], |spans, node| {
            let range = if node.range().is_empty() {
                TextRange::from_len(node.range().start(), TextUnit::from_usize(1))
            } else {
                node.range()
            };
            spans.push((range, "error"))
        })
        .visit_nodes(&[PARAMETER], |spans, node| colorize_node(node, "value_parameter", spans))
        .visit::<LexRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "token", spans))
        .visit::<SynRule, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<AstNodeDef, _>(|spans, rule| colorize_child(rule.node(), IDENT, "rule", spans))
        .visit::<RefExpr, _>(|spans, ref_| {
            let color = match ref_.resolve() {
                Some(RefKind::Token(_)) => "token",
                Some(RefKind::RuleReference { .. }) => "rule",
                Some(RefKind::Param(..)) => "value_parameter",
                None => return
            };
            colorize_node(ref_.node(), color, spans)
        })
        .visit::<AstSelector, _>(|spans, sel| {
            let color = match sel.child_kind() {
                Some(ChildKind::Token(..)) => "token",
                Some(ChildKind::AstClass(..)) | Some(ChildKind::AstNode(..)) => "rule",
                None => return
            };
            colorize_child(sel.node(), IDENT, color, spans)
        })
        .visit::<CallExpr, _>(|spans, call| {
            let color = match call.kind() {
                Err(_) => "unresolved",
                Ok(CallKind::RuleCall(..)) => "rule",
                _ => "builtin"
            };

            colorize_child(call.node(), IDENT, color, spans);
            colorize_child(call.node(), L_ANGLE, color, spans);
            colorize_child(call.node(), R_ANGLE, color, spans);
        })
        .visit::<Attributes, _>(|spans, attrs| {
            colorize_node(attrs.node(), "meta", spans)
        })
        .walk_recursively_children_first(file.node());
}

fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
    spans.push((node.range(), color))
}

fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
    if let Some(child) = child_of_type(node, child) {
        colorize_node(child, color, spans);
    }
}

#[test]
fn test_highlighting() {
    let file = parse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"####);

    let spans = highlight(&file);
    assert_eq!(
        format!("{:?}", spans),
        r#"[([1; 10), "keyword"), ([20; 26), "string"), ([13; 19), "token"), ([28; 31), "keyword"), ([32; 36), "keyword"), ([43; 46), "rule"), ([37; 40), "rule"), ([49; 53), "keyword"), ([60; 66), "token"), ([54; 57), "rule")]"#
    );
}
