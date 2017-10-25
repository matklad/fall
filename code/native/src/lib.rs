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

use std::sync::{Arc, Mutex, MutexGuard};

use neon::vm::{Call, VmResult, JsResult};
use neon::scope::{Scope, RootScope};
use neon::mem::Handle;
use neon::js::{JsString, JsArray, JsInteger, JsNull, JsValue, JsFunction, Object};
use neon::task::Task;

use lang_fall::editor_api;
use lang_fall::editor_api::{Severity, Diagnostic};
use fall_tree::{TextRange, TextUnit, File, TextEdit};
use fall_gen::TestRenderer;

lazy_static! {
    static ref FILE: Mutex<Option<Arc<File>>> = Mutex::new(None);
}

lazy_static! {
    static ref TEST_RENDERER: Mutex<TestRenderer> = Mutex::new(TestRenderer);
}

fn renderer() -> MutexGuard<'static, TestRenderer> {
    TEST_RENDERER.lock().unwrap()
}

macro_rules! get_file_or_return_null {
    ($guard:expr) => {
        match *$guard {
            None => return Ok(JsNull::new().upcast()),
            Some(ref f) => f,
        }
    }
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

fn file_find_test_at_offset(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);
    if let Some(idx) = editor_api::find_test_at_offset(file, offset) {
        Ok(to_value(&idx, scope)?)
    } else {
        Ok(JsNull::new().upcast())
    }
}

fn file_diagnostics(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);

    #[derive(Serialize)]
    struct Diag {
        text: String,
        range: (u32, u32),
        severity: &'static str
    }

    impl From<Diagnostic> for Diag {
        fn from(d: Diagnostic) -> Diag {
            Diag {
                text: d.text,
                range: (d.range.start().as_u32(), d.range.end().as_u32()),
                severity: match d.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                },
            }
        }
    }

    let result = editor_api::diagnostics(file).into_iter()
        .map(Into::<Diag>::into)
        .collect::<Vec<_>>();
    Ok(to_value(&result, scope)?)
}

fn file_resolve_reference(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);

    let result = if let Some(range) = editor_api::resolve_reference(file, offset) {
        range_to_js(scope, range).upcast()
    } else {
        JsNull::new().upcast()
    };

    Ok(result)
}

fn file_find_usages(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);

    let result = editor_api::find_usages(file, offset)
        .into_iter()
        .map(|range| (range.start().as_u32(), range.end().as_u32()))
        .collect::<Vec<_>>();
    Ok(to_value(&result, scope)?)
}

fn file_reformat(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let edit = editor_api::reformat(file);

    Ok(text_edit_to_js(scope, edit).upcast())
}

fn parse_test(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let test = call.arguments.require(scope, 0)?.check::<JsInteger>()?.value() as usize;
    let callback = call.arguments.require(scope, 1)?.check::<JsFunction>()?;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file).clone();

    struct RenderTask(Arc<File>, usize);
    impl Task for RenderTask {
        type Output = String;
        type Error = ();
        type JsEvent = JsString;

        fn perform(&self) -> Result<String, ()> {
            let mut renderer = renderer();
            let tree = renderer.render_one(&self.0, self.1);
            Ok(tree)
        }

        fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<String, ()>) -> JsResult<JsString> {
            Ok(JsString::new(scope, &result.unwrap()).unwrap())
        }
    }

    RenderTask(file, test).schedule(callback);
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
    m.export("apply_context_action", |call| file_fn2(call, |file, offset: TextUnit, aid: String| {
        editor_api::apply_context_action(file, offset, &aid)
    }))?;
    m.export("file_create", file_create)?;
    m.export("file_diagnostics", file_diagnostics)?;
    m.export("file_resolve_reference", file_resolve_reference)?;
    m.export("file_find_usages", file_find_usages)?;
    m.export("file_reformat", file_reformat)?;

    m.export("file_find_test_at_offset", file_find_test_at_offset)?;
    m.export("parse_test", parse_test)?;
    Ok(())
});

fn text_unit_to_js<'a, T: Scope<'a>>(scope: &mut T, u: TextUnit) -> Handle<'a, JsInteger> {
    JsInteger::new(scope, u.as_u32() as i32)
}

fn range_to_js<'a, T: Scope<'a>>(scope: &mut T, range: TextRange) -> Handle<'a, JsArray> {
    let result = JsArray::new(scope, 2);
    result.set(0, text_unit_to_js(scope, range.start())).unwrap();
    result.set(1, text_unit_to_js(scope, range.end())).unwrap();
    result
}

fn text_edit_to_js<'a>(scope: &'a mut RootScope, edit: TextEdit) -> Handle<'a, JsValue> {
    let tuple = (
        edit.delete.start().as_u32(),
        edit.delete.end().as_u32(),
        &edit.insert
    );
    to_value(&tuple, scope).unwrap()
}

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

