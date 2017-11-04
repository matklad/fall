use lazycell::AtomicLazyCell;



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
