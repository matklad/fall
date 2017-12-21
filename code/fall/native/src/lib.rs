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
    m.export("parse", generic_backend::parse::<JsFallEditorFile>)?;
    m.export("edit", generic_backend::edit::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("metrics", generic_backend::metrics::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("syntaxTree", generic_backend::syntax_tree::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("extendSelection", generic_backend::extend_selection::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("structure", generic_backend::structure::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("reformat", generic_backend::reformat::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("highlight", generic_backend::highlight::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("diagnostics", generic_backend::diagnostics::<FileWithAnalysis, JsFallEditorFile>)?;

    m.export("contextActions", generic_backend::context_actions::<FileWithAnalysis, JsFallEditorFile>)?;
    m.export("applyContextAction", generic_backend::apply_context_action::<FileWithAnalysis, JsFallEditorFile>)?;

    m.export("resolveReference", resolve_reference)?;
    m.export("findUsages", find_usages)?;
    m.export("testAtOffset", test_at_offset)?;
    m.export("renderTest", render_test)?;
    Ok(())
});
