use fall_tree::Node;

use fall_editor::{Diagnostic, Severity};

pub (crate) struct DiagnosticSink<'d> {
    diagnostics: &'d mut Vec<Diagnostic>
}

impl<'d> DiagnosticSink<'d> {
    pub fn new(diagnostics: &'d mut Vec<Diagnostic>) -> DiagnosticSink<'d> {
        DiagnosticSink { diagnostics }
    }

    pub fn error<T: Into<String>>(&mut self, node: Node, message: T) {
        self.diagnostics.push(Diagnostic {
            range: node.range(),
            severity: Severity::Error,
            message: message.into(),
        })
    }

    pub fn warning<T: Into<String>>(&mut self, node: Node, message: T) {
        self.diagnostics.push(Diagnostic {
            range: node.range(),
            severity: Severity::Warning,
            message: message.into(),
        })
    }
}
