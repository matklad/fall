use serde::{Serialize, Serializer};
use std::fmt;
use std::borrow::Cow;

mod text_unit;
mod text_range;

pub use self::text_unit::TextUnit;
pub use self::text_range::TextRange;

#[derive(Clone, Copy)]
pub struct Text<'f> {
    owned: &'f String,
    range: TextRange
}

impl<'f> Text<'f> {
    pub fn from_owned(owned: &String) -> Text {
        Text {
            owned,
            range: TextRange::from_to(TextUnit::zero(), TextUnit::measure(owned))
        }
    }

    pub fn len(&self) -> TextUnit {
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

    pub fn contains(&self, needle: &str) -> bool {
        self.as_str().contains(needle)
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
        self.slice(TextRange::from_to(
            TextUnit(left as u32),
            TextUnit((left + right) as u32)
        ))
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
