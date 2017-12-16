use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use neon_serde::{to_value, from_value};

use neon::vm::{VmResult, This, Arguments, JsResult};
use neon::js::JsValue;
use neon::scope::RootScope;

pub fn arg1<A: DeserializeOwned + Send, T: This>(scope: &mut RootScope, args: &Arguments<T>) -> VmResult<A> {
    let arg = args.require(scope, 0)?;
    let arg = from_value(scope, arg)?;
    Ok(arg)
}

pub fn ret<'j, T: Serialize>(scope: &mut RootScope<'j>, value: T) -> JsResult<'j, JsValue> {
    Ok(to_value(scope, &value)?)
}
