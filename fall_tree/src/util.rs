use {File, Node, WHITESPACE};
use std::fmt::Write;

pub fn dump_file(f: &File) -> String {
    dump(f.root(), f.text(), false)
}

pub fn dump_file_ws(f: &File) -> String {
    dump(f.root(), f.text(), true)
}


fn dump(root: Node, text: &str, include_whitespace: bool) -> String {
    let mut buf = String::new();
    go(0, root, text, &mut buf, include_whitespace);
    return buf;

    fn go(level: usize, n: Node, text: &str, buf: &mut String, include_whitespace: bool) {
        if n.ty() == WHITESPACE && !include_whitespace {
            return
        }

        for _ in 0..level {
            buf.push_str("  ")
        }

        if n.children().next().is_none() {
            write!(buf, "{} {:?}\n", n.ty().name(), &text[n.range()])
                .unwrap();
        } else {
            write!(buf, "{}\n", n.ty().name())
                .unwrap();
            for child in n.children() {
                go(level + 1, child, text, buf, include_whitespace);
            }
        }
    }
}
