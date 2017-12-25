use std::path::PathBuf;

use fst;
use fst::IntoStreamer;
use file;

use fall_tree::TextRange;
use indxr::{FileIndex, IndexableFileSet};

use editor::line_index::{LineCol, LineIndex};
use editor::file_symbols::process_symbols;

pub struct SymbolIndex {
    index: FileIndex<FileSymbols>,
}

impl SymbolIndex {
    pub fn new(roots: Vec<PathBuf>) -> SymbolIndex {
        let file_set = IndexableFileSet::new(roots, "rs");
        let index = FileIndex::new(file_set, Box::new(|path| {
            let text = file::get_text(path).ok()?;
            Some(FileSymbols::new(text))
        }));
        SymbolIndex { index }
    }

    pub fn query(&self, query: &str) -> Vec<(PathBuf, Symbol)> {
        #[derive(Clone, Copy)]
        struct A<'a> { query: &'a String }
        impl<'a> fst::Automaton for A<'a> {
            type State = usize;
            fn start(&self) -> usize {
                0
            }
            fn is_match(&self, &state: &usize) -> bool {
                state == self.query.len()
            }
            fn accept(&self, &state: &usize, byte: u8) -> usize {
                if state >= self.query.len() {
                    return state;
                }
                if byte == self.query.as_bytes()[state] {
                    return state + 1;
                }
                return state;
            }
            fn can_match(&self, _: &usize) -> bool {
                true
            }
            fn will_always_match(&self, &state: &usize) -> bool {
                state == self.query.len()
            }
        }

        let query = query.to_lowercase();
        let a = A { query: &query };

        let mut result = Vec::new();
        self.index.process_files(&mut |file| {
            let file_symbols = &file.value;
            for idx in file_symbols.map.search(a).into_stream().into_values() {
                let idx = idx as usize;
                result.push((file.path.clone(), file_symbols.symbols[idx].clone()))
            }
            result.len() > 512
        });
        result
    }
}

struct FileSymbols {
    symbols: Vec<Symbol>,
    map: fst::Map,
}

impl FileSymbols {
    fn new(text: String) -> FileSymbols {
        let file = ::lang_rust().parse(text);
        let line_index = LineIndex::new(file.text());
        let mut symbols = Vec::new();
        process_symbols(&file, &mut |name, node| {
            let range = node.range();
            symbols.push(Symbol {
                name: name.to_string(),
                range,
                lc_range: [line_index.translate(range.start()), line_index.translate(range.end())],
            })
        });
        symbols.sort_by_key(|s| s.name.to_lowercase());
        symbols.dedup_by_key(|s| s.name.to_lowercase());

        let map = fst::Map::from_iter(
            symbols.iter().map(|s| (s.name.to_lowercase())).zip(0u64..)
        ).unwrap();

        FileSymbols { symbols, map }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Symbol {
    pub name: String,
    pub range: TextRange,
    pub lc_range: [LineCol; 2]
}