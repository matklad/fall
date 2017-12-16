#[macro_use]
extern crate neon;

use neon::vm::{Call, VmResult, JsResult, Lock, FunctionCall};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction, JsUndefined};
use neon::js::class::{Class, JsClass};
use neon::task::Task;

register_module!(m, {
    m.export("status", status)?;
    Ok(())
});

fn status(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let result = JsString::new(scope, &"Hello from Rust").unwrap();
    Ok(result)
}



