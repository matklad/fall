extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

use regex::Regex;
use syn_engine::BlackTokens;
use fall_tree::{Text, Language, NodeType, IToken, INode, Metrics};

mod lex_engine;

mod syn_engine;

/// Describes both lexical and syntactical grammar
/// of a language.
///
/// `ParserDefinition` is interpreted by `lex_engine` and `syn_engine`
/// and is used to create an instance of `Language`.
pub struct ParserDefinition {
    pub node_types: Vec<NodeType>,
    pub lexical_rules: Vec<LexRule>,
    pub syntactical_rules: Vec<SynRule>,
    pub whitespace_binder: fn(ty: NodeType, adjacent_spaces: Vec<(NodeType, &str)>, leading: bool) -> usize
}

impl Default for ParserDefinition {
    fn default() -> Self {
        fn no_binder(_: NodeType, _: Vec<(NodeType, &str)>, _: bool) -> usize {
            0
        }

        ParserDefinition {
            node_types: Vec::new(),
            lexical_rules: Vec::new(),
            syntactical_rules: Vec::new(),
            whitespace_binder: no_binder,
        }
    }
}

impl ParserDefinition {
    pub fn tokenize<'t>(&'t self, text: Text<'t>) -> Box<Iterator<Item=IToken> + 't> {
        Box::new(lex_engine::tokenize(text, &self.lexical_rules))
    }

    pub fn parse(&self, text: Text, tokens: &[IToken], lang: &Language, metrics: &Metrics) -> INode {
        let black_tokens = BlackTokens::new(lang, text, &tokens);
        let text = text.to_cow();
        let text = text.as_ref();
        let (black_node, ticks) = metrics.measure_time("parsing", || {
            syn_engine::parse_black(&self.node_types, &self.syntactical_rules, black_tokens.seq())
        });
        metrics.record("parsing ticks", ticks, "");

        let white_node = metrics.measure_time("whitespace binding", || {
            syn_engine::into_white(text, black_node, &tokens, self.whitespace_binder)
        });

        let inode = metrics.measure_time("inode construction", || {
            white_node.into_inode(&tokens)
        });
        inode
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
        LexRule {
            ty,
            re: Regex::new(&format!("^({})", re)).unwrap(),
            f,
        }
    }
}

/// Syntactical (aka parser) rule
#[derive(Serialize, Deserialize)]
pub struct SynRule {
    pub body: Expr,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Pub {
        ty_idx: usize,
        body: Box<Expr>,
        replaceable: bool,
    },
    PubReplace {
        ty_idx: usize,
        body: Box<Expr>,
    },
    Or(Vec<Expr>),
    And(Vec<Expr>, Option<usize>),
    Rule(usize),
    Token(usize),
    ContextualToken(usize, String),
    Rep(Box<Expr>),
    WithSkip(Box<Expr>, Box<Expr>),
    Opt(Box<Expr>),
    Not(Box<Expr>),
    Eof,
    Any,
    Layer(Box<Expr>, Box<Expr>),
    Pratt(PrattTable),
    Enter(u32, Box<Expr>),
    Exit(u32, Box<Expr>),
    IsIn(u32),
    Call(Box<Expr>, Vec<(u32, Expr)>),
    Var(u32),
    PrevIs(Vec<usize>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrattTable {
    pub atoms: Vec<Expr>,
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
    pub ty: usize,
    pub op: Expr,
    pub priority: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infix {
    pub ty: usize,
    pub op: Expr,
    pub priority: u32,
    pub has_rhs: bool,
}

pub mod runtime {
    pub use serde_json;
    pub use regex;
    pub use fall_tree;
    pub use lazy_static::*;
}
