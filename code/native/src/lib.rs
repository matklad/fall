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

use serde::{Serialize, Deserialize};

use std::sync::{Arc, Mutex};

use neon::vm::{Call, VmResult, JsResult};
use neon::scope::{Scope, RootScope};
use neon::mem::Handle;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction};
use neon::task::Task;

use lang_fall::editor_api;
use fall_tree::{TextRange, File};
use fall_gen::TestRenderer;

lazy_static! {
    static ref FILE: Mutex<Option<Arc<File>>> = Mutex::new(None);
}

fn file_create(call: Call) -> JsResult<JsNull> {
    let scope = call.scope;
    let text = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let text = text.value();
    *FILE.lock().unwrap() = Some(Arc::new(editor_api::parse(text)));
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

struct RenderTask(Arc<File>, usize);
impl Task for RenderTask {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<String, ()> {
        lazy_static! {
            static ref TEST_RENDERER: Mutex<TestRenderer> = Mutex::new(TestRenderer);
        }

        let mut renderer = TEST_RENDERER.lock().unwrap();
        let tree = renderer.render_one(&self.0, self.1);
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

fn file_fn<'a, S, F>(call: &mut Call<'a>, f: F) -> JsResult<'a, JsValue>
    where S: Serialize,
          F: Fn(&mut Call<'a>, &File) -> VmResult<S> {
    let file = FILE.lock().unwrap();
    let file = match *file {
        None => return Ok(JsNull::new().upcast()),
        Some(ref file) => file,
    };
    let result = f(call, file)?;
    Ok(to_value(&result, call.scope)?)
}

fn file_fn0<S: Serialize, F: Fn(&File) -> S>(mut call: Call, f: F) -> JsResult<JsValue> {
    file_fn(&mut call, |_, file| Ok(f(file)))
}

fn file_fn1<'c, S: Serialize, D: Deserialize<'c>>(
    mut call: Call<'c>,
    f: fn(&File, D) -> S
) -> JsResult<'c, JsValue> {
    file_fn(&mut call, |call, file| {
        let arg: D = from_handle(call.arguments.require(call.scope, 0)?, call.scope)?;
        Ok(f(file, arg))
    })
}

fn file_fn2<'c, S: Serialize, D1: Deserialize<'c>, D2: Deserialize<'c>>(
    mut call: Call<'c>,
    f: fn(&File, D1, D2) -> S
) -> JsResult<'c, JsValue> {
    file_fn(&mut call, |call, file| {
        let arg1: D1 = from_handle(call.arguments.require(call.scope, 0)?, call.scope)?;
        let arg2: D2 = from_handle(call.arguments.require(call.scope, 1)?, call.scope)?;
        Ok(f(file, arg1, arg2))
    })
}

register_module!(m, {
    m.export("tree_as_text", |call| file_fn0(call, editor_api::tree_as_text))?;
    m.export("performance_counters", |call| file_fn0(call, performance_counters))?;
    m.export("highlight", |call| file_fn0(call, editor_api::highlight))?;
    m.export("structure", |call| file_fn0(call, editor_api::structure))?;
    m.export("extend_selection", |call| file_fn1(call, editor_api::extend_selection))?;
    m.export("context_actions", |call| file_fn1(call, editor_api::context_actions))?;
    m.export("apply_context_action", |call| file_fn2(call, |file, range: TextRange, aid: String| {
        editor_api::apply_context_action(file, range, &aid)
    }))?;
    m.export("resolve_reference", |call| file_fn1(call, editor_api::resolve_reference))?;
    m.export("find_usages", |call| file_fn1(call, editor_api::find_usages))?;
    m.export("diagnostics", |call| file_fn0(call, editor_api::diagnostics))?;
    m.export("reformat", |call| file_fn0(call, editor_api::reformat))?;
    m.export("test_at_offset", |call| file_fn1(call, editor_api::test_at_offset))?;
    m.export("parse_test", parse_test)?;
    m.export("file_create", file_create)?;
    Ok(())
});

fn from_handle<'a, T: serde::Deserialize<'a> + ? Sized>(
    input: Handle<'a, JsValue>,
    scope: &mut RootScope<'a>,
) -> neon_serde::errors::Result<T> {
    let scope: &'a mut RootScope<'a> = unsafe { ::std::mem::transmute(scope) };
    neon_serde::from_handle(input, scope)
}

fn to_value<'value, 'scope, V: Serialize + ?Sized>(
    value: &'value V,
    scope: &mut RootScope<'scope>,
) -> neon_serde::errors::Result<Handle<'scope, JsValue>> {
    let scope: &'scope mut RootScope<'scope> = unsafe { ::std::mem::transmute(scope) };
    neon_serde::to_value(value, scope)
}

