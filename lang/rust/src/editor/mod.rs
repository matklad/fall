use fall_tree::{File, TextEdit, TextRange};
use fall_tree::visitor::{process_subtree_bottom_up, visitor};
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode};
use fall_editor::hl::{self, Highlights};
use *;

mod actions;
use self::actions::ACTIONS;

mod file_symbols;

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
        fn process_name_owner<'f, D: NameOwner<'f>>(def: D, nodes: &mut Vec<FileStructureNode>) {
            if let Some(name_ident) = def.name_ident() {
                nodes.push(FileStructureNode {
                    name: name_ident.text().to_string(),
                    range: def.node().range(),
                    children: Vec::new(),
                })
            }
        }
        process_subtree_bottom_up(
            self.file().root(),
            visitor(Vec::new())
                .visit::<FnDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<StructDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<EnumDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<TypeDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<TraitDef, _>(|nodes, def| process_name_owner(def, nodes)),
        )
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
