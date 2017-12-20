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
        let mut result = Vec::new();
        for &(action_id, action) in ACTIONS.iter() {
            if action(self.file(), range.start(), false).is_some() {
                result.push(action_id)
            }
        }
        result
    }

    fn apply_context_action(&self, range: TextRange, id: &str) -> Option<TextEdit> {
        for &(action_id, action) in ACTIONS.iter() {
            if action_id == id {
                let edit = action(self.file(), range.start(), true)?.into_edit();
                return Some(edit);
            }
        }
        None
    }
}

const ACTIONS: &[(&str, fn(&File, TextUnit, bool) -> Option<ActionResult>)] = &[
    ("Add braces", add_use_braces),
    ("Add impl", add_impl)
];

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


fn add_impl(file: &File, offset: TextUnit, apply: bool) -> Option<ActionResult> {
    None
        .or_else(|| add_impl_for::<StructDef>(file, offset, apply))
        .or_else(|| add_impl_for::<EnumDef>(file, offset, apply))
}

fn add_impl_for<'f, T: NameOwner<'f> + TypeParametersOwner<'f>>(
    file: &'f File,
    offset: TextUnit,
    apply: bool,
) -> Option<ActionResult> {
    let decl: T = ast::node_at_offset(file.root(), offset)?;
    let name = decl.name()?;
    if !apply {
        return Some(ActionResult::Available);
    }
    let mut result = String::new();
    result += "\n\n";
    result += "impl";
    if let Some(params) = decl.type_parameters() {
        result += params.node().text().to_string().as_str();
    }
    result += " ";
    result += name.to_string().as_str();
    if let Some(params) = decl.type_parameters() {
        result += "<";
        let mut first = true;
        let mut add_comma = |result: &mut String| {
            if !first {
                result.push_str(", ")
            }
            first = false;
        };
        for lf in params.lifetime_parameters() {
            add_comma(&mut result);
            result += lf.lifetime().to_string().as_str();
        }
        for t in params.type_parameters() {
            add_comma(&mut result);
            result += t.name().unwrap().to_string().as_str();
        }
        result += ">";
    }
    result += " {\n\n}";

    let mut edit = FileEdit::new(file);
    edit.insert_text_after(decl.node(), result);
    Some(ActionResult::Applied(edit.into_text_edit()))
}


#[test]
fn test_add_impl() {
    use fall_editor::check_context_action;

    check_context_action::<RustEditorFile>("Add impl", r"
struct ^^Foo<X, Y: Clone> {}
", r"
struct Foo<X, Y: Clone> {}

impl<X, Y: Clone> Foo<X, Y> {

}
");
}
