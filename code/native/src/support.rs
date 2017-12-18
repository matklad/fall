use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use neon_serde::{to_value, from_value};

use neon::vm::{VmResult, This, Arguments, JsResult};
use neon::js::JsValue;
use neon::scope::RootScope;

pub fn arg1<A: DeserializeOwned + Send, T: This>(scope: &mut RootScope, args: &Arguments<T>) -> VmResult<A> {
    arg(scope, args, 0)
}

pub fn arg2<A1, A2, T>(
    scope: &mut RootScope,
    args: &Arguments<T>
) -> VmResult<(A1, A2)>
    where A1: DeserializeOwned + Send,
          A2: DeserializeOwned + Send,
          T: This {
    let arg1: A1 = arg(scope, args, 0)?;
    let arg2: A2 = arg(scope, args, 1)?;
    Ok((arg1, arg2))
}

pub fn ret<'j, T: Serialize>(scope: &mut RootScope<'j>, value: T) -> JsResult<'j, JsValue> {
    Ok(to_value(scope, &value)?)
}

fn arg<A: DeserializeOwned + Send, T: This>(scope: &mut RootScope, args: &Arguments<T>, idx: i32) -> VmResult<A> {
    let arg = args.require(scope, idx)?;
    Ok(from_value(scope, arg)?)
}
