use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use neon_serde::{to_value, from_value};

use neon::vm::{VmResult, This, Arguments, JsResult};
use neon::js::JsValue;
use neon::scope::RootScope;

pub fn arg<A: DeserializeOwned + Send, T: This>(scope: &mut RootScope, args: &Arguments<T>, idx: i32) -> VmResult<A> {
    let arg = args.require(scope, idx)?;
    Ok(from_value(scope, arg)?)
}

pub fn ret<'j, T: Serialize>(scope: &mut RootScope<'j>, value: T) -> JsResult<'j, JsValue> {
    Ok(to_value(scope, &value)?)
}
