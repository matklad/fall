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

use neon::vm::{Call, VmResult, JsResult};
use neon::scope::Scope;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction};
use neon::task::Task;
use neon_serde::{from_value, to_value};

use lang_fall::{editor_api, FileWithAnalysis, Analysis};
use fall_tree::{TextUnit, TextRange, File, TextEdit, TextEditOp, tu};
use fall_gen::TestRenderer;

lazy_static! {
    static ref FILE: Mutex<Option<Arc<FileWithAnalysis>>> = Mutex::new(None);
}

fn file_create(call: Call) -> JsResult<JsNull> {
    let scope = call.scope;
    let text = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let text = text.value();
    *FILE.lock().unwrap() = Some(Arc::new(editor_api::analyse(text)));
    Ok(JsNull::new())
}

#[derive(Serialize)]
struct PerformanceCounters {
    lexing_time: u32,
    parsing_time: u32,
    reparsed_region: TextRange,
}

fn performance_counters(file: &File) -> PerformanceCounters {
    let stats = file.stats();
    PerformanceCounters {
        lexing_time: stats.lexing_time.subsec_nanos(),
        parsing_time: stats.parsing_time.subsec_nanos(),
        reparsed_region: stats.reparsed_region,
    }
}

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

fn parse_test(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let test = call.arguments.require(scope, 0)?.check::<JsInteger>()?.value() as usize;
    let callback = call.arguments.require(scope, 1)?.check::<JsFunction>()?;
    let file = FILE.lock().unwrap();

    if let Some(ref file) = *file {
        RenderTask(file.clone(), test).schedule(callback);
    }
    Ok(JsNull::new().upcast())
}

fn a_fn<'a, S, F>(call: &mut Call<'a>, f: F) -> JsResult<'a, JsValue>
    where S: Serialize,
          F: Fn(&mut Call<'a>, &Analysis) -> VmResult<S> {
    let file = FILE.lock().unwrap();
    let file: &FileWithAnalysis = match *file {
        None => return Ok(JsNull::new().upcast()),
        Some(ref file) => &*file,
    };
    file.analyse(|a| {
        let result = f(call, a)?;
        Ok(to_value(call.scope, &result)?)
    })
}

fn a_fn0<S: Serialize, F: Fn(&Analysis) -> S>(mut call: Call, f: F) -> JsResult<JsValue> {
    a_fn(&mut call, |_, file| Ok(f(file)))
}

fn a_fn1<'c, S: Serialize, D: DeserializeOwned>(
    mut call: Call<'c>,
    f: fn(&Analysis, D) -> S
) -> JsResult<'c, JsValue> {
    a_fn(&mut call, |call, file| {
        let arg0 = call.arguments.require(call.scope, 0)?;
        let arg: D = from_value(call.scope, arg0)?;
        Ok(f(file, arg))
    })
}

fn file_fn<'a, S, F>(call: &mut Call<'a>, f: F) -> JsResult<'a, JsValue>
    where S: Serialize,
          F: Fn(&mut Call<'a>, &File) -> VmResult<S> {
    let file = FILE.lock().unwrap();
    let file = match *file {
        None => return Ok(JsNull::new().upcast()),
        Some(ref file) => file.file(),
    };
    let result = f(call, file)?;
    Ok(to_value(call.scope, &result)?)
}

fn file_fn0<S: Serialize, F: Fn(&File) -> S>(mut call: Call, f: F) -> JsResult<JsValue> {
    file_fn(&mut call, |_, file| Ok(f(file)))
}

fn file_fn1<'c, S: Serialize, D: DeserializeOwned>(
    mut call: Call<'c>,
    f: fn(&File, D) -> S
) -> JsResult<'c, JsValue> {
    file_fn(&mut call, |call, file| {
        let arg0 = call.arguments.require(call.scope, 0)?;
        let arg: D = from_value(call.scope, arg0)?;
        Ok(f(file, arg))
    })
}

fn file_fn2<'c, S: Serialize, D1: DeserializeOwned, D2: DeserializeOwned>(
    mut call: Call<'c>,
    f: fn(&File, D1, D2) -> S
) -> JsResult<'c, JsValue> {
    file_fn(&mut call, |call, file| {
        let arg1 = call.arguments.require(call.scope, 0)?;
        let arg2 = call.arguments.require(call.scope, 1)?;
        let arg1: D1 = from_value(call.scope, arg1)?;
        let arg2: D2 = from_value(call.scope, arg2)?;
        Ok(f(file, arg1, arg2))
    })
}

#[derive(Serialize)]
struct VsEdit {
    ops: Vec<VsOp>
}

#[derive(Serialize)]
enum VsOp {
    Delete(TextRange),
    Insert(TextUnit, String),
}

fn convert_edit(edit: TextEdit) -> VsEdit {
    let mut result = VsEdit { ops: Vec::new() };
    let mut offset = tu(0);
    for op in edit.ops {
        match op {
            TextEditOp::Copy(range) => {
                if range.start() != offset {
                    result.ops.push(VsOp::Delete(TextRange::from_to(offset, range.start())))
                }
                offset = range.end();
            },
            TextEditOp::Insert(text) => {
                result.ops.push(VsOp::Insert(offset, text.to_string()))
            },
        }
    }
    return result;
}



register_module!(m, {
    m.export("create", file_create)?;
    m.export("treeAsText", |call| file_fn0(call, editor_api::tree_as_text))?;
    m.export("performanceCounters", |call| file_fn0(call, performance_counters))?;
    m.export("highlight", |call| a_fn0(call, editor_api::highlight))?;
    m.export("structure", |call| file_fn0(call, editor_api::structure))?;
    m.export("extendSelection", |call| file_fn1(call, editor_api::extend_selection))?;
    m.export("contextActions", |call| file_fn1(call, editor_api::context_actions))?;
    m.export("applyContextAction", |call| file_fn2(call, |file, range: TextRange, aid: String| {
        convert_edit(editor_api::apply_context_action(file, range, &aid))
    }))?;
    m.export("resolveReference", |call| a_fn1(call, editor_api::resolve_reference))?;
    m.export("findUsages", |call| a_fn1(call, editor_api::find_usages))?;
    m.export("diagnostics", |call| a_fn0(call, editor_api::diagnostics))?;
    m.export("reformat", |call| file_fn0(call, |file| convert_edit(editor_api::reformat(file))))?;
    m.export("testAtOffset", |call| file_fn1(call, editor_api::test_at_offset))?;
    m.export("parseTest", parse_test)?;
    Ok(())
});
