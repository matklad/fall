use analysis::Analysis;

use fall_tree::{File, TextEdit, TextRange};
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode, Diagnostic};
use fall_editor::hl::Highlights;
use syntax::lang_fall;

mod highlighting;
mod structure;
mod actions;
mod formatter;

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
    fn record_analysis<R, F: FnOnce(&Analysis) -> R>(&self, tag: &'static str, f: F) -> R {
        self.analyse(|a| {
            self.file().metrics().measure_time(tag, || f(a))
        })
    }
}
