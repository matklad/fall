use ::{Text, TextRange, TextUnit, tu};

pub struct TextBuf {
    data: String
}

impl From<String> for TextBuf {
    fn from(s: String) -> Self {
        TextBuf { data: s }
    }
}

impl TextBuf {
    pub fn as_slice(&self) -> Text {
        Text {
            owned: &self.data,
            range: TextRange::from_len(tu(0), tu(self.data.len() as u32)),
        }
    }

    pub fn slice(&self, range: TextRange) -> Text {
        self.as_slice().slice(range)
    }

    pub fn len(&self) -> TextUnit {
        self.as_slice().len()
    }
}
