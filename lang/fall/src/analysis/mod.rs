use std::collections::HashSet;

use fall_tree::{File, Node, AstNode, Text};
use fall_tree::search::child_of_type;
use fall_tree::search::ast;

use {FallFile, SynRule, RefExpr, RefKind, CallExpr, SYN_RULE};
use editor_api::Diagnostic;

mod calls;
mod diagnostics;
mod cache;

use self::diagnostics::DiagnosticSink;
use self::cache::{FileCache, NodeCache};
pub use self::calls::CallKind;


pub struct Analysis<'f> {
    file: FallFile<'f>,

    diagnostics: DiagnosticSink,

    used_rules: FileCache<HashSet<Node<'f>>>,
    contexts: FileCache<Vec<Text<'f>>>,

    calls: NodeCache<'f, Option<CallKind<'f>>>,
}

impl<'f> Analysis<'f> {
    pub fn new(file: FallFile) -> Analysis {
        Analysis {
            file,
            diagnostics: DiagnosticSink::new(),
            used_rules: Default::default(),
            contexts: Default::default(),
            calls: Default::default(),
        }
    }

    pub fn file(&self) -> FallFile<'f> {
        self.file
    }

    pub fn is_unused(&self, rule: SynRule<'f>) -> bool {
        !self.used_rules().contains(&rule.node())
    }

    pub fn resolve_call(&self, call: CallExpr<'f>) -> Option<CallKind<'f>> {
        self.calls.get(call.node(), || calls::resolve(self, call))
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.diagnostics()
    }

    fn contexts(&self) -> &[Text<'f>] {
        self.contexts.get(|| calls::contexts(self)).as_ref()
    }

    fn used_rules(&self) -> &HashSet<Node<'f>> {
        self.used_rules.get(|| self.calculate_used_rules())
    }

    fn calculate_used_rules(&self) -> HashSet<Node<'f>> {
        ast::descendants_of_type::<RefExpr>(self.file.node())
            .into_iter()
            .filter_map(|node| node.resolve())
            .filter_map(|r| match r {
                RefKind::RuleReference(rule) => Some(rule.node()),
                _ => None
            })
            .chain(
                ast::descendants_of_type::<CallExpr>(self.file.node())
                    .into_iter()
                    .filter_map(|call| self.resolve_call(call))
                    .filter_map(|kind| match kind {
                        CallKind::RuleCall(rule, ..) => Some(rule.node()),
                        _ => None,
                    })
            )
            .chain(child_of_type(self.file.node(), SYN_RULE).into_iter())
            .collect()
    }
}


pub struct FileWithAnalysis {
    rent: rent::R
}

impl FileWithAnalysis {
    pub fn new(text: String) -> FileWithAnalysis {
        FileWithAnalysis::from_file(::parse(text))
    }

    fn from_file(file: File) -> FileWithAnalysis {
        FileWithAnalysis {
            rent: rent::R::new(Box::new(file), |file| {
                Analysis::new(FallFile::new(file.root()))
            })
        }
    }

    pub fn file(&self) -> &File {
        self.rent.head()
    }

    pub fn analyse<T, F: FnOnce(&Analysis) -> T>(&self, f: F) -> T {
        self.rent.rent(|a: &Analysis| f(a))
    }
}

rental! {
    mod rent {
        use super::*;

        #[rental]
        pub struct R {
            file: Box<File>,
            analysis: Analysis<'file>
        }
    }
}
