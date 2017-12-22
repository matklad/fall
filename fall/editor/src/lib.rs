extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fall_tree;

use fall_tree::{File, dump_file, TextEdit, TextRange, FileEdit};
use fall_tree::test_util;

pub mod hl;
mod extend_selection;

use self::hl::Highlights;

#[derive(Serialize, Debug)]
pub struct FileStructureNode {
    pub name: String,
    pub range: TextRange,
    pub children: Vec<FileStructureNode>
}

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Severity { Error, Warning }

#[derive(Serialize, Debug, Clone)]
pub struct Diagnostic {
    pub range: TextRange,
    pub severity: Severity,
    pub message: String
}

pub trait EditorFileImpl: Sync + 'static {
    fn parse(text: &str) -> Self;
    fn edit(&self, edit: &TextEdit) -> Self;

    fn metrics(&self) -> String {
        self.file().metrics().to_string()
    }

    fn syntax_tree(&self) -> String;
    fn extend_selection(&self, range: TextRange) -> Option<TextRange> {
        extend_selection::extend_selection(self.file(), range)
    }
    fn structure(&self) -> Vec<FileStructureNode> {
        Vec::new()
    }
    fn reformat(&self) -> TextEdit {
        FileEdit::new(self.file()).into_text_edit()
    }

    fn highlight(&self) -> Highlights {
        Vec::new()
    }
    fn diagnostics(&self) -> Vec<Diagnostic> {
        Vec::new()
    }

    fn context_actions(&self, _range: TextRange) -> Vec<&'static str> {
        Vec::new()
    }
    fn apply_context_action(&self, _range: TextRange, _id: &str) -> Option<TextEdit> {
        None
    }

    fn file(&self) -> &File;
}


pub fn gen_syntax_tree(file: &File) -> String {
    dump_file(&file)
}

pub fn check_context_action<E: EditorFileImpl>(
    action_id: &str,
    before: &str,
    after: &str
) {
    let (before, range) = test_util::extract_range(before, "^");
    let file = E::parse(&before);
    let actions = file.context_actions(range);
    if !actions.contains(&action_id) {
        panic!("Action `{}` is not avialable", action_id);
    }
    match file.apply_context_action(range, action_id) {
        None => panic!("Failed to apply `{}` action", action_id),
        Some(edit) => {
            let actual = edit.apply(file.file().text());
            test_util::report_diff(after.trim(), actual.as_text().to_cow().trim())
        }
    }
}
