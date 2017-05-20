use fall_tree::{AstNode, Node, NodeType, AstClass, AstClassChildren};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use ::lang::{STRING, IDENT, SIMPLE_STRING, HASH_STRING, AST_SELECTOR, QUESTION, DOT, STAR,
             BLOCK_EXPR, REF_EXPR, CALL_EXPR, SEQ_EXPR,
             LexRule, SynRule, NodesDef, File, VerbatimDef, MethodDef,
             BlockExpr, RefExpr, CallExpr, SeqExpr};

impl<'f> File<'f> {
    pub fn resolve_rule(&self, name: &str) -> Option<usize> {
        self.syn_rules().position(|r| r.name() == name)
    }
}

impl<'f> NodesDef<'f> {
    pub fn nodes(&self) -> Vec<&'f str> {
        children_of_type(self.node(), IDENT)
            .map(|n| n.text())
            .collect()
    }
}

impl<'f> LexRule<'f> {
    pub fn token_re(&self) -> String {
        let raw = self.raw_re();
        if raw.starts_with('r') {
            lit_body(raw).to_owned()
        } else {
            ::regex::escape(lit_body(raw))
        }
    }

    pub fn extern_fn(&self) -> Option<&'f str> {
        children_of_type(self.node(), STRING).nth(1).map(|n| {
            lit_body(n.text())
        })
    }

    pub fn token_name(&self) -> &'f str {
        let r = self.raw_re();
        if r.starts_with('\'') {
            return r
        }
        self.node_type()
    }

    fn raw_re(&self) -> &'f str {
        children_of_type(self.node(), STRING).next().unwrap().text()
    }
}

impl<'f> SynRule<'f> {
    pub fn is_public(&self) -> bool {
        let file = ast_parent_exn::<File>(self.node());
        file.nodes_def().unwrap().nodes().contains(&self.name())
    }
}

impl<'f> VerbatimDef<'f> {
    pub fn contents(&self) -> &'f str {
        let literal_text = child_of_type_exn(self.node(), HASH_STRING).text();
        lit_body(literal_text).trim()
    }
}

impl<'f> MethodDef<'f> {
    pub fn selector_kind(&self) -> SelectorKind<'f> {
        let selector = child_of_type_exn(self.node(), AST_SELECTOR);
        let name = child_of_type_exn(selector, IDENT).text();
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
}

pub enum SelectorKind<'f> {
    Single(&'f str),
    Opt(&'f str),
    Many(&'f str),
    Text(&'f str),
}

#[derive(Clone, Copy)]
pub enum Expr<'f> {
    BlockExpr(BlockExpr<'f>),
    SeqExpr(SeqExpr<'f>),
    RefExpr(RefExpr<'f>),
    CallExpr(CallExpr<'f>),
}

impl<'f> AstClass<'f> for Expr<'f> {
    fn tys() -> &'static [NodeType] {
        const TYS: &[NodeType] = &[BLOCK_EXPR, SEQ_EXPR, REF_EXPR, CALL_EXPR];
        TYS
    }

    fn new(node: Node<'f>) -> Self {
        match node.ty() {
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr::new(node)),
            SEQ_EXPR => Expr::SeqExpr(SeqExpr::new(node)),
            REF_EXPR => Expr::RefExpr(RefExpr::new(node)),
            CALL_EXPR => Expr::CallExpr(CallExpr::new(node)),
            _ => unreachable!()
        }
    }

    fn node(&self) -> Node<'f> {
        match *self {
            Expr::BlockExpr(e) => e.node(),
            Expr::SeqExpr(e) => e.node(),
            Expr::RefExpr(e) => e.node(),
            Expr::CallExpr(e) => e.node(),
        }
    }
}

pub enum RefKind<'f> {
    Token(&'f str),
    RuleReference { idx: usize },
}

impl<'f> RefExpr<'f> {
    pub fn resolve(&self) -> Option<RefKind<'f>> {
        let file = ast_parent_exn::<File>(self.node());

        if let Some(ident) = child_of_type(self.node(), IDENT) {
            if let Some(idx) = file.resolve_rule(ident.text()) {
                return Some(RefKind::RuleReference { idx: idx })
            }
        }
        let token_name = child_of_type(self.node(), IDENT)
            .unwrap_or_else(|| child_of_type_exn(self.node(), SIMPLE_STRING))
            .text();

        match file.tokenizer_def().and_then(|td| td.lex_rules().find(|r| r.token_name() == token_name)) {
            Some(rule) => Some(RefKind::Token(rule.node_type())),
            None => None,
        }
    }
}

impl<'f> SeqExpr<'f> {
    pub fn parts(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node().children())
    }
}

impl<'f> CallExpr<'f> {
    pub fn args(&self) -> AstClassChildren<'f, Expr<'f>> {
        AstClassChildren::new(self.node().children())
    }
}


fn lit_body(lit: &str) -> &str {
    let q = if lit.starts_with('\'') { '\'' } else { '"' };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    &lit[s + 1..e]
}

fn has(node: Node, ty: NodeType) -> bool {
    child_of_type(node, ty).is_some()
}
