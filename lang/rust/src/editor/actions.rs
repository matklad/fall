use fall_tree::{File, TextUnit, FileEdit, AstNode, Node};
use fall_tree::search::{find_leaf_at_offset, LeafAtOffset, Direction, sibling};
use fall_tree::search::ast;
use fall_editor::actions::ActionResult;
use {NameOwner, TypeParametersOwner, EnumDef, StructDef, UseDecl, COMMA, WHITESPACE};


pub const ACTIONS: &[(&str, fn(&File, TextUnit, bool) -> Option<ActionResult>)] = &[
    ("Add braces", add_use_braces),
    ("Add impl", add_impl),
    ("Swap", swap),
];

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
    use fall_editor::actions::check_context_action;

    check_context_action::<::editor::RustEditorFile>("Add braces", r"
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
    use fall_editor::actions::check_context_action;

    check_context_action::<::editor::RustEditorFile>("Add impl", r"
struct ^^Foo<X, Y: Clone> {}
", r"
struct Foo<X, Y: Clone> {}

impl<X, Y: Clone> Foo<X, Y> {

}
");
}

checkpoint!(TRAILING_COMMA);

fn swap(file: &File, offset: TextUnit, apply: bool) -> Option<ActionResult> {
    let comma = find_comma(file.root(), offset)?;
    let left = nonws_sibling(comma, Direction::Left)?;
    let right = nonws_sibling(comma, Direction::Right)?;
    if left.ty() != right.ty() {
        TRAILING_COMMA.pass();
        return None;
    }

    if !apply {
        return Some(ActionResult::Available);
    }
    let mut edit = FileEdit::new(file);
    edit.replace(left, right);
    edit.replace(right, left);
    Some(ActionResult::Applied(edit.into_text_edit()))
}

fn nonws_sibling<'f>(node: Node<'f>, direction: Direction) -> Option<Node<'f>> {
    let mut node = sibling(node, direction)?;
    while node.ty() == WHITESPACE {
        node = sibling(node, direction)?;
    }
    Some(node)
}

fn find_comma<'f>(node: Node<'f>, offset: TextUnit) -> Option<Node<'f>> {
    match find_leaf_at_offset(node, offset) {
        LeafAtOffset::None => None,
        LeafAtOffset::Single(node) => if node.ty() == COMMA { Some(node) } else { None },
        LeafAtOffset::Between(left, right) => match (left.ty(), right.ty()) {
            (COMMA, _) => Some(left),
            (_, COMMA) => Some(right),
            _ => None,
        }
    }
}


#[test]
fn test_swap() {
    use fall_editor::actions::{check_context_action, check_no_context_action};

    check_context_action::<::editor::RustEditorFile>(
        "Swap",
        r"struct Foo<X,^^ Y: Clone> {}",
        r"struct Foo<Y: Clone, X> {}",
    );

    TRAILING_COMMA.check(|| {
        check_no_context_action::<::editor::RustEditorFile>(
            "Swap",
            r"struct Foo<X, Y: Clone,^^> {}",
        );
    })
}
