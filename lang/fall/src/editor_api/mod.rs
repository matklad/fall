use fall_editor::hl;
use fall_tree::{AstNode, TextRange, TextUnit, TextEdit};
use fall_tree::search::find_leaf_at_offset;
use fall_tree::search::ast;

use analysis::{FileWithAnalysis, Analysis};
use syntax::{TestDef};

mod references;

pub fn analyse(text: String) -> FileWithAnalysis {
    FileWithAnalysis::new(::parse(text))
}

pub fn resolve_reference(analysis: &Analysis, offset: TextUnit) -> Option<TextRange> {
    references::resolve_reference(analysis, offset)
}

pub fn find_usages(analysis: &Analysis, offset: TextUnit) -> Vec<TextRange> {
    references::find_usages(analysis, offset)
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
