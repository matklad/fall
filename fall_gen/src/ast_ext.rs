use fall_tree::{AstNode, AstChildren, Node, NodeType};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type, ast_parent_exn};

use syntax::{STRING, IDENT, SIMPLE_STRING, HASH_STRING, LANGLE, AST_SELECTOR, QUESTION, DOT, STAR,
             LexRule, SynRule, NodesDef, File, Alt, Part, VerbatimDef, MethodDef};

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
        file.nodes_def().nodes().contains(&self.name())
    }
}

pub enum PartKind<'f> {
    Token(&'f str),
    RuleReference { idx: usize },
    Call { name: &'f str, alts: AstChildren<'f, Alt<'f>> }
}

impl<'f> Part<'f> {
    pub fn kind(&self) -> Option<PartKind<'f>> {
        if child_of_type(self.node(), LANGLE).is_some() {
            return Some(PartKind::Call {
                name: child_of_type_exn(self.node(), IDENT).text(),
                alts: AstChildren::new(self.node().children()),
            })
        }
        let file = ast_parent_exn::<File>(self.node());

        if let Some(ident) = child_of_type(self.node(), IDENT) {
            if let Some(idx) = file.syn_rules().position(|r| r.name() == ident.text()) {
                return Some(PartKind::RuleReference { idx: idx })
            }
        }
        let token_name = child_of_type(self.node(), IDENT)
            .unwrap_or_else(|| child_of_type_exn(self.node(), SIMPLE_STRING))
            .text();

        match file.tokenizer_def().lex_rules().find(|r| r.token_name() == token_name) {
            Some(rule) => Some(PartKind::Token(rule.node_type())),
            None => None,
        }
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


fn lit_body(lit: &str) -> &str {
    let q = if lit.starts_with('\'') { '\'' } else { '"' };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    &lit[s + 1..e]
}

fn has(node: Node, ty: NodeType) -> bool {
    child_of_type(node, ty).is_some()
}
