use std::cell::RefCell;
use fall_tree::{Text, Node};

use editor_api::{Diagnostic, Severity};

pub struct DiagnosticSink {
    errors: RefCell<Vec<Diagnostic>>,
}

impl DiagnosticSink {
    pub fn new() -> DiagnosticSink {
        DiagnosticSink { errors: Default::default() }
    }

    pub fn error<T: Into<String>>(&self, node: Node, message: T) {
        self.errors.borrow_mut().push(Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            message: message.into(),
        })
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.errors.borrow().clone()
    }

    #[allow(unused)] // for debugging
    pub fn debug(&self, text: Text) -> String {
        let contents =
            self.errors.borrow()
                .iter()
                .map(|d| {
                    format!("({}, {:?})", text.slice(d.range), d.message)
                })
                .collect::<Vec<String>>()
                .join(", ");
        format!("[{}]", contents)
    }
}
