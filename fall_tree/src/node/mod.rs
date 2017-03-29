use {TextRange, NodeType, WHITESPACE};

use std::fmt::Write;

mod imp;

use self::imp::NodeChildren;

pub struct File(imp::FileImpl);

impl File {
    pub fn root(&self) -> Node {
        self.0.root()
    }

    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn node_containing_range(&self, range: TextRange) -> Node {
        node_containing_range(self.root(), range)
    }

    pub fn dump_ws(&self) -> String {
        dump(self.root(), &self.text(), true)
    }

    pub fn dump(&self) -> String {
        dump(self.root(), &self.text(), false)
    }
}

#[derive(Clone, Copy)]
pub struct Node<'f>(imp::NodeImpl<'f>);

impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.0.ty()
    }

    pub fn range(&self) -> TextRange {
        self.0.range()
    }

    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn parent(&self) -> Option<Node> {
        self.0.parent()
    }

    pub fn children(&self) -> NodeChildren<'f> {
        self.0.children()
    }

    pub fn is_leaf(&self) -> bool {
        self.children().next().is_none()
    }
}

impl<'f> NodeChildren<'f> {
    pub fn single_of_type(mut self, ty: NodeType) -> Option<Node<'f>> {
        self.find(|n| n.ty() == ty)
    }

    pub fn many_of_type(self, ty: NodeType) -> Box<Iterator<Item=Node<'f>> + 'f> {
        Box::new(self.filter(move |n| n.ty() == ty))
    }
}


pub struct FileBuilder(imp::FileBuilderImpl);

#[derive(Clone, Copy)]
pub struct NodeBuilder(imp::NodeId);

impl FileBuilder {
    pub fn new(text: String) -> FileBuilder {
        FileBuilder(imp::FileBuilderImpl::new(text))
    }

    pub fn node(&mut self, parent: Option<NodeBuilder>, ty: NodeType, range: TextRange)
                -> NodeBuilder {
        self.0.node(parent, ty, range)
    }

    pub fn build(self) -> File {
        self.0.build()
    }
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

        if n.is_leaf() {
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

fn node_containing_range(node: ::Node, range: TextRange) -> ::Node {
    fn go<'f>(node: ::Node<'f>, range: TextRange) -> Option<::Node<'f>> {
        if !range.is_subrange_of(node.range()) {
            return None
        }

        for child in node.children() {
            if child.range() == node.range() {
                break;
            }
            if let Some(n) = go(child, range) {
                return Some(n);
            }
        }
        return Some(node)
    }

    assert!(range.is_subrange_of(node.range()));
    go(node, range).unwrap()
}