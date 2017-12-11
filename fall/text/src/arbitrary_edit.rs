use rand::Rand;
use quickcheck::{Arbitrary, Gen};
use itertools::Itertools;

use ::{TextEdit, TextEditOp, Text, TextUnit, TextRange, tu};

#[derive(Clone, Debug)]
pub struct ArbitraryEdit {
    ops: Vec<Op>
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Copy(TextRange),
    Insert(TextRange),
}

impl ArbitraryEdit {
    pub fn into_text_edit(self, text: Text) -> TextEdit {
        let len: TextUnit = text.len();
        let scale_unit = |x: TextUnit| scale(x, tu(LEN), len);
        let scale_range = |r: TextRange| {
            let start = scale_unit(r.start());
            let end = scale_unit(r.end());
            TextRange::from_to(start, end)
        };
        TextEdit {
            ops: self.ops.into_iter().map(|op| match op {
                Op::Copy(range) => {
                    TextEditOp::Copy(scale_range(range))
                },
                Op::Insert(range) => {
                    TextEditOp::Insert(text.slice(scale_range(range)).to_text_buf())
                },
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
            let op = if u32::rand(g) % 3 == 0 {
                let start = u32::rand(g) % (LEN + 1);
                let end = u32::rand(g) % (LEN + 1);
                let (start, end) = if start < end { (start, end) } else { (end, start) };
                Op::Insert(TextRange::from_to(tu(start), tu(end)))
            } else {
                Op::Copy(TextRange::from_to(b, e))
            };
            ops.push(op)

        }
        ArbitraryEdit { ops }
    }
}

fn scale(x: TextUnit, from_range: TextUnit, to_range: TextUnit) -> TextUnit {
    let x: u32 = x.into();
    let from_range: u32 = from_range.into();
    let to_range: u32 = to_range.into();
    tu(x * to_range / from_range)
}
