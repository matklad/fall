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

use neon::vm::{Call, JsResult};
use neon::scope::{Scope, RootScope};
use neon::mem::Handle;
use neon::js::{JsString, JsArray, JsInteger, JsNull, JsValue, JsFunction, Object};
use neon::task::Task;
use neon_serde::to_value;

use lang_fall::editor_api;
use lang_fall::editor_api::{FileStructureNode, Severity, Diagnostic};
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


fn file_stats(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let stats = file.stats();

    #[derive(Serialize)]
    struct Stats {
        lexing_time: u32,
        parsing_time: u32,
        reparse_start: u32,
        reparse_end: u32,
    }

    let stats = Stats {
        lexing_time: stats.lexing_time.subsec_nanos(),
        parsing_time: stats.parsing_time.subsec_nanos(),
        reparse_start: stats.reparsed_region.start().as_u32(),
        reparse_end: stats.reparsed_region.end().as_u32(),
    };
    Ok(to_value(&stats, scope)?)
}

fn file_tree(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let tree = editor_api::tree_as_text(&file);
    Ok(to_value(&tree, scope)?)
}

fn file_find_context_actions(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);

    let actions = editor_api::collect_applicable_context_actions(file, offset)
        .into_iter()
        .map(|id| id.0)
        .collect::<Vec<_>>();
    Ok(to_value(&actions, scope)?)
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

fn file_apply_context_action(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);
    let id = call.arguments.require(scope, 1)?.check::<JsString>()?;

    let edit = editor_api::apply_context_action(file, offset, &id.value());

    Ok(text_edit_to_js(scope, edit).upcast())
}

fn file_structure(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let nodes = editor_api::file_structure(file);

    #[derive(Serialize)]
    struct Node {
        name: String,
        children: Vec<Node>,
        range: (u32, u32)
    }
    impl Node {
        fn new(fsn: FileStructureNode) -> Node {
            Node {
                name: fsn.name,
                children: fsn.children.into_iter().map(Node::new).collect(),
                range: (fsn.range.start().as_u32(), fsn.range.end().as_u32()),
            }
        }
    }
    let nodes = nodes.into_iter()
        .map(Node::new)
        .collect::<Vec<_>>();
    return Ok(to_value(&nodes, scope)?);
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

fn file_fn0<S: Serialize>(call: Call, f: fn(&File) -> S) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = match *file {
        None => return Ok(JsNull::new().upcast()),
        Some(ref file) => file,
    };
    Ok(to_value(&f(file), scope)?)
}

fn file_fn1<'c, S: Serialize, D: Deserialize<'c>>(
    call: Call<'c>,
    f: fn(&File, D) -> Option<S>
) -> JsResult<'c, JsValue> {
    let scope: &'c mut RootScope<'c> = call.scope;
    let file = FILE.lock().unwrap();
    let file = match *file {
        None => return Ok(JsNull::new().upcast()),
        Some(ref file) => file,
    };
    let arg: D = from_handle(call.arguments.require(scope, 0)?, scope)?;
    Ok(to_value(&f(file, arg), scope)?)
}

register_module!(m, {
    m.export("highlight", |call| file_fn0(call, editor_api::highlight))?;
    m.export("extend_selection", |call| file_fn1(call, editor_api::extend_selection))?;
    m.export("file_create", file_create)?;
    m.export("file_stats", file_stats)?;
    m.export("file_tree", file_tree)?;
    m.export("file_find_context_actions", file_find_context_actions)?;
    m.export("file_apply_context_action", file_apply_context_action)?;
    m.export("file_structure", file_structure)?;
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

pub fn from_handle<'a, T: serde::Deserialize<'a> + ? Sized>(
    input: Handle<'a, JsValue>,
    scope: &mut RootScope<'a>,
) -> neon_serde::errors::Result<T> {
    let scope: &'a mut RootScope<'a> = unsafe { ::std::mem::transmute(scope) };
    neon_serde::from_handle(input, scope)
}
