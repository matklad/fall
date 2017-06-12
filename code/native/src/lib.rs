#[macro_use]
extern crate neon;
#[macro_use]
extern crate lazy_static;
extern crate fall_tree;
extern crate lang_fall;

use std::sync::Mutex;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsArray, JsInteger, JsObject, JsNull, JsValue, Object};

use lang_fall::{LANG_FALL, highlight};
use fall_tree::{File, dump_file};

lazy_static! {
    static ref FILE: Mutex<Option<File>> = Mutex::new(None);
}

fn file_create(call: Call) -> JsResult<JsNull> {
    let scope = call.scope;
    let text = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let text = text.value();
    let file = LANG_FALL.parse(text);
    *FILE.lock().unwrap() = Some(file);
    Ok(JsNull::new())
}

fn file_highlight(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    match *file {
        None => Ok(JsNull::new().upcast()),
        Some(ref file) => {
            let spans = highlight(&file);

            let result = JsArray::new(scope, spans.len() as u32);
            for (i, &(start, end, color)) in spans.iter().enumerate() {
                let arr = JsArray::new(scope, 3);
                arr.set(0, JsInteger::new(scope, start as i32))?;
                arr.set(1, JsInteger::new(scope, end as i32))?;
                arr.set(2, JsString::new(scope, color).unwrap())?;
                result.set(i as u32, arr)?;
            }
            Ok(result.upcast())
        }
    }
}

fn file_stats(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    match *file {
        None => Ok(JsNull::new().upcast()),
        Some(ref file) => {
            let stats = file.stats();
            let result = JsObject::new(scope);
            result.set("lexing_time",
                       JsInteger::new(scope, stats.lexing_time.subsec_nanos() as i32))?;
            result.set("parsing_time",
                       JsInteger::new(scope, stats.parsing_time.subsec_nanos() as i32))?;
            result.set("reparse_start",
                        JsInteger::new(scope, stats.reparsed_region.start().as_u32() as i32))?;
            result.set("reparse_end",
                        JsInteger::new(scope, stats.reparsed_region.end().as_u32() as i32))?;
            Ok(result.upcast())
        }
    }
}

fn file_tree(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let file = FILE.lock().unwrap();
    match *file {
        None => Ok(JsNull::new().upcast()),
        Some(ref file) => {
            let tree = dump_file(file);
            let result = JsString::new(scope, &tree).unwrap();
            Ok(result.upcast())
        }
    }
}



register_module!(m, {
    m.export("file_create", file_create)?;
    m.export("file_highlight", file_highlight)?;
    m.export("file_stats", file_stats)?;
    m.export("file_tree", file_tree)?;
    Ok(())
});
