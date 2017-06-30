use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::{Text, TextRange, AstNode, Node, NodeType};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use ::{STRING, IDENT, SIMPLE_STRING, HASH_STRING, AST_SELECTOR, QUESTION, DOT, STAR, PUB,
       LexRule, SynRule, FallFile, VerbatimDef, MethodDef,
       RefExpr, AstClassDef, AstDef, Expr, Attributes, Attribute, ExampleDef,
       CallExpr};

impl<'f> FallFile<'f> {
    pub fn resolve_rule(&self, name: Text<'f>) -> Option<SynRule<'f>> {
        self.syn_rules()
            .find(|r| r.name().is_some() && r.name().unwrap() == name)
    }

    pub fn resolve_ty(&self, name: Text<'f>) -> Option<usize> {
        self.node_types().iter().position(|&it| it.0 == name)
            .map(|idx| idx + 1)
    }

    pub fn node_types(&self) -> Vec<(Text<'f>, bool)> {
        let mut result = Vec::new();
        if let Some(tokenizer) = self.tokenizer_def() {
            result.extend(
                tokenizer.lex_rules()
                    .map(|r| (r.node_type(), r.is_skip()))
            )
        }
        result.extend(
            self.syn_rules()
                .filter(|r| r.is_pub() && r.type_attr().is_none())
                .filter_map(|r| r.name())
                .map(|n| (n, false))
        );
        result
    }

    pub fn contexts(&self) -> Vec<Text<'f>> {
        let mut result = Vec::new();
        Visitor(&mut result)
            .visit::<CallExpr, _>(|contexts, call| {
                if call.fn_name() == "is_in" || call.fn_name() == "enter" {
                    if let Some(ctx) = call.context() {
                        contexts.push(ctx);
                    }
                }
            })
            .walk_recursively_children_first(self.node());
        result
    }
}

impl<'f> LexRule<'f> {
    pub fn token_re(&self) -> Option<String> {
        let raw = match self.raw_re() {
            Some(raw) => raw,
            None => return None,
        };

        if raw.starts_with("r") {
            Some(lit_body(raw).to_string())
        } else {
            Some(::regex::escape(&lit_body(raw).to_cow()))
        }
    }

    pub fn extern_fn(&self) -> Option<Text<'f>> {
        children_of_type(self.node(), STRING).nth(1).map(|n| {
            lit_body(n.text())
        })
    }

    pub fn token_name(&self) -> Text<'f> {
        if let Some(r) = self.raw_re() {
            if r.starts_with("'") {
                return r
            }
        }
        self.node_type()
    }

    pub fn is_contextual(&self) -> bool {
        if let Some(attrs) = self.attributes() {
            return attrs.has_attribute("contextual")
        }
        false
    }

    pub fn is_skip(&self) -> bool {
        if let Some(attrs) = self.attributes() {
            return attrs.has_attribute("skip")
        }
        false
    }

    pub fn node_type_index(&self) -> usize {
        let file = ast_parent_exn::<FallFile>(self.node());
        file.resolve_ty(self.node_type()).unwrap()
    }

    fn raw_re(&self) -> Option<Text<'f>> {
        children_of_type(self.node(), STRING).next().map(|child| child.text())
    }
}

pub enum PratKind {
    Atom,
    Bin(u32),
    Postfix
}

impl<'f> SynRule<'f> {
    pub fn resolve_ty(&self) -> Option<usize> {
        if !self.is_pub() || self.is_pratt() {
            return None
        }

        let file = ast_parent_exn::<FallFile>(self.node());
        if let Some(name) = self.ty_name() {
            file.resolve_ty(name)
        } else {
            None
        }
    }

    pub fn is_pub(&self) -> bool {
        child_of_type(self.node(), PUB).is_some()
    }

    pub fn index(&self) -> usize {
        let file = ast_parent_exn::<FallFile>(self.node());
        file.syn_rules().position(|r| r.node() == self.node()).unwrap()
    }

    pub fn pratt_kind(&self) -> Option<PratKind> {
        let attrs = match self.attributes() {
            Some(attrs) => attrs,
            None => return None,
        };

        if attrs.has_attribute("atom") {
            Some(PratKind::Atom)
        } else if attrs.has_attribute("postfix") {
            Some(PratKind::Postfix)
        } else if let Some(priority) = attrs.find("bin").and_then(|attr| attr.u32_value()) {
            Some(PratKind::Bin(priority))
        } else {
            None
        }
    }

    pub fn is_pratt(&self) -> bool {
        self.has_attribute("pratt")
    }


    fn has_attribute(&self, attribute: &str) -> bool {
        if let Some(attrs) = self.attributes() {
            attrs.has_attribute(attribute)
        } else {
            false
        }
    }

    fn type_attr(&self) -> Option<Attribute<'f>> {
        self.attributes().and_then(|attrs| attrs.find("type"))
    }

    fn ty_name(&self) -> Option<Text<'f>> {
        if let Some(ty) = self.type_attr() {
            return ty.text_value();
        }
        self.name()
    }
}

impl<'f> VerbatimDef<'f> {
    pub fn contents(&self) -> Text<'f> {
        let literal_text = child_of_type_exn(self.node(), HASH_STRING).text();
        lit_body(literal_text).trim()
    }
}

impl<'f> ExampleDef<'f> {
    pub fn contents(&self) -> Text<'f> {
        let literal_text = child_of_type_exn(self.node(), HASH_STRING).text();
        lit_body(literal_text).trim()
    }
}

impl<'f> MethodDef<'f> {
    pub fn selector_kind(&self) -> SelectorKind<'f> {
        let selector = child_of_type_exn(self.node(), AST_SELECTOR);
        let name = self.selector_name();
        if has(selector, QUESTION) && has(selector, DOT) {
            SelectorKind::OptText(name)
        } else if has(selector, QUESTION) {
            SelectorKind::Opt(name)
        } else if has(selector, STAR) {
            SelectorKind::Many(name)
        } else if has(selector, DOT) {
            SelectorKind::Text(name)
        } else {
            SelectorKind::Single(name)
        }
    }

    pub fn is_class(&self) -> bool {
        let ast_def = ast_parent_exn::<AstDef>(self.node());
        let name = self.selector_name();
        ast_def.ast_classes().find(|cls| cls.name() == name).is_some()
    }

    fn selector_name(&self) -> Text<'f> {
        let selector = child_of_type_exn(self.node(), AST_SELECTOR);
        child_of_type_exn(selector, IDENT).text()
    }
}

impl<'f> AstClassDef<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type(self.node(), IDENT).unwrap().text()
    }

    pub fn variants<'a>(&'a self) -> Box<Iterator<Item=Text<'f>> + 'a> {
        Box::new(children_of_type(self.node(), IDENT).skip(1).map(|it| it.text()))
    }
}

pub enum SelectorKind<'f> {
    Single(Text<'f>),
    Opt(Text<'f>),
    Many(Text<'f>),
    Text(Text<'f>),
    OptText(Text<'f>),
}

pub enum RefKind<'f> {
    Token(LexRule<'f>),
    RuleReference(SynRule<'f>),
}

impl<'f> RefExpr<'f> {
    pub fn resolve(&self) -> Option<RefKind<'f>> {
        let file = ast_parent_exn::<FallFile>(self.node());

        if let Some(ident) = child_of_type(self.node(), IDENT) {
            if let Some(rule) = file.resolve_rule(ident.text()) {
                return Some(RefKind::RuleReference(rule))
            }
        }
        let token_name = child_of_type(self.node(), IDENT)
            .unwrap_or_else(|| child_of_type_exn(self.node(), SIMPLE_STRING))
            .text();

        file.tokenizer_def()
            .and_then(|td| td.lex_rules().find(|r| r.token_name() == token_name))
            .map(|rule| RefKind::Token(rule))
    }
}

pub enum CallKind<'f> {
    Eof,
    Enter(u32, Expr<'f>),
    IsIn(u32),
    Not(Vec<usize>),
    Rep(Expr<'f>),
    NotAhead(Expr<'f>),
    Opt(Expr<'f>),
    Layer(Expr<'f>, Expr<'f>),
    WithSkip(Expr<'f>, Expr<'f>),
    RuleCall(SynRule<'f>, Vec<(u32, Expr<'f>)>)
}

impl<'f> CallExpr<'f> {
    pub fn context(&self) -> Option<Text<'f>> {
        if let Some(Expr::RefExpr(e)) = self.args().next() {
            if let Some(context) = child_of_type(e.node(), SIMPLE_STRING) {
                return Some(lit_body(context.text()))
            }
        }
        None
    }

    pub fn resolve_context(&self) -> Option<u32> {
        if let Some(Expr::RefExpr(e)) = self.args().next() {
            if let Some(context) = child_of_type(e.node(), SIMPLE_STRING) {
                let ctx_name = lit_body(context.text());
                let file: FallFile = ast_parent_exn(self.node());
                return file.contexts().into_iter().position(|c| c == ctx_name).map(|i| i as u32)
            }
        }
        None
    }

    pub fn kind(&self) -> Result<CallKind<'f>, &'static str> {
        let file: FallFile = ast_parent_exn(self.node());
        macro_rules! check_args {
            ($n:expr) => {
                if self.args().count() != $n {
                    return Err(concat!("expected ", $n, " arguments"))
                }
            }
        };

        let kind = match self.fn_name().to_cow().as_ref() {
            "eof" => {
                check_args!(0);
                CallKind::Eof
            }
            "enter" => {
                check_args!(2);
                let ctx = self.resolve_context().ok_or("enter without context")?;
                let arg = self.args().nth(1).unwrap();
                CallKind::Enter(ctx, arg)
            }
            "is_in" => {
                check_args!(1);
                let ctx = self.resolve_context().ok_or("is_in without context")?;
                CallKind::IsIn(ctx)
            }
            "not" => {
                check_args!(1);
                let arg = self.args().next().unwrap();
                let ts = arg.token_set().ok_or("expected a tokenst")?;
                CallKind::Not(ts)
            }
            "rep" => {
                check_args!(1);
                CallKind::Rep(self.args().next().unwrap())
            }
            "not_ahead" => {
                check_args!(1);
                CallKind::NotAhead(self.args().next().unwrap())
            }
            "opt" => {
                check_args!(1);
                CallKind::Opt(self.args().next().unwrap())
            }
            "layer" => {
                check_args!(2);
                CallKind::Layer(self.args().nth(0).unwrap(), self.args().nth(1).unwrap())
            },
            "with_skip" => {
                check_args!(2);
                CallKind::WithSkip(self.args().nth(0).unwrap(), self.args().nth(1).unwrap())
            },
            _ => {
                if let Some(rule) = file.resolve_rule(self.fn_name()) {

                }
                return Err("unknown rule")
            },

        };

        Ok(kind)
    }
}

impl<'f> Expr<'f> {
    pub fn token_set(&self) -> Option<Vec<usize>> {
        match *self {
            Expr::RefExpr(ref_) => {
                if let Some(RefKind::Token(rule)) = ref_.resolve() {
                    Some(vec![rule.node_type_index()])
                } else {
                    None
                }
            }
            Expr::CallExpr(_) => None,
            Expr::SeqExpr(seq) => {
                let mut parts = seq.parts();
                match (parts.next(), parts.next()) {
                    (None, None) => Some(Vec::new()),
                    (Some(expr), None) => expr.token_set(),
                    _ => None
                }
            }
            Expr::BlockExpr(block) => {
                let mut result = Vec::new();
                for alt in block.alts() {
                    if let Some(ts) = alt.token_set() {
                        result.extend(ts.into_iter())
                    } else {
                        return None
                    }
                }
                Some(result)
            }
        }
    }
}

impl<'f> Attributes<'f> {
    pub fn has_attribute(&self, name: &str) -> bool {
        self.find(name).is_some()
    }

    pub fn find(&self, name: &str) -> Option<Attribute<'f>> {
        self.attributes().find(|a| a.name() == name)
    }
}

impl<'f> Attribute<'f> {
    pub fn u32_value(&self) -> Option<u32> {
        self.text_value()
            .and_then(|text| text.to_cow().parse().ok())
    }

    pub fn text_value(&self) -> Option<Text<'f>> {
        self.value().map(|v| v.node().text())
    }
}

fn lit_body(lit: Text) -> Text {
    let q = if lit.starts_with("'") { "'" } else { "\"" };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    lit.slice(TextRange::from_to(s + 1, e))
}

fn has(node: Node, ty: NodeType) -> bool {
    child_of_type(node, ty).is_some()
}
