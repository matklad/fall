extern crate fall_test;
extern crate fall_tree;

use fall_test::{json, match_ast};
use fall_tree::dump_file_ws;


fn ast(code: &str) -> String {
    dump_file_ws(&json::parse(code.to_owned()))
}

#[test]
fn obj() {
    match_ast(&ast(r##"{ "foo": [1, 2, 3] }"##), r#"
FILE
  OBJECT
    LBRACE "{"
    WHITESPACE " "
    FIELD
      STRING "\"foo\""
      COLON ":"
      WHITESPACE " "
      ARRAY
        LBRACK "["
        PRIMITIVE
          NUMBER "1"
        COMMA ","
        WHITESPACE " "
        PRIMITIVE
          NUMBER "2"
        COMMA ","
        WHITESPACE " "
        PRIMITIVE
          NUMBER "3"
        RBRACK "]"
    WHITESPACE " "
    RBRACE "}"
"#);
}

