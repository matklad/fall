extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;

mod rust;

pub use self::rust::*;
pub use self::rust::language as lang_rust;

pub use self::editor::RUST_EDITOR_SUPPORT;

mod editor {
    use fall_tree::File;
    use fall_tree::visitor::process_subtree_bottom_up;
    use fall_editor::{EditorSupport, gen_parse, gen_syntax_tree};
    use fall_editor::hl;
    use *;

    pub const RUST_EDITOR_SUPPORT: EditorSupport = EditorSupport {
        extension: "rs",
        parse: |text| gen_parse(lang_rust(), text),
        syntax_tree: Some(|file| gen_syntax_tree(lang_rust(), file)),
        highlight: Some(highlight),
    };

    fn highlight(file: &File) -> hl::Highlights {
        process_subtree_bottom_up(
            file.root(),
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
