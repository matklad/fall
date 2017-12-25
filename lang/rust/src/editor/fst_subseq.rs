use fst;

#[derive(Clone, Copy)]
pub struct FstSubSeq<'a> { query: &'a str }

impl<'a> FstSubSeq<'a> {
    pub fn new<S: AsRef<str>>(query: &'a S) -> FstSubSeq<'a> {
        FstSubSeq { query: query.as_ref() }
    }
}

impl<'a> fst::Automaton for FstSubSeq<'a> {
    type State = usize;

    fn start(&self) -> usize {
        0
    }

    fn is_match(&self, &state: &usize) -> bool {
        state == self.query.len()
    }

    fn accept(&self, &state: &usize, byte: u8) -> usize {
        if state >= self.query.len() {
            return state;
        }
        if byte == self.query.as_bytes()[state] {
            return state + 1;
        }
        return state;
    }

    fn can_match(&self, _: &usize) -> bool {
        true
    }

    fn will_always_match(&self, &state: &usize) -> bool {
        state == self.query.len()
    }
}
