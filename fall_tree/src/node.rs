use {TextRange, NodeType, WHITESPACE};
pub use self::core::{File, Node, NodeChildren, FileBuilder, NodeBuilder};

use std::fmt::Write;

mod core {
    use std::ops::Index;
    use {TextRange, NodeType};

    pub struct File {
        text: String,
        root: NodeId,
        nodes: Vec<RawNode>,
    }

    #[derive(Clone, Copy)]
    pub struct Node<'f> {
        id: NodeId,
        file: &'f File,
    }

    pub struct FileBuilder {
        nodes: Vec<RawNode>,
    }

    #[derive(Clone, Copy)]
    pub struct NodeBuilder(NodeId);

    impl File {
        pub fn root(&self) -> Node {
            self.node(self.root)
        }

        pub fn text(&self) -> &str {
            &self.text
        }

        fn node(&self, id: NodeId) -> Node {
            Node { id: id, file: self }
        }
    }

    impl<'f> Node<'f> {
        pub fn ty(&self) -> NodeType {
            self.raw().ty
        }

        pub fn range(&self) -> TextRange {
            self.raw().range
        }

        pub fn text(&self) -> &str {
            &self.file.text[self.range()]
        }

        pub fn parent(&self) -> Option<Node> {
            self.raw().parent.map(|id| self.file.node(id))
        }

        pub fn children<'n>(&'n self) -> NodeChildren<'n, 'f> {
            NodeChildren {
                file: self.file,
                inner: self.raw().children.iter(),
            }
        }

        fn raw(&self) -> &RawNode {
            &self.file[self.id]
        }
    }

    impl FileBuilder {
        pub fn new() -> FileBuilder {
            FileBuilder { nodes: vec![] }
        }

        pub fn node(&mut self,
                    parent: Option<NodeBuilder>,
                    ty: NodeType,
                    range: TextRange
        ) -> NodeBuilder {

            let parent = parent.map(|n| n.0);
            let id = NodeId(self.nodes.len() as u32);
            self.nodes.push(RawNode {
                ty: ty,
                parent: parent,
                children: vec![],
                range: range,
            });
            if let Some(parent) = parent {
                self.nodes[parent.0 as usize].children.push(id)
            }

            NodeBuilder(id)
        }

        pub fn build(self, text: String) -> File {
            assert!(!self.nodes.is_empty());
            File {
                text: text,
                root: NodeId(0),
                nodes: self.nodes,
            }
        }
    }

    pub struct NodeChildren<'n, 'f: 'n> {
        file: &'f File,
        inner: ::std::slice::Iter<'n, NodeId>,
    }

    impl<'n, 'f: 'n> Iterator for NodeChildren<'n, 'f> {
        type Item = Node<'f>;

        fn next(&mut self) -> Option<Node<'f>> {
            self.inner.next().map(|&id| self.file.node(id))
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct NodeId(u32);

    struct RawNode {
        ty: NodeType,
        parent: Option<NodeId>,
        children: Vec<NodeId>,
        range: TextRange,
    }

    impl Index<NodeId> for File {
        type Output = RawNode;
        fn index(&self, index: NodeId) -> &Self::Output {
            &self.nodes[index.0 as usize]
        }
    }
}

impl File {
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

impl<'f> Node<'f> {
    pub fn is_leaf(&self) -> bool {
        self.children().next().is_none()
    }
}


impl<'n, 'f: 'n> NodeChildren<'n, 'f> {
    pub fn single_of_type(mut self, ty: NodeType) -> Option<Node<'f>> {
        self.find(|n| n.ty() == ty)
    }

    pub fn many_of_type(self, ty: NodeType) -> Box<Iterator<Item=Node<'f>> + 'n> {
        Box::new(self.filter(move |n| n.ty() == ty))
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