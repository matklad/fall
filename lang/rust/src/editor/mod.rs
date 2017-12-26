use fall_tree::{File, TextEdit, TextRange, TextUnit, tu, AstNode, TextSuffix};
use fall_tree::search::{find_leaf_at_offset, ancestors};
use fall_tree::search::ast;
use fall_tree::visitor::{visitor, process_node, process_subtree_bottom_up};
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode};
use fall_editor::actions::ActionResult;
use fall_editor::hl::{self, Highlights};
use *;

mod actions;
use self::actions::ACTIONS;

mod file_symbols;
use self::file_symbols::process_symbols;

mod symbol_index;
pub use self::symbol_index::SymbolIndex;

mod line_index;
mod fst_subseq;

pub struct RustEditorFile {
    file: File
}

impl RustEditorFile {
    fn new(file: File) -> RustEditorFile {
        RustEditorFile { file }
    }

    pub fn after_space_typed(&self, offset: TextUnit) -> Option<String> {
        let let_kw = find_leaf_at_offset(self.file.root(), offset - tu(1)).left_biased()?;
        if let_kw.ty() != LET {
            return None;
        }
        let let_stmt: LetStmt = ast::ancestor(let_kw)?;
        if let_stmt.node().children().any(|node| node.ty() == SEMI || node.ty() == EQ) {
            return None;
        }
        if self.file.text().slice(TextSuffix::from(let_stmt.node().range().end())).starts_with(";") {
            return None;
        }
        Some(";".into())
    }

    pub fn breadcrumbs(&self, offset: TextUnit) -> Vec<String> {
        let leaf = match find_leaf_at_offset(self.file.root(), offset - tu(1)).left_biased() {
            Some(leaf) => leaf,
            None => return Vec::new(),
        };
        let mut acc = Vec::new();
        for node in ancestors(leaf) {
            fn process_decl<'f, D: NameOwner<'f>>(tag: &str, decl: D, acc: &mut Vec<String>) {
                if let Some(name) = decl.name() {
                    acc.push(format!("{} {}", tag, name))
                }
            }
            process_node(
                node,
                visitor(&mut acc)
                    .visit::<StructDef, _>(|def, acc| process_decl("struct", def, acc))
                    .visit::<EnumDef, _>(|def, acc| process_decl("enum", def, acc))
                    .visit::<TraitDef, _>(|def, acc| process_decl("trait", def, acc))
                    .visit::<FnDef, _>(|def, acc| process_decl("fn", def, acc))
                    .visit::<ImplDef, _>(|def, acc| {
                        let mut trefs = ast::children_of_type::<TypeReference>(def.node());
                        let t1 = trefs.next();
                        let t2 = trefs.next();
                        match (t1, t2) {
                            (Some(t1), Some(t2)) => acc.push(format!("impl {} for {}", t1.node().text(), t2.node().text())),
                            (Some(t1), None) => acc.push(format!("impl {}", t1.node().text())),
                            _ => ()
                        }
                    }),
            );
        }
        acc.reverse();
        acc
    }

}

impl EditorFileImpl for RustEditorFile {
    fn parse(text: &str) -> Self {
        RustEditorFile::new(lang_rust().parse(text))
    }

    fn edit(&self, edit: &TextEdit) -> Self {
        RustEditorFile::new(self.file.edit(edit))
    }

    fn file(&self) -> &File {
        &self.file
    }

    fn syntax_tree(&self) -> String {
        gen_syntax_tree(&self.file)
    }

    fn highlight(&self) -> Highlights {
        process_subtree_bottom_up(
            self.file.root(),
            hl::visitor(
                &[]
            ),
        )
    }

    fn structure(&self) -> Vec<FileStructureNode> {
        let mut nodes = Vec::new();
        process_symbols(self.file(), &mut|name, node| {
            nodes.push(FileStructureNode {
                name: name.to_string(),
                range: node.range(),
                children: Vec::new(),
            })
        });
        nodes
    }

    fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        let mut result = Vec::new();
        fall_editor::actions::default_context_actions(
            self.file(),
            range,
            &mut result,
        );
        for &(action_id, action) in ACTIONS {
            if action(self.file(), range.start(), false).is_some() {
                result.push(action_id);
            }
        }
        result
    }

    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        let def = fall_editor::actions::apply_default_context_action(
            self.file(),
            range,
            id,
        );
        if let Some(result) = def {
            return result
        }
        let &(_, action) = ACTIONS.iter().find(|&&(aid, _)| aid == id)?;
        action(self.file(), range.start(), true).map(ActionResult::into_edit)
    }
}

#[test]
fn extend_selection() {
    use fall_tree::{tu};
    let (text, range) = ::fall_tree::test_util::extract_range(r"
impl S {

^^    fn foo() {

    }
}", "^");

    let file = RustEditorFile::parse(&text);
    let range = file.extend_selection(range).unwrap();
    assert_eq!(range, TextRange::from_to(tu(15), tu(32)));
}

#[test]
fn space_after_let() {
    fn do_test(expcted_result: Option<&str>, text: &str) {
        let (text, offset) = ::fall_tree::test_util::extract_offset(text, "^");
        let file = RustEditorFile::parse(&text);
        let actual = file.after_space_typed(offset);
        assert_eq!(actual.as_ref().map(|s| s.as_str()), expcted_result)
    }

    do_test(Some(";"), r"
fn main() {
    let ^
}");

    do_test(None, r"
fn main() {
    if let ^
}");

    do_test(None, r"
fn main() {
    let ^;
}");

    do_test(None, r"
fn main() {
    if let ^x = 92;
}");
}