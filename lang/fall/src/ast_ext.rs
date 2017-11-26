use fall_tree::{Text, TextRange, AstNode};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type};

use ::{STRING, IDENT, SIMPLE_STRING, PUB,
       LexRule, SynRule, VerbatimDef,
       RefExpr, AstClassDef, Attributes, Attribute, TestDef,
       CallExpr};

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

    fn re(&self) -> Option<Text<'f>> {
        child_of_type(self.node(), STRING).map(|n| n.text())
    }
}

impl<'f> SynRule<'f> {
    pub fn is_pub(&self) -> bool {
        child_of_type(self.node(), PUB).is_some()
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

    pub fn type_attr(&self) -> Option<Attribute<'f>> {
        self.attributes().and_then(|attrs| attrs.find("type"))
    }

    pub fn ty_name(&self) -> Option<Text<'f>> {
        if !self.is_pub() || self.is_pratt() {
            return None;
        }

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

impl<'f> AstClassDef<'f> {
    pub fn name(&self) -> Text<'f> {
        child_of_type(self.node(), IDENT).unwrap().text()
    }

    pub fn variants<'a>(&'a self) -> Box<Iterator<Item=Text<'f>> + 'a> {
        Box::new(children_of_type(self.node(), IDENT).skip(1).map(|it| it.text()))
    }
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

pub(crate) fn lit_body(lit: Text) -> Text {
    let q = if lit.starts_with("'") { "'" } else { "\"" };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    lit.slice(TextRange::from_to(s + 1, e))
}
