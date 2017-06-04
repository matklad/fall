use serde::{Serialize, Serializer};
use std::fmt;
use std::borrow::Cow;
use std::ops;

#[derive(Clone, Copy)]
pub struct Text<'f> {
    owned: &'f String,
    range: TextRange
}

impl<'f> Text<'f> {
    pub fn from_owned(owned: &String) -> Text {
        Text {
            owned: owned,
            range: TextRange::from_to(0, owned.len() as u32)
        }
    }

    pub fn len(&self) -> u32 {
        self.range.end() - self.range.start()
    }

    pub fn slice(&self, r: TextRange) -> Text<'f> {
        assert!(r.end() <= self.len());
        let start = self.range.start() + r.start();
        assert!(start <= self.range.end());
        Text {
            owned: self.owned,
            range: TextRange::from_to(start, start + (r.end() - r.start())),
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.as_str().starts_with(prefix)
    }

    pub fn ends_with(&self, suffix: &str) -> bool {
        self.as_str().ends_with(suffix)
    }

    pub fn find(&self, needle: &str) -> Option<TextUnit> {
        self.as_str().find(needle).map(|off| TextUnit(off as u32))
    }

    pub fn rfind(&self, needle: &str) -> Option<TextUnit> {
        self.as_str().rfind(needle).map(|off| TextUnit(off as u32))
    }

    pub fn trim(&self) -> Text<'f> {
        fn non_ws(c: char) -> bool {
            !c.is_whitespace()
        }

        let left = self.as_str()
            .find(non_ws)
            .unwrap_or(self.as_str().len());
        let s = &self.as_str()[left..];
        let right = s
            .rfind(non_ws)
            .map(|last| last + s[last..].chars().next().unwrap().len_utf8())
            .unwrap_or(0);
        self.slice(TextRange::from_to(left as u32, (left + right) as u32))
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_owned()
    }

    pub fn to_cow(&self) -> Cow<'f, str> {
        Cow::Borrowed(self.as_str())
    }

    fn as_str(&self) -> &'f str {
        &self.owned[self.range]
    }
}

impl<'f> Serialize for Text<'f> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        self.as_str().serialize(serializer)
    }
}

impl<'f> fmt::Display for Text<'f> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'f> fmt::Debug for Text<'f> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}


impl<'a, 'b> PartialEq<Text<'b>> for Text<'a> {
    fn eq(&self, other: &Text<'b>) -> bool {
        self.as_str() == other.as_str()
    }
}


impl<'f, 's> PartialEq<&'s str> for Text<'f> {
    fn eq(&self, other: &&str) -> bool {
        &self.owned[self.range] == *other
    }
}

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

    pub fn from_to_off(start: TextUnit, end: TextUnit) -> TextRange {
        TextRange::from_to(start.0, end.0)
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn len(&self) -> TextUnit {
        TextUnit(self.end - self.start)
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
pub struct TextUnit(u32);

impl TextUnit {
    pub fn zero() -> TextUnit {
        TextUnit(0)
    }

    pub fn in_range(range: TextRange, off: usize) -> Option<TextUnit> {
        let off = TextUnit(off as u32);
        if is_offset_in_range(off, range) {
            Some(off)
        } else {
            None
        }
    }
}

impl ops::Add<u32> for TextUnit {
    type Output = TextUnit;
    fn add(self, rhs: u32) -> TextUnit {
        TextUnit(self.0 + rhs)
    }
}

impl ops::Add<TextUnit> for TextUnit {
    type Output = TextUnit;
    fn add(self, rhs: TextUnit) -> TextUnit {
        TextUnit(self.0 + rhs.0)
    }
}

impl ops::AddAssign<TextUnit> for TextUnit {
    fn add_assign(&mut self, rhs: TextUnit) {
        self.0 += rhs.0
    }
}

impl ::std::iter::Sum for TextUnit {
    fn sum<I: Iterator<Item=TextUnit>>(iter: I) -> Self {
        TextUnit(iter.map(|u| u.0).sum())
    }
}

impl ops::Sub<u32> for TextUnit {
    type Output = TextUnit;
    fn sub(self, rhs: u32) -> TextUnit {
        TextUnit(self.0 - rhs)
    }
}

impl fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub fn is_offset_in_range(offset: TextUnit, range: TextRange) -> bool {
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
