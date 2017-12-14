use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, TextEdit, TreeBuilder};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const LBRACE: NodeType = NodeType(101);
pub const RBRACE: NodeType = NodeType(102);
pub const LBRACK: NodeType = NodeType(103);
pub const RBRACK: NodeType = NodeType(104);
pub const COLON: NodeType = NodeType(105);
pub const COMMA: NodeType = NodeType(106);
pub const NULL: NodeType = NodeType(107);
pub const BOOL: NodeType = NodeType(108);
pub const STRING: NodeType = NodeType(109);
pub const NUMBER: NodeType = NodeType(110);
pub const FILE: NodeType = NodeType(111);
pub const OBJECT: NodeType = NodeType(112);
pub const FIELD: NodeType = NodeType(113);
pub const ARRAY: NodeType = NodeType(114);
pub const PRIMITIVE: NodeType = NodeType(115);


pub fn language() -> &'static Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(LBRACE, "\\{", None),
            ::fall_parse::LexRule::new(RBRACE, "\\}", None),
            ::fall_parse::LexRule::new(LBRACK, "\\[", None),
            ::fall_parse::LexRule::new(RBRACK, "\\]", None),
            ::fall_parse::LexRule::new(COLON, ":", None),
            ::fall_parse::LexRule::new(COMMA, ",", None),
            ::fall_parse::LexRule::new(NULL, "null", None),
            ::fall_parse::LexRule::new(BOOL, "true|false", None),
            ::fall_parse::LexRule::new(STRING, "\"[^\"]*\"", None),
            ::fall_parse::LexRule::new(NUMBER, "\\d+", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":12,"body":14,"replaceable":false}},{"Pub":{"ty":13,"body":19,"replaceable":false}},{"Or":[32]},{"Pub":{"ty":14,"body":36,"replaceable":false}},{"Pub":{"ty":15,"body":41,"replaceable":false}},{"Or":[66]},{"Or":[67,68,69]},{"Pub":{"ty":16,"body":78,"replaceable":false}},{"Or":[80]},{"Or":[83,89]},{"Or":[91]},{"Or":[94,100]},{"And":[[1],null]},{"And":[[4],null]},{"Or":[12,13]},{"Token":2},{"Layer":[8,2]},{"Token":3},{"And":[[15,16,17],1]},{"Or":[18]},{"Token":10},{"Token":7},"Eof",{"Not":22},{"And":[[21,23],null]},"Eof",{"And":[[25],null]},{"Or":[24,26]},{"And":[[3,27],1]},{"Or":[28]},{"WithSkip":[20,29]},{"Rep":30},{"And":[[31],null]},{"Token":10},{"Token":6},{"And":[[33,34,6],1]},{"Or":[35]},{"Token":4},{"Layer":[10,5]},{"Token":5},{"And":[[37,38,39],1]},{"Or":[40]},{"Token":8},{"And":[[42],null]},{"Token":11},{"And":[[44],null]},{"Token":10},{"And":[[46],null]},{"Token":9},{"And":[[48],null]},{"Token":2},{"And":[[50],null]},{"Token":4},{"And":[[52],null]},{"Or":[43,45,47,49,51,53]},{"Token":7},"Eof",{"Not":56},{"And":[[55,57],null]},"Eof",{"And":[[59],null]},{"Or":[58,60]},{"And":[[6,61],1]},{"Or":[62]},{"WithSkip":[54,63]},{"Rep":64},{"And":[[65],null]},{"And":[[7],null]},{"And":[[1],null]},{"And":[[4],null]},{"Token":8},{"And":[[70],null]},{"Token":11},{"And":[[72],null]},{"Token":10},{"And":[[74],null]},{"Token":9},{"And":[[76],null]},{"Or":[71,73,75,77]},{"Rep":9},{"And":[[79],null]},{"Token":2},{"Token":3},{"And":[[81,8,82],1]},{"Token":3},{"Not":84},"Any",{"And":[[85,86],null]},{"Or":[87]},{"And":[[88],null]},{"Rep":11},{"And":[[90],null]},{"Token":4},{"Token":5},{"And":[[92,10,93],1]},{"Token":5},{"Not":95},"Any",{"And":[[96,97],null]},{"Or":[98]},{"And":[[99],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, LBRACE, RBRACE, LBRACK, RBRACK, COLON, COMMA, NULL, BOOL, STRING, NUMBER, FILE, OBJECT, FIELD, ARRAY, PRIMITIVE,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::{ParserDefinition, parse, reparse};
            use std::any::Any;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &Any,
                    edit: &TextEdit,
                    new_text: Text,
                    metrics: &Metrics,
                    builder: &mut TreeBuilder,
                ) -> Option<Box<Any + Sync + Send>> {
                    reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                    match ty {
                        ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LBRACE => NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                        RBRACE => NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                        LBRACK => NodeTypeInfo { name: "LBRACK", whitespace_like: false },
                        RBRACK => NodeTypeInfo { name: "RBRACK", whitespace_like: false },
                        COLON => NodeTypeInfo { name: "COLON", whitespace_like: false },
                        COMMA => NodeTypeInfo { name: "COMMA", whitespace_like: false },
                        NULL => NodeTypeInfo { name: "NULL", whitespace_like: false },
                        BOOL => NodeTypeInfo { name: "BOOL", whitespace_like: false },
                        STRING => NodeTypeInfo { name: "STRING", whitespace_like: false },
                        NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                        OBJECT => NodeTypeInfo { name: "OBJECT", whitespace_like: false },
                        FIELD => NodeTypeInfo { name: "FIELD", whitespace_like: false },
                        ARRAY => NodeTypeInfo { name: "ARRAY", whitespace_like: false },
                        PRIMITIVE => NodeTypeInfo { name: "PRIMITIVE", whitespace_like: false },
                        _ => panic!("Unknown NodeType: {:?}", ty)
                    }
                }
            }

            Language::new(Impl {
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
        };
    }

    &*LANG
}



