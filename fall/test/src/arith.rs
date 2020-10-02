use fall_parse::runtime as rt;
pub use self::rt::ERROR;

pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const NUMBER: rt::NodeType = rt::NodeType(101);
pub const PLUS: rt::NodeType = rt::NodeType(102);
pub const MINUS: rt::NodeType = rt::NodeType(103);
pub const STAR: rt::NodeType = rt::NodeType(104);
pub const SLASH: rt::NodeType = rt::NodeType(105);
pub const BANG: rt::NodeType = rt::NodeType(106);
pub const LPAREN: rt::NodeType = rt::NodeType(107);
pub const RPAREN: rt::NodeType = rt::NodeType(108);
pub const FILE: rt::NodeType = rt::NodeType(109);
pub const PRODUCT_EXPR: rt::NodeType = rt::NodeType(110);
pub const SUM_EXPR: rt::NodeType = rt::NodeType(111);
pub const CONSTANT_EXPR: rt::NodeType = rt::NodeType(112);
pub const PAREN_EXPR: rt::NodeType = rt::NodeType(113);
pub const FACTORIAL_EXPR: rt::NodeType = rt::NodeType(114);
pub const NEGATE_EXPR: rt::NodeType = rt::NodeType(115);


pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(NUMBER, "\\d+", None),
            rt::LexRule::new(PLUS, "\\+", None),
            rt::LexRule::new(MINUS, "\\-", None),
            rt::LexRule::new(STAR, "\\*", None),
            rt::LexRule::new(SLASH, "/", None),
            rt::LexRule::new(BANG, "!", None),
            rt::LexRule::new(LPAREN, "\\(", None),
            rt::LexRule::new(RPAREN, "\\)", None),
        ])
    }

    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":10,"body":9,"replaceable":false}},{"Pratt":{"atoms":[4,5],"prefixes":[{"ty":16,"op":21,"priority":999}],"infixes":[{"ty":12,"op":14,"priority":1,"has_rhs":true},{"ty":11,"op":19,"priority":2,"has_rhs":true},{"ty":15,"op":20,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":11,"body":28,"replaceable":false}},{"Pub":{"ty":12,"body":35,"replaceable":false}},{"Pub":{"ty":13,"body":38,"replaceable":false}},{"Pub":{"ty":14,"body":42,"replaceable":false}},{"Pub":{"ty":15,"body":45,"replaceable":false}},{"Pub":{"ty":16,"body":48,"replaceable":false}},{"And":[[1],null]},{"Or":[8]},{"Token":3},{"And":[[10],null]},{"Token":4},{"And":[[12],null]},{"Or":[11,13]},{"Token":5},{"And":[[15],null]},{"Token":6},{"And":[[17],null]},{"Or":[16,18]},{"Token":7},{"Token":4},{"Token":5},{"And":[[22],null]},{"Token":6},{"And":[[24],null]},{"Or":[23,25]},{"And":[[1,26,1],null]},{"Or":[27]},{"Token":3},{"And":[[29],null]},{"Token":4},{"And":[[31],null]},{"Or":[30,32]},{"And":[[1,33,1],null]},{"Or":[34]},{"Token":2},{"And":[[36],null]},{"Or":[37]},{"Token":8},{"Token":9},{"And":[[39,1,40],null]},{"Or":[41]},{"Token":7},{"And":[[1,43],null]},{"Or":[44]},{"Token":4},{"And":[[46,1],null]},{"Or":[47]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, NUMBER, PLUS, MINUS, STAR, SLASH, BANG, LPAREN, RPAREN, FILE, PRODUCT_EXPR, SUM_EXPR, CONSTANT_EXPR, PAREN_EXPR, FACTORIAL_EXPR, NEGATE_EXPR,
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
                        NUMBER => rt::NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        PLUS => rt::NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        MINUS => rt::NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        STAR => rt::NodeTypeInfo { name: "STAR", whitespace_like: false },
                        SLASH => rt::NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        BANG => rt::NodeTypeInfo { name: "BANG", whitespace_like: false },
                        LPAREN => rt::NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                        RPAREN => rt::NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                        FILE => rt::NodeTypeInfo { name: "FILE", whitespace_like: false },
                        PRODUCT_EXPR => rt::NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                        SUM_EXPR => rt::NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                        CONSTANT_EXPR => rt::NodeTypeInfo { name: "CONSTANT_EXPR", whitespace_like: false },
                        PAREN_EXPR => rt::NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                        FACTORIAL_EXPR => rt::NodeTypeInfo { name: "FACTORIAL_EXPR", whitespace_like: false },
                        NEGATE_EXPR => rt::NodeTypeInfo { name: "NEGATE_EXPR", whitespace_like: false },
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



