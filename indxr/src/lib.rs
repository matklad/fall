use std::path::PathBuf;

struct IndexedFiles {
    roots: Vec<PathBuf>,
    filter: Box<Fn(&Path)>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
