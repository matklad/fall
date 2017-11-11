use ::{Text, TextRange, TextUnit, tu};
use std::fmt;

pub struct TextBuf {
    data: String
}

impl From<String> for TextBuf {
    fn from(s: String) -> Self {
        TextBuf { data: s }
    }
}

impl<'s> From<&'s str> for TextBuf {
    fn from(s: &'s str) -> Self {
        TextBuf { data: s.to_owned() }
    }
}

impl TextBuf {
    pub fn as_slice(&self) -> Text {
        Text {
            owned: &self.data,
            range: TextRange::from_len(tu(0), tu(self.data.len() as u32)),
        }
    }

    pub fn to_string(&self) -> String {
        self.data.clone()
    }

    pub fn slice(&self, range: TextRange) -> Text {
        self.as_slice().slice(range)
    }

    pub fn len(&self) -> TextUnit {
        self.as_slice().len()
    }
}

impl PartialEq<str> for TextBuf {
    fn eq(&self, other: &str) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<TextBuf> for str {
    fn eq(&self, other: &TextBuf) -> bool {
        other == self
    }
}

impl<'s> PartialEq<&'s str> for TextBuf {
    fn eq(&self, other: &&'s str) -> bool {
        self.as_slice() == *other
    }
}

impl<'s> PartialEq<TextBuf> for &'s str {
    fn eq(&self, other: &TextBuf) -> bool {
        other == self
    }
}

impl fmt::Display for TextBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl fmt::Debug for TextBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}
