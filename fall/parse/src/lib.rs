extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

use regex::Regex;
use fall_tree::{Text, Language, NodeType, IToken, INode, Metrics, Event, TextEdit};

mod lex_engine;

pub use lex_engine::RegexLexer;

mod syn_engine;

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
    pub fn parse(&self, text: Text, tokens: &[IToken], lang: &Language, metrics: &Metrics) -> (Vec<Event>, INode) {
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
            syn_engine::parse(None, g, lang, text, &tokens)
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

    pub fn reparse(
        &self,
        old_tokens: &[IToken],
        old_events: &[Event],
        edit: &TextEdit,
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


        let salvaged = syn_engine::salvage_segments(
            old_events, old_tokens, &|t| lang.node_type_info(t.ty).whitespace_like,
            edit
        );

        let (events, ticks) = metrics.measure_time("parsing", || {
            syn_engine::parse(Some((salvaged, old_events)), g, lang, text, &tokens)
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
