use fall_tree::{File, AstNode, NodeType, Node, dump_file, TextRange, TextUnit};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type, ancestors, find_leaf_at_offset};
use fall_tree::edit::TextEdit;
use ::{ast, LANG_FALL, RefKind, CallKind};
use ::syntax::*;

mod actions;

use self::actions::{ACTIONS, ContextActionId};

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
        .visit_nodes(&[EOL_COMMENT], |spans, node| {
            colorize_node(node, "comment", spans)
        })
        .visit_nodes(&[HASH_STRING, SIMPLE_STRING], |spans, node| {
            colorize_node(node, "string", spans)
        })
        .visit_nodes(&[RULE, VERBATIM, TOKENIZER, AST, NODE, CLASS, PUB, EXAMPLE], |spans, node| {
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
                None => "unresolved"
            };
            colorize_node(ref_.node(), color, spans)
        })
        .visit::<CallExpr, _>(|spans, call| {
            let color = match call.kind() {
                Err(_) => "unresolved",
                Ok(CallKind::RuleCall(..)) => "rule",
                _ => "builtin"
            };

            colorize_child(call.node(), IDENT, color, spans);
            colorize_child(call.node(), LANGLE, color, spans);
            colorize_child(call.node(), RANGLE, color, spans);
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


pub fn collect_applicable_context_actions(file: &File, offset: TextUnit) -> Vec<ContextActionId> {
    ACTIONS.iter()
        .filter(|action| action.apply(file, offset).is_some())
        .map(|action| action.id())
        .collect()
}

pub fn apply_context_action(file: &File, offset: TextUnit, action_id: &str) -> TextEdit {
    let action = ACTIONS.iter().find(|action| action.id().0 == action_id).unwrap();
    action.apply(file, offset).unwrap().into_text_edit()
}

pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

pub fn file_structure(file: &File) -> Vec<FileStructureNode> {
    let mut nodes = Vec::new();
    Visitor(&mut nodes)
        .visit::<SynRule, _>(|nodes, rule| {
            if let Some(name) = rule.name() {
                nodes.push(FileStructureNode {
                    name: name.to_string(),
                    range: rule.node().range(),
                    children: Vec::new(),
                })
            }
        })
        .walk_recursively_children_first(file.root());

    nodes
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
