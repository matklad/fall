use ::{TextUnit, TextRange, TextBuf, Text, tu};
use std::cmp::Ordering;

pub struct TextEdit {
    segments: Vec<Segment>,
}

impl TextEdit {
    pub fn apply(self, text: Text) -> TextBuf {
        let mut result = String::new();
        for s in self.segments {
            match s {
                Segment::Copy(range) => result += &text.slice(range).to_cow(),
                Segment::Insert(i) => result += &i.as_slice().to_cow(),
            }
        }

        result.into()
    }
}

pub struct TextEditBuilder {
    segments: Vec<Segment>,
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
        TextEdit { segments: self.segments }
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
        self.segments.push(Segment::Copy(range));
        self.last_offset += len
    }

    fn insert_(&mut self, text: TextBuf) {
        self.segments.push(Segment::Insert(text))
    }

    fn delete_len(&mut self, len: TextUnit) {
        self.last_offset += len
    }
}

enum Segment {
    Copy(TextRange),
    Insert(TextBuf),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edits() {
        let text: TextBuf = "Hello, World!".into();
        let edit = {
            let mut e = TextEditBuilder::new(text.as_slice());
            e.replace(TextRange::from_len(tu(0), tu(5)), "Goodbye");
            e.insert(tu(7), "cruel ");
            e.delete(TextRange::from_len(tu(12), tu(1)));
            e.build()
        };
        let new_text = edit.apply(text.as_slice());
        assert_eq!(new_text, "Goodbye, cruel World");
    }
}
