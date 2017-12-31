use {TextEdit, TextEditBuilder, Node, File, TextRange, TextEditOp, tu};
use search::find_covering_node;

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

    pub fn replace_substring(&mut self, range: TextRange, replacement: String) {
        let root = self.file.root();
        assert!(range.is_subrange_of(root.range()));
        let node = find_covering_node(root, range);
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
        let mut edit_builder = TextEditBuilder::new(self.file.text());
        self.text_edit_for_node(self.file.root(), &mut edit_builder);
        // TODO: :(
        let mut edit = edit_builder.build();
        edit.ops.push(TextEditOp::Copy(TextRange::from_len(
            self.file.text().len(),
            tu(0),
        )));
        edit
    }

    fn text_edit_for_node(&self, node: Node<'f>, edit_builder: &mut TextEditBuilder) {
        if self.deleted.iter().find(|&&n| n == node).is_some() {
            edit_builder.delete(node.range());
            return;
        }

        if let Some(&(_, ref replacement)) = self.replaced.iter().find(|&&(n, _)| n == node) {
            edit_builder.replace(node.range(), replacement.as_ref());
            return;
        }

        for child in node.children() {
            self.text_edit_for_node(child, edit_builder);
        }

        if let Some(&(_, ref replacement)) = self.inserted.iter().find(|&&(n, _)| n == node) {
            edit_builder.insert(node.range().end(), replacement.as_ref());
        }
    }
}
