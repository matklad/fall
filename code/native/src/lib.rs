extern crate serde;
#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate fall_tree;
extern crate fall_editor;
extern crate lang_rust;
extern crate lang_fall;

use std::iter;

use neon::vm::{Call, JsResult, Lock};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::js::{JsString, JsInteger, JsNull, JsValue, JsFunction};
use neon::js::class::{Class, JsClass};
use neon::task::Task;

use fall_tree::File;
use fall_editor::{EditorSupport};

mod support;
use self::support::{arg1, ret};

const LANGUAGES: &[EditorSupport] = &[
    lang_fall::FALL_EDITOR_SUPPORT,
    lang_rust::RUST_EDITOR_SUPPORT,
];

pub struct VsFile {
    file: File,
    support: EditorSupport,
}

declare_types! {
    pub class JsSupport for EditorSupport {
        init(call) {
            let scope = call.scope;
            let idx: usize = arg1(scope, &call.arguments)?;
            Ok(LANGUAGES[idx])
        }

        method parse(call) {
            let scope = call.scope;
            let text = call.arguments.require(scope, 0)?.check::<JsString>()?;
            let support = call.arguments.this(scope);

            let class: Handle<JsClass<JsFile>> = JsFile::class(scope)?;
            let ctor: Handle<JsFunction<JsFile>> = class.constructor(scope)?;
            let ctor_args = iter::once(support.upcast()).chain(iter::once(text.upcast()));
            let file = ctor.construct::<_, JsValue, _>(scope, ctor_args)?;
            Ok(file.upcast())
        }
    }

    pub class JsFile for VsFile {
        init(call) {
            let scope = call.scope;
            let mut support = call.arguments.require(scope, 0)?.check::<JsSupport>()?;
            let text = call.arguments.require(scope, 1)?.check::<JsString>()?.value();
            let file = support.grab(move |support| support.parse(&text));
            let support = support.grab(|support| *support);
            Ok(VsFile {file, support })
        }

        method syntaxTree(call) {
            let scope = call.scope;
            let tree = call.arguments.this(scope).grab(move |vs_file| {
                vs_file.support.syntax_tree(&vs_file.file)
            });
            ret(scope, tree)
        }

        method highlight(call) {
            let scope = call.scope;
            let highlights = call.arguments.this(scope).grab(move |vs_file| {
                vs_file.support.highlight(&vs_file.file)
            });
            ret(scope, highlights)
        }

        method change(call) {
            let scope = call.scope;

            let this = call.arguments.this(scope);
            let edits = call.arguments.require(scope, 0)?;

            let class: Handle<JsClass<JsFile>> = JsFile::class(scope)?;
            let constructor: Handle<JsFunction<JsFile>> = class.constructor(scope)?;
            let file = constructor.construct::<_, JsValue, _>(scope, ::std::iter::once(this.upcast()).chain(::std::iter::once(edits)))?;
            Ok(file.upcast())
        }
    }
}

register_module!(m, {
    m.export("status", status)?;
    m.export("supportForExtension", support_for_extension)?;
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
