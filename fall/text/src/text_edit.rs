use crate::{TextUnit, TextRange, TextBuf, Text, tu};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct TextEdit {
    pub ops: Vec<TextEditOp>,
}

#[derive(Clone, Debug)]
pub enum TextEditOp {
    Copy(TextRange), // TODO: check for disjoint ranges
    Insert(TextBuf),
}


impl TextEdit {
    pub fn apply(&self, text: Text) -> TextBuf {
        let mut result = String::new();
        for s in self.ops.iter() {
            match *s {
                TextEditOp::Copy(range) => result += &text.slice(range).to_cow(),
                TextEditOp::Insert(ref i) => result += &i.as_text().to_cow(),
            }
        }

        result.into()
    }
}

pub struct TextEditBuilder {
    segments: Vec<TextEditOp>,
    last_offset: TextUnit,
    text_len: TextUnit,

}

impl TextEditBuilder {
    pub fn new(text: Text) -> TextEditBuilder {
        TextEditBuilder {
            segments: Vec::new(),
            last_offset: tu(0),
            text_len: text.len()
        }
    }

    pub fn build(mut self) -> TextEdit {
        let len = self.text_len;
        self.advance_to(len);
        TextEdit { ops: self.segments }
    }

    pub fn insert<T: Into<TextBuf>>(&mut self, offset: TextUnit, text: T) {
        self.advance_to(offset);
        self.insert_(text.into());
    }

    pub fn delete(&mut self, range: TextRange) {
        self.advance_to(range.start());
        self.delete_len(range.len());
    }

    pub fn replace<T: Into<TextBuf>>(&mut self, range: TextRange, text: T) {
        self.advance_to(range.start());
        self.insert_(text.into());
        self.delete_len(range.len());
    }

    fn advance_to(&mut self, offset: TextUnit) {
        match self.last_offset.cmp(&offset) {
            Ordering::Less => self.copy_up_to(offset),
            Ordering::Equal => (),
            Ordering::Greater => panic!("Invalid edit"),
        }
    }

    fn copy_up_to(&mut self, offset: TextUnit) {
        let len = offset - self.last_offset;
        self.copy_len(len)
    }

    fn copy_len(&mut self, len: TextUnit) {
        let range = TextRange::from_len(self.last_offset, len);
        self.segments.push(TextEditOp::Copy(range));
        self.last_offset += len
    }

    fn insert_(&mut self, text: TextBuf) {
        self.segments.push(TextEditOp::Insert(text))
    }

    fn delete_len(&mut self, len: TextUnit) {
        self.last_offset += len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edits() {
        let text: TextBuf = "Hello, World!".into();
        let edit = {
            let mut e = TextEditBuilder::new(text.as_text());
            e.replace(TextRange::from_len(tu(0), tu(5)), "Goodbye");
            e.insert(tu(7), "cruel ");
            e.delete(TextRange::from_len(tu(12), tu(1)));
            e.build()
        };
        let new_text = edit.apply(text.as_text());
        assert_eq!(new_text, "Goodbye, cruel World");
    }
}
