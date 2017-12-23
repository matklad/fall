use fall_parse::runtime as rt;
pub use self::rt::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const LBRACE: rt::NodeType = rt::NodeType(101);
pub const RBRACE: rt::NodeType = rt::NodeType(102);
pub const LBRACK: rt::NodeType = rt::NodeType(103);
pub const RBRACK: rt::NodeType = rt::NodeType(104);
pub const COLON: rt::NodeType = rt::NodeType(105);
pub const COMMA: rt::NodeType = rt::NodeType(106);
pub const NULL: rt::NodeType = rt::NodeType(107);
pub const BOOL: rt::NodeType = rt::NodeType(108);
pub const STRING: rt::NodeType = rt::NodeType(109);
pub const NUMBER: rt::NodeType = rt::NodeType(110);
pub const FILE: rt::NodeType = rt::NodeType(111);
pub const OBJECT: rt::NodeType = rt::NodeType(112);
pub const FIELD: rt::NodeType = rt::NodeType(113);
pub const ARRAY: rt::NodeType = rt::NodeType(114);
pub const PRIMITIVE: rt::NodeType = rt::NodeType(115);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(LBRACE, "\\{", None),
            rt::LexRule::new(RBRACE, "\\}", None),
            rt::LexRule::new(LBRACK, "\\[", None),
            rt::LexRule::new(RBRACK, "\\]", None),
            rt::LexRule::new(COLON, ":", None),
            rt::LexRule::new(COMMA, ",", None),
            rt::LexRule::new(NULL, "null", None),
            rt::LexRule::new(BOOL, "true|false", None),
            rt::LexRule::new(STRING, "\"[^\"]*\"", None),
            rt::LexRule::new(NUMBER, "\\d+", None),
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":12,"body":14,"replaceable":false}},{"Pub":{"ty":13,"body":19,"replaceable":false}},{"Or":[32]},{"Pub":{"ty":14,"body":36,"replaceable":false}},{"Pub":{"ty":15,"body":41,"replaceable":false}},{"Or":[66]},{"Or":[67,68,69]},{"Pub":{"ty":16,"body":78,"replaceable":false}},{"Or":[80]},{"Or":[83,89]},{"Or":[91]},{"Or":[94,100]},{"And":[[1],null]},{"And":[[4],null]},{"Or":[12,13]},{"Token":2},{"Layer":[8,2]},{"Token":3},{"And":[[15,16,17],1]},{"Or":[18]},{"Token":10},{"Token":7},"Eof",{"Not":22},{"And":[[21,23],null]},"Eof",{"And":[[25],null]},{"Or":[24,26]},{"And":[[3,27],1]},{"Or":[28]},{"WithSkip":[20,29]},{"Rep":30},{"And":[[31],null]},{"Token":10},{"Token":6},{"And":[[33,34,6],1]},{"Or":[35]},{"Token":4},{"Layer":[10,5]},{"Token":5},{"And":[[37,38,39],1]},{"Or":[40]},{"Token":8},{"And":[[42],null]},{"Token":11},{"And":[[44],null]},{"Token":10},{"And":[[46],null]},{"Token":9},{"And":[[48],null]},{"Token":2},{"And":[[50],null]},{"Token":4},{"And":[[52],null]},{"Or":[43,45,47,49,51,53]},{"Token":7},"Eof",{"Not":56},{"And":[[55,57],null]},"Eof",{"And":[[59],null]},{"Or":[58,60]},{"And":[[6,61],1]},{"Or":[62]},{"WithSkip":[54,63]},{"Rep":64},{"And":[[65],null]},{"And":[[7],null]},{"And":[[1],null]},{"And":[[4],null]},{"Token":8},{"And":[[70],null]},{"Token":11},{"And":[[72],null]},{"Token":10},{"And":[[74],null]},{"Token":9},{"And":[[76],null]},{"Or":[71,73,75,77]},{"Rep":9},{"And":[[79],null]},{"Token":2},{"Token":3},{"And":[[81,8,82],1]},{"Token":3},{"Not":84},"Any",{"And":[[85,86],null]},{"Or":[87]},{"And":[[88],null]},{"Rep":11},{"And":[[90],null]},{"Token":4},{"Token":5},{"And":[[92,10,93],1]},{"Token":5},{"Not":95},"Any",{"And":[[96,97],null]},{"Or":[98]},{"And":[[99],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, LBRACE, RBRACE, LBRACK, RBRACK, COLON, COMMA, NULL, BOOL, STRING, NUMBER, FILE, OBJECT, FIELD, ARRAY, PRIMITIVE,
            ],
            syntactical_rules: rt::parser_from_str(parser_json),
            
            .. Default::default()
        }
    }
    use self::rt::*;
    lazy_static! {
        static ref LANG: rt::Language = {
            struct Impl { parser_definition: rt::ParserDefinition, lexer: rt::RegexLexer };
            impl rt::LanguageImpl for Impl {
                fn parse(
                    &self,
                    text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &::std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<::std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => rt::NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        LBRACE => rt::NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                        RBRACE => rt::NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                        LBRACK => rt::NodeTypeInfo { name: "LBRACK", whitespace_like: false },
                        RBRACK => rt::NodeTypeInfo { name: "RBRACK", whitespace_like: false },
                        COLON => rt::NodeTypeInfo { name: "COLON", whitespace_like: false },
                        COMMA => rt::NodeTypeInfo { name: "COMMA", whitespace_like: false },
                        NULL => rt::NodeTypeInfo { name: "NULL", whitespace_like: false },
                        BOOL => rt::NodeTypeInfo { name: "BOOL", whitespace_like: false },
                        STRING => rt::NodeTypeInfo { name: "STRING", whitespace_like: false },
                        NUMBER => rt::NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        FILE => rt::NodeTypeInfo { name: "FILE", whitespace_like: false },
                        OBJECT => rt::NodeTypeInfo { name: "OBJECT", whitespace_like: false },
                        FIELD => rt::NodeTypeInfo { name: "FIELD", whitespace_like: false },
                        ARRAY => rt::NodeTypeInfo { name: "ARRAY", whitespace_like: false },
                        PRIMITIVE => rt::NodeTypeInfo { name: "PRIMITIVE", whitespace_like: false },
                        _ => panic!("Unknown rt::NodeType: {:?}", ty)
                    }
                }
            }

            rt::Language::new(Impl {
                parser_definition: create_parser_definition(),
                lexer: create_lexer()
            })
        };
    }

    &*LANG
}



