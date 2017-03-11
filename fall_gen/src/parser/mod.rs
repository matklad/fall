pub mod grammar;

use fall;
use fall::builder::TreeBuilder;
use self::grammar::*;

pub fn parse(text: String) -> fall::File {
    register_node_types();
    fall::builder::parse(
        text,
        grammar::FILE,
        grammar::TOKENIZER,
        &parse_file
    )
}

fn parse_file(b: &mut TreeBuilder) {
    parse_nodes(b);
    b.skip_until(&[TOKENIZER_KW]);
    parse_tokenizer(b);
}

fn parse_nodes(b: &mut TreeBuilder) -> bool {
    b.start(NODES_DEF);
    if !b.try_eat(NODES) {
        b.rollback(NODES_DEF);
        return false;
    }
    let r = b.try_eat(EQ) && b.try_eat(LBRACE);
    if r {
        b.parse_many(&|b| b.try_eat(IDENT));
        b.try_eat(RBRACE);
    }
    b.finish(NODES_DEF);
    true
}

fn parse_tokenizer(b: &mut TreeBuilder) -> bool {
    b.start(TOKENIZER_DEF);
    if !b.try_eat(TOKENIZER_KW) {
        b.rollback(TOKENIZER_DEF);
        return false;
    }
    let r = b.try_eat(EQ) && b.try_eat(LBRACE);
    if r {
        b.parse_many(&|b| {
            b.skip_until(&[RBRACE, IDENT]);
            parse_rule(b)
        });
        b.try_eat(RBRACE);
    }
    b.finish(TOKENIZER_DEF);
    true
}

fn parse_rule(b: &mut TreeBuilder) -> bool {
    b.start(RULE);
    if !b.try_eat(IDENT) {
        b.rollback(RULE);
        return false
    }
    b.try_eat(STRING);
    b.finish(RULE);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_ast(actual: &str, expected: &str) {
        let actual = actual.trim();
        let expected = expected.trim();
        if actual != expected {
            panic!("Actual:\n{}\nExpected:\n{}\n", actual, expected)
        }
    }

    fn ast(code: &str) -> String {
        parse(code.to_owned()).dump()
    }

    #[test]
    fn empty_file() {
        match_ast(&ast(""), r#"FILE """#)
    }

    #[test]
    fn nodes() {
        match_ast(&ast("nodes = { foo bar }"), r#"
FILE
  NODES_DEF
    NODES "nodes"
    EQ "="
    LBRACE "{"
    IDENT "foo"
    IDENT "bar"
    RBRACE "}"
"#)
    }

    #[test]
    fn tokenizer() {
        match_ast(&ast(r#"nodes={} tokenizer = { foo "foo" id r"\w+" }"#), r#"
FILE
  NODES_DEF
    NODES "nodes"
    EQ "="
    LBRACE "{"
    RBRACE "}"
  TOKENIZER_DEF
    TOKENIZER_KW "tokenizer"
    EQ "="
    LBRACE "{"
    RULE
      IDENT "foo"
      STRING "\"foo\""
    RULE
      IDENT "id"
      STRING "r\"\\w+\""
    RBRACE "}"
"#) } }
