use std::cmp::Ordering;
use std::fmt;
use std::ops;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use super::{TextUnit, tu};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    start: TextUnit,
    end: TextUnit,
}

impl fmt::Debug for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {})", self.start(), self.end())
    }
}

impl Serialize for TextRange {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.start, self.end).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TextRange {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (start, end) = Deserialize::deserialize(deserializer)?;
        Ok(TextRange { start, end })
    }
}


impl TextRange {
    pub fn empty() -> TextRange {
        TextRange::from_to(tu(0), tu(0))
    }

    pub fn from_to(from: TextUnit, to: TextUnit) -> TextRange {
        TextRange { start: from, end: to }
    }

    pub fn from_len(from: TextUnit, len: TextUnit) -> TextRange {
        TextRange::from_to(from, from + len)
    }

    pub fn start(&self) -> TextUnit {
        self.start
    }

    pub fn end(&self) -> TextUnit {
        self.end
    }

    pub fn len(&self) -> TextUnit {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start() == self.end()
    }

    pub fn is_subrange_of(&self, other: TextRange) -> bool {
        other.start() <= self.start() && self.end() <= other.end()
    }

    pub fn intersects(&self, other: TextRange) -> bool {
        !self.disjoint(other)
    }

    pub fn disjoint(&self, other: TextRange) -> bool {
        self.end() <= other.start() || other.end() <= self.start()
    }

    pub fn contains_offset_nonstrict(&self, offset: TextUnit) -> bool {
        self.start() <= offset && offset <= self.end()
    }

    pub fn glue(&self, right: TextRange) -> TextRange {
        assert_eq!(self.end(), right.start());
        TextRange::from_to(self.start(), right.end())
    }

    pub fn shift_right(&self, offset: TextUnit) -> TextRange {
        TextRange::from_len(self.start() + offset, self.len())
    }
}


impl ops::Index<TextRange> for str {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start().0 as usize..index.end().0 as usize]
    }
}

impl ops::Index<TextRange> for String {
    type Output = str;

    fn index(&self, index: TextRange) -> &str {
        &self[index.start().0 as usize..index.end().0 as usize]
    }
}

impl PartialOrd for TextRange {
    fn partial_cmp(&self, other: &TextRange) -> Option<Ordering> {
        if self.end() <= other.start() {
            Some(Ordering::Less)
        } else if other.end() <= self.start() {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
