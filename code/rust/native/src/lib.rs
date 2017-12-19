#[macro_use]
extern crate neon;
#[macro_use]
extern crate generic_backend;
extern crate lang_rust;

use lang_rust::editor::RustEditorFile;

declare_editor_file_class! {
    JsRustEditorFile, RustEditorFile
}

register_module!(m, {
    m.export("parse", |call| generic_backend::parse::<JsRustEditorFile>(call))?;
    m.export("edit", |call| generic_backend::edit::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("metrics", |call| generic_backend::metrics::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("syntaxTree", |call| generic_backend::syntax_tree::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("extendSelection", |call| generic_backend::extend_selection::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("structure", |call| generic_backend::structure::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("reformat", |call| generic_backend::reformat::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("highlight", |call| generic_backend::highlight::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("diagnostics", |call| generic_backend::diagnostics::<RustEditorFile, JsRustEditorFile>(call))?;

    m.export("contextActions", |call| generic_backend::context_actions::<RustEditorFile, JsRustEditorFile>(call))?;
    m.export("applyContextAction", |call| generic_backend::apply_context_action::<RustEditorFile, JsRustEditorFile>(call))?;
    Ok(())
});
