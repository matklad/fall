use ordslice::Ext;
use fall_tree::{TextUnit, tu, Text};

pub struct LineIndex {
    newlines: Vec<TextUnit>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

impl LineIndex {
    pub fn new(text: Text) -> LineIndex {
        let mut newlines = vec![tu(0)];
        let mut curr = tu(0);
        for c in text.to_string().chars() {
            curr += tu(c.len_utf8() as u32);
            if c == '\n' {
                newlines.push(curr);
            }
        }
        LineIndex { newlines }
    }

    pub fn translate(&self, offset: TextUnit) -> LineCol {
        let line = self.newlines.upper_bound(&offset) - 1;
        let line_start_offset = self.newlines[line];
        let col = offset - line_start_offset;
        return LineCol { line: line as u32, col: col.into() }
    }
}

#[test]
fn test_line_index() {
    use fall_tree::TextBuf;

    let text = "hello\nworld";
    let text = TextBuf::from(text);
    let index = LineIndex::new(text.as_slice());
    assert_eq!(index.translate(tu(0)), LineCol { line: 0, col: 0});
    assert_eq!(index.translate(tu(1)), LineCol { line: 0, col: 1});
    assert_eq!(index.translate(tu(5)), LineCol { line: 0, col: 5});
    assert_eq!(index.translate(tu(6)), LineCol { line: 1, col: 0});
    assert_eq!(index.translate(tu(7)), LineCol { line: 1, col: 1});
    assert_eq!(index.translate(tu(8)), LineCol { line: 1, col: 2});
    assert_eq!(index.translate(tu(10)), LineCol { line: 1, col: 4});
    assert_eq!(index.translate(tu(11)), LineCol { line: 1, col: 5});
    assert_eq!(index.translate(tu(12)), LineCol { line: 1, col: 6});

    let text = "\nhello\nworld";
    let text = TextBuf::from(text);
    let index = LineIndex::new(text.as_slice());
    assert_eq!(index.translate(tu(0)), LineCol { line: 0, col: 0});
    assert_eq!(index.translate(tu(1)), LineCol { line: 1, col: 0});
    assert_eq!(index.translate(tu(2)), LineCol { line: 1, col: 1});
    assert_eq!(index.translate(tu(6)), LineCol { line: 1, col: 5});
    assert_eq!(index.translate(tu(7)), LineCol { line: 2, col: 0});
}