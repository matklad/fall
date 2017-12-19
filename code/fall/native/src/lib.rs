#[macro_use]
extern crate neon;
#[macro_use]
extern crate generic_backend;
extern crate lang_fall;

use neon::vm::{Call, JsResult};
use neon::js::JsValue;

use lang_fall::editor::FileWithAnalysis;

declare_editor_file_class! {
    JsFallEditorFile, FileWithAnalysis
}

register_module!(m, {
    m.export("parse", |call| generic_backend::parse::<JsFallEditorFile>(call))?;
    m.export("edit", |call| generic_backend::edit::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("metrics", |call| generic_backend::metrics::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("syntaxTree", |call| generic_backend::syntax_tree::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("extendSelection", |call| generic_backend::extend_selection::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("structure", |call| generic_backend::structure::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("reformat", |call| generic_backend::reformat::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("highlight", |call| generic_backend::highlight::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("diagnostics", |call| generic_backend::diagnostics::<FileWithAnalysis, JsFallEditorFile>(call))?;

    m.export("contextActions", |call| generic_backend::context_actions::<FileWithAnalysis, JsFallEditorFile>(call))?;
    m.export("applyContextAction", |call| generic_backend::apply_context_action::<FileWithAnalysis, JsFallEditorFile>(call))?;
    Ok(())
});
