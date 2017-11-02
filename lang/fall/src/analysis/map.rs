use std::sync::Mutex;
use std::collections::HashMap;

use fall_tree::Node;

pub (super) struct NodeMap<'f, V: Sync> {
    map: Mutex<HashMap<Node<'f>, V>>
}

impl<'f, V: Sync + Clone> NodeMap<'f, V> {
    pub fn new() -> Self {
        NodeMap { map: Default::default() }
    }

    pub fn get<F: FnOnce() -> V>(&self, node: Node<'f>, f: F) -> V {
        {
            let guard = self.map.lock().unwrap();
            if let Some(v) = guard.get(&node) {
                return v.clone();
            }
        }
        let v = f();
        self.map.lock().unwrap().entry(node).or_insert(v).clone()
    }
}
