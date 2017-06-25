use {TextRange};

pub struct TextEdit {
    pub delete: TextRange,
    pub insert: String,
}
