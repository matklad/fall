use fall_tree::{TextRange, Node, NodeType, tu, AstNode};
use fall_tree::search::child_of_type;
use fall_tree::visitor::{Visitor, NodeVisitor};

use ::*;
use analysis::{CallKind, RefKind};

type Spans = Vec<(TextRange, &'static str)>;

pub(crate) fn highlight(analysis: &Analysis) -> Spans {
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
                TextRange::from_len(node.range().start(), tu(1))
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
            let color = match analysis.resolve_reference(ref_) {
                Some(RefKind::Token(_)) => "token",
                Some(RefKind::RuleReference { .. }) => "rule",
                Some(RefKind::Param(..)) => "value_parameter",
                None => return
            };
            colorize_node(ref_.node(), color, spans)
        })
        .visit::<MethodDef, _>(|spans, method| {

            let color = match analysis.resolve_method(method) {
                Some(MethodKind::NodeAccessor(child_kind, _)) => match child_kind {
                    ChildKind::Token(..) => "token",
                    ChildKind::AstClass(..) | ChildKind::AstNode(..) => "rule",
                },
                None | Some(_) => return
            };
            colorize_child(method.selector().node(), IDENT, color, spans)
        })
        .visit::<CallExpr, _>(|spans, call| {
            let color = match analysis.resolve_call(call) {
                None | Some(CallKind::RuleCall(..)) => "rule",
                Some(_) => "builtin"
            };

            colorize_child(call.node(), IDENT, color, spans);
            colorize_child(call.node(), L_ANGLE, color, spans);
            colorize_child(call.node(), R_ANGLE, color, spans);
        })
        .visit::<Attributes, _>(|spans, attrs| {
            colorize_node(attrs.node(), "meta", spans)
        })
        .walk_recursively_children_first(analysis.ast().node());
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
    let file = ::editor_api::analyse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar <eof> <abracadabra> }
rule bar { number <m foo> }

rule m(f) {}
"####.to_owned());

    file.analyse(|a| {
        let spans = highlight(a);
        let result = spans.into_iter().map(|(range, d)| {
            format!("{}: {}", a.ast().node().text().slice(range), d)
        }).collect::<Vec<_>>().join("\n");
        assert_eq!(result, r##"tokenizer: keyword
r"\d+": string
number: token
pub: keyword
rule: keyword
bar: rule
eof: builtin
<: builtin
>: builtin
abracadabra: rule
<: rule
>: rule
foo: rule
rule: keyword
number: token
foo: rule
m: rule
<: rule
>: rule
bar: rule
rule: keyword
f: value_parameter
m: rule"##);
    });
}
