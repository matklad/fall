use std::collections::HashMap;
use std::sync::Arc;

use fall_tree::Text;

use super::db::Query;
use ::{SynRule, LexRule, RefExpr, Parameter, Expr, CallExpr};


pub ( crate ) struct FindLexRule<'f>(pub Text<'f>);

impl<'f> Query<'f> for FindLexRule<'f> {
    type Result = Option<LexRule<'f>>;
}

mod find_lex_rule;


pub ( crate ) struct FindSynRule<'f>(pub Text<'f>);

impl<'f> Query<'f> for FindSynRule<'f> {
    type Result = Option<SynRule<'f>>;
}

mod find_syn_rule;


#[derive(Eq, PartialEq, Hash, Clone)]
pub ( crate ) struct AllRules;

impl<'f> Query<'f> for AllRules {
    type Result = Arc<HashMap<Text<'f>, SynRule<'f>>>;
}

mod all_rules;


#[derive(Eq, PartialEq, Hash, Clone)]
pub ( crate ) struct UnusedRules;

impl<'f> Query<'f> for UnusedRules {
    type Result = ();
}

mod unused_rules;


#[derive(Eq, PartialEq, Hash, Clone)]
pub ( crate ) struct AllContexts;

impl<'f> Query<'f> for AllContexts {
    type Result = Arc<Vec<Text<'f>>>;
}

mod all_context_ids;


#[derive(Copy, Clone)]
pub enum RefKind<'f> {
    Token(LexRule<'f>),
    RuleReference(SynRule<'f>),
    Param(Parameter<'f>),
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub ( crate ) struct ResolveRefExpr<'f>(pub RefExpr<'f>);

impl<'f> Query<'f> for ResolveRefExpr<'f> {
    type Result = Option<RefKind<'f>>;
}

mod resolve_ref_expr;


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CallKind<'f> {
    Eof,
    Any,
    Commit,

    Not(Expr<'f>),
    Layer(Expr<'f>, Expr<'f>),
    WithSkip(Expr<'f>, Expr<'f>),

    Enter(u32, Expr<'f>),
    Exit(u32, Expr<'f>),
    IsIn(u32),

    RuleCall(SynRule<'f>, Arc<Vec<(u32, Expr<'f>)>>),
    PrevIs(Arc<Vec<usize>>)
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub (crate) struct ResolveCall<'f>(pub CallExpr<'f>);

impl<'f> Query<'f> for ResolveCall<'f> {
    type Result = Option<CallKind<'f>>;
}

mod resolve_call;


#[derive(Copy, Clone)]
pub enum PratVariant<'f> {
    Atom(Expr<'f>),
    Bin(PrattOp<'f>),
    Postfix(PrattOp<'f>),
    Prefix(PrattOp<'f>),
}

#[derive(Copy, Clone)]
pub struct PrattOp<'f> {
    pub op: Expr<'f>,
    pub priority: u32
}


#[derive(Eq, PartialEq, Hash, Clone)]
pub (crate) struct ResolvePrattVariant<'f>(pub SynRule<'f>);

impl<'f> Query<'f> for ResolvePrattVariant<'f> {
    type Result = Option<PratVariant<'f>>;
}

mod resolve_pratt_variant;
