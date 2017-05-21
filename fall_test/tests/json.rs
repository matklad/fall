extern crate fall_test;
extern crate fall_tree;

use fall_test::{json, match_ast};
use fall_tree::dump_file;


fn ast(code: &str) -> String {
    dump_file(&json::LANG.parse(code.to_owned()))
}

#[test]
fn obj() {
    match_ast(&ast(r##"{ "foo": [1, 2, 3] }"##), r#"
FILE
  OBJECT
    LBRACE "{"
    FIELD
      STRING "\"foo\""
      COLON ":"
      ARRAY
        LBRACK "["
        PRIMITIVE
          NUMBER "1"
        COMMA ","
        PRIMITIVE
          NUMBER "2"
        COMMA ","
        PRIMITIVE
          NUMBER "3"
        RBRACK "]"
    RBRACE "}"
"#);
}

#[test]
fn obj_recovery() {
    match_ast(&ast(r##"{"foo": 1, 92, "bar": 3}"##), r##"
FILE
  OBJECT
    LBRACE "{"
    FIELD
      STRING "\"foo\""
      COLON ":"
      PRIMITIVE
        NUMBER "1"
    COMMA ","
    ERROR
      NUMBER "92"
      COMMA ","
    FIELD
      STRING "\"bar\""
      COLON ":"
      PRIMITIVE
        NUMBER "3"
    RBRACE "}"
"##);

    match_ast(&ast(r##"{"foo": 1, "baz":: 92, "bar": 3}"##), r##"
FILE
  OBJECT
    LBRACE "{"
    FIELD
      STRING "\"foo\""
      COLON ":"
      PRIMITIVE
        NUMBER "1"
    COMMA ","
    FIELD
      STRING "\"baz\""
      COLON ":"
      ERROR ""
    ERROR
      COLON ":"
      NUMBER "92"
      COMMA ","
    FIELD
      STRING "\"bar\""
      COLON ":"
      PRIMITIVE
        NUMBER "3"
    RBRACE "}"
"##);
}

#[test]
fn example() {
    match_ast(&ast(r#"
{"widget": {
    "debug": "on",
    "window": {
        "title": "Sample Konfabulator Widget",
        "name": "main_window",
        "width": 500,
        "height": 500
    },
    "image": {
        "src": "Images/Sun.png",
        "name": "sun1",
        "hOffset": 250,
        "vOffset": 250,
        "alignment": "center"
    },
    "text": {
        "data": "Click Here",
        "size": 36,
        "style": "bold",
        "name": "text1",
        "hOffset": 250,
        "vOffset": 100,
        "alignment": "center",
        "onMouseUp": "sun1.opacity = (sun1.opacity / 100) * 90;"
    }
}}"#), r#"
FILE
  OBJECT
    LBRACE "{"
    FIELD
      STRING "\"widget\""
      COLON ":"
      OBJECT
        LBRACE "{"
        FIELD
          STRING "\"debug\""
          COLON ":"
          PRIMITIVE
            STRING "\"on\""
        COMMA ","
        FIELD
          STRING "\"window\""
          COLON ":"
          OBJECT
            LBRACE "{"
            FIELD
              STRING "\"title\""
              COLON ":"
              PRIMITIVE
                STRING "\"Sample Konfabulator Widget\""
            COMMA ","
            FIELD
              STRING "\"name\""
              COLON ":"
              PRIMITIVE
                STRING "\"main_window\""
            COMMA ","
            FIELD
              STRING "\"width\""
              COLON ":"
              PRIMITIVE
                NUMBER "500"
            COMMA ","
            FIELD
              STRING "\"height\""
              COLON ":"
              PRIMITIVE
                NUMBER "500"
            RBRACE "}"
        COMMA ","
        FIELD
          STRING "\"image\""
          COLON ":"
          OBJECT
            LBRACE "{"
            FIELD
              STRING "\"src\""
              COLON ":"
              PRIMITIVE
                STRING "\"Images/Sun.png\""
            COMMA ","
            FIELD
              STRING "\"name\""
              COLON ":"
              PRIMITIVE
                STRING "\"sun1\""
            COMMA ","
            FIELD
              STRING "\"hOffset\""
              COLON ":"
              PRIMITIVE
                NUMBER "250"
            COMMA ","
            FIELD
              STRING "\"vOffset\""
              COLON ":"
              PRIMITIVE
                NUMBER "250"
            COMMA ","
            FIELD
              STRING "\"alignment\""
              COLON ":"
              PRIMITIVE
                STRING "\"center\""
            RBRACE "}"
        COMMA ","
        FIELD
          STRING "\"text\""
          COLON ":"
          OBJECT
            LBRACE "{"
            FIELD
              STRING "\"data\""
              COLON ":"
              PRIMITIVE
                STRING "\"Click Here\""
            COMMA ","
            FIELD
              STRING "\"size\""
              COLON ":"
              PRIMITIVE
                NUMBER "36"
            COMMA ","
            FIELD
              STRING "\"style\""
              COLON ":"
              PRIMITIVE
                STRING "\"bold\""
            COMMA ","
            FIELD
              STRING "\"name\""
              COLON ":"
              PRIMITIVE
                STRING "\"text1\""
            COMMA ","
            FIELD
              STRING "\"hOffset\""
              COLON ":"
              PRIMITIVE
                NUMBER "250"
            COMMA ","
            FIELD
              STRING "\"vOffset\""
              COLON ":"
              PRIMITIVE
                NUMBER "100"
            COMMA ","
            FIELD
              STRING "\"alignment\""
              COLON ":"
              PRIMITIVE
                STRING "\"center\""
            COMMA ","
            FIELD
              STRING "\"onMouseUp\""
              COLON ":"
              PRIMITIVE
                STRING "\"sun1.opacity = (sun1.opacity / 100) * 90;\""
            RBRACE "}"
        RBRACE "}"
    RBRACE "}"
"#)
}


