use fall_parse::runtime::*;
use self::fall_tree::{Text, NodeType, NodeTypeInfo, Language, LanguageImpl, Metrics, IToken, INode, Event, TextEdit};
pub use self::fall_tree::ERROR;

pub const WHITESPACE: NodeType = NodeType(100);
pub const NUMBER: NodeType = NodeType(101);
pub const PLUS: NodeType = NodeType(102);
pub const MINUS: NodeType = NodeType(103);
pub const STAR: NodeType = NodeType(104);
pub const SLASH: NodeType = NodeType(105);
pub const BANG: NodeType = NodeType(106);
pub const LPAREN: NodeType = NodeType(107);
pub const RPAREN: NodeType = NodeType(108);
pub const FILE: NodeType = NodeType(109);
pub const PRODUCT_EXPR: NodeType = NodeType(110);
pub const SUM_EXPR: NodeType = NodeType(111);
pub const CONSTANT_EXPR: NodeType = NodeType(112);
pub const PAREN_EXPR: NodeType = NodeType(113);
pub const FACTORIAL_EXPR: NodeType = NodeType(114);
pub const NEGATE_EXPR: NodeType = NodeType(115);


pub fn language() -> &'static Language {
    fn create_lexer() -> ::fall_parse::RegexLexer {
        ::fall_parse::RegexLexer::new(vec![
            ::fall_parse::LexRule::new(WHITESPACE, "\\s+", None),
            ::fall_parse::LexRule::new(NUMBER, "\\d+", None),
            ::fall_parse::LexRule::new(PLUS, "\\+", None),
            ::fall_parse::LexRule::new(MINUS, "\\-", None),
            ::fall_parse::LexRule::new(STAR, "\\*", None),
            ::fall_parse::LexRule::new(SLASH, "/", None),
            ::fall_parse::LexRule::new(BANG, "!", None),
            ::fall_parse::LexRule::new(LPAREN, "\\(", None),
            ::fall_parse::LexRule::new(RPAREN, "\\)", None),
        ])
    }

    fn create_parser_definition() -> ::fall_parse::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":10,"body":9,"replaceable":false}},{"Pratt":{"atoms":[4,5],"prefixes":[{"ty":16,"op":21,"priority":999}],"infixes":[{"ty":12,"op":14,"priority":1,"has_rhs":true},{"ty":11,"op":19,"priority":2,"has_rhs":true},{"ty":15,"op":20,"priority":999,"has_rhs":false}]}},{"Pub":{"ty":11,"body":28,"replaceable":false}},{"Pub":{"ty":12,"body":35,"replaceable":false}},{"Pub":{"ty":13,"body":38,"replaceable":false}},{"Pub":{"ty":14,"body":42,"replaceable":false}},{"Pub":{"ty":15,"body":45,"replaceable":false}},{"Pub":{"ty":16,"body":48,"replaceable":false}},{"And":[[1],null]},{"Or":[8]},{"Token":3},{"And":[[10],null]},{"Token":4},{"And":[[12],null]},{"Or":[11,13]},{"Token":5},{"And":[[15],null]},{"Token":6},{"And":[[17],null]},{"Or":[16,18]},{"Token":7},{"Token":4},{"Token":5},{"And":[[22],null]},{"Token":6},{"And":[[24],null]},{"Or":[23,25]},{"And":[[1,26,1],null]},{"Or":[27]},{"Token":3},{"And":[[29],null]},{"Token":4},{"And":[[31],null]},{"Or":[30,32]},{"And":[[1,33,1],null]},{"Or":[34]},{"Token":2},{"And":[[36],null]},{"Or":[37]},{"Token":8},{"Token":9},{"And":[[39,1,40],null]},{"Or":[41]},{"Token":7},{"And":[[1,43],null]},{"Or":[44]},{"Token":4},{"And":[[46,1],null]},{"Or":[47]}]"##;

        ::fall_parse::ParserDefinition {
            node_types: vec![
                ERROR,
                WHITESPACE, NUMBER, PLUS, MINUS, STAR, SLASH, BANG, LPAREN, RPAREN, FILE, PRODUCT_EXPR, SUM_EXPR, CONSTANT_EXPR, PAREN_EXPR, FACTORIAL_EXPR, NEGATE_EXPR,
            ],
            syntactical_rules: serde_json::from_str(parser_json).unwrap(),
            
            .. Default::default()
        }
    }

    lazy_static! {
        static ref LANG: Language = {
            use fall_parse::ParserDefinition;

            struct Impl { parser_definition: ParserDefinition, lexer: ::fall_parse::RegexLexer };
            impl LanguageImpl for Impl {
                fn lexer(&self) -> &self::fall_tree::Lexer {
                    &self.lexer
                }

                fn parse(&self, text: Text, tokens: &[IToken], metrics: &Metrics) -> (Vec<Event>, INode) {
                    self.parser_definition.parse(text, tokens, &LANG, metrics)
                }

                fn reparse(&self, old_tokens: &[IToken], old_events: &[Event], edit: &TextEdit, text: Text, tokens: &[IToken], metrics: &Metrics) -> (Vec<Event>, INode) {
                    self.parser_definition.reparse(old_tokens, old_events, edit, text, tokens, &LANG, metrics)
                }

                fn node_type_info(&self, ty: NodeType) -> NodeTypeInfo {
                    match ty {
                        ERROR => NodeTypeInfo { name: "ERROR", whitespace_like: false },
                        WHITESPACE => NodeTypeInfo { name: "WHITESPACE", whitespace_like: true },
                        NUMBER => NodeTypeInfo { name: "NUMBER", whitespace_like: false },
                        PLUS => NodeTypeInfo { name: "PLUS", whitespace_like: false },
                        MINUS => NodeTypeInfo { name: "MINUS", whitespace_like: false },
                        STAR => NodeTypeInfo { name: "STAR", whitespace_like: false },
                        SLASH => NodeTypeInfo { name: "SLASH", whitespace_like: false },
                        BANG => NodeTypeInfo { name: "BANG", whitespace_like: false },
                        LPAREN => NodeTypeInfo { name: "LPAREN", whitespace_like: false },
                        RPAREN => NodeTypeInfo { name: "RPAREN", whitespace_like: false },
                        FILE => NodeTypeInfo { name: "FILE", whitespace_like: false },
                        PRODUCT_EXPR => NodeTypeInfo { name: "PRODUCT_EXPR", whitespace_like: false },
                        SUM_EXPR => NodeTypeInfo { name: "SUM_EXPR", whitespace_like: false },
                        CONSTANT_EXPR => NodeTypeInfo { name: "CONSTANT_EXPR", whitespace_like: false },
                        PAREN_EXPR => NodeTypeInfo { name: "PAREN_EXPR", whitespace_like: false },
                        FACTORIAL_EXPR => NodeTypeInfo { name: "FACTORIAL_EXPR", whitespace_like: false },
                        NEGATE_EXPR => NodeTypeInfo { name: "NEGATE_EXPR", whitespace_like: false },
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



