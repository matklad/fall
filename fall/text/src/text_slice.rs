use ::{Text, TextRange, TextUnit};

pub trait TextSlice: Copy {
    fn into_proper_range(self, text: Text) -> TextRange;
}

impl TextSlice for TextRange {
    fn into_proper_range(self, _text: Text) -> TextRange {
        self
    }
}


#[derive(Copy, Clone)]
pub struct TextSuffix(TextUnit);

impl TextSuffix {
    pub fn from(offset: TextUnit) -> TextSuffix {
        TextSuffix(offset)
    }
}

impl TextSlice for TextSuffix {
    fn into_proper_range(self, text: Text) -> TextRange {
        TextRange::from_to(self.0, text.len())
    }
}
