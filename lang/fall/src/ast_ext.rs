use fall_tree::{Text, TextRange, AstNode, Node, NodeType};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use ::{STRING, IDENT, SIMPLE_STRING, HASH_STRING, AST_SELECTOR, QUESTION, DOT, STAR, KW_PUB,
       LexRule, SynRule, FallFile, VerbatimDef, MethodDef,
       RefExpr, AstClassDef, AstDef, Expr};

impl<'f> FallFile<'f> {
    pub fn resolve_rule(&self, name: Text<'f>) -> Option<usize> {
        self.syn_rules().position(|r| r.name() == name)
    }

    pub fn resolve_ty(&self, name: Text<'f>) -> Option<usize> {
        self.node_types().iter().position(|&it| it == name)
            .map(|idx| idx + 2)
    }

    pub fn node_types(&self) -> Vec<Text<'f>> {
        let mut result = Vec::new();
        if let Some(tokenizer) = self.tokenizer_def() {
            result.extend(
                tokenizer.lex_rules()
                    .map(|r| r.node_type())
                    .filter(|&n| n != "whitespace")
            )
        }
        result.extend(
            self.syn_rules()
                .filter(|r| r.is_pub())
                .map(|r| r.name())
        );
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

    fn raw_re(&self) -> Option<Text<'f>> {
        children_of_type(self.node(), STRING).next().map(|child| child.text())
    }
}

impl<'f> SynRule<'f> {
    pub fn resolve_ty(&self) -> Option<usize> {
        let file = ast_parent_exn::<FallFile>(self.node());
        file.resolve_ty(self.name())
    }

    pub fn is_pub(&self) -> bool {
        child_of_type(self.node(), KW_PUB).is_some()
    }
}

impl<'f> VerbatimDef<'f> {
    pub fn contents(&self) -> Text<'f> {
        let literal_text = child_of_type_exn(self.node(), HASH_STRING).text();
        lit_body(literal_text).trim()
    }
}

impl<'f> MethodDef<'f> {
    pub fn selector_kind(&self) -> SelectorKind<'f> {
        let selector = child_of_type_exn(self.node(), AST_SELECTOR);
        let name = self.selector_name();
        if has(selector, QUESTION) {
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
}

pub enum RefKind {
    Token(usize),
    RuleReference { idx: usize },
}

impl<'f> RefExpr<'f> {
    pub fn resolve(&self) -> Option<RefKind> {
        let file = ast_parent_exn::<FallFile>(self.node());

        if let Some(ident) = child_of_type(self.node(), IDENT) {
            if let Some(idx) = file.resolve_rule(ident.text()) {
                return Some(RefKind::RuleReference { idx: idx })
            }
        }
        let token_name = child_of_type(self.node(), IDENT)
            .unwrap_or_else(|| child_of_type_exn(self.node(), SIMPLE_STRING))
            .text();

        match file.tokenizer_def().and_then(|td| td.lex_rules().find(|r| r.token_name() == token_name)) {
            Some(rule) => {
                let ty_name = rule.node_type();
                file.resolve_ty(ty_name).map(RefKind::Token)
            }
            None => None,
        }
    }
}

impl<'f> Expr<'f> {
    pub fn token_set(&self) -> Option<Vec<usize>> {
        match *self {
            Expr::RefExpr(ref_) => {
                if let Some(RefKind::Token(idx)) = ref_.resolve() {
                    Some(vec![idx])
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

fn lit_body(lit: Text) -> Text {
    let q = if lit.starts_with("'") { "'" } else { "\"" };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    lit.slice(TextRange::from_to(s + 1, e))
}

fn has(node: Node, ty: NodeType) -> bool {
    child_of_type(node, ty).is_some()
}
