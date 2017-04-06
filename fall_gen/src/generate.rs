use fall_tree::{AstNode, AstChildren};
use fall_tree::search::{children_of_type, child_of_type_exn, child_of_type};
use ast::*;
use syntax::*;
use util::{Buff, scream};

impl<'f> LexRule<'f> {
    fn re(&self) -> String {
        let raw = self.raw_re();
        if raw.starts_with('r') {
            lit_body(raw).to_owned()
        } else {
            ::regex::escape(lit_body(raw))
        }
    }

    fn f(&self) -> Option<&'f str> {
        children_of_type(self.node(), STRING).nth(1).map(|n| {
            lit_body(n.text())
        })
    }

    fn name(&self) -> &'f str {
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

impl<'f> NodesDef<'f> {
    pub fn nodes(&self) -> Vec<&'f str> {
        children_of_type(self.node(), IDENT)
            .map(|n| n.text())
            .collect()
    }
}

impl<'f> Part<'f> {
    pub fn name(&self) -> Option<&'f str> {
        if child_of_type(self.node(), LANGLE).is_some() {
            return None
        }

        if let Some(s) = child_of_type(self.node(), SIMPLE_STRING) {
            return Some(s.text())
        }

        Some(child_of_type_exn(self.node(), IDENT).text())
    }

    pub fn op(&self) -> Option<(&'f str, AstChildren<'f, Alt<'f>>)> {
        if child_of_type(self.node(), LANGLE).is_none() {
            return None
        }

        Some((
            child_of_type_exn(self.node(), IDENT).text(),
            AstChildren::new(self.node().children())
        ))
    }
}

impl<'f> File<'f> {
    pub fn generate(&self) -> String {
        let mut buff = Buff::new();
        buff.line("use std::sync::{Once, ONCE_INIT};");
        buff.line("use fall_tree::{NodeType, NodeTypeInfo};");
        buff.line("use fall_parse::Rule;");
        buff.line("use fall_parse::syn;");
        buff.line("pub use fall_tree::{ERROR, WHITESPACE};");
        buff.blank_line();

        for (i, t) in self.nodes_def().nodes().iter().enumerate() {
            ln!(buff, "pub const {:10}: NodeType = NodeType({});", scream(t), 100 + i);
        }

        buff.blank_line();

        buff.line("fn register_node_types() {");
        {
            buff.indent();
            buff.line("static REGISTER: Once = ONCE_INIT;");
            buff.line("REGISTER.call_once(||{");
            buff.indent();
            for t in self.nodes_def().nodes().iter() {
                ln!(buff, "{}.register(NodeTypeInfo {{ name: {:?} }});", scream(t), scream(t));
            }
            buff.dedent();
            buff.line("});");
            buff.dedent();
        }
        buff.line("}");
        buff.blank_line();

        buff.line("const TOKENIZER: &'static [Rule] = &[");
        {
            buff.indent();
            for rule in self.tokenizer_def().lex_rules() {
                let f = match rule.f() {
                    None => "None".to_owned(),
                    Some(ref f) => format!("Some({})", f)
                };
                ln!(buff, "Rule {{ ty: {}, re: {:?}, f: {} }},", scream(&rule.node_type()), rule.re(), f);
            }
            buff.dedent();
        }
        buff.line("];");


        buff.blank_line();
        self.generate_syn_rules(&mut buff);
        buff.blank_line();
        buff.line("pub fn parse(text: String) -> ::fall_tree::File {");
        buff.indent();
        buff.line("register_node_types();");
        buff.line("::fall_parse::parse(text, FILE, TOKENIZER, &|b| syn::Parser::new(PARSER).parse(b))");
        buff.dedent();
        buff.line("}");

        if let Some(v) = self.verbatim_def() {
            buff.blank_line();
            for l in lit_body(v.verbatim()).trim().lines() {
                buff.line(l)
            }
        }

        buff.done()
    }

    fn generate_syn_rules(&self, buff: &mut Buff) {
        buff.line("const PARSER: &'static [syn::Rule] = &[");
        buff.indent();
        for rule in self.syn_rules() {
            let ty = if self.nodes_def().nodes().contains(&rule.name()) {
                format!("Some({})", scream(&rule.name()))
            } else {
                "None".to_owned()
            };
            let alts = rule.alts().map(|a| self.generate_alt(a)).collect::<Vec<_>>();
            ln!(buff, r#"syn::Rule {{ ty: {}, alts: &[{}] }},"#, ty, alts.join(", "));
        }
        buff.dedent();
        buff.line("];");
    }

    fn generate_alt(&self, alt: Alt) -> String {
        fn is_commit(part: Part) -> bool {
            part.node().text() == "<commit>"
        }
        let commit = alt.parts().position(is_commit);

        let parts = alt.parts()
            .filter(|&p| !is_commit(p))
            .map(|p| self.generate_part(p))
            .collect::<Vec<_>>();
        format!("syn::Alt {{ parts: &[{}], commit: {:?} }}", parts.join(", "), commit)
    }

    fn generate_part(&self, part: Part) -> String {
        if let Some(name) = part.name() {
            return if let Some(r) = self.find_lex_rule(name) {
                format!("syn::Part::Token({})", scream(&r.node_type()))
            } else {
                let id = self.syn_rules().position(|r| r.name() == name)
                    .expect(&format!("Not a rule or token: `{}`", name));
                format!("syn::Part::Rule({:?})", id)
            }
        }
        let (op, mut alts) = part.op().unwrap();
        let o = match op {
            "rep" => "Rep",
            "opt" => "Opt",
            _ => unimplemented!(),
        };
        format!("syn::Part::{}({})", o, self.generate_alt(alts.next().unwrap()))
    }

    fn find_lex_rule(&self, name: &str) -> Option<LexRule<'f>> {
        self.tokenizer_def().lex_rules().find(|r| r.name() == name)
    }
}

fn lit_body(lit: &str) -> &str {
    let q = if lit.starts_with('\'') { '\'' } else { '"' };
    let s = lit.find(q).unwrap();
    let e = lit.rfind(q).unwrap();
    &lit[s + 1..e]
}
