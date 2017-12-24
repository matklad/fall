#[macro_use]
extern crate neon;
#[macro_use]
extern crate generic_backend;
extern crate lang_rust;
extern crate fall_tree;

use std::path::PathBuf;
use std::iter;

use neon::vm::{Call, JsResult, Lock};
use neon::mem::Handle;
use neon::js::{JsValue, JsFunction};
use neon::js::class::{JsClass, Class};

use generic_backend::{arg, ret};
use fall_tree::TextUnit;
use lang_rust::editor::RustEditorFile;
use lang_rust::editor::SymbolIndex;

declare_editor_file_class! {
    JsRustEditorFile, RustEditorFile
}

declare_types! {
    class JsIndex for SymbolIndex {
        init(call) {
            let scope = call.scope;
            let roots: Vec<PathBuf> = arg(scope, &call.arguments, 0)?;
            let index = SymbolIndex::new(roots);
            Ok(index)
        }
    }
}

fn create_index(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let paths = call.arguments.require(scope, 0)?;
    let class: Handle<JsClass<JsIndex>> = JsIndex::class(scope)?;
    let ctor: Handle<JsFunction<JsIndex>> = class.constructor(scope)?;
    let index = ctor.construct::<_, JsValue, _>(scope, iter::once(paths))?;
    Ok(index.upcast())
}

fn query_index(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut index = call.arguments.require(scope, 0)?.check::<JsIndex>()?;
    let query: String = arg(scope, &call.arguments, 1)?;
    let result = index.grab(move |index| index.query(&query));
    ret(scope, result)
}

fn after_space_typed(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut file = call.arguments.require(scope, 0)?.check::<JsRustEditorFile>()?;
    let offset: TextUnit = arg(scope, &call.arguments, 1)?;
    let result = file.grab(move |file| file.after_space_typed(offset));
    ret(scope, result)
}

register_module!(m, {
    m.export("parse", generic_backend::parse::<JsRustEditorFile>)?;
    m.export("edit", generic_backend::edit::<RustEditorFile, JsRustEditorFile>)?;
    m.export("metrics", generic_backend::metrics::<RustEditorFile, JsRustEditorFile>)?;
    m.export("syntaxTree", generic_backend::syntax_tree::<RustEditorFile, JsRustEditorFile>)?;
    m.export("extendSelection", generic_backend::extend_selection::<RustEditorFile, JsRustEditorFile>)?;
    m.export("structure", generic_backend::structure::<RustEditorFile, JsRustEditorFile>)?;
    m.export("reformat", generic_backend::reformat::<RustEditorFile, JsRustEditorFile>)?;
    m.export("highlight", generic_backend::highlight::<RustEditorFile, JsRustEditorFile>)?;
    m.export("diagnostics", generic_backend::diagnostics::<RustEditorFile, JsRustEditorFile>)?;

    m.export("contextActions", generic_backend::context_actions::<RustEditorFile, JsRustEditorFile>)?;
    m.export("applyContextAction", generic_backend::apply_context_action::<RustEditorFile, JsRustEditorFile>)?;

    m.export("createIndex", create_index)?;
    m.export("queryIndex", query_index)?;

    m.export("afterSpaceTyped", after_space_typed)?;
    Ok(())
});
