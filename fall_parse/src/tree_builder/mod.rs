use elapsed::measure_time;

use {Token, LexRule, tokenize};
use fall_tree::{NodeType, File};

mod imp;

pub struct TreeBuilder(imp::TreeBuilderImpl);

impl TreeBuilder {
    pub fn current(&self) -> Option<Token> {
        self.0.current()
    }

    pub fn bump(&mut self) {
        self.0.bump()
    }

    pub fn start(&mut self, ty: NodeType) {
        self.0.start(ty)
    }

    pub fn finish(&mut self, ty: NodeType) {
        self.0.finish(ty)
    }

    pub fn rollback(&mut self, ty: NodeType) {
        self.0.rollback(ty)
    }

    pub fn next_is(&mut self, ty: NodeType) -> bool {
        if let Some(t) = self.current() {
            t.ty == ty
        } else {
            false
        }
    }

    pub fn try_eat(&mut self, ty: NodeType) -> bool {
        self.next_is(ty) && {
            self.bump();
            true
        }
    }
}

pub fn parse(
    text: String,
    file_type: NodeType,
    tokenizer: &[LexRule],
    parser: &Fn(&mut TreeBuilder)
) -> File {
    let (elapsed_lex, tokens) = measure_time(|| tokenize(&text, tokenizer).collect());
    let mut builder = TreeBuilder(imp::TreeBuilderImpl::new(text, file_type, tokens));
    let (elapsed_parse, _) = measure_time(|| parser(&mut builder));
    builder.0.into_file(elapsed_lex, elapsed_parse)
}


