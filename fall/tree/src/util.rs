use crate::{File, Node, Language};
use std::fmt::Write;

pub fn dump_file(f: &File) -> String {
    dump(f.language(), f.root(), &f.text().to_cow(), false)
}

pub fn dump_file_ws(f: &File) -> String {
    dump(f.language(), f.root(), &f.text().to_cow(), true)
}

pub fn walk_tree<F: FnMut(Node)>(node: Node, mut f: F) {
    go(node, &mut f);

    fn go<F: FnMut(Node)>(node: Node, f: &mut F) {
        f(node);
        for child in node.children() {
            go(child, f)
        }
    }
}


fn dump(lang: &Language, root: Node, text: &str, include_whitespace: bool) -> String {
    let mut buf = String::new();
    go(lang, 0, root, text, &mut buf, include_whitespace);
    return buf;

    fn go(lang: &Language, level: usize, n: Node, text: &str, buf: &mut String, include_whitespace: bool) {
        if lang.node_type_info(n.ty()).whitespace_like && !include_whitespace {
            return
        }

        for _ in 0..level {
            buf.push_str("  ")
        }

        let ty_name = lang.node_type_info(n.ty()).name;
        if n.children().next().is_none() {
            write!(buf, "{} {:?}\n", ty_name, &text[n.range()])
                .unwrap();
        } else {
            write!(buf, "{}\n", ty_name)
                .unwrap();
            for child in n.children() {
                go(lang, level + 1, child, text, buf, include_whitespace);
            }
        }
    }
}
