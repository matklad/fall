extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;

mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;

pub mod editor {
    use fall_tree::{File, TextEdit, AstNode};
    use fall_tree::visitor::{process_subtree_bottom_up, visitor};
    use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode};
    use fall_editor::hl::{self, Highlights};
    use *;

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
                ).visit::<FnDef, _>(|hls, fn_def| {
                    if let Some(ident) = fn_def.name_ident() {
                        hl::hl(ident, hl::FUNCTION, hls)
                    }
                }),
            )
        }

        fn structure(&self) -> Vec<FileStructureNode> {
            process_subtree_bottom_up(
                self.file().root(),
                visitor(Vec::new())
                    .visit::<FnDef, _>(|nodes, fn_def| {
                        if let Some(name_ident) = fn_def.name_ident() {
                            nodes.push(FileStructureNode {
                                name: name_ident.text().to_string(),
                                range: fn_def.node().range(),
                                children: Vec::new(),
                            })
                        }
                    }),
            )
        }
    }
}
