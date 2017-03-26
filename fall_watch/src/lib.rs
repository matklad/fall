extern crate notify;
extern crate file;

use std::path::Path;
use std::fs::File;

use notify::{Watcher, RecursiveMode, raw_watcher};
use std::sync::mpsc::channel;

pub fn watch<T: AsRef<Path>>(path: T, transform: &Fn(&str) -> String) {
    let path = ::std::env::current_dir().unwrap().join(path);
    println!("watching {}", path.display());
    if !path.exists() {
        println!("creating {}", path.display());
        File::create(&path).unwrap();
    }


    let dir = path.parent().unwrap();

    let (tx, rx) = channel();

    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(dir, RecursiveMode::NonRecursive).unwrap();

    update(&path, transform);
    loop {
        match rx.recv() {
            Ok(event) => {
                if let Some(ref p) = event.path {
                    if p == &path {
                        update(&path, transform);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn update(path: &Path, transform: &Fn(&str) -> String) {
    println!("updating {}", path.display());
    let text = match file::get_text(path) {
        Ok(text) => text,
        Err(_) => {
            println!("fail on read");
            return
        },
    };
    if let Err(_) = file::put_text(path.with_extension("out"), &transform(&text)) {
        println!("fail on write");
    }
    println!("ok\n");
}