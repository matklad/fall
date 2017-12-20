use std::path::{PathBuf, Path};

struct IndexedFiles {
    roots: Vec<PathBuf>,
    filter: Box<Fn(&Path) -> bool>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
