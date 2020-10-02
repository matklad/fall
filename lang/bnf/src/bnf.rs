use fall_parse::runtime as rt;
pub use self::rt::ERROR;
pub const WHITESPACE: rt::NodeType = rt::NodeType(100);
pub const ARROW: rt::NodeType = rt::NodeType(101);
pub const PIPE: rt::NodeType = rt::NodeType(102);
pub const TERMINAL: rt::NodeType = rt::NodeType(103);
pub const NONTERMINAL: rt::NodeType = rt::NodeType(104);
pub const GRAMMAR: rt::NodeType = rt::NodeType(105);
pub const PROD: rt::NodeType = rt::NodeType(106);
pub const PROD_BODY: rt::NodeType = rt::NodeType(107);
pub const ALT: rt::NodeType = rt::NodeType(108);
pub const SYMBOL: rt::NodeType = rt::NodeType(109);
pub fn language() -> &'static rt::Language {
    fn create_lexer() -> rt::RegexLexer {
        rt::RegexLexer::new(vec![
            rt::LexRule::new(WHITESPACE, "\\s+", None),
            rt::LexRule::new(ARROW, "\\->", None),
            rt::LexRule::new(PIPE, "\\|", None),
            rt::LexRule::new(TERMINAL, "\'[^\']+\'", None),
            rt::LexRule::new(NONTERMINAL, "\\w+", None),
        ])
    }
    fn create_parser_definition() -> rt::ParserDefinition {
        let parser_json = r##"[{"Pub":{"ty":6,"body":9,"replaceable":false}},{"Or":[12]},{"Pub":{"ty":7,"body":14,"replaceable":false}},{"Pub":{"ty":8,"body":20,"replaceable":false}},{"Pub":{"ty":9,"body":23,"replaceable":false}},{"Pub":{"ty":10,"body":30,"replaceable":false}},{"WithSkip":[1,2]},{"Rep":6},{"And":[[7],null]},{"Or":[8]},{"Token":5},{"Token":2},{"And":[[10,11],null]},{"And":[[1,3],1]},{"Or":[13]},{"Token":3},{"And":[[15,4],null]},{"Or":[16]},{"Rep":17},{"And":[[4,18],null]},{"Or":[19]},{"Rep":5},{"And":[[5,21],null]},{"Or":[22]},{"Token":5},{"Token":2},{"Not":25},{"And":[[24,26],null]},{"Token":4},{"And":[[28],null]},{"Or":[27,29]}]"##;
        ::fall_parse::ParserDefinition {
            node_types: vec![
                rt::ERROR,
                WHITESPACE, ARROW, PIPE, TERMINAL, NONTERMINAL, GRAMMAR, PROD, PROD_BODY, ALT, SYMBOL,
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
                        ARROW => rt::NodeTypeInfo { name: "ARROW", whitespace_like: false },
                        PIPE => rt::NodeTypeInfo { name: "PIPE", whitespace_like: false },
                        TERMINAL => rt::NodeTypeInfo { name: "TERMINAL", whitespace_like: false },
                        NONTERMINAL => rt::NodeTypeInfo { name: "NONTERMINAL", whitespace_like: false },
                        GRAMMAR => rt::NodeTypeInfo { name: "GRAMMAR", whitespace_like: false },
                        PROD => rt::NodeTypeInfo { name: "PROD", whitespace_like: false },
                        PROD_BODY => rt::NodeTypeInfo { name: "PROD_BODY", whitespace_like: false },
                        ALT => rt::NodeTypeInfo { name: "ALT", whitespace_like: false },
                        SYMBOL => rt::NodeTypeInfo { name: "SYMBOL", whitespace_like: false },
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
#[allow(unused)]
use self::rt::AstNode;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grammar<'f> { node: rt::Node<'f> }
impl<'f> rt::AstNode<'f> for Grammar<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == GRAMMAR {
            Some(Grammar { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}
impl<'f> Grammar<'f> {
    pub fn productions(&self) -> rt::AstChildren<'f, Prod<'f>> {
        rt::AstChildren::new(self.node().children())
    }
}
impl<'f> ::std::fmt::Debug for Grammar<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Grammar@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Prod<'f> { node: rt::Node<'f> }
impl<'f> rt::AstNode<'f> for Prod<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == PROD {
            Some(Prod { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}
impl<'f> Prod<'f> {
    pub fn head(&self) -> Nonterminal<'f> {
        rt::AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn alts(&self) -> rt::AstChildren<'f, Alt<'f>> {
        rt::AstChildren::new(self.node().children())
    }
}
impl<'f> ::std::fmt::Debug for Prod<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Prod@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Alt<'f> { node: rt::Node<'f> }
impl<'f> rt::AstNode<'f> for Alt<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == ALT {
            Some(Alt { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}
impl<'f> Alt<'f> {
    pub fn symbols(&self) -> rt::AstChildren<'f, Symbol<'f>> {
        rt::AstChildren::new(self.node().children())
    }
}
impl<'f> ::std::fmt::Debug for Alt<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Alt@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Terminal<'f> { node: rt::Node<'f> }
impl<'f> rt::AstNode<'f> for Terminal<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == TERMINAL {
            Some(Terminal { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}
impl<'f> Terminal<'f> {
}
impl<'f> ::std::fmt::Debug for Terminal<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Terminal@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonterminal<'f> { node: rt::Node<'f> }
impl<'f> rt::AstNode<'f> for Nonterminal<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if node.ty() == NONTERMINAL {
            Some(Nonterminal { node })
        } else {
            None
        }
    }
    fn node(self) -> rt::Node<'f> { self.node }
}
impl<'f> Nonterminal<'f> {
}
impl<'f> ::std::fmt::Debug for Nonterminal<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("Nonterminal@")?;
        self.node().range().fmt(f)?;
        Ok(())
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Symbol<'f> {
        Terminal(Terminal<'f>),
        Nonterminal(Nonterminal<'f>),
}
impl<'f> rt::AstNode<'f> for Symbol<'f> {
    fn wrap(node: rt::Node<'f>) -> Option<Self> {
        if let Some(n) = Terminal::wrap(node) {
            return Some(Symbol::Terminal(n))
        }
        if let Some(n) = Nonterminal::wrap(node) {
            return Some(Symbol::Nonterminal(n))
        }
        None
    }
    fn node(self) -> rt::Node<'f> {
        match self {
                Symbol::Terminal(n) => n.node(),
                Symbol::Nonterminal(n) => n.node(),
        }
    }
}
impl<'f> ::std::fmt::Debug for Symbol<'f> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(match *self {
                Symbol::Terminal(..) => "Terminal@",
                Symbol::Nonterminal(..) => "Nonterminal@",
        })?;
        rt::AstNode::node(*self).range().fmt(f)?;
        Ok(())
    }
}