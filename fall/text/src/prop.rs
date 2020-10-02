use proptest::prelude::*;
use itertools::Itertools;

use crate::{TextUnit, TextRange, TextEdit, TextEditOp, Text, tu};

pub fn arb_text_unit() -> BoxedStrategy<TextUnit> {
    prop::num::u32::ANY.prop_map(TextUnit)
        .boxed()
}

pub fn arb_text_range() -> BoxedStrategy<TextRange> {
    (arb_text_unit(), arb_text_unit())
        .prop_map(|(x, y)| range(x, y))
        .boxed()
}

pub fn arb_text_edit() -> BoxedStrategy<ArbTextEdit> {
    (
        prop::collection::vec(arb_text_unit(), 0..10),
        prop::collection::vec(prop::option::of(arb_text_range()), 0..3),
    )
        .prop_map(|(mut cuts, inserts)| {
            cuts.sort();
            let copies = cuts.into_iter().tuples::<(_, _)>()
                .map(|(s, e)| (true, range(s, e)))
                .map(Some);
            let inserts = inserts.into_iter()
                .map(|opt_insert| opt_insert.map(|r| (false, r)));
            ArbTextEdit {
                ops: inserts.interleave(copies).filter_map(|x| x).collect()
            }
        })
        .boxed()
}

#[derive(Debug)]
pub struct ArbTextEdit {
    pub ops: Vec<(bool, TextRange)>,
}

impl ArbTextEdit {
    pub fn as_text_edit(&self, text: Text) -> TextEdit {
        let len: TextUnit = text.len();
        let scale_range = |r: TextRange| scale_range(r, len);
        TextEdit {
            ops: self.ops.iter().map(|&(is_insert, range)| {
                let range = scale_range(range);
                if is_insert {
                    TextEditOp::Copy(range)
                } else {
                    TextEditOp::Insert(text.slice(range).to_text_buf())
                }
            }).collect()
        }
    }
}

fn range(end_point1: TextUnit, end_point2: TextUnit) -> TextRange {
    let (start, end) = if end_point1 < end_point2 {
        (end_point1, end_point2)
    } else {
        (end_point2, end_point1)
    };
    TextRange::from_to(start, end)
}

fn scale_range(range: TextRange, target_len: TextUnit) -> TextRange {
    TextRange::from_to(
        scale(range.start(), target_len),
        scale(range.end(), target_len),
    )
}

fn scale(x: TextUnit, target_len: TextUnit) -> TextUnit {
    let x: u32 = x.into();
    let from_range: u32 = <u32>::max_value();
    let to_range: u32 = target_len.into();
    let hi_res = (x as u64) * (to_range as u64) / (from_range as u64);
    tu(hi_res as u32)
}
