extern crate fall;

use fall::builder::TreeBuilder;

mod grammar;

use grammar::*;

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
    WHITESPACE " "
    EQ "="
    WHITESPACE " "
    LBRACE "{"
    WHITESPACE " "
    IDENT "foo"
    WHITESPACE " "
    IDENT "bar"
    WHITESPACE " "
    RBRACE "}"
"#)
    }

    //    #[test]
    //    fn tokenizer() {
    //        match_ast(&ast("tokenizer = {}"), r#"FILE """#)
    //    }
}
