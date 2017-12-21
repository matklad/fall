extern crate file;

use std::path::{PathBuf, Path};

use fall_tree::TextRange;
use fall_tree::visitor::{visitor, process_subtree_bottom_up};
use indxr::{FileIndex, IndexableFileSet};

use ::{NameOwner, StructDef, FnDef, EnumDef, TraitDef};

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

    fn process_name_owner<'f, D: NameOwner<'f>>(def: D, nodes: &mut Vec<Symbol>) {
        if let Some(name_ident) = def.name_ident() {
            nodes.push(Symbol {
                name: name_ident.text().to_string(),
                range: def.node().range(),
            })
        }
    }

    let symbols = process_subtree_bottom_up(
        file.root(),
        visitor(Vec::new())
            .visit::<FnDef, _>(|nodes, def| process_name_owner(def, nodes))
            .visit::<StructDef, _>(|nodes, def| process_name_owner(def, nodes))
            .visit::<EnumDef, _>(|nodes, def| process_name_owner(def, nodes))
            .visit::<TraitDef, _>(|nodes, def| process_name_owner(def, nodes)),
    );
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
}