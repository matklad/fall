use fall_tree::{File, AstNode};
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use fall_editor::{Diagnostic, Severity};

use std::sync::Arc;
use crate::syntax::{FallFile, RefExpr, CallExpr, SynRule, MethodDef, AstNodeDef, AstTraitDef};

mod diagnostics;
mod db;
mod query;

use self::diagnostics::DiagnosticSink;
pub use self::query::{CallKind, RefKind, PratVariant, PrattOp, MethodKind, ChildKind, Arity};


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

    pub fn resolve_method(&self, method: MethodDef<'f>) -> Option<MethodKind<'f>> {
        self.db.get(query::ResolveMethod(method))
    }

    pub fn ast_node_traits(&self, node: AstNodeDef<'f>) -> Arc<Vec<AstTraitDef<'f>>> {
        self.db.get(query::AstNodeTraits(node))
    }

    pub fn collect_all_diagnostics(&self) -> Vec<Diagnostic> {
        process_subtree_bottom_up(
            self.ast().node(),
            visitor(())
                .visit::<RefExpr, _>(|ref_, _| { self.db.get(query::ResolveRefExpr(ref_)); })
                .visit::<CallExpr, _>(|call, _| { self.db.get(query::ResolveCall(call)); })
                .visit::<SynRule, _>(|rule, _| { self.db.get(query::ResolvePrattVariant(rule)); })
                .visit::<AstNodeDef, _>(|rule, _| { self.db.get(query::AstNodeTraits(rule)); })
        );
        self.db.get(query::UnusedRules);
        self.db.get(query::AllLexRules);

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
    pub fn new(file: File) -> FileWithAnalysis {
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
    use fall_tree::ERROR;
    use fall_tree::test_util::report_diff;
    use fall_tree::search::subtree;
    let file = crate::analyse(code.to_string());

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
fn test_syn_rule_diagnostics() {
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

#[test]
fn test_lex_rule_diagnostics() {
    check_diagnostics(r"
       tokenizer {
           class 'class'
           class 'trait'
       }
    ", "\
E class 'trait': Duplicate token
");
}

#[test]
fn test_ast_diagnostics() {
    check_diagnostics(r"
       ast {
           trait foo {}
           node a: foo {}
           node b: bar {}
       }
    ", "\
E bar: Unresolved trait
");
}
