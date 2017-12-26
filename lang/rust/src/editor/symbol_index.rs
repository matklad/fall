use std::path::PathBuf;

use fst;
use fst::IntoStreamer;
use file;

use fall_tree::{TextRange, NodeType};
use indxr::{FileIndex, IndexableFileSet};

use editor::line_index::{LineCol, LineIndex};
use editor::fst_subseq::FstSubSeq;
use editor::file_symbols::process_symbols;

use syntax::{STRUCT_DEF, ENUM_DEF, TRAIT_DEF, TYPE_DEF};


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
        let mut query = Query::new(query);
        let mut result = Vec::new();
        self.process_query(&query, &mut result);
        if result.is_empty() && !query.all_symbols {
            query.all_symbols = true;
            self.process_query(&query, &mut result);
        }
        result
    }

    fn process_query(&self, query: &Query, acc: &mut Vec<(PathBuf, Symbol)>) {
        self.index.process_files(&mut |file| {
            query.process(&file.value, &mut |symbol| {
                acc.push((file.path.clone(), symbol))
            });
            acc.len() > 512
        });
    }
}

struct Query {
    query: String,
    all_symbols: bool,
}

impl Query {
    fn new(query: &str) -> Query {
        let all_symbols = query.contains("#");
        let query: String = query.chars()
            .filter(|&c| c != '#')
            .flat_map(char::to_lowercase)
            .collect();
        Query { query, all_symbols }
    }

    fn process(&self, file: &FileSymbols, acc: &mut FnMut(Symbol)) {
        fn is_type(ty: NodeType) -> bool {
            match ty {
                STRUCT_DEF | ENUM_DEF | TRAIT_DEF| TYPE_DEF => true,
                _ => false,
            }
        }

        let a = FstSubSeq::new(&self.query);
        for idx in file.map.search(a).into_stream().into_values() {
            let idx = idx as usize;
            let symbol = file.symbols[idx].clone();
            if self.all_symbols || is_type(symbol.ty) {
                acc(symbol)
            }
        }
    }
}

struct FileSymbols {
    symbols: Vec<Symbol>,
    map: fst::Map,
}

impl FileSymbols {
    fn new(text: String) -> FileSymbols {
        let file = ::syntax::lang_rust().parse(text);
        let line_index = LineIndex::new(file.text());
        let mut symbols = Vec::new();
        process_symbols(&file, &mut |name, node| {
            let range = node.range();
            symbols.push(Symbol {
                ty: node.ty(),
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
    pub ty: NodeType,
    pub name: String,
    pub range: TextRange,
    pub lc_range: [LineCol; 2]
}