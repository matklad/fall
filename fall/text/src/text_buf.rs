use ::{Text, TextRange, tu};
use std::fmt;

#[derive(Clone)]
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

impl<'s> From<Text<'s>> for TextBuf {
    fn from(s: Text<'s>) -> Self {
        TextBuf { data: s.to_string() }
    }
}

impl TextBuf {
    pub fn as_text(&self) -> Text {
        Text {
            owned: &self.data,
            range: TextRange::from_len(tu(0), tu(self.data.len() as u32)),
        }
    }
}

impl PartialEq<str> for TextBuf {
    fn eq(&self, other: &str) -> bool {
        self.as_text() == other
    }
}

impl PartialEq<TextBuf> for str {
    fn eq(&self, other: &TextBuf) -> bool {
        other == self
    }
}

impl<'s> PartialEq<&'s str> for TextBuf {
    fn eq(&self, other: &&'s str) -> bool {
        self.as_text() == *other
    }
}

impl<'s> PartialEq<TextBuf> for &'s str {
    fn eq(&self, other: &TextBuf) -> bool {
        other == self
    }
}

impl fmt::Display for TextBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_text().fmt(f)
    }
}

impl fmt::Debug for TextBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_text().fmt(f)
    }
}
