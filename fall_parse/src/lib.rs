extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate elapsed;
pub extern crate regex;
pub extern crate fall_tree;
pub extern crate serde_json;

use regex::Regex;
use elapsed::measure_time;
use syn_engine::BlackTokens;
use fall_tree::{Language, NodeType, FileStats, INode, TextUnit, TextRange};

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
    pub syntactical_rules: Vec<SynRule>
}

impl ParserDefinition {
    pub fn parse(&self, text: &str, lang: &Language) -> (FileStats, INode) {
        let (lex_time, tokens) = measure_time(|| {
            lex_engine::tokenize(&text, &self.lexical_rules).collect::<Vec<_>>()
        });

        let black_tokens = BlackTokens::new(lang, text, &tokens);
        let (parse_time, (black_node, ticks)) = measure_time(|| {
            syn_engine::parse_black(&self.node_types, &self.syntactical_rules, black_tokens.seq())
        });

        let stats = FileStats {
            lexing_time: lex_time.duration(),
            parsing_time: parse_time.duration(),
            parsing_ticks: ticks,
            reparsed_region: TextRange::from_to(TextUnit::zero(), TextUnit::from_usize(text.len())),
        };

        let white_node = syn_engine::into_white(black_node, lang, &tokens);

        let inode = white_node.into_inode(&tokens);
        (stats, inode)
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
            ty: ty,
            re: Regex::new(&format!("^({})", re)).unwrap(),
            f: f,
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
    Pratt(Vec<PrattVariant>),
    Enter(u32, Box<Expr>),
    Exit(u32, Box<Expr>),
    IsIn(u32),
    Call(Box<Expr>, Vec<(u32, Expr)>),
    Var(u32),
    PrevIs(Vec<usize>)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PrattVariant {
    Atom { body: Box<Expr> },
    Binary {
        ty: usize,
        op: Box<Expr>,
        priority: u32,
    },
    Postfix {
        ty: usize,
        op: Box<Expr>
    },
    Prefix {
        ty: usize,
        op: Box<Expr>
    }
}

pub mod runtime {
    pub use serde_json;
    pub use regex;
    pub use fall_tree;
    pub use lazy_static::*;
}
