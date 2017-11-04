use std::marker::PhantomData;
use std::collections::HashMap;
use std::sync::Arc;

use fall_tree::Text;

use super::db::{Query};
use ::{SynRule, LexRule, FallFile};


pub(crate) struct FindLexRule<'f>(pub Text<'f>);

impl<'f> Query<'f> for FindLexRule<'f> {
    type Result = Option<LexRule<'f>>;
}
mod find_lex_rule;


pub(crate) struct FindSynRule<'f>(pub Text<'f>);

impl<'f> Query<'f> for FindSynRule<'f> {
    type Result = Option<SynRule<'f>>;
}
mod find_syn_rule;



#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct AllRules<'f>(pub PhantomData<FallFile<'f>>);

impl<'f> Query<'f> for AllRules<'f> {
    type Result = Arc<HashMap<Text<'f>, SynRule<'f>>>;
}
mod all_rules;


#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct AllContexts<'f>(pub PhantomData<FallFile<'f>>);

impl<'f> Query<'f> for AllContexts<'f> {
    type Result = Arc<Vec<Text<'f>>>;
}

mod all_context_ids;



