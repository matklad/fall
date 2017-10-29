use std::collections::HashSet;

use lazycell::AtomicLazyCell;

use fall_tree::{File, Node, AstNode};
use fall_tree::search::child_of_type;
use fall_tree::search::ast;

use {FallFile, SynRule, RefExpr, RefKind, CallKind, CallExpr, SYN_RULE};

pub struct Analysis<'f> {
    file: FallFile<'f>,

    used_rules: AtomicLazyCell<HashSet<Node<'f>>>
}

impl<'f> Analysis<'f> {
    pub fn new(file: FallFile) -> Analysis {
        Analysis {
            file,
            used_rules: AtomicLazyCell::new(),
        }
    }

    pub fn file(&self) -> FallFile<'f> {
        self.file
    }

    pub fn is_unused(&self, rule: SynRule<'f>) -> bool {
        !self.used_rules().contains(&rule.node())
    }

    fn used_rules(&self) -> &HashSet<Node<'f>> {
        if !self.used_rules.filled() {
            let _ = self.used_rules.fill(self.calculate_used_rules());
        }
        self.used_rules.borrow().unwrap()
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
                    .filter_map(|call| call.kind().ok())
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
