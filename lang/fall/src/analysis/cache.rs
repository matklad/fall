use std::sync::Mutex;
use std::collections::HashMap;

use lazycell::AtomicLazyCell;

use fall_tree::Node;


pub (super) struct FileCache<V> {
    value: AtomicLazyCell<V>
}

impl<V> FileCache<V> {
    pub fn get<F: FnOnce() -> V>(&self, f: F) -> &V {
        if !self.value.filled() {
            let v = f();
            let _ = self.value.fill(v);
        }
        self.value.borrow().unwrap()
    }
}

impl<V> Default for FileCache<V> {
    fn default() -> Self {
        FileCache { value: AtomicLazyCell::new() }
    }
}


#[derive(Default)]
pub (super) struct NodeCache<'f, V: Sync> {
    map: Mutex<HashMap<Node<'f>, V>>
}

impl<'f, V: Sync + Clone> NodeCache<'f, V> {
    pub fn get<F: FnOnce() -> V>(&self, node: Node<'f>, f: F) -> (V, bool) {
        {
            let guard = self.map.lock().unwrap();
            if let Some(v) = guard.get(&node) {
                return (v.clone(), false);
            }
        }
        let value = self.map.lock().unwrap().entry(node).or_insert_with(f).clone();
        (value, true)
    }
}
