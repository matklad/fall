use fall_tree::{AstNode, Node, NodeType};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use ::lang::{STRING, IDENT, SIMPLE_STRING, HASH_STRING, AST_SELECTOR, QUESTION, DOT, STAR,
             LexRule, SynRule, NodesDef, File, VerbatimDef, MethodDef,
             RefExpr, AstClassDef, AstDef};

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

    fn selector_name(&self) -> &'f str {
        let selector = child_of_type_exn(self.node(), AST_SELECTOR);
        child_of_type_exn(selector, IDENT).text()
    }
}

impl<'f> AstClassDef<'f> {
    pub fn name(&self) -> &'f str {
        child_of_type(self.node(), IDENT).unwrap().text()
    }

    pub fn variants<'a>(&'a self) -> Box<Iterator<Item=&'f str> + 'a> {
        Box::new(children_of_type(self.node(), IDENT).skip(1).map(|it| it.text()))
    }
}

pub enum SelectorKind<'f> {
    Single(&'f str),
    Opt(&'f str),
    Many(&'f str),
    Text(&'f str),
}

pub enum RefKind {
    Token(usize),
    RuleReference { idx: usize },
}

impl<'f> RefExpr<'f> {
    pub fn resolve(&self) -> Option<RefKind> {
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
            Some(rule) => {
                let ty_name = rule.node_type();
                let idx = file.nodes_def().unwrap().nodes().iter().position(|&it| it == ty_name).unwrap();
                Some(RefKind::Token(idx + 2))
            },
            None => None,
        }
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
