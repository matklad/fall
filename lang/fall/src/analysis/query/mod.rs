use std::collections::HashMap;
use std::sync::Arc;

use fall_tree::Text;

use super::db::Query;
use syntax::{SynRule, LexRule, RefExpr, Parameter, Expr, CallExpr, MethodDef, AstNodeDef, AstClassDef};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct AllLexRules;
mod all_lex_rules;
impl<'f> Query<'f> for AllLexRules {
    type Result = Arc<HashMap<Text<'f>, LexRule<'f>>>;
}

#[derive(Debug)]
pub(crate) struct FindLexRule<'f>(pub Text<'f>);
mod find_lex_rule;
impl<'f> Query<'f> for FindLexRule<'f> {
    type Result = Option<LexRule<'f>>;
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct AllSynRules;
mod all_syn_rules;
impl<'f> Query<'f> for AllSynRules {
    type Result = Arc<HashMap<Text<'f>, SynRule<'f>>>;
}

#[derive(Debug)]
pub(crate) struct FindSynRule<'f>(pub Text<'f>);
mod find_syn_rule;
impl<'f> Query<'f> for FindSynRule<'f> {
    type Result = Option<SynRule<'f>>;
}


#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct UnusedRules;

impl<'f> Query<'f> for UnusedRules {
    type Result = ();
}

mod unused_rules;


#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct AllContexts;

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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct ResolveRefExpr<'f>(pub RefExpr<'f>);

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

    RuleCall(SynRule<'f>, Arc<Vec<(Parameter<'f>, Expr<'f>)>>),
    PrevIs(Arc<Vec<SynRule<'f>>>),
    Inject(Expr<'f>, Expr<'f>),
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct ResolveCall<'f>(pub CallExpr<'f>);

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


#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct ResolvePrattVariant<'f>(pub SynRule<'f>);

impl<'f> Query<'f> for ResolvePrattVariant<'f> {
    type Result = Option<PratVariant<'f>>;
}

mod resolve_pratt_variant;

#[derive(Copy, Clone)]
pub enum Arity {
    Single,
    Optional,
    Many
}

#[derive(Copy, Clone)]
pub enum ChildKind<'f> {
    AstNode(AstNodeDef<'f>),
    AstClass(AstClassDef<'f>),
    Token(LexRule<'f>)
}

#[derive(Copy, Clone)]
pub enum MethodKind<'f> {
    NodeAccessor(ChildKind<'f>, Arity),
    TextAccessor(LexRule<'f>, Arity),
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct ResolveMethod<'f>(pub MethodDef<'f>);

impl<'f> Query<'f> for ResolveMethod<'f> {
    type Result = Option<MethodKind<'f>>;
}

mod resolve_method;
