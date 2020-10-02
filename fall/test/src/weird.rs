use fall_parse::runtime as rt;
pub use self::rt::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const RAW_STRING: rt::NodeType = rt::NodeType(101);
pub const FOO: rt::NodeType = rt::NodeType(102);
pub const BAR: rt::NodeType = rt::NodeType(103);
pub const T1: rt::NodeType = rt::NodeType(104);
pub const T2: rt::NodeType = rt::NodeType(105);
pub const T3: rt::NodeType = rt::NodeType(106);
pub const T4: rt::NodeType = rt::NodeType(107);
pub const LBRACE: rt::NodeType = rt::NodeType(108);
pub const RBRACE: rt::NodeType = rt::NodeType(109);
pub const ATOM: rt::NodeType = rt::NodeType(110);
pub const FILE: rt::NodeType = rt::NodeType(111);
pub const PRIVATE_PARTIAL: rt::NodeType = rt::NodeType(112);
pub const EMPTY: rt::NodeType = rt::NodeType(113);
pub const BLOCK: rt::NodeType = rt::NodeType(114);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(RAW_STRING, "r#+\"", Some(parse_raw_string)),
            rt::LexRule::new(FOO, "foo", None),
            rt::LexRule::new(BAR, "bar", None),
            rt::LexRule::new(T1, "_1", None),
            rt::LexRule::new(T2, "_2", None),
            rt::LexRule::new(T3, "_3", None),
            rt::LexRule::new(T4, "_4", None),
            rt::LexRule::new(LBRACE, "\\{", None),
            rt::LexRule::new(RBRACE, "\\}", None),
            rt::LexRule::new(ATOM, "\\w+", None),
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":12,"body":19,"replaceable":false}},{"Pub":{"ty":13,"body":22,"replaceable":false}},{"Or":[25]},{"Or":[28]},{"Pub":{"ty":14,"body":31,"replaceable":false}},{"Or":[32]},{"Pub":{"ty":15,"body":36,"replaceable":false}},{"Or":[38]},{"Or":[41,47]},{"Token":5},{"Token":2},{"And":[[9,10],null]},{"Token":6},{"Token":11},{"And":[[12,4,13,4],null]},{"Token":7},{"And":[[15,1],null]},{"Token":8},{"And":[[17,6],null]},{"Or":[11,14,16,18]},{"And":[[2],null]},{"And":[[3],null]},{"Or":[20,21]},{"Token":3},{"Token":4},{"And":[[23,24],null]},{"Token":3},{"Token":3},{"And":[[26,27],null]},{"Opt":5},{"And":[[29],null]},{"Or":[30]},{"And":[[],null]},{"Token":9},{"Token":10},{"And":[[33,7,34],1]},{"Or":[35]},{"Rep":8},{"And":[[37],null]},{"Token":9},{"Token":10},{"And":[[39,7,40],1]},{"Token":10},{"Not":42},"Any",{"And":[[43,44],null]},{"Or":[45]},{"And":[[46],null]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, RAW_STRING, FOO, BAR, T1, T2, T3, T4, LBRACE, RBRACE, ATOM, FILE, PRIVATE_PARTIAL, EMPTY, BLOCK,
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
                ) -> Option<Box<dyn std::any::Any + Sync + Send>> {
                    rt::parse(&LANG, &self.lexer, &self.parser_definition, text, metrics, builder)
                }

                fn reparse(
                    &self,
                    incremental_data: &dyn std::any::Any,
                    edit: &rt::TextEdit,
                    new_text: rt::Text,
                    metrics: &rt::Metrics,
                    builder: &mut rt::TreeBuilder,
                ) -> Option<Box<dyn std::any::Any + Sync + Send>> {
                    rt::reparse(&LANG, &self.lexer, &self.parser_definition, incremental_data, edit, new_text, metrics, builder)
                }

                fn node_type_info(&self, ty: rt::NodeType) -> rt::NodeTypeInfo {
                    match ty {
                        ERROR => rt::NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => rt::NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        RAW_STRING => rt::NodeTypeInfo { name: "RAW_STRING", whitespace_like: false },
                        FOO => rt::NodeTypeInfo { name: "FOO", whitespace_like: false },
                        BAR => rt::NodeTypeInfo { name: "BAR", whitespace_like: false },
                        T1 => rt::NodeTypeInfo { name: "T1", whitespace_like: false },
                        T2 => rt::NodeTypeInfo { name: "T2", whitespace_like: false },
                        T3 => rt::NodeTypeInfo { name: "T3", whitespace_like: false },
                        T4 => rt::NodeTypeInfo { name: "T4", whitespace_like: false },
                        LBRACE => rt::NodeTypeInfo { name: "LBRACE", whitespace_like: false },
                        RBRACE => rt::NodeTypeInfo { name: "RBRACE", whitespace_like: false },
                        ATOM => rt::NodeTypeInfo { name: "ATOM", whitespace_like: false },
                        FILE => rt::NodeTypeInfo { name: "FILE", whitespace_like: false },
                        PRIVATE_PARTIAL => rt::NodeTypeInfo { name: "PRIVATE_PARTIAL", whitespace_like: false },
                        EMPTY => rt::NodeTypeInfo { name: "EMPTY", whitespace_like: false },
                        BLOCK => rt::NodeTypeInfo { name: "BLOCK", whitespace_like: false },
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

fn parse_raw_string(s: &str) -> Option<usize> {
    let quote_start = s.find('"').unwrap();
    let q_hashes = concat!('"', "######", "######", "######", "######", "######");
    let closing = &q_hashes[..quote_start];
    s[quote_start + 1..].find(closing).map(|i| i + quote_start + 1 + closing.len())
}

