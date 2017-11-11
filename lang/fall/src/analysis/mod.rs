use fall_tree::{File, AstNode};
use fall_tree::visitor::{Visitor, NodeVisitor};

use {FallFile, RefExpr, CallExpr, SynRule};
use editor_api::{Diagnostic, Severity};

mod diagnostics;
mod db;
mod query;

use self::diagnostics::DiagnosticSink;
pub use self::query::{CallKind, RefKind, PratVariant, PrattOp};


pub struct Analysis<'f> {
    db: db::DB<'f>,
    file: FallFile<'f>,
}

impl<'f> Analysis<'f> {
    pub fn new(file: FallFile) -> Analysis {
        Analysis { db: db::DB::new(file), file }
    }

    pub fn ast(&self) -> FallFile<'f> {
        self.file
    }

    pub fn file(&self) -> &'f File {
        self.file.node().file()
    }

    pub fn resolve_reference(&self, ref_: RefExpr<'f>) -> Option<RefKind<'f>> {
        self.db.get(query::ResolveRefExpr(ref_))
    }

    pub fn resolve_call(&self, call: CallExpr<'f>) -> Option<CallKind<'f>> {
        self.db.get(query::ResolveCall(call))
    }

    pub fn resolve_pratt_variant(&self, rule: SynRule<'f>) -> Option<PratVariant<'f>> {
        self.db.get(query::ResolvePrattVariant(rule))
    }

    pub fn collect_all_diagnostics(&self) -> Vec<Diagnostic> {
        Visitor(())
            .visit::<RefExpr, _>(|_, ref_| { self.db.get(query::ResolveRefExpr(ref_)); })
            .visit::<CallExpr, _>(|_, call| { self.db.get(query::ResolveCall(call)); })
            .visit::<SynRule, _>(|_, rule| { self.db.get(query::ResolvePrattVariant(rule)); })
            .walk_recursively_children_first(self.ast().node());
        self.db.get(query::UnusedRules);

        let mut result = self.db.diagnostics.lock().unwrap().clone();
        result.sort_by_key(|d| {
            let priority = match d.severity {
                Severity::Error => 0,
                Severity::Warning => 1,
            };
            (priority, d.range.start(), d.range.end())
        });
        result
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
                Analysis::new(FallFile::wrap(file.root()).unwrap())
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

#[test]
fn analysis_is_sync() {
    fn assert_is_sync<T: Sync>() {}
    assert_is_sync::<Analysis>();
}

#[cfg(test)]
fn check_diagnostics(code: &str, expected_diagnostics: &str) {
    use editor_api::Severity;
    use fall_tree::ERROR;
    use fall_tree::test_util::report_diff;
    use fall_tree::search::subtree;
    let file = ::editor_api::analyse(code.to_string());

    for node in subtree(file.file().root()) {
        if node.ty() == ERROR {
            panic!("\nSYNTAX ERROR: {:?}\n\n{}\n\n", node.range(), code)
        }
    }

    file.analyse(|a| {
        let d = a.collect_all_diagnostics();
        let actual = d.into_iter().map(|d| {
            let s = match d.severity {
                Severity::Error => 'E',
                Severity::Warning => 'W',
            };
            format!("{} {}: {}", s, a.ast().node().text().slice(d.range), d.message)
        }).collect::<Vec<_>>().join("\n");

        report_diff(expected_diagnostics, &actual);
    })
}

#[test]
fn test_diagnostics() {
    check_diagnostics(r"
       pub rule foo { <eof x> }
       rule bar { foo <abracadabra>}
       rule baz { <prev_is foo> <prev_is bar> <prev_is {foo}>}
       rule dupe { dupe }
       rule dupe { dupe }
    ", "\
E <eof x>: Wrong number of arguments, expected 0, got 1
E x: Unresolved reference
E abracadabra: Unresolved reference
E <prev_is bar>: <prev_is> arguments must be public rules
E <prev_is {foo}>: <prev_is> arguments must be public rules
E dupe: Duplicate rule
W baz: Unused rule
W dupe: Unused rule");
}
