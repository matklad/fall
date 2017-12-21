extern crate file;

use std::path::{PathBuf, Path};

use fall_tree::TextRange;
use indxr::{FileIndex, IndexableFileSet};

use editor::line_index::{LineCol, LineIndex};
use editor::file_symbols::process_symbols;

pub struct SymbolIndex {
    index: FileIndex<Vec<Symbol>>,
}

impl SymbolIndex {
    pub fn new(roots: Vec<PathBuf>) -> SymbolIndex {
        let file_set = IndexableFileSet::new(roots, "rs");
        let index = FileIndex::new(file_set, Box::new(mapper));
        SymbolIndex { index }
    }

    pub fn query(&self, query: &str) -> Vec<(PathBuf, Symbol)> {
        let mut result = Vec::new();
        self.index.process_files(&mut |file| {
            for symbol in file.value.iter() {
                if fuzzy_match(&symbol.name, query) {
                    result.push((file.path.clone(), symbol.clone()))
                }
            }
            result.len() > 512
        });
        result
    }
}

fn mapper(path: &Path) -> Option<Vec<Symbol>> {
    let text = file::get_text(path).ok()?;
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
    Some(symbols)
}

fn fuzzy_match(text: &str, query: &str) -> bool {
    let mut text_chars = text.chars();
    'outer: for qc in query.chars() {
        loop {
            match text_chars.next() {
                Some(tc) if tc == qc => continue 'outer,
                Some(_) => (),
                None => return false,
            }
        }
    }
    true
}

#[derive(Clone, Debug, Serialize)]
pub struct Symbol {
    pub name: String,
    pub range: TextRange,
    pub lc_range: [LineCol; 2]
}