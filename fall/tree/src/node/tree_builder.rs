use crate::{NodeType, TextUnit, TextRange, tu};

pub struct TreeBuilder {
    nodes: Vec<NodeData>,
    in_progress: Vec<NodeId>,
    pos: TextUnit,
}

impl TreeBuilder {
    pub fn leaf(&mut self, ty: NodeType, len: TextUnit) {
        let leaf = NodeData {
            ty,
            parent: None,
            children: Vec::new(),
            range: TextRange::from_len(self.pos, len),
        };
        self.pos += len;
        let id = self.push_child(leaf);
        self.add_len(id);
    }

    pub fn start_internal(&mut self, ty: NodeType) {
        let node = NodeData {
            ty,
            parent: None,
            children: Vec::new(),
            range: TextRange::from_len(self.pos, tu(0)),
        };
        let id = if self.in_progress.is_empty() {
            self.new_node(node)
        } else {
            self.push_child(node)
        };
        self.in_progress.push(id)
    }

    pub fn finish_internal(&mut self) {
        let id = self.in_progress.pop().unwrap();
        if !self.in_progress.is_empty() {
            self.add_len(id);
        }
    }

    pub(crate) fn new() -> TreeBuilder {
        TreeBuilder {
            nodes: Vec::new(),
            in_progress: Vec::new(),
            pos: tu(0),
        }
    }

    pub(crate) fn finish(self) -> Vec<NodeData> {
        assert!(self.in_progress.is_empty());
        self.nodes
    }

    fn new_node(&mut self, data: NodeData) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(data);
        id
    }

    fn push_child(&mut self, mut child: NodeData) -> NodeId {
        child.parent = Some(self.current_id());
        let id = self.new_node(child);
        self.current().children.push(id);
        id
    }

    fn add_len(&mut self, child: NodeId) {
        let range = self.nodes[child.0 as usize].range;
        grow(&mut self.current().range, range);
    }

    fn current_id(&self) -> NodeId {
        *self.in_progress.last().unwrap()
    }

    fn current(&mut self) -> &mut NodeData {
        let NodeId(idx) = self.current_id();
        &mut self.nodes[idx as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NodeId(pub u32);


pub(crate) struct NodeData {
    pub(crate) ty: NodeType,
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) range: TextRange,
}

fn grow(left: &mut TextRange, right: TextRange) {
    let start = left.start();
    assert_eq!(left.end(), right.start());
    let end = right.end();
    *left = TextRange::from_to(start, end)
}
