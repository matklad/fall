extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate neon;
pub extern crate neon_serde;
extern crate fall_tree;
pub extern crate fall_editor;

use std::iter;
use std::sync::Arc;

use neon::vm::{Call, JsResult, Lock};
use neon::mem::Handle;
use neon::js::{JsString, JsValue, JsFunction};
use neon::js::class::{Class, JsClass};

use fall_tree::{TextEditBuilder, Text, TextRange, TextEdit, TextEditOp, tu};
use fall_editor::EditorFileImpl;

mod support;

pub use self::support::{arg, ret};

pub struct EditorFile<I: EditorFileImpl> {
    imp: Arc<I>
}

impl<I: EditorFileImpl> Clone for EditorFile<I> {
    fn clone(&self) -> Self {
        EditorFile { imp: self.imp.clone() }
    }
}

impl<I: EditorFileImpl> ::std::ops::Deref for EditorFile<I> {
    type Target = I;

    fn deref(&self) -> &I {
        self.imp.deref()
    }
}

impl<I: EditorFileImpl> EditorFile<I> {
    pub fn parse(text: &str) -> Self {
        EditorFile { imp: Arc::new(I::parse(text)) }
    }
    pub fn edit(prev: &EditorFile<I>, edit: &TextEdit) -> Self {
        let imp = prev.imp.edit(edit);
        EditorFile { imp: Arc::new(imp) }
    }
}

#[macro_export]
macro_rules! declare_editor_file_class {
    ($cls:ident, $t:tt) => {
        type EditorFileHack = $crate::EditorFile<$t>;

        declare_types! {
            pub class $cls for EditorFileHack {
                init(call) {
                    use $crate::neon::js::{JsString};
                    use $crate::neon::mem::Handle;
                    use $crate::neon::vm::Lock;

                    let scope = call.scope;
                    let file = match call.arguments.len() {
                        1 => {
                            let text = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
                            $crate::EditorFile::parse(&text)
                        }
                        2 => {
                            let mut file: Handle<$cls> = call.arguments.require(scope, 0)?.check::<$cls>()?;
                            let edits = call.arguments.require(scope, 1)?;
                            let edits: Vec<$crate::VsEdit> = $crate::neon_serde::from_value(scope, edits)?;
                            file.grab(|file| {
                                let edit = $crate::from_vs_edits(file.file().text(), edits);
                                $crate::EditorFile::edit(file, &edit)
                            })
                        }
                        _ => unreachable!()
                    };
                    Ok(file)
                }
            }
        }
    }
}

pub fn parse<C: Class>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let text = call.arguments.require(scope, 0)?.check::<JsString>()?;

    let class: Handle<JsClass<C>> = C::class(scope)?;
    let ctor: Handle<JsFunction<C>> = class.constructor(scope)?;
    let ctor_args = iter::once(text.upcast());
    let file = ctor.construct::<_, JsValue, _>(scope, ctor_args)?;
    Ok(file.upcast())
}

pub fn edit<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;

    let editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let edits = call.arguments.require(scope, 1)?;

    let class: Handle<JsClass<C>> = C::class(scope)?;
    let constructor: Handle<JsFunction<C>> = class.constructor(scope)?;
    let args = iter::once(editor_file.upcast()).chain(iter::once(edits));
    let file = constructor.construct(scope, args)?;
    Ok(file.upcast())
}

pub fn metrics<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| file.metrics());
    ret(scope, result)
}

pub fn syntax_tree<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| file.syntax_tree());
    ret(scope, result)
}

pub fn extend_selection<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let range: TextRange = arg(scope, &call.arguments, 1)?;
    let result = editor_file.grab(|file| file.extend_selection(range));
    ret(scope, result)
}

pub fn structure<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| file.structure());
    ret(scope, result)
}

pub fn reformat<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| {
        let edits = file.reformat();
        to_vs_edits(edits)
    });
    ret(scope, result)
}

pub fn highlight<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| file.highlight());
    ret(scope, result)
}

pub fn diagnostics<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let result = editor_file.grab(|file| file.diagnostics());
    ret(scope, result)
}

pub fn context_actions<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let range: TextRange = arg(scope, &call.arguments, 1)?;
    let result = editor_file.grab(|file| file.context_actions(range));
    ret(scope, result)
}

pub fn apply_context_action<I: EditorFileImpl, C: Class<Internals=EditorFile<I>>>(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut editor_file = call.arguments.require(scope, 0)?.check::<C>()?;
    let range: TextRange = arg(scope, &call.arguments, 1)?;
    let id: String = arg(scope, &call.arguments, 2)?;
    let result = editor_file.grab(move |file| {
        file.apply_context_action(range, &id).map(to_vs_edits)
    });
    ret(scope, result)
}

#[derive(Serialize, Deserialize)]
pub struct VsEdit {
    delete: TextRange,
    insert: String,
}

pub fn from_vs_edits(text: Text, edits: Vec<VsEdit>) -> TextEdit {
    let mut edit = TextEditBuilder::new(text);
    for e in edits {
        edit.replace(e.delete, e.insert)
    }
    edit.build()
}

pub fn to_vs_edits(edit: TextEdit) -> Vec<VsEdit> {
    let mut result = Vec::new();
    let mut offset = tu(0);
    for op in edit.ops {
        match op {
            TextEditOp::Copy(range) => {
                if range.start() != offset {
                    let range = TextRange::from_to(offset, range.start());
                    result.push(VsEdit { delete: range, insert: String::new() })
                }
                offset = range.end();
            }
            TextEditOp::Insert(text) => {
                let range = TextRange::from_len(offset, tu(0));
                result.push(VsEdit { delete: range, insert: text.to_string() })
            }
        }
    }
    return result;
}
