use std::collections::HashSet;

use fall_tree::{File, AstNode, Node, dump_file, TextRange, TextUnit, TextEdit};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::{child_of_type, find_leaf_at_offset};
use fall_tree::search::ast;

use ::*;

mod actions;
mod formatter;
mod refdec;
mod api_impl;

use self::actions::{ACTIONS, ContextActionId};
use self::refdec::{Declaration, Reference};

pub fn parse(text: String) -> File {
    lang_fall().parse(text)
}

pub fn tree_as_text(file: &File) -> String {
    dump_file(file)
}

pub fn highlight(file: &File) -> Vec<(TextRange, &'static str)> {
    api_impl::highlighting::highlight(file)
}

pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    api_impl::extend_selection::extend_selection(file, range)
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
    Visitor(Vec::new())
        .visit::<SynRule, _>(|nodes, rule| {
            if let Some(name) = rule.name() {
                nodes.push(FileStructureNode {
                    name: name.to_string(),
                    range: rule.node().range(),
                    children: Vec::new(),
                })
            }
        })
        .walk_recursively_children_first(file.root())
}

pub enum Severity {
    Error,
    Warning
}

pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub text: String
}

impl Diagnostic {
    fn error(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            text: message,
        }
    }

    fn warning(node: Node, message: String) -> Diagnostic {
        Diagnostic {
            range: node.range(),
            severity: Severity::Warning,
            text: message,
        }
    }
}

pub fn diagnostics(file: &File) -> Vec<Diagnostic> {
    let used_rules: HashSet<Node> = ast::descendants_of_type::<RefExpr>(file.root())
        .into_iter()
        .filter_map(|node| node.resolve())
        .filter_map(|r| match r {
            RefKind::RuleReference(rule) => Some(rule.node()),
            _ => None
        })
        .chain(
            ast::descendants_of_type::<CallExpr>(file.root())
                .into_iter()
                .filter_map(|call| call.kind().ok())
                .filter_map(|kind| match kind {
                    CallKind::RuleCall(rule, ..) => Some(rule.node()),
                    _ => None,
                })
        )
        .chain(child_of_type(file.root(), SYN_RULE).into_iter())
        .collect();

    Visitor(Vec::new())
        .visit::<RefExpr, _>(|acc, ref_| {
            if ref_.resolve().is_none() {
                if let Some(call) = ast::parent::<CallExpr>(ref_.node()) {
                    if call.resolve_context().is_some() {
                        return;
                    }
                }

                acc.push(Diagnostic::error(ref_.node(), "Unresolved reference".to_string()))
            }
        })
        .visit::<CallExpr, _>(|acc, call| {
            match call.kind() {
                Err(e) => acc.push(Diagnostic::error(call.node(), e.to_string())),
                _ => {}
            }
        })
        .visit::<SynRule, _>(|acc, rule| {
            if !used_rules.contains(&rule.node()) {
                acc.push(Diagnostic::warning(rule.node(), "Unused rule".to_string()))
            }
        })
        .walk_recursively_children_first(file.root())
}

pub fn resolve_reference(file: &File, offset: TextUnit) -> Option<TextRange> {
    return refdec::resolve_reference(file, offset, ref_provider);
}

pub fn find_usages(file: &File, offset: TextUnit) -> Vec<TextRange> {
    return refdec::find_usages(file, offset, ref_provider, def_provider);
}


fn ref_provider<'f>(node: Node<'f>) -> Option<Reference<'f>> {
    Visitor(None)
        .visit::<RefExpr, _>(|result, ref_expr| {
            *result = Some(Reference::new(ref_expr.node(), |node| {
                let ref_expr = RefExpr::new(node);
                let target = match ref_expr.resolve() {
                    None => return None,
                    Some(t) => t
                };

                Some(match target {
                    RefKind::RuleReference(rule) => rule.into(),
                    RefKind::Param(param) => param.into(),
                    RefKind::Token(token) => token.into(),
                })
            }))
        })
        .visit::<AstSelector, _>(|result, selector| {
            *result = Some(Reference::new(selector.node(), |node| {
                let selector = AstSelector::new(node);
                let target = match selector.child_kind() {
                    None => return None,
                    Some(t) => t
                };
                Some(match target {
                    ChildKind::AstNode(node) => node.into(),
                    ChildKind::AstClass(cls) => cls.into(),
                    ChildKind::Token(token) => token.into()
                })
            }))
        })
        .visit_nodes(&[IDENT], |result, ident| {
            match ident.parent() {
                Some(parent) if parent.ty() == CALL_EXPR => {
                    let call = CallExpr::new(parent);
                    if let Ok(CallKind::RuleCall(..)) = call.kind() {
                        *result = Some(Reference::new(ident, |node| {
                            let call = CallExpr::new(node.parent().unwrap());
                            match call.kind().unwrap() {
                                CallKind::RuleCall(rule, ..) => Some(rule.into()),
                                _ => unimplemented!()
                            }
                        }))
                    }
                }
                _ => {}
            }
        })
        .walk_single_node(node)
}

fn def_provider<'f>(node: Node<'f>) -> Option<Declaration<'f>> {
    Visitor(None)
        .visit::<SynRule, _>(|result, node| *result = Some(node.into()))
        .visit::<LexRule, _>(|result, node| *result = Some(node.into()))
        .visit::<Parameter, _>(|result, node| *result = Some(node.into()))
        .visit::<AstNodeDef, _>(|result, node| *result = Some(node.into()))
        .visit::<AstClassDef, _>(|result, node| *result = Some(node.into()))
        .walk_single_node(node)
}

impl<'f> From<SynRule<'f>> for Declaration<'f> {
    fn from(rule: SynRule<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), rule.name_ident())
    }
}

impl<'f> From<LexRule<'f>> for Declaration<'f> {
    fn from(rule: LexRule<'f>) -> Self {
        Declaration::new(rule.node())
    }
}

impl<'f> From<Parameter<'f>> for Declaration<'f> {
    fn from(rule: Parameter<'f>) -> Self {
        Declaration::new(rule.node())
    }
}

impl<'f> From<AstNodeDef<'f>> for Declaration<'f> {
    fn from(rule: AstNodeDef<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), Some(rule.name_ident()))
    }
}

impl<'f> From<AstClassDef<'f>> for Declaration<'f> {
    fn from(rule: AstClassDef<'f>) -> Self {
        Declaration::with_name_ident(rule.node(), Some(rule.name_ident()))
    }
}

pub fn reformat(file: &File) -> TextEdit {
    self::formatter::reformat_file(file, self::formatter::FALL_SPACING, WHITESPACE)
}

pub fn find_test_at_offset(file: &File, offset: TextUnit) -> Option<usize> {
    let test = find_leaf_at_offset(file.root(), offset)
        .right_biased()
        .and_then(|node| ast::parent::<TestDef>(node));

    if let Some(test) = test {
        Some(FallFile::new(file.root()).tests().position(|t| t.node() == test.node()).unwrap())
    } else {
        None
    }
}
