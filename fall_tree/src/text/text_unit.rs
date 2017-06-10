use std::ops;
use std::fmt;
use super::{is_offset_in_range, TextRange};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextUnit(pub(in super) u32);

impl TextUnit {
    pub fn zero() -> TextUnit {
        TextUnit(0)
    }

    pub fn from_usize(n: usize) -> TextUnit {
        TextUnit(n as u32)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn measure(text: &str) -> TextUnit {
        TextUnit(text.len() as u32)
    }

    pub fn in_range(range: TextRange, off: usize) -> Option<TextUnit> {
        let off = TextUnit(off as u32);
        if is_offset_in_range(off, range) {
            Some(off)
        } else {
            None
        }
    }
}

impl ops::Add<u32> for TextUnit {
    type Output = TextUnit;
    fn add(self, rhs: u32) -> TextUnit {
        TextUnit(self.0 + rhs)
    }
}

impl ops::Add<TextUnit> for TextUnit {
    type Output = TextUnit;
    fn add(self, rhs: TextUnit) -> TextUnit {
        TextUnit(self.0 + rhs.0)
    }
}

impl ops::AddAssign<TextUnit> for TextUnit {
    fn add_assign(&mut self, rhs: TextUnit) {
        self.0 += rhs.0
    }
}

impl ::std::iter::Sum for TextUnit {
    fn sum<I: Iterator<Item=TextUnit>>(iter: I) -> Self {
        TextUnit(iter.map(|u| u.0).sum())
    }
}

impl ops::Sub<u32> for TextUnit {
    type Output = TextUnit;
    fn sub(self, rhs: u32) -> TextUnit {
        TextUnit(self.0 - rhs)
    }
}

impl ops::Sub<TextUnit> for TextUnit {
    type Output = TextUnit;
    fn sub(self, rhs: TextUnit) -> TextUnit {
        TextUnit(self.0 - rhs.0)
    }
}

impl fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for TextUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
