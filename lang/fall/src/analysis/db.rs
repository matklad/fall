use std::hash::Hash;
use std::collections::hash_map::{self, HashMap};
use std::sync::Mutex;

use super::{DiagnosticSink, Diagnostic};
use ::FallFile;
use super::query;


type QMap<'f, Q> = Mutex<HashMap<Q, <Q as Query<'f>>::Result>>;

pub ( crate ) struct DB<'f> {
    file: FallFile<'f>,
    pub(super) diagnostics: Mutex<Vec<Diagnostic>>,
    all_rules: QMap<'f, query::AllRules<'f>>,
    all_contexts: QMap<'f, query::AllContexts<'f>>,
    resolve_ref_expr: QMap<'f, query::ResolveRefExpr<'f>>,
    resolve_call: QMap<'f, query::ResolveCall<'f>>,
}

impl<'f> DB<'f> {
    pub fn new(file: FallFile<'f>) -> Self {
        DB {
            file,
            diagnostics: Default::default(),
            all_rules: Default::default(),
            all_contexts: Default::default(),
            resolve_ref_expr: Default::default(),
            resolve_call: Default::default(),
        }
    }
}

impl<'f> DB<'f> {
    pub fn file(&self) -> FallFile<'f> {
        self.file
    }

    pub fn get<Q: QExecutor<'f>>(&self, q: Q) -> Q::Result {
        q.execute(self)
    }
}

pub ( crate ) trait Query<'f> {
    type Result;
}


pub ( crate ) trait QExecutor<'f>: Query<'f> {
    fn execute(self, db: &DB<'f>) -> Self::Result;
}


pub ( crate ) trait OnceQExecutor<'f>: Query<'f> + Eq + Hash
    where Self: Clone, Self::Result: Clone
{
    fn execute(self, db: &DB<'f>, d: &mut DiagnosticSink) -> Self::Result;
}

pub ( crate ) trait QueryCache<'f, Q>
    where
        Q: Query<'f> + Eq + Hash
{
    fn get_cache(&self) -> &QMap<'f, Q>;
}

impl<'f, Q> QExecutor<'f> for Q
    where
        Q: OnceQExecutor<'f>, Q: Clone, Q::Result: Clone,
        DB<'f>: QueryCache<'f, Q>
{
    fn execute(self, db: &DB<'f>) -> Self::Result {
        if let Some(result) = db.get_cache().lock().unwrap().get(&self) {
            return result.clone();
        }
        let mut diagnostics = Vec::new();

        let key = self.clone();
        let value = {
            let mut sink = DiagnosticSink::new(&mut diagnostics);
            <Self as OnceQExecutor>::execute(self, db, &mut sink)
        };

        let mut cache = db.get_cache().lock().unwrap();

        match cache.entry(key) {
            hash_map::Entry::Vacant(vacant) => {
                db.diagnostics.lock().unwrap().extend(diagnostics);
                vacant.insert(value).clone()
            }
            hash_map::Entry::Occupied(occupied) => occupied.get().clone()
        }
    }
}


impl<'f> QueryCache<'f, query::AllRules<'f>> for DB<'f> {
    fn get_cache(&self) -> &QMap<'f, query::AllRules<'f>> {
        &self.all_rules
    }
}

impl<'f> QueryCache<'f, query::AllContexts<'f>> for DB<'f> {
    fn get_cache(&self) -> &QMap<'f, query::AllContexts<'f>> {
        &self.all_contexts
    }
}

impl<'f> QueryCache<'f, query::ResolveRefExpr<'f>> for DB<'f> {
    fn get_cache(&self) -> &QMap<'f, query::ResolveRefExpr<'f>> {
        &self.resolve_ref_expr
    }
}

impl<'f> QueryCache<'f, query::ResolveCall<'f>> for DB<'f> {
    fn get_cache(&self) -> &QMap<'f, query::ResolveCall<'f>> {
        &self.resolve_call
    }
}
