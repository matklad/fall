use fall_tree::search::{children_of_type, child_of_type_exn};

mod syntax;

use self::syntax::*;
use util::{Buff, scream, snake};

pub fn generate(text: &str) -> String {
    let file = parse(text.to_owned());
    let root = file.root();

    let mut buff = Buff::new();
    buff.line("use fall_tree::{AstNode, AstChildren, Node, NodeType};");
    buff.line("use fall_tree::search::{child_of_type_exn, children_of_type};");
    buff.line("use syntax::*;");
    buff.blank_line();
    for node in children_of_type(root, NODE) {
        let name = child_of_type_exn(node, IDENT);
        let sn = snake(name.text());
        ln!(buff, "struct {}<'f> {{ node: Node<'f> }}", sn);
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
                let (node_type, many) = if node_type.ends_with("*") {
                    (&node_type[..node_type.len() - 1], true)
                } else {
                    (node_type, false)
                };
                let ret_type = if many {
                    format!("AstChildren<{}>", snake(node_type))
                } else {
                    snake(node_type)
                };
                ln!(buff, "fn {}(&self) -> {} {{", name, ret_type);
                buff.indent();
                if many {
                    ln!(buff, "AstChildren::new(self.node.children())")
                } else {
                    ln!(buff, "AstChildren::new(self.node.children()).next().unwrap()")
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
