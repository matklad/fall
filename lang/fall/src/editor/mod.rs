use analysis::Analysis;

use fall_tree::{File, TextEdit, TextRange, TextUnit, AstNode};
use fall_tree::search::find_leaf_at_offset;
use fall_tree::search::ast;
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode, Diagnostic};
use fall_editor::hl::Highlights;
use syntax::lang_fall;
use syntax::TestDef;

mod highlighting;
mod structure;
mod actions;
mod formatter;
mod references;

pub use analysis::FileWithAnalysis;

impl EditorFileImpl for FileWithAnalysis {
    fn parse(text: &str) -> Self {
        FileWithAnalysis::new(lang_fall().parse(text))
    }

    fn edit(&self, edit: &TextEdit) -> Self {
        let file = self.file().edit(edit);
        FileWithAnalysis::new(file)
    }

    fn file(&self) -> &File {
        self.file()
    }


    fn syntax_tree(&self) -> String {
        gen_syntax_tree(self.file())
    }

    fn highlight(&self) -> Highlights {
        self.record_analysis("highlight", |a| highlighting::highlight(a))
    }

    fn structure(&self) -> Vec<FileStructureNode> {
        structure::structure(self.file())
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        self.record_analysis("diagnostics", |a| a.collect_all_diagnostics())
    }

    fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        self.record_analysis("context_actions", |a| actions::context_actions(a, range))
    }

    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        let edit = self.analyse(|a| actions::apply_context_action(a, range, id));
        Some(edit)
    }

    fn reformat(&self) -> TextEdit {
        formatter::reformat(self.file())
    }
}

impl FileWithAnalysis {
    pub fn resolve_reference(&self, offset: TextUnit) -> Option<TextRange> {
        self.analyse(|a| references::resolve_reference(a, offset))
    }

    pub fn find_usages(&self, offset: TextUnit) -> Vec<TextRange> {
        self.analyse(|a| references::find_usages(a, offset))
    }

    pub fn test_at_offset(&self, offset: TextUnit) -> Option<usize> {
        self.analyse(|a| {
            find_leaf_at_offset(a.ast().node(), offset)
                .right_biased()
                .and_then(ast::ancestor::<TestDef>)
                .map(|test| {
                    a.ast()
                        .tests()
                        .position(|t| t.node() == test.node())
                        .unwrap()
                })
        })
    }

    fn record_analysis<R, F: FnOnce(&Analysis) -> R>(&self, tag: &'static str, f: F) -> R {
        self.analyse(|a| {
            self.file().metrics().measure_time(tag, || f(a))
        })
    }
}

#[test]
fn test_extend_selection() {
    use fall_tree::tu;

    let file = ::analyse(r####"
tokenizer { number r"\d+"}
pub rule foo { bar }
rule bar { number }
"####);
    let s1 = file.extend_selection(TextRange::from_len(tu(44), tu(0)))
        .unwrap();
    let s2 = file.extend_selection(s1).unwrap();
    let s3 = file.extend_selection(s2).unwrap();
    assert_eq!(
        format!("{:?}", (s1, s2, s3)),
        "([43; 46), [41; 48), [28; 48))"
    );
}
