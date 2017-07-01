#[macro_use]
extern crate neon;
#[macro_use]
extern crate lazy_static;
extern crate fall_tree;
extern crate lang_fall;

use std::sync::Mutex;

use neon::vm::{Call, JsResult, };
use neon::scope::Scope;
use neon::mem::Handle;
use neon::js::{JsString, JsArray, JsInteger, JsObject, JsNull, JsValue, Object};

use lang_fall::editor_api;
use lang_fall::editor_api::{FileStructureNode, Severity};
use fall_tree::{TextRange, TextUnit, File};

lazy_static! {
    static ref FILE: Mutex<Option<File>> = Mutex::new(None);
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
    *FILE.lock().unwrap() = Some(editor_api::parse(text));
    Ok(JsNull::new())
}

fn file_highlight(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let spans = editor_api::highlight(&file);

    let result = JsArray::new(scope, spans.len() as u32);
    for (i, &(range, color)) in spans.iter().enumerate() {
        let arr = JsArray::new(scope, 3);
        arr.set(0, text_unit_to_js(scope, range.start()))?;
        arr.set(1, text_unit_to_js(scope, range.end()))?;
        arr.set(2, JsString::new(scope, color).unwrap())?;
        result.set(i as u32, arr)?;
    }
    Ok(result.upcast())
}

pub fn text_unit_to_js<'a, T: Scope<'a>>(scope: &mut T, u: TextUnit) -> Handle<'a, JsInteger> {
    JsInteger::new(scope, u.as_u32() as i32)
}

pub fn range_to_js<'a, T: Scope<'a>>(scope: &mut T, range: TextRange) -> Handle<'a, JsArray> {
    let result = JsArray::new(scope, 2);
    result.set(0, text_unit_to_js(scope, range.start())).unwrap();
    result.set(1, text_unit_to_js(scope, range.end())).unwrap();
    result
}


fn file_stats(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let stats = file.stats();
    let result = JsObject::new(scope);
    result.set("lexing_time",
               JsInteger::new(scope, stats.lexing_time.subsec_nanos() as i32))?;
    result.set("parsing_time",
               JsInteger::new(scope, stats.parsing_time.subsec_nanos() as i32))?;
    result.set("reparse_start",
               text_unit_to_js(scope, stats.reparsed_region.start()))?;
    result.set("reparse_end",
               text_unit_to_js(scope, stats.reparsed_region.end()))?;
    Ok(result.upcast())
}

fn file_tree(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let tree = editor_api::tree_as_text(&file);
    let result = JsString::new(scope, &tree).unwrap();
    Ok(result.upcast())
}

fn file_extend_selection(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let start = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let end = call.arguments.require(scope, 1)?.check::<JsInteger>()?;
    let range = TextRange::from_to(
        TextUnit::from_usize(start.value() as usize),
        TextUnit::from_usize(end.value() as usize),
    );
    match editor_api::extend_selection(&file, range) {
        None => Ok(JsNull::new().upcast()),
        Some(range) => {
            let arr = JsArray::new(scope, 3);
            arr.set(0, JsInteger::new(scope, range.start().as_u32() as i32))?;
            arr.set(1, JsInteger::new(scope, range.end().as_u32() as i32))?;
            Ok(arr.upcast())
        }
    }
}

fn file_find_context_actions(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);

    let actions = editor_api::collect_applicable_context_actions(file, offset);
    let arr = JsArray::new(scope, actions.len() as u32);
    for (i, id) in actions.into_iter().enumerate() {
        let s = JsString::new(scope, id.0).unwrap();
        arr.set(i as u32, s)?;
    }
    Ok(arr.upcast())
}

fn file_apply_context_action(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let offset = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
    let offset = TextUnit::from_usize(offset.value() as usize);
    let id = call.arguments.require(scope, 1)?.check::<JsString>()?;

    let edit = editor_api::apply_context_action(file, offset, &id.value());

    let arr = JsArray::new(scope, 3);
    arr.set(0, JsInteger::new(scope, edit.delete.start().as_u32() as i32))?;
    arr.set(1, JsInteger::new(scope, edit.delete.end().as_u32() as i32))?;
    arr.set(2, JsString::new(scope, &edit.insert).unwrap())?;
    Ok(arr.upcast())
}

fn file_structure(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let nodes = editor_api::file_structure(file);
    let arr = JsArray::new(scope, nodes.len() as u32);
    for (i, node) in nodes.into_iter().enumerate() {
        let node_js = to_js(scope, node);
        arr.set(i as u32, node_js?)?;
    }
    return Ok(arr.upcast());

    fn to_js<'s, S: Scope<'s>>(scope: &mut S, node: FileStructureNode) -> JsResult<'s, JsValue> {
        let result = JsObject::new(scope);
        let children = JsArray::new(scope, node.children.len() as u32);
        for (i, node) in node.children.into_iter().enumerate() {
            children.set(i as u32, to_js(scope, node)?)?;
        }
        result.set("name", JsString::new(scope, &node.name).unwrap())?;
        result.set("children", children)?;

        result.set("range", range_to_js(scope, node.range))?;
        Ok(result.upcast())
    }
}

fn file_diagnostics(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    let file = get_file_or_return_null!(file);
    let diagnostics = editor_api::diagnostics(file);
    let result = JsArray::new(scope, diagnostics.len() as u32);

    for (i, diag) in diagnostics.into_iter().enumerate() {
        let d = JsObject::new(scope);
        d.set("text", JsString::new(scope, &diag.text).unwrap())?;
        d.set("range", range_to_js(scope, diag.range))?;
        let severity = match diag.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        d.set("severity", JsString::new(scope, severity).unwrap())?;
        result.set(i as u32, d)?;
    }
    Ok(result.upcast())
}

register_module!(m, {
    m.export("file_create", file_create)?;
    m.export("file_highlight", file_highlight)?;
    m.export("file_stats", file_stats)?;
    m.export("file_tree", file_tree)?;
    m.export("file_extend_selection", file_extend_selection)?;
    m.export("file_find_context_actions", file_find_context_actions)?;
    m.export("file_apply_context_action", file_apply_context_action)?;
    m.export("file_structure", file_structure)?;
    m.export("file_diagnostics", file_diagnostics)?;
    Ok(())
});
