use rand::Rand;
use quickcheck::{Arbitrary, Gen};
use itertools::Itertools;

use ::{TextEdit, TextEditOp, Text, TextUnit, TextRange, tu};

#[derive(Clone, Debug)]
pub struct ArbitraryEdit {
    inner: TextEdit
}

impl ArbitraryEdit {
    pub fn into_text_edit(self, text: Text) -> TextEdit {
        let len: TextUnit = text.len();
        let scale = |x: TextUnit| scale(x, tu(LEN), len);
        TextEdit {
            ops: self.inner.ops.into_iter().map(|op| match op {
                TextEditOp::Copy(range) => TextEditOp::Copy(
                    TextRange::from_to(scale(range.start()), scale(range.end()))
                ),
                insert @ TextEditOp::Insert(_) => insert,
            }).collect()
        }
    }
}

const LEN: u32 = 1000;

impl Arbitrary for ArbitraryEdit {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let n_copies = (usize::rand(g) % 5) as usize;
        let mut copy_ends = ::rand::sample(g, 0..(LEN + 1), 2 * n_copies);
        copy_ends.sort();

        let mut ops = Vec::new();
        for (b, e) in copy_ends.into_iter().map(tu).tuples() {
            ops.push(TextEditOp::Copy(TextRange::from_to(b, e)))
        }
        let inner = TextEdit { ops };
        ArbitraryEdit { inner }
    }
}

fn scale(x: TextUnit, from_range: TextUnit, to_range: TextUnit) -> TextUnit {
    let x: u32 = x.into();
    let from_range: u32 = from_range.into();
    let to_range: u32 = to_range.into();
    tu(x * to_range / from_range)
}
