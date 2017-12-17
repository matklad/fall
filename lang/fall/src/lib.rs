extern crate itertools;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rental;

extern crate fall_tree;
extern crate fall_parse;
extern crate fall_editor;
pub extern crate lang_fall_syntax as syntax;

use fall_tree::{AstNode, File};

mod analysis;
pub mod editor_api;


pub use self::analysis::{Analysis, FileWithAnalysis, CallKind, RefKind, PratVariant, PrattOp,
                         MethodKind, Arity, ChildKind};

pub fn parse<S: Into<String>>(text: S) -> File {
    syntax::lang_fall().parse(text.into())
}

pub fn ast(file: &File) -> syntax::FallFile {
    syntax::FallFile::wrap(file.root()).unwrap()
}

pub use self::editor::FALL_EDITOR_SUPPORT;

mod editor {
    use analysis::{Analysis, FileWithAnalysis};

    use fall_tree::{File, TextEdit};
    use fall_editor::{EditorSupport, EditorFile, EditorFileImpl, gen_syntax_tree};
    use fall_editor::hl::{self, Highlights};
    use syntax::lang_fall;

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
            self.analyse(|a| ::editor_api::highlight(a))
        }
    }
}

#[cfg(test)]
mod test_util {
    use fall_tree::{File, TextUnit, TextRange};

    pub fn parse_with_caret(text: &str) -> (File, TextUnit) {
        ::fall_tree::test_util::parse_with_caret(::syntax::lang_fall(), text, "^")
    }

    pub fn parse_with_range(text: &str) -> (File, TextRange) {
        ::fall_tree::test_util::parse_with_range(::syntax::lang_fall(), text, "^")
    }
}
