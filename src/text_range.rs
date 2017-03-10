#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl TextRange {
    pub fn is_subrange_of(self, other: TextRange) -> bool {
        other.start <= self.start && self.end <= other.end
    }
}

impl ::std::ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start as usize..index.end as usize]
    }
}
