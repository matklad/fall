use fall_tree::search::{children_of_type, child_of_type_exn};

mod syntax;

use self::syntax::*;
use util::{Buff, scream, snake};

pub fn generate(text: &str) -> String {
    let file = parse(text.to_owned());
    let root = file.root();

    let mut buff = Buff::new();
    buff.line("use fall_tree::{AstNode, AstChildren, Node, NodeType};");
    buff.line("use fall_tree::search::child_of_type_exn;");
    buff.line("use syntax::*;");
    buff.blank_line();
    for node in children_of_type(root, NODE) {
        let name = child_of_type_exn(node, IDENT);
        let sn = snake(name.text());
        buff.line("#[derive(Clone, Copy)]");
        ln!(buff, "pub struct {}<'f> {{ node: Node<'f> }}", sn);
        ln!(buff, "impl<'f> AstNode<'f> for {}<'f> {{", sn);
        buff.indent();
        ln!(buff, "fn ty() -> NodeType {{ {} }}", scream(name.text()));
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

        let has_methods = children_of_type(node, METHOD).count() > 0;
        if has_methods {
            let methods = children_of_type(node, METHOD);
            ln!(buff, "impl<'f> {}<'f> {{", sn);
            buff.indent();
            for method in methods {
                let mut ids = children_of_type(method, IDENT);
                let name = ids.next().unwrap().text();
                let node_type = ids.next().unwrap().text();
                let (suffix, kind) = if node_type.ends_with("*") {
                    ("*", MethodKind::Many)
                } else if node_type.ends_with("?") {
                    ("?", MethodKind::Opt)
                } else if node_type.ends_with(".text") {
                    (".text", MethodKind::Text)
                } else {
                    ("", MethodKind::Single)
                };
                let node_type = &node_type[..node_type.len() - suffix.len()];
                let ret_type = match kind {
                    MethodKind::Single => format!("{}<'f>", snake(node_type)),
                    MethodKind::Opt => format!("Option<{}<'f>>", snake(node_type)),
                    MethodKind::Many => format!("AstChildren<'f, {}<'f>>", snake(node_type)),
                    MethodKind::Text => "&'f str".to_owned(),
                };

                ln!(buff, "pub fn {}(&self) -> {} {{", name, ret_type);
                buff.indent();
                match kind {
                    MethodKind::Single => {
                        ln!(buff, "AstChildren::new(self.node.children()).next().unwrap()")
                    }
                    MethodKind::Opt => {
                        ln!(buff, "AstChildren::new(self.node.children()).next()")
                    }
                    MethodKind::Many => {
                        ln!(buff, "AstChildren::new(self.node.children())")
                    }
                    MethodKind::Text => {
                        ln!(buff, "child_of_type_exn(self.node, {}).text()", node_type)
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
    buff.done()
}

enum MethodKind {
    Single,
    Opt,
    Many,
    Text,
}
