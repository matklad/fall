extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

use std::any::Any;
use std::collections::HashMap;

use regex::Regex;
use fall_tree::{Text, Language, NodeType, IToken, INode, Metrics, TextEdit, TextUnit};

mod lex_engine;
pub use lex_engine::RegexLexer;

mod syn_engine;

struct IncrementalData {
    tokens: Vec<IToken>,
    events: Vec<Event>,
}

pub fn parse(
    lang: &Language,
    lexer_def: &RegexLexer,
    parser_def: &ParserDefinition,
    text: Text,
    metrics: &Metrics,
) -> (Option<Box<Any + Sync + Send>>, INode) {
    let tokens: Vec<IToken> = metrics.measure_time("lexing", || {
        lex_engine::lex(lexer_def, text)
    });
    metrics.record("relexed region", text.len().utf8_len() as u64, "");

    let (events, inode) = parser_def.parse(None, text, &tokens, lang, metrics);
    let incremental_data = IncrementalData { tokens, events };
    (Some(Box::new(incremental_data)), inode)
}

pub fn reparse(
    lang: &Language,
    lexer_def: &RegexLexer,
    parser_def: &ParserDefinition,
    incremental_data: &Any,
    edit: &TextEdit,
    new_text: Text,
    metrics: &Metrics,
) -> (Option<Box<Any + Sync + Send>>, INode) {
    let incremental_data: &IncrementalData = incremental_data.downcast_ref().unwrap();
    let (tokens, relexed_region) = metrics.measure_time("lexing", || {
        lex_engine::relex(lexer_def, &incremental_data.tokens, edit, new_text)
    });
    metrics.record("relexed region", relexed_region as u64, "");

    let salvaged = syn_engine::salvage_segments(
        &incremental_data.events,
        &incremental_data.tokens,
        &|t| lang.node_type_info(t.ty).whitespace_like,
        edit
    );
    let prev = Some((salvaged, incremental_data.events.as_ref()));
    let (events, inode) = parser_def.parse(prev, new_text, &tokens, lang, metrics);
    let incremental_data = IncrementalData { tokens, events };
    (Some(Box::new(incremental_data)), inode)
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Event {
    Start { ty: NodeType, forward_parent: Option<u32> },
    Token { ty: NodeType, n_raw_tokens: u16 },
    End,
    Cached { key: u32, n_events: u32 },
}


/// Describes both lexical and syntactical grammar
/// of a language.
///
/// `ParserDefinition` is interpreted by `lex_engine` and `syn_engine`
/// and is used to create an instance of `Language`.
pub struct ParserDefinition {
    pub node_types: Vec<NodeType>,
    pub syntactical_rules: Vec<Expr>,
    pub whitespace_binder: fn(ty: NodeType, adjacent_spaces: Vec<(NodeType, &str)>, leading: bool) -> usize,
}

impl Default for ParserDefinition {
    fn default() -> Self {
        fn no_binder(_: NodeType, _: Vec<(NodeType, &str)>, _: bool) -> usize {
            0
        }

        ParserDefinition {
            node_types: Vec::new(),
            syntactical_rules: Vec::new(),
            whitespace_binder: no_binder,
        }
    }
}

impl ParserDefinition {
    fn parse(&self,
             prev: Option<(HashMap<(TextUnit, ExprRef), (u32, u32, u32)>, &[Event])>,
             text: Text,
             tokens: &[IToken],
             lang: &Language,
             metrics: &Metrics
    ) -> (Vec<Event>, INode) {
        let g = syn_engine::Grammar {
            node_types: &self.node_types,
            rules: &self.syntactical_rules,
            start_rule: ExprRef(0),
        };
        let file_ty = match self.syntactical_rules[0] {
            Expr::Pub { ty, .. } => self.node_types[ty.0 as usize],
            _ => unreachable!()
        };

        let (events, ticks) = metrics.measure_time("parsing", || {
            syn_engine::parse(prev, g, lang, text, &tokens)
        });
        metrics.record("parsing ticks", ticks, "");

        metrics.measure_time("inode construction", || {
            let inode = syn_engine::convert(
                text,
                tokens,
                &events,
                &|ty| lang.node_type_info(ty).whitespace_like,
                &|ty, spaces, leading| {
                    if ty == file_ty {
                        return spaces.len();
                    }
                    let owned: Vec<_> = spaces.iter().map(|&(t, text)| (t, text.to_cow())).collect();
                    let spaces = owned.iter().map(|&(t, ref text)| (t, text.as_ref())).collect();
                    (self.whitespace_binder)(ty, spaces, leading)
                }
            );
            (events, inode)
        })
    }
}

/// Lexical (aka tokenizer) rule:
/// either a regular expression, or a user-supplied
/// custom Rust function
pub struct LexRule {
    pub ty: NodeType,
    pub re: Regex,
    pub f: Option<CustomLexRule>,
}

pub type CustomLexRule = fn(&str) -> Option<usize>;

impl LexRule {
    pub fn new(ty: NodeType, re: &str, f: Option<CustomLexRule>) -> LexRule {
        let re = Regex::new(&format!("^({})", re)).unwrap();
        LexRule { ty, re, f }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct NodeTypeRef(pub u32);

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Context(pub u32);

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Arg(pub u32);

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct ExprRef(pub u32);

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Pub {
        ty: NodeTypeRef,
        body: ExprRef,
        replaceable: bool,
    },
    PubReplace {
        ty: NodeTypeRef,
        body: ExprRef,
    },
    Or(Vec<ExprRef>),
    And(Vec<ExprRef>, Option<usize>),
    Token(NodeTypeRef),
    ContextualToken(NodeTypeRef, String),
    Rep(ExprRef),
    WithSkip(ExprRef, ExprRef),
    Opt(ExprRef),
    Not(ExprRef),
    Eof,
    Any,
    Layer(ExprRef, ExprRef),
    Pratt(Box<PrattTable>),
    Enter(Context, ExprRef),
    Exit(Context, ExprRef),
    IsIn(Context),
    Call(ExprRef, Vec<(Arg, ExprRef)>),
    Var(Arg),
    PrevIs(Vec<NodeTypeRef>),
    Inject(ExprRef, ExprRef),
    Cached(ExprRef),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrattTable {
    pub atoms: Vec<ExprRef>,
    pub prefixes: Vec<Prefix>,
    pub infixes: Vec<Infix>
}

impl PrattTable {
    fn infixes<'p>(&'p self, min_prior: u32) -> Box<Iterator<Item=&'p Infix> + 'p> {
        Box::new(
            self.infixes.iter()
                .filter(move |ix| ix.priority >= min_prior)
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prefix {
    pub ty: NodeTypeRef,
    pub op: ExprRef,
    pub priority: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infix {
    pub ty: NodeTypeRef,
    pub op: ExprRef,
    pub priority: u32,
    pub has_rhs: bool,
}

pub mod runtime {
    pub use serde_json;
    pub use regex;
    pub use fall_tree;
    pub use lazy_static::*;
}
