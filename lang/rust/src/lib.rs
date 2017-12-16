extern crate fall_parse;
extern crate fall_editor;


mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;

pub use self::editor::RUST_EDITOR_SUPPORT;

mod editor {
    use fall_editor::{EditorSupport, gen_parse, gen_syntax_tree};
    use lang_rust;

    pub const RUST_EDITOR_SUPPORT: EditorSupport = EditorSupport {
        extension: "rs",
        parse: |text| gen_parse(lang_rust(), text),
        syntax_tree: Some(|file| gen_syntax_tree(lang_rust(), file)),
    };
}
