use fall_tree::visitor::{Visitor, NodeVisitor};
use fall_tree::{Text, TextRange, AstNode};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use ::{STRING, IDENT, SIMPLE_STRING, PUB,
       LexRule, SynRule, FallFile, VerbatimDef, MethodDef,
       RefExpr, AstNodeDef, AstClassDef, AstDef, Expr, Attributes, Attribute, TestDef,
       CallExpr, Parameter, AstSelector};

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
        Visitor(Vec::new())
            .visit::<CallExpr, _>(|contexts, call| {
                if call.fn_name() == "is_in" || call.fn_name() == "enter" || call.fn_name() == "exit" {
                    if let Some(ctx) = call.context() {
                        contexts.push(ctx);
                    }
                }
            })
            .walk_recursively_children_first(self.node())
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
        let file = ast_parent_exn::<FallFile>(self.node());
        file.resolve_ty(self.node_type()).unwrap()
    }

    fn re(&self) -> Option<Text<'f>> {
        child_of_type(self.node(), STRING).map(|n| n.text())
    }
}

pub enum PratKind {
    Atom,
    Bin(u32),
    Postfix,
    Prefix(u32),
}

impl<'f> SynRule<'f> {
    pub fn resolve_ty(&self) -> Option<usize> {
        if !self.is_pub() || self.is_pratt() {
            return None;
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
        } else if attrs.has_attribute("prefix") {
            let priority = attrs.find("prefix").and_then(|attr| attr.u32_value()).unwrap_or(999);
            Some(PratKind::Prefix(priority))
        } else if let Some(priority) = attrs.find("bin").and_then(|attr| attr.u32_value()) {
            Some(PratKind::Bin(priority))
        } else {
            None
        }
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
        let file = ast_parent_exn::<FallFile>(self.node());
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
        let file = ast_parent_exn::<FallFile>(self.node());

        let ast_def = ast_parent_exn::<AstDef>(self.node());
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

pub enum RefKind<'f> {
    Token(LexRule<'f>),
    RuleReference(SynRule<'f>),
    Param(Parameter<'f>),
}

impl<'f> RefExpr<'f> {
    pub fn resolve(&self) -> Option<RefKind<'f>> {
        let file = ast_parent_exn::<FallFile>(self.node());

        if let Some(ident) = child_of_type(self.node(), IDENT) {
            let rule: SynRule = ast_parent_exn(self.node());
            if let Some(parameters) = rule.parameters() {
                if let Some(p) = parameters.parameters().find(|p| p.name() == ident.text()) {
                    return Some(RefKind::Param(p));
                }
            }

            if let Some(rule) = file.resolve_rule(ident.text()) {
                return Some(RefKind::RuleReference(rule));
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
    Any,
    Commit,
    Enter(u32, Expr<'f>),
    Exit(u32, Expr<'f>),
    IsIn(u32),
    Not(Expr<'f>),
    Layer(Expr<'f>, Expr<'f>),
    WithSkip(Expr<'f>, Expr<'f>),
    RuleCall(SynRule<'f>, Vec<(u32, Expr<'f>)>),
    PrevIs(Vec<usize>)
}

impl<'f> CallExpr<'f> {
    pub fn context(&self) -> Option<Text<'f>> {
        if let Some(Expr::RefExpr(e)) = self.args().next() {
            if let Some(context) = child_of_type(e.node(), SIMPLE_STRING) {
                return Some(lit_body(context.text()));
            }
        }
        None
    }

    pub fn resolve_context(&self) -> Option<u32> {
        if let Some(Expr::RefExpr(e)) = self.args().next() {
            if let Some(context) = child_of_type(e.node(), SIMPLE_STRING) {
                let ctx_name = lit_body(context.text());
                let file: FallFile = ast_parent_exn(self.node());
                return file.contexts().into_iter().position(|c| c == ctx_name).map(|i| i as u32);
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
            "any" => {
                check_args!(0);
                CallKind::Any
            }
            "commit" => {
                check_args!(0);
                CallKind::Commit
            }
            "enter" => {
                check_args!(2);
                let ctx = self.resolve_context().ok_or("enter without context")?;
                let arg = self.args().nth(1).unwrap();
                CallKind::Enter(ctx, arg)
            }
            "exit" => {
                check_args!(2);
                let ctx = self.resolve_context().ok_or("exit without context")?;
                let arg = self.args().nth(1).unwrap();
                CallKind::Exit(ctx, arg)
            }
            "is_in" => {
                check_args!(1);
                let ctx = self.resolve_context().ok_or("is_in without context")?;
                CallKind::IsIn(ctx)
            }
            "not" => {
                check_args!(1);
                CallKind::Not(self.args().next().unwrap())
            }
            "layer" => {
                check_args!(2);
                CallKind::Layer(self.args().nth(0).unwrap(), self.args().nth(1).unwrap())
            }
            "with_skip" => {
                check_args!(2);
                CallKind::WithSkip(self.args().nth(0).unwrap(), self.args().nth(1).unwrap())
            }
            "prev_is" => {
                let mut args = Vec::new();
                for arg in self.args() {
                    if let Expr::RefExpr(expr) = arg {
                        if let Some(RefKind::RuleReference(rule)) = expr.resolve() {
                            if let Some(ty) = rule.resolve_ty() {
                                args.push(ty)
                            } else {
                                return Err("Bad prev_is");
                            }
                        } else {
                            return Err("Bad prev_is");
                        }
                    } else {
                        return Err("Bad prev_is");
                    }
                }
                CallKind::PrevIs(args)
            }
            _ => {
                if let Some(rule) = file.resolve_rule(self.fn_name()) {
                    if let Some(parameters) = rule.parameters() {
                        let params = parameters.parameters()
                            .map(|p| p.idx())
                            .zip(self.args())
                            .collect();
                        return Ok(CallKind::RuleCall(rule, params));
                    }
                }
                return Err("unknown rule");
            }
        };

        Ok(kind)
    }
}

impl<'f> Parameter<'f> {
    pub fn idx(&self) -> u32 {
        let file: FallFile = ast_parent_exn(self.node());
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

fn lit_body(lit: Text) -> Text {
    let q = if lit.starts_with("'") { "'" } else { "\"" };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    lit.slice(TextRange::from_to(s + 1, e))
}
