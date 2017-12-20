extern crate walkdir;
extern crate notify;

use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use walkdir::WalkDir;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

type Mapper<V: Send + 'static> = Box<Fn(&Path) -> Option<V> + Sync + Send>;

pub struct IndexableFileSet {
    roots: Vec<PathBuf>,
    filter: Box<Fn(&Path) -> bool + Sync + Send>,
}

impl IndexableFileSet {
    pub fn new(roots: Vec<PathBuf>, ext: &'static str) -> IndexableFileSet {
        IndexableFileSet {
            roots,
            filter: Box::new(move |path| path.extension().unwrap_or_default() == ext)
        }
    }
}

pub struct FileIndex<V: Send + 'static> {
    imp: Arc<FileIndexImpl<V>>,
}

impl<V: Send + 'static> FileIndex<V> {
    pub fn new(file_set: IndexableFileSet, mapper: Mapper<V>) -> Self {
        let imp = FileIndexImpl { file_set, data: Default::default(), mapper };
        let imp = Arc::new(imp);
        let imp2 = imp.clone();
        ::std::thread::spawn(move || watch(&imp2));
        FileIndex { imp }
    }

    pub fn process_files(&self, sink: &mut FnMut(&IndexedFile<V>) -> bool) {
        let data = self.imp.data.lock().unwrap();
        for file in data.values() {
            if sink(file) {
                return
            }
        }
    }
}

pub struct IndexedFile<V> {
    pub path: PathBuf,
    pub value: V,
}

pub struct FileIndexImpl<V> {
    file_set: IndexableFileSet,
    data: Mutex<HashMap<PathBuf, IndexedFile<V>>>,
    mapper: Mapper<V>,
}

fn watch<V>(index: &FileIndexImpl<V>) {
    let initial_indexing_start = ::std::time::Instant::now();
    for path in index.file_set.roots.iter() {
        index.change(path)
    }
    let elapsed = initial_indexing_start.elapsed();
    eprintln!("indexing took = {}s", elapsed.as_secs());
    let (tx, rx) = channel();

    for path in index.file_set.roots.iter() {
        let tx = tx.clone();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
    }
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::NoticeWrite(path)
                | DebouncedEvent::NoticeRemove(path)
                | DebouncedEvent::Create(path)
                | DebouncedEvent::Write(path)
                | DebouncedEvent::Remove(path) => {
                    index.change(&path)
                }
                DebouncedEvent::Rename(p1, p2) => {
                    index.change(&p1);
                    index.change(&p2);
                }
                _ => continue,
            },
            Err(e) => {
                eprintln!("watch error: {:?}", e);
                continue
            }
        }
    }
}

impl<V> FileIndexImpl<V> {
    fn change(&self, path: &Path) {
        let added = path.exists();
        match (path.is_dir(), added) {
            (false, true) => {
                if !(self.file_set.filter)(path) {
                    return
                }
                let value = (self.mapper)(path);
                let mut data = self.data.lock().unwrap();
                match value {
                    None => {
                        data.remove(path);
                    },
                    Some(value) => {
                        let file = IndexedFile { path: path.to_owned(), value};
                        data.insert(path.to_owned(), file);
                    }
                };
            }
            (false, false) => {
                self.data.lock().unwrap().remove(path);
            }
            (true, false) => {
                self.data.lock().unwrap().retain(|k, _| {
                    !k.starts_with(path)
                });
            }
            (true, true) => {
                let files = WalkDir::new(path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().is_file() && (self.file_set.filter)(entry.path()));

                let mut local = HashMap::new();
                for entry in files {
                    let path = entry.path().to_owned();
                    if let Some(value) = (self.mapper)(&path) {
                        let file = IndexedFile { path: path.clone(), value };
                        local.insert(path.clone(), file);
                    }
                }

                self.data.lock().unwrap().extend(local);
            }
        };
    }
}