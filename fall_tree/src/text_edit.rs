use std::cmp::{min, max, Ordering, PartialOrd};
use {Text, TextRange, TextUnit};

#[derive(Clone)]
pub struct TextEdit {
    pub delete: TextRange,
    pub insert: String,
}

impl TextEdit {
    pub fn insert(offset: TextUnit, text: String) -> TextEdit {
        TextEdit { delete: TextRange::from_len(offset, TextUnit::zero()), insert: text }
    }

    pub fn delete_range(range: TextRange) -> TextEdit {
        TextEdit { delete: range, insert: String::new() }
    }

    pub fn replace_range(range: TextRange, replacement: String) -> TextEdit {
        TextEdit { delete: range, insert: replacement }
    }

    pub fn empty() -> TextEdit {
        TextEdit { delete: TextRange::empty(), insert: String::new() }
    }

    pub fn apply(&self, text: Text) -> String {
        let before = text.slice(TextRange::from_to(TextUnit::zero(), self.delete.start()));
        let after = text.slice(TextRange::from_to(self.delete.end(), text.len()));
        before.to_string() + &self.insert + &after.to_string()
    }
}

pub(crate) fn combine_edits(text: Text, edit1: &TextEdit, edit2: &TextEdit) -> Option<TextEdit> {
    if is_empty_edit(edit1) {
        return Some(edit2.clone());
    }
    if is_empty_edit(edit2) {
        return Some(edit1.clone());
    }

    let (left, right) = match edit1.delete.partial_cmp(&edit2.delete) {
        Some(Ordering::Less) => (edit1, edit2),
        Some(Ordering::Greater) => (edit2, edit1),
        _ => return None
    };

    let delete = covering_range(left.delete, right.delete);
    let insert = left.insert.clone()
        + text.slice(TextRange::from_to(left.delete.end(), right.delete.start())).to_cow().as_ref()
        + &right.insert;
    Some(TextEdit { delete: delete, insert: insert })
}

fn is_empty_edit(edit: &TextEdit) -> bool {
    edit.delete.is_empty() && edit.insert.is_empty()
}

fn covering_range(range1: TextRange, range2: TextRange) -> TextRange {
    TextRange::from_to(
        min(range1.start(), range2.start()),
        max(range1.end(), range2.end()),
    )
}
