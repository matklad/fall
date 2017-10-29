use text_edit::{TextEdit, combine_edits};
use {Node, File, TextRange};

pub struct FileEdit<'f> {
    file: &'f File,
    inserted: Vec<(Node<'f>, String)>,
    replaced: Vec<(Node<'f>, String)>,
    deleted: Vec<Node<'f>>,
}

impl<'f> FileEdit<'f> {
    pub fn new(file: &File) -> FileEdit {
        FileEdit {
            file,
            inserted: Vec::new(),
            replaced: Vec::new(),
            deleted: Vec::new(),
        }
    }

    pub fn replace(&mut self, node: Node<'f>, replacement: Node) {
        self.replace_with_text(node, replacement.text().to_string())
    }

    pub fn replace_with_text(&mut self, node: Node<'f>, replacement: String) {
        self.replaced.push((node, replacement))
    }

    pub fn replace_substring(&mut self, node: Node<'f>, range: TextRange, replacement: String) {
        assert!(range.is_subrange_of(node.range()));
        let file_text = self.file.text();
        let prefix = file_text.slice(TextRange::from_to(node.range().start(), range.start()));
        let suffix = file_text.slice(TextRange::from_to(range.end(), node.range().end()));
        let new_text = prefix.to_string()
            + &replacement
            + &suffix.to_cow();
        self.replace_with_text(node, new_text);
    }

    pub fn delete(&mut self, node: Node<'f>) {
        self.deleted.push(node)
    }

    pub fn insert_text_after(&mut self, anchor: Node<'f>, text: String) {
        self.inserted.push((anchor, text))
    }

    pub fn into_text_edit(self) -> TextEdit {
        self.text_edit_for_node(self.file.root())
    }

    fn text_edit_for_node(&self, node: Node<'f>) -> TextEdit {
        if self.deleted.iter().find(|&&n| n == node).is_some() {
            return TextEdit::delete_range(node.range());
        }

        if let Some(&(_, ref replacement)) = self.replaced.iter().find(|&&(n, _)| n == node) {
            return TextEdit::replace_range(node.range(), replacement.clone());
        }

        let mut result = TextEdit::empty();

        for child in node.children() {
            let new_edit = combine_edits(
                self.file.text(),
                &result,
                &self.text_edit_for_node(child)
            ).expect("conflicting edits");
            result = new_edit;
        }

        if let Some(&(_, ref replacement)) = self.inserted.iter().find(|&&(n, _)| n == node) {
            let insert = TextEdit::insert(node.range().end(), replacement.clone());
            let new_edit = combine_edits(self.file.text(), &result, &insert)
                .expect("conflicting edits");
            result = new_edit;
        }
        result
    }
}
