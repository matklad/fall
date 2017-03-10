#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: u32,
    end: u32,
}

impl TextRange {
    pub fn from_to(start: u32, end: u32) -> TextRange {
        assert!(start <= end);
        TextRange { start: start, end: end }
    }

    pub fn empty() -> TextRange {
        TextRange::from_to(0, 0)
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn is_subrange_of(self, other: TextRange) -> bool {
        other.start <= self.start && self.end <= other.end
    }

    pub fn glue(&self, right: TextRange) -> TextRange {
        assert_eq!(self.end, right.start);
        TextRange { start: self.start, end: right.end }
    }
}

impl ::std::ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start as usize..index.end as usize]
    }
}
