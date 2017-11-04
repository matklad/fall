use std::collections::HashSet;
use std::sync::Mutex;

use fall_tree::{File, Node, AstNode};
use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::search::child_of_type;
use fall_tree::search::ast;

use {FallFile, SynRule, RefExpr, CallExpr, SYN_RULE};
use editor_api::{Diagnostic, Severity};

mod diagnostics;
mod cache;
mod db;
mod query;

use self::diagnostics::DiagnosticSink;
use self::cache::FileCache;
pub use self::query::{CallKind, RefKind};


pub struct Analysis<'f> {
    db: db::DB<'f>,
    file: FallFile<'f>,

    diagnostics: Mutex<Vec<Diagnostic>>,

    used_rules: FileCache<HashSet<Node<'f>>>,
}

impl<'f> Analysis<'f> {
    pub fn new(file: FallFile) -> Analysis {
        Analysis {
            db: db::DB::new(file),
            file,
            diagnostics: Default::default(),
            used_rules: Default::default(),
        }
    }

    pub fn file(&self) -> FallFile<'f> {
        self.file
    }

    pub fn resolve_call(&self, call: CallExpr<'f>) -> Option<CallKind<'f>> {
        self.db.get(query::ResolveCall(call))
    }

    pub fn resolve_reference(&self, ref_: RefExpr<'f>) -> Option<RefKind<'f>> {
        self.db.get(query::ResolveRefExpr(ref_))
    }

    pub fn collect_all_diagnostics(&self) -> Vec<Diagnostic> {
        let mut result = Visitor(Vec::new())
            .visit::<RefExpr, _>(|_, ref_| { self.resolve_reference(ref_); })
            .visit::<CallExpr, _>(|_, call| { self.resolve_call(call); })
            .visit::<SynRule, _>(|acc, rule| {
                if !self.used_rules().contains(&rule.node()) {
                    if let Some(rule_name) = rule.name_ident() {
                        acc.push(Diagnostic {
                            range: rule_name.range(),
                            severity: Severity::Warning,
                            message: "Unused rule".to_string(),
                        })
                    }
                }
            })
            .walk_recursively_children_first(self.file().node());

        result.extend(self.diagnostics());
        result.sort_by_key(|d| (d.range.start(), d.range.end()));
        result
    }

    #[allow(unused)] // for debugging
    pub fn debug_diagnostics(&self) -> String {
        diagnostics::debug_diagnostics(&self.diagnostics(), self.file().node().text())
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.lock().unwrap().clone()
    }

    fn used_rules(&self) -> &HashSet<Node<'f>> {
        self.used_rules.get(|| self.calculate_used_rules())
    }

    fn calculate_used_rules(&self) -> HashSet<Node<'f>> {
        ast::descendants_of_type::<RefExpr>(self.file.node())
            .into_iter()
            .filter_map(|ref_| self.resolve_reference(ref_))
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

#[test]
fn analysis_is_sync() {
    fn assert_is_sync<T: Sync>() {}
    assert_is_sync::<Analysis>();
}

#[cfg(test)]
fn check_diagnostics(code: &str, expected_diagnostics: &str) {
    use fall_tree::test_util::report_diff;
    let file = ::editor_api::analyse(code.to_string());

    file.analyse(|a| {
        let d = a.collect_all_diagnostics();
        let actual = d.into_iter().map(|d| {
            let s = match d.severity {
                Severity::Error => 'E',
                Severity::Warning => 'W',
            };
            format!("{}: {} {}", a.file().node().text().slice(d.range), s, d.message)
        }).collect::<Vec<_>>().join("\n");

        report_diff(&actual, expected_diagnostics);
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use analysis::CallKind;

    use fall_tree::search;
    use fall_tree::search::ast;

    use ::test_util::parse_with_caret;

    #[test]
    fn test_diagnostics() {
        check_diagnostics(r"
       pub rule foo { <eof x> }
       rule bar { foo <abracadabra>}
       rule baz { <prev_is foo> <prev_is bar> <prev_is {foo}>}
       rule dupe { dupe }
       rule dupe { dupe }
    ", "\
<eof x>: E Wrong number of arguments, expected 0, got 1
x: E Unresolved reference
abracadabra: E Unresolved reference
baz: W Unused rule
<prev_is bar>: E <prev_is> arguments must be public rules
<prev_is {foo}>: E <prev_is> arguments must be public rules");
    }

    #[test]
    fn test_resolve_no_args() {
        check_resolved("rule foo { <^eof> }", "Eof");
        check_resolved("rule foo { <^any> }", "Any");
        check_resolved("rule foo { <^commit> }", "Commit");
    }

    #[test]
    fn test_resolve_simple_args() {
        check_resolved("rule foo { <^not a> }", "Not(RefExpr@[16; 17))");
        check_resolved("rule foo { <^with_skip a b> }", "WithSkip(RefExpr@[22; 23), RefExpr@[24; 25))");
        check_resolved("rule foo { <^layer a b> }", "Layer(RefExpr@[18; 19), RefExpr@[20; 21))");
    }

    #[test]
    fn test_resolve_layer() {
        check_resolved("rule foo { <^enter 'ctx' a> }", "Enter(0, RefExpr@[24; 25))");
        check_resolved("rule foo { <^exit  'ctx' a> }", "Exit(0, RefExpr@[24; 25))");

        check_resolved("rule foo { <is_in 'ctx1'> <^enter 'ctx2' a> }", "Enter(1, RefExpr@[40; 41))");
        check_resolved("rule foo { <is_in 'ctx1'> <^exit  'ctx2' a> }", "Exit(1, RefExpr@[40; 41))");

        check_resolved(
            "rule foo { <enter 'ctx1' a> <enter 'ctx2' b> <^is_in 'ctx2'> }",
            "IsIn(1)"
        );

        check(
            r" rule foo { <^is_in foo>}",
            None,
            r#"[(foo, "Context should be a single quoted string")]"#,
        )
    }

    #[test]
    fn test_not() {
        resolve_call("rule foo { <^not bar>}", |c, _| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), "Not(RefExpr@[16; 19))");
        })
    }

    #[test]
    fn test_unresolved_call() {
        check(
            r"rule foo { <^abracadabra>}",
            None,
            r#"[(abracadabra, "Unresolved reference")]"#
        );
    }

    #[test]
    fn wrong_arity() {
        check(
            "rule foo { <^eof foo> }",
            Some(CallKind::Eof),
            r#"[(<eof foo>, "Wrong number of arguments, expected 0, got 1")]"#
        );

        check(
            "rule foo { <^any foo bar> }",
            Some(CallKind::Any),
            r#"[(<any foo bar>, "Wrong number of arguments, expected 0, got 2")]"#
        );

        check(
            "rule foo { <^commit foo bar baz> }",
            Some(CallKind::Commit),
            r#"[(<commit foo bar baz>, "Wrong number of arguments, expected 0, got 3")]"#
        );

        check(
            "rule foo { <^not> }",
            None,
            r#"[(<not>, "Wrong number of arguments, expected 1, got 0")]"#
        );
    }

    #[test]
    fn check_prev_is() {
        check_resolved(
            "pub rule foo {} pub rule bar {} rule baz { <^prev_is foo bar> }",
            "PrevIs([1, 2])"
        )
    }

    #[test]
    fn call_custom_rule() {
        check_resolved(
            "rule foo { <^bar a b>} rule bar(x, y) { }",
            "RuleCall(SynRule@[22; 40), [(0, RefExpr@[16; 17)), (1, RefExpr@[18; 19))])"
        );
        ::analysis::check_diagnostics(
            "rule foo { <bar <eof>>} rule bar(x, y) { }",
            "<bar <eof>>: E Expected 2 arguments, got 1"
        )
    }

    fn check(text: &str, kind: Option<CallKind>, diagnostics: &str) {
        resolve_call(text, |c, a| {
            assert_eq!(c, kind);
            assert_eq!(a.debug_diagnostics(), diagnostics);
        })
    }

    fn check_resolved(text: &str, kind: &str) {
        resolve_call(text, |c, a| {
            let c = c.unwrap();
            assert_eq!(format!("{:?}", c), kind);
            assert_eq!(a.debug_diagnostics(), "[]");
        })
    }

    fn resolve_call<F: FnOnce(Option<CallKind>, Analysis)>(text: &str, f: F) {
        let (file, caret) = parse_with_caret(&text);
        let call = {
            let leaf = search::find_leaf_at_offset(file.root(), caret)
                .left_biased().unwrap();
            ast::ancestor_exn(leaf)
        };
        let a = Analysis::new(::ast(&file));
        let c = a.resolve_call(call);
        f(c, a)
    }
}

