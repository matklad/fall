extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;

mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;

pub mod editor {
    use fall_tree::{File, TextEdit};
    use fall_tree::visitor::process_subtree_bottom_up;
    use fall_editor::{EditorFileImpl, gen_syntax_tree};
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
                    &[
                        (hl::KEYWORD, &[
                            USE, IMPL, STRUCT, TRAIT, ENUM, UNION, FN, MOD, CRATE,
                            PUB, LET, IF, ELSE, RETURN, FOR, WHILE, LOOP, WHERE, MATCH,
                            EXTERN, SELF, SUPER,
                        ]),
                        (hl::STRING, &[STRING, RAW_STRING]),
                        (hl::COMMENT, &[LINE_COMMENT, BLOCK_COMMENT]),
                    ]
                ).visit::<FnDef, _>(|hls, fn_def| {
                    if let Some(ident) = fn_def.name_ident() {
                        hl::hl(ident, hl::FUNCTION, hls)
                    }
                }),
            )
        }
    }
}
