use fall_tree::{File, TextEdit, TextRange, TextUnit, FileEdit, AstNode};
use fall_tree::visitor::{process_subtree_bottom_up, visitor};
use fall_tree::search::ast;
use fall_editor::{EditorFileImpl, gen_syntax_tree, FileStructureNode};
use fall_editor::hl::{self, Highlights};
use *;

pub struct RustEditorFile {
    file: File
}

impl RustEditorFile {
    fn new(file: File) -> RustEditorFile {
        RustEditorFile { file }
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
        fn process_name_owner<'f, D: NameOwner<'f>>(def: D, nodes: &mut Vec<FileStructureNode>) {
            if let Some(name_ident) = def.name_ident() {
                nodes.push(FileStructureNode {
                    name: name_ident.text().to_string(),
                    range: def.node().range(),
                    children: Vec::new(),
                })
            }
        }
        process_subtree_bottom_up(
            self.file().root(),
            visitor(Vec::new())
                .visit::<FnDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<StructDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<EnumDef, _>(|nodes, def| process_name_owner(def, nodes))
                .visit::<TraitDef, _>(|nodes, def| process_name_owner(def, nodes)),
        )
    }

    fn context_actions(&self, range: TextRange) -> Vec<&'static str> {
        if add_use_braces(self.file(), range.start(), false).is_some() {
            return vec!["Add braces"];
        }
        Vec::new()
    }

    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        if id == "Add braces" {
            let edit = add_use_braces(self.file(), range.start(), true)?.into_edit();
            return Some(edit);
        }
        None
    }
}

enum ActionResult {
    Available,
    Applied(TextEdit),
}

impl ActionResult {
    fn into_edit(self) -> TextEdit {
        match self {
            ActionResult::Available =>
                panic!("Context action should provide edit when apply is set to true"),
            ActionResult::Applied(edit) => edit,
        }
    }
}


fn add_use_braces(file: &File, offset: TextUnit, apply: bool) -> Option<ActionResult> {
    let use_decl: UseDecl = ast::node_at_offset(file.root(), offset)?;
    let path = use_decl.path()?;
    let last_segment = path.segment()?.node();
    if use_decl.spec().is_some() {
        return None;
    }
    if !apply {
        return Some(ActionResult::Available);
    }
    let mut edit = FileEdit::new(&file);
    edit.replace_with_text(last_segment, format!("{{{}}}", last_segment.text()));
    Some(ActionResult::Applied(edit.into_text_edit()))
}

#[test]
fn test_add_use_braces() {
    use fall_editor::check_context_action;

    check_context_action::<RustEditorFile>("Add braces", r"
use foo::^^bar;
", r"
use foo::{bar};
");
}