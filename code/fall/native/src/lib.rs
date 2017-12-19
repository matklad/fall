#[macro_use]
extern crate neon;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate generic_backend;
extern crate lang_fall;
extern crate fall_tree;
extern crate fall_gen;

use std::sync::Mutex;

use neon::vm::{Call, JsResult, Lock};
use neon::scope::Scope;
use neon::js::{JsValue, JsNull, JsString, JsFunction};
use neon::task::Task;

use fall_tree::TextUnit;
use fall_gen::TestRenderer;
use lang_fall::editor::FileWithAnalysis;

use generic_backend::{arg, ret, EditorFile};

declare_editor_file_class! {
    JsFallEditorFile, FileWithAnalysis
}

pub fn resolve_reference(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<JsFallEditorFile>()?;
    let offset: TextUnit = arg(scope, &call.arguments, 1)?;
    let result = editor_file.grab(|file| file.resolve_reference(offset));
    ret(scope, result)
}

pub fn find_usages(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<JsFallEditorFile>()?;
    let offset: TextUnit = arg(scope, &call.arguments, 1)?;
    let result = editor_file.grab(|file| file.find_usages(offset));
    ret(scope, result)
}

pub fn test_at_offset(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<JsFallEditorFile>()?;
    let offset: TextUnit = arg(scope, &call.arguments, 1)?;
    let result = editor_file.grab(|file| file.test_at_offset(offset));
    ret(scope, result)
}

pub fn render_test(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<JsFallEditorFile>()?;
    let file = editor_file.grab(|file| file.clone());
    let test_n: usize = arg(scope, &call.arguments, 1)?;
    let callback = call.arguments.require(scope, 2)?.check::<JsFunction>()?;
    RenderTask(file, test_n).schedule(callback);
    Ok(JsNull::new().upcast())
}

struct RenderTask(EditorFile<FileWithAnalysis>, usize);

impl Task for RenderTask {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<String, ()> {
        lazy_static! {
            static ref TEST_RENDERER: Mutex<TestRenderer> = Mutex::new(TestRenderer);
        }

        let mut renderer = TEST_RENDERER.lock().unwrap();
        let tree = renderer.render_one(&self.0.file(), self.1);
        Ok(tree)
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<String, ()>) -> JsResult<JsString> {
        Ok(JsString::new(scope, &result.unwrap()).unwrap())
    }
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

    m.export("resolveReference", |call| resolve_reference(call))?;
    m.export("findUsages", |call| find_usages(call))?;
    m.export("testAtOffset", |call| test_at_offset(call))?;
    m.export("renderTest", |call| render_test(call))?;
    Ok(())
});
