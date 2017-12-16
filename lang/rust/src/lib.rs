extern crate fall_parse;
extern crate fall_editor;


mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;

pub use self::editor::RUST_EDITOR_SUPPORT;

mod editor {
    use fall_editor::{EditorSupport, gen_syntax_tree};
    use lang_rust;

    pub const RUST_EDITOR_SUPPORT: EditorSupport = EditorSupport {
        extension: "rs",
        syntax_tree: Some(|text| gen_syntax_tree(lang_rust(), text)),
    };
}
