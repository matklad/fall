use fall_tree::{File, TextEdit, TextRange};
use fall_tree::visitor::{process_subtree_bottom_up};
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode};
use fall_editor::hl::{self, Highlights};
use *;

mod actions;
use self::actions::ACTIONS;

mod file_symbols;
use self::file_symbols::process_symbols;

mod symbol_index;
pub use self::symbol_index::SymbolIndex;

mod line_index;

pub struct RustEditorFile {
    file: File
}

impl RustEditorFile {
    fn new(file: File) -> RustEditorFile {
        RustEditorFile { file }
    }
}

impl EditorFileImpl for RustEditorFile {
    fn parse(text: &str) -> Self {
        RustEditorFile::new(lang_rust().parse(text))
    }

    fn edit(&self, edit: &TextEdit) -> Self {
        RustEditorFile::new(self.file.edit(edit))
    }

    fn file(&self) -> &File {
        &self.file
    }

    fn syntax_tree(&self) -> String {
        gen_syntax_tree(&self.file)
    }

    fn highlight(&self) -> Highlights {
        process_subtree_bottom_up(
            self.file.root(),
            hl::visitor(
                &[]
            ),
        )
    }

    fn structure(&self) -> Vec<FileStructureNode> {
        let mut nodes = Vec::new();
        process_symbols(self.file(), &mut|name, node| {
            nodes.push(FileStructureNode {
                name: name.to_string(),
                range: node.range(),
                children: Vec::new(),
            })
        });
        nodes
    }

    fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        let mut result = Vec::new();
        for &(action_id, action) in ACTIONS.iter() {
            if action(self.file(), range.start(), false).is_some() {
                result.push(action_id)
            }
        }
        result
    }

    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        for &(action_id, action) in ACTIONS.iter() {
            if action_id == id {
                let edit = action(self.file(), range.start(), true)?.into_edit();
                return Some(edit);
            }
        }
        None
    }
}
