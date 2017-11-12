#[macro_use]
extern crate neon;
#[macro_use]
extern crate lazy_static;
extern crate fall_tree;
extern crate fall_gen;
extern crate lang_fall;

extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde::Serialize;
use serde::de::DeserializeOwned;


use std::sync::{Arc, Mutex};

use neon::vm::{Call, VmResult, JsResult, Lock, FunctionCall};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction, JsUndefined};
use neon::js::class::{Class, JsClass};
use neon::task::Task;
use neon_serde::{from_value, to_value};

use lang_fall::{editor_api, FileWithAnalysis, Analysis};
use fall_tree::{TextRange, TextEdit, TextEditOp, tu};
use fall_gen::TestRenderer;



declare_types! {
  pub class JsFile for FileObj {
    init(call) { FileObj::init(call) }
    method highlight(call) { m(call, editor_api::highlight) }

    method syntaxTree(call) { m(call, editor_api::tree_as_text) }
    method structure(call) { m(call, editor_api::structure) }
    method reformat(call) {  m(call, |a| convert_edit(editor_api::reformat(a))) }
    method metrics(call) { m(call, |a| format!("{}", a.file().metrics())) }
    method diagnostics(call) { m(call, editor_api::diagnostics) }

    method extendSelection (call) { m1(call, editor_api::extend_selection) }
    method contextActions (call) { m1(call, editor_api::context_actions) }
    method testAtOffset (call) { m1(call, editor_api::test_at_offset) }
    method resolveReference(call) { m1(call, editor_api::resolve_reference) }
    method findUsages(call) { m1(call, editor_api::find_usages) }

    method applyContextAction(call) {
      m2(call, |a, arg1, arg2: String| {
        convert_edit(editor_api::apply_context_action(a, arg1, &arg2))
      })
    }

    method parseTest(call) {
      let scope = call.scope;
      let test = call.arguments.require(scope, 0)?.check::<JsInteger>()?.value() as usize;
      let callback = call.arguments.require(scope, 1)?.check::<JsFunction>()?;
      let file = call.arguments.this(scope).grab(move |file| file.inner.clone());
      RenderTask(file, test).schedule(callback);
      Ok(JsNull::new().upcast())
    }
  }
}

register_module!(m, {
    m.export("newFile", new_file)?;
    Ok(())
});

struct RenderTask(Arc<FileWithAnalysis>, usize);

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

#[derive(Serialize)]
struct VsEdit {
    ops: Vec<VsOp>
}

#[derive(Serialize)]
struct VsOp {
    delete: TextRange,
    insert: String,
}

fn convert_edit(edit: TextEdit) -> VsEdit {
    let mut result = VsEdit { ops: Vec::new() };
    let mut offset = tu(0);
    for op in edit.ops {
        match op {
            TextEditOp::Copy(range) => {
                if range.start() != offset {
                    let range = TextRange::from_to(offset, range.start());
                    result.ops.push(VsOp { delete: range, insert: String::new() })
                }
                offset = range.end();
            }
            TextEditOp::Insert(text) => {
                let range = TextRange::from_len(offset, tu(0));
                result.ops.push(VsOp { delete: range, insert: text.to_string() })
            }
        }
    }
    return result;
}

pub struct FileObj {
    inner: Arc<FileWithAnalysis>
}

impl FileObj {
    fn init(call: FunctionCall<JsUndefined>) -> VmResult<FileObj> {
        let scope = call.scope;
        let text: Handle<JsString> = call.arguments.require(scope, 0)?.check::<JsString>()?;
        Ok(FileObj { inner: Arc::new(editor_api::analyse(text.value())) })
    }
}

fn new_file(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let arg: Handle<JsValue> = call.arguments.require(scope, 0)?;
    let class: Handle<JsClass<JsFile>> = JsFile::class(scope)?;
    let constructor: Handle<JsFunction<JsFile>> = class.constructor(scope)?;
    let file = constructor.construct::<_, JsValue, _>(scope, ::std::iter::once(arg))?;
    Ok(file.upcast())
}

fn m<'j, R, F>(call: FunctionCall<'j, JsFile>, f: F) -> JsResult<'j, JsValue>
    where R: Serialize,
          F: Fn(&Analysis) -> R + Send
{
    let scope = call.scope;
    let result = call.arguments.this(scope).grab(move |file| file.inner.analyse(|a| f(a)));
    Ok(to_value(scope, &result)?)
}

fn m1<'j, A, R, F>(call: FunctionCall<'j, JsFile>, f: F) -> JsResult<'j, JsValue>
    where R: Serialize,
          A: DeserializeOwned + Send,
          F: Fn(&Analysis, A) -> R + Send
{
    let scope = call.scope;
    let arg0 = call.arguments.require(scope, 0)?;
    let arg: A = from_value(scope, arg0)?;
    let result = call.arguments.this(scope).grab(move |file| file.inner.analyse(|a| f(a, arg)));
    Ok(to_value(scope, &result)?)
}

fn m2<'j, A1, A2, R, F>(call: FunctionCall<'j, JsFile>, f: F) -> JsResult<'j, JsValue>
    where R: Serialize,
          A1: DeserializeOwned + Send,
          A2: DeserializeOwned + Send,
          F: Fn(&Analysis, A1, A2) -> R + Send
{
    let scope = call.scope;
    let arg1 = call.arguments.require(scope, 0)?;
    let arg1: A1 = from_value(scope, arg1)?;
    let arg2 = call.arguments.require(scope, 1)?;
    let arg2: A2 = from_value(scope, arg2)?;
    let result = call.arguments.this(scope).grab(move |file| file.inner.analyse(|a| f(a, arg1, arg2)));
    Ok(to_value(scope, &result)?)
}
