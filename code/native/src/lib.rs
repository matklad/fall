#[macro_use]
extern crate neon;
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

const LANGUAGES: &[EditorSupport] = &[
    lang_fall::FALL_EDITOR_SUPPORT,
    lang_rust::RUST_EDITOR_SUPPORT,
];


declare_types! {
    pub class JsSupport for EditorSupport {
        init(call) {
            let scope = call.scope;
            let idx: Handle<JsInteger> = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
            let idx = idx.value() as usize;
            Ok(LANGUAGES[idx])
        }

        method syntax_tree(call) {
            let scope = call.scope;
            let text = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
            let tree = call.arguments.this(scope).grab(move |support| {
                support.syntax_tree(&text)
            });
            Ok(match tree {
                None => JsNull::new().upcast(),
                Some(tree) => JsString::new(scope, &tree).unwrap().upcast(),
            })
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
    let ext = call.arguments.require(scope, 0)?.check::<JsString>()?;
    let ext = ext.value();
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

fn status(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let result = JsString::new(scope, &"Hello from Rust").unwrap();
    Ok(result)
}



