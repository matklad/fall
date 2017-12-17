use analysis::{Analysis, FileWithAnalysis};

use fall_tree::{File, TextEdit};
use fall_editor::{EditorSupport, EditorFile, EditorFileImpl, gen_syntax_tree, FileStructureNode};
use fall_editor::hl::{self, Highlights};
use syntax::lang_fall;

mod highlighting;
mod structure;

pub const FALL_EDITOR_SUPPORT: EditorSupport = EditorSupport {
    extension: "fall",
    parse: |text| EditorFile::new(FileWithAnalysis::new(lang_fall().parse(text))),
};

impl EditorFileImpl for FileWithAnalysis {
    fn file(&self) -> &File {
        self.file()
    }

    fn edit(&self, edit: &TextEdit) -> Box<EditorFileImpl> {
        let file = self.file().edit(edit);
        Box::new(FileWithAnalysis::new(file))
    }

    fn syntax_tree(&self) -> String {
        gen_syntax_tree(self.file())
    }

    fn highlight(&self) -> Highlights {
        self.analyse(|a| {
            a.file().metrics().measure_time("highlight", || {
                highlighting::highlight(a)
            })
        })
    }

    fn structure(&self) -> Vec<FileStructureNode> {
        structure::structure(self.file())
    }
}

