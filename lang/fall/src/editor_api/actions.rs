use fall_tree::{File, Node, TextRange, TextUnit};
use fall_tree::edit::TextEdit;

pub struct FileChange<'f> {
    edit: TextEdit,
    file: &'f File
}

impl<'f> FileChange<'f> {
    fn empty(file: &File) -> FileChange {
        FileChange {
            edit: TextEdit { delete: TextRange::empty(), insert: String::new() },
            file: file
        }
    }

    fn replace(&mut self, source: Node<'f>, target: Node) {
        if !(source.range().is_disjoint(self.edit.delete)) {
            panic!("Invalid edit")
        }
        if is_empty_edit(&self.edit) {
            self.edit = TextEdit {
                delete: source.range(),
                insert: target.text().to_string(),
            };
            return;
        }
        
    }
}

fn is_empty_edit(edit: &TextEdit) -> bool {
    edit.delete.is_empty() && edit.insert.is_empty()
}

trait ContextAction {
    fn apply<'f>(&self, file: &'f File, offset: TextUnit) -> Option<FileChange<'f>>;
}


struct SwapAlternatives;

impl ContextAction for SwapAlternatives {
    fn apply<'f>(&self, file: &'f File, offset: TextUnit) -> Option<FileChange<'f>> {
        let (left, right) = match find_swappable_nodes(file, offset) {
            Some((left, right)) => (left, right),
            None => return None
        };
        let mut change = FileChange::empty(file);
        change.replace(left, right);
        change.replace(right, left);
        Some(change)
    }
}

fn find_swappable_nodes<'f>(file: &'f File, offset: TextUnit) -> Option<(Node<'f>, Node<'f>)> {
    unimplemented!()
}

