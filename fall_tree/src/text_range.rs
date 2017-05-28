use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: u32,
    end: u32,
}

impl fmt::Debug for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {})", self.start(), self.end())
    }
}

impl TextRange {
    pub fn from_to(start: u32, end: u32) -> TextRange {
        assert!(start <= end, "Invalid range, start: {} end: {}", start, end);
        TextRange { start: start, end: end }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn empty() -> TextRange {
        TextRange::from_to(0, 0)
    }

    pub fn is_subrange_of(self, other: TextRange) -> bool {
        other.start() <= self.start() && self.end() <= other.end()
    }

    pub fn is_empty(&self) -> bool {
        self.start() == self.end()
    }

    pub fn glue(&self, right: TextRange) -> TextRange {
        assert_eq!(self.end(), right.start());
        TextRange::from_to(self.start(), right.end())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextOffset(u32);

impl TextOffset {
    pub fn in_range(range: TextRange, off: usize) -> Option<TextOffset> {
        let off = TextOffset(off as u32);
        if is_offset_in_range(off, range) {
            Some(off)
        } else {
            None
        }
    }
}

impl fmt::Debug for TextOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub fn is_offset_in_range(offset: TextOffset, range: TextRange) -> bool {
    return range.start <= offset.0 && offset.0 <= range.end
}


impl ::std::ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start() as usize..index.end() as usize]
    }
}

impl ::std::ops::Index<TextRange> for String {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start() as usize..index.end() as usize]
    }
}
