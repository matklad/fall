extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fall_tree;

use std::sync::Arc;
use fall_tree::{File, dump_file, TextEdit, TextRange, FileEdit};

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

#[derive(Clone)]
pub struct EditorFile {
    imp: Arc<EditorFileImpl>
}

impl EditorFile {
    pub fn new<F: EditorFileImpl>(imp: F) -> EditorFile {
        EditorFile { imp: Arc::new(imp) }
    }

    pub fn edit(&self, edit: &TextEdit) -> EditorFile {
        EditorFile { imp: self.imp.edit(edit).into() }
    }
    pub fn file(&self) -> &File {
        self.imp.file()
    }
    pub fn syntax_tree(&self) -> String {
        self.imp.syntax_tree()
    }
    pub fn highlight(&self) -> Highlights {
        self.imp.highlight()
    }
    pub fn metrics(&self) -> String {
        self.file().metrics().to_string()
    }
    pub fn structure(&self) -> Vec<FileStructureNode> {
        self.imp.structure()
    }
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.imp.diagnostics()
    }
    pub fn extend_selection(&self, range: TextRange) -> Option<TextRange> {
        self.imp.extend_selection(range)
    }
    pub fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        self.imp.context_actions(range)
    }
    pub fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        self.imp.apply_context_action(range, id)
    }
    pub fn reformat(&self) -> TextEdit {
        self.imp.reformat()
    }
}

pub trait EditorFileImpl: Sync + 'static {
    fn file(&self) -> &File;
    fn edit(&self, edit: &TextEdit) -> Box<EditorFileImpl>;
    fn syntax_tree(&self) -> String;
    fn highlight(&self) -> Highlights {
        Vec::new()
    }
    fn structure(&self) -> Vec<FileStructureNode> {
        Vec::new()
    }
    fn diagnostics(&self) -> Vec<Diagnostic> {
        Vec::new()
    }
    fn extend_selection(&self, range: TextRange) -> Option<TextRange> {
        extend_selection::extend_selection(self.file(), range)
    }
    fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        Vec::new()
    }
    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        None
    }
    fn reformat(&self) -> TextEdit {
        FileEdit::new(self.file()).into_text_edit()
    }
}

#[derive(Clone, Copy)]
pub struct EditorSupport {
    pub extension: &'static str,
    pub parse: fn(text: &str) -> EditorFile,
}

impl EditorSupport {
    pub fn parse(&self, text: &str) -> EditorFile {
        (self.parse)(text)
    }
}

pub fn gen_syntax_tree(file: &File) -> String {
    dump_file(&file)
}
