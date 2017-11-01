use fall_tree::{File, AstNode, dump_file, TextRange, TextUnit, TextEdit};
use fall_tree::search::find_leaf_at_offset;
use fall_tree::search::ast;

use ::*;

mod api_impl;

pub fn parse(text: String) -> File {
    ::parse(text)
}

pub fn analyse(text: String) -> FileWithAnalysis {
    FileWithAnalysis::new(text)
}

pub fn tree_as_text(file: &File) -> String {
    dump_file(file)
}

pub fn highlight(analysis: &Analysis) -> Vec<(TextRange, &'static str)> {
    api_impl::highlighting::highlight(analysis)
}

pub fn extend_selection(file: &File, range: TextRange) -> Option<TextRange> {
    api_impl::extend_selection::extend_selection(file, range)
}

pub fn context_actions(file: &File, range: TextRange) -> Vec<&'static str> {
    api_impl::actions::context_actions(file, range)
}

pub fn apply_context_action(file: &File, range: TextRange, action_id: &str) -> TextEdit {
    api_impl::actions::apply_context_action(file, range, action_id)
}

#[derive(Serialize, Debug)]
pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

pub fn structure(file: &File) -> Vec<FileStructureNode> {
    api_impl::structure::structure(file)
}

#[derive(Serialize, Debug)]
pub enum Severity { Error, Warning }

#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub message: String
}

pub fn diagnostics(analysis: &Analysis) -> Vec<Diagnostic> {
    api_impl::diagnostics::diagnostics(analysis)
}

pub fn resolve_reference(file: &File, offset: TextUnit) -> Option<TextRange> {
    api_impl::references::resolve_reference(file, offset)
}

pub fn find_usages(file: &File, offset: TextUnit) -> Vec<TextRange> {
    api_impl::references::find_usages(file, offset)
}

pub fn reformat(file: &File) -> TextEdit {
    api_impl::formatter::reformat(file)
}

pub fn test_at_offset(file: &File, offset: TextUnit) -> Option<usize> {
    find_leaf_at_offset(file.root(), offset)
        .right_biased()
        .and_then(|node| ast::ancestor::<TestDef>(node))
        .map(|test| {
            ::ast(file)
                .tests()
                .position(|t| t.node() == test.node())
                .unwrap()
        })
}
