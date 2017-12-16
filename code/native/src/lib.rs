extern crate serde;
#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate fall_editor;
extern crate lang_rust;
extern crate lang_fall;

use neon::vm::{Call, VmResult, JsResult, Lock, FunctionCall};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction, JsUndefined};
use neon::js::class::{Class, JsClass};
use neon::task::Task;

use fall_editor::{EditorSupport};

mod support;
use self::support::{arg1, ret};

const LANGUAGES: &[EditorSupport] = &[
    lang_fall::FALL_EDITOR_SUPPORT,
    lang_rust::RUST_EDITOR_SUPPORT,
];


declare_types! {
    pub class JsSupport for EditorSupport {
        init(call) {
            let scope = call.scope;
            let idx: usize = arg1(scope, &call.arguments)?;
            Ok(LANGUAGES[idx])
        }

        method syntax_tree(call) {
            let scope = call.scope;
            let text: String = arg1(scope, &call.arguments)?;
            let tree = call.arguments.this(scope).grab(move |support| {
                support.syntax_tree(&text)
            });
            ret(scope, tree)
        }
    }
}

register_module!(m, {
    m.export("status", status)?;
    m.export("support_for_extension", support_for_extension)?;
    Ok(())
});

fn support_for_extension(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let ext: String = arg1(scope, &call.arguments)?;
    let idx = match LANGUAGES.iter().position(|s| s.extension == ext) {
        None => return Ok(JsNull::new().upcast()),
        Some(idx) => idx,
    };

    let arg: Handle<JsValue> = JsInteger::new(scope, idx as i32).upcast();
    let class: Handle<JsClass<JsSupport>> = JsSupport::class(scope)?;
    let constructor: Handle<JsFunction<JsSupport>> = class.constructor(scope)?;
    let sup = constructor.construct::<_, JsValue, _>(scope, ::std::iter::once(arg))?;
    Ok(sup.upcast())
}

fn status(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    ret(scope, "Hello from Rust")
}
