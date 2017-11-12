use fall_tree::{AstNode, dump_file, TextRange, TextUnit, TextEdit};
use fall_tree::search::find_leaf_at_offset;
use fall_tree::search::ast;

use ::*;

mod api_impl;

pub fn analyse(text: String) -> FileWithAnalysis {
    FileWithAnalysis::new(::parse(text))
}

pub fn edit(analysis: &Analysis, edit: TextEdit) -> FileWithAnalysis {
    let new_file = analysis.file().edit(edit);
    FileWithAnalysis::new(new_file)
}

pub fn tree_as_text(analysis: &Analysis) -> String {
    dump_file(analysis.file())
}

pub fn highlight(analysis: &Analysis) -> Vec<(TextRange, &'static str)> {
    analysis.file().metrics().measure_time("highlight", || {
        api_impl::highlighting::highlight(analysis)
    })
}

pub fn extend_selection(analysis: &Analysis, range: TextRange) -> Option<TextRange> {
    api_impl::extend_selection::extend_selection(analysis.file(), range)
}

pub fn context_actions(analysis: &Analysis, range: TextRange) -> Vec<&'static str> {
    api_impl::actions::context_actions(analysis, range)
}

pub fn apply_context_action(analysis: &Analysis, range: TextRange, action_id: &str) -> TextEdit {
    api_impl::actions::apply_context_action(analysis, range, action_id)
}

#[derive(Serialize, Debug)]
pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

pub fn structure(analysis: &Analysis) -> Vec<FileStructureNode> {
    api_impl::structure::structure(analysis.file())
}

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Severity { Error, Warning }

#[derive(Serialize, Debug, Clone)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub message: String
}

pub fn diagnostics(analysis: &Analysis) -> Vec<Diagnostic> {
    analysis.file().metrics().measure_time("diagnostics", || {
        analysis.collect_all_diagnostics()
    })
}

pub fn resolve_reference(analysis: &Analysis, offset: TextUnit) -> Option<TextRange> {
    api_impl::references::resolve_reference(analysis, offset)
}

pub fn find_usages(analysis: &Analysis, offset: TextUnit) -> Vec<TextRange> {
    api_impl::references::find_usages(analysis, offset)
}

pub fn reformat(analysis: &Analysis) -> TextEdit {
    api_impl::formatter::reformat(analysis.file())
}

pub fn test_at_offset(analysis: &Analysis, offset: TextUnit) -> Option<usize> {
    find_leaf_at_offset(analysis.ast().node(), offset)
        .right_biased()
        .and_then(|node| ast::ancestor::<TestDef>(node))
        .map(|test| {
            analysis.ast()
                .tests()
                .position(|t| t.node() == test.node())
                .unwrap()
        })
}
