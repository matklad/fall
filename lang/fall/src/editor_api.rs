use fall_tree::{File, AstNode, NodeType, Node, dump_file, TextRange, TextUnit};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type, ancestors, find_leaf_at_offset};
use ::{ast, LANG_FALL, RefKind};
use ::syntax::*;

pub fn parse(text: String) -> File {
    LANG_FALL.parse(text)
}

pub fn tree_as_text(file: &File) -> String {
     dump_file(file)
}

type Spans = Vec<(TextRange, &'static str)>;
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
            let range = if node.range().is_empty() {
                TextRange::from_len(node.range().start(), TextUnit::from_usize(1))
            } else {
                node.range()
            };
            spans.push((range, "error"))
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
    return spans;

    fn colorize_node(node: Node, color: &'static str, spans: &mut Spans) {
        spans.push((node.range(), color))
    }

    fn colorize_child(node: Node, child: NodeType, color: &'static str, spans: &mut Spans) {
        if let Some(child) = child_of_type(node, child) {
            colorize_node(child, color, spans);
        }
    }
}

pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    let node = match find_node_at_range(&file, range) {
        Some(node) => node,
        None => return None,
    };

    match ancestors(node).skip_while(|n| n.range() == range).next() {
        None => None,
        Some(parent) => Some(parent.range()),
    }

}

fn find_node_at_range(file: &File, range: TextRange) -> Option<Node> {
    let root = file.root();
    let (left, right) = match (
        find_leaf_at_offset(root, range.start()).right_biased(),
        find_leaf_at_offset(root, range.end()).left_biased()
    ) {
        (Some(l), Some(r)) => (l, r),
        _ => return None
    };

    Some(common_ancestor(left, right))
}

fn common_ancestor<'f>(n1: Node<'f>, n2: Node<'f>) -> Node<'f> {
    for p in ancestors(n1) {
        if ancestors(n2).any(|a| a == p) {
            return p
        }
    }
    panic!("Can't find common ancestor of {:?} and {:?}", n1, n2)
}
