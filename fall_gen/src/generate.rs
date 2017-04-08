use fall_tree::AstNode;
use ast_ext::{PartKind, SelectorKind};
use util::{Buff, scream, snake};

use syntax::{File, Alt, Part, AstDef};


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
                let f = match rule.extern_fn() {
                    None => "None".to_owned(),
                    Some(ref f) => format!("Some({})", f)
                };
                ln!(buff, "Rule {{ ty: {}, re: {:?}, f: {} }},",
                    scream(&rule.node_type()), rule.token_re(), f);
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
            for l in v.contents().lines() {
                buff.line(l)
            }
        }

        if let Some(ast_def) = self.ast_def() {
            generate_ast(ast_def, &mut buff);
        }

        buff.done()
    }

    fn generate_syn_rules(&self, buff: &mut Buff) {
        buff.line("const PARSER: &'static [syn::Rule] = &[");
        buff.indent();
        for rule in self.syn_rules() {
            let ty = if rule.is_public() {
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
        match part.kind() {
            PartKind::Token(t) => format!("syn::Part::Token({})", scream(t)),
            PartKind::RuleReference { idx } => format!("syn::Part::Rule({:?})", idx),
            PartKind::Call { name, mut alts } => {
                let op = match name {
                    "rep" => "Rep",
                    "opt" => "Opt",
                    _ => unimplemented!(),
                };
                format!("syn::Part::{}({})", op, self.generate_alt(alts.next().unwrap()))
            }
        }
    }
}

fn generate_ast(ast: AstDef, buff: &mut Buff) {
    buff.blank_line();
    buff.line("use fall_tree::{AstNode, AstChildren, Node};");
    buff.line("use fall_tree::search::child_of_type_exn;");
    buff.blank_line();
    for node in ast.ast_nodes() {
        let sn = snake(node.name());
        buff.line("#[derive(Clone, Copy)]");
        ln!(buff, "pub struct {}<'f> {{ node: Node<'f> }}", sn);
        ln!(buff, "impl<'f> AstNode<'f> for {}<'f> {{", sn);
        buff.indent();
        ln!(buff, "fn ty() -> NodeType {{ {} }}", scream(node.name()));
        buff.line("fn new(node: Node<'f>) -> Self {");
        buff.indent();
        buff.line("assert_eq!(node.ty(), Self::ty());");
        ln!(buff, "{} {{ node: node }}", sn);
        buff.dedent();
        buff.line("}");
        buff.line("fn node(&self) -> Node<'f> { self.node }");
        buff.dedent();
        buff.line("}");
        buff.blank_line();

        if node.methods().count() > 0 {
            ln!(buff, "impl<'f> {}<'f> {{", sn);
            buff.indent();
            for method in node.methods() {
                let ret_type = match method.selector_kind() {
                    SelectorKind::Single(name) => format!("{}<'f>", snake(name)),
                    SelectorKind::Opt(name) => format!("Option<{}<'f>>", snake(name)),
                    SelectorKind::Many(name) => format!("AstChildren<'f, {}<'f>>", snake(name)),
                    SelectorKind::Text(_) => "&'f str".to_owned(),
                };

                ln!(buff, "pub fn {}(&self) -> {} {{", method.name(), ret_type);
                buff.indent();
                match method.selector_kind() {
                    SelectorKind::Single(_) => {
                        ln!(buff, "AstChildren::new(self.node.children()).next().unwrap()")
                    }
                    SelectorKind::Opt(_) => {
                        ln!(buff, "AstChildren::new(self.node.children()).next()")
                    }
                    SelectorKind::Many(_) => {
                        ln!(buff, "AstChildren::new(self.node.children())")
                    }
                    SelectorKind::Text(name) => {
                        ln!(buff, "child_of_type_exn(self.node, {}).text()", name)
                    }
                }
                buff.dedent();
                buff.line("}");
            }
            buff.dedent();
            buff.line("}");
        }
        buff.blank_line();
    }
}
