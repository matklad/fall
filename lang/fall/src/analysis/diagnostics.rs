use std::cell::RefCell;
use fall_tree::{Text, Node, TextRange};

pub struct Diagnostics {
    errors: RefCell<Vec<(TextRange, String)>>,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics { errors: Default::default() }
    }


    pub fn error<T: Into<String>>(&self, node: Node, message: T) {
        self.errors.borrow_mut().push((node.range(), message.into()))
    }

    #[allow(unused)] // for debugging
    pub fn debug(&self, text: Text) -> String {
        let contents =
            self.errors.borrow()
                .iter()
                .map(|&(range, ref message)| {
                    format!("({}, {:?})", text.slice(range), message)
                })
                .collect::<Vec<String>>()
                .join(", ");
        format!("[{}]", contents)
    }
}
