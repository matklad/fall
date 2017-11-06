use fall_tree::{Text, TextRange, AstNode, AstClass};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type};
use fall_tree::search::ast;

use ::{STRING, IDENT, SIMPLE_STRING, PUB,
       LexRule, SynRule, FallFile, VerbatimDef, MethodDef,
       RefExpr, AstNodeDef, AstClassDef, AstDef, Attributes, Attribute, TestDef,
       CallExpr, Parameter, AstSelector};

impl<'f> FallFile<'f> {
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

    fn resolve_ty(&self, name: Text<'f>) -> Option<usize> {
        self.node_types().iter().position(|&it| it.0 == name)
            .map(|idx| idx + 1)
    }
}

impl<'f> LexRule<'f> {
    pub fn token_re(&self) -> Option<String> {
        let raw = match self.re() {
            Some(raw) => raw,
            None => return None,
        };

        if raw.starts_with("r") {
            Some(lit_body(raw).to_string())
        } else {
            Some(::regex::escape(&lit_body(raw).to_cow()))
        }
    }

    pub fn token_text(&self) -> Option<Text<'f>> {
        let raw = match self.re() {
            Some(raw) => raw,
            None => return None,
        };

        if raw.starts_with("r") {
            None
        } else {
            Some(lit_body(raw))
        }
    }

    pub fn extern_fn(&self) -> Option<Text<'f>> {
        children_of_type(self.node(), STRING).nth(1).map(|n| {
            lit_body(n.text())
        })
    }

    pub fn token_name(&self) -> Text<'f> {
        if let Some(r) = self.re() {
            if r.starts_with("'") {
                return r;
            }
        }
        self.node_type()
    }

    pub fn is_contextual(&self) -> bool {
        if let Some(attrs) = self.attributes() {
            return attrs.has_attribute("contextual");
        }
        false
    }

    pub fn is_skip(&self) -> bool {
        if let Some(attrs) = self.attributes() {
            return attrs.has_attribute("skip");
        }
        false
    }

    pub fn node_type_index(&self) -> usize {
        let file = ast::ancestor_exn::<FallFile>(self.node());
        file.resolve_ty(self.node_type()).unwrap()
    }

    fn re(&self) -> Option<Text<'f>> {
        child_of_type(self.node(), STRING).map(|n| n.text())
    }
}

impl<'f> SynRule<'f> {
    pub fn resolve_ty(&self) -> Option<usize> {
        if !self.is_pub() || self.is_pratt() {
            return None;
        }

        let file = ast::ancestor_exn::<FallFile>(self.node());
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
        let file = ast::ancestor_exn::<FallFile>(self.node());
        file.syn_rules().position(|r| r.node() == self.node()).unwrap()
    }

    pub fn is_pratt(&self) -> bool {
        self.has_attribute("pratt")
    }

    pub fn is_replaceable(&self) -> bool {
        self.has_attribute("replaceable")
    }

    pub fn is_replaces(&self) -> bool {
        self.has_attribute("replaces")
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
        lit_body(self.literal_string()).trim()
    }
}

impl<'f> TestDef<'f> {
    pub fn contents(&self) -> Option<Text<'f>> {
        self.literal_string().map(|s| lit_body(s).trim())
    }
}

impl<'f> MethodDef<'f> {
    pub fn resolve(&self) -> Option<MethodDescription<'f>> {
        let file = ast::ancestor_exn::<FallFile>(self.node());
        let kind = match self.selector().child_kind() {
            None => return None,
            Some(kind) => kind,
        };
        let name = self.selector().child();

        if self.selector().dot().is_some() {
            return match (file.resolve_ty(name), kind) {
                (Some(_), ChildKind::Token(t)) => Some(MethodDescription::TextAccessor(t, self.arity())),
                _ => None
            };
        }

        Some(MethodDescription::NodeAccessor(kind, self.arity()))
    }

    fn arity(&self) -> Arity {
        if self.selector().optional().is_some() {
            Arity::Optional
        } else if self.selector().many().is_some() {
            Arity::Many
        } else {
            Arity::Single
        }
    }
}

impl<'f> AstSelector<'f> {
    pub fn child_kind(&self) -> Option<ChildKind<'f>> {
        let file = ast::ancestor_exn::<FallFile>(self.node());

        let ast_def = ast::ancestor_exn::<AstDef>(self.node());
        if let Some(ast) = ast_def.ast_nodes().find(|a| a.name() == self.child()) {
            return Some(ChildKind::AstNode(ast));
        }
        if let Some(class) = ast_def.ast_classes().find(|c| c.name() == self.child()) {
            return Some(ChildKind::AstClass(class));
        }

        if let Some(lex_rule) = file.tokenizer_def().and_then(|td| td.lex_rules().find(|r| r.node_type() == self.child())) {
            return Some(ChildKind::Token(lex_rule));
        }

        None
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

pub enum Arity {
    Single,
    Optional,
    Many
}

pub enum ChildKind<'f> {
    AstNode(AstNodeDef<'f>),
    AstClass(AstClassDef<'f>),
    Token(LexRule<'f>)
}

pub enum MethodDescription<'f> {
    NodeAccessor(ChildKind<'f>, Arity),
    TextAccessor(LexRule<'f>, Arity),
}

impl<'f> RefExpr<'f> {
    pub fn reference_name(&self) -> Text<'f> {
        child_of_type(self.node(), IDENT)
            .unwrap_or_else(|| child_of_type_exn(self.node(), SIMPLE_STRING))
            .text()
    }
}

impl<'f> CallExpr<'f> {
    pub fn context_name(&self) -> Option<Text<'f>> {
        if !(self.fn_name() == "is_in" || self.fn_name() == "enter" || self.fn_name() == "exit") {
            return None;
        }
        return self.args().next()
            .and_then(|arg| child_of_type(arg.node(), SIMPLE_STRING))
            .map(|ctx| lit_body(ctx.text()));
    }
}

impl<'f> Parameter<'f> {
    pub fn idx(&self) -> u32 {
        let file: FallFile = ast::ancestor_exn(self.node());
        let idx = file.syn_rules()
            .filter_map(|rule| rule.parameters())
            .flat_map(|p| p.parameters())
            .position(|p| p.node() == self.node())
            .unwrap();

        idx as u32
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

pub ( crate ) fn lit_body(lit: Text) -> Text {
    let q = if lit.starts_with("'") { "'" } else { "\"" };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    lit.slice(TextRange::from_to(s + 1, e))
}
