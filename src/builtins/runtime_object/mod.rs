use boa_engine::{
    js_str, js_string, object::ObjectInitializer, property::Attribute, Context, JsArgs, JsError,
    JsResult, JsString, JsValue, NativeFunction,
};
use futures_util::Future;
use std::time::{Duration, Instant};

use crate::tanxium::Tanxium;

/// A simple sleep function that sleeps for the given number of milliseconds
fn sleep(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> impl Future<Output = JsResult<JsValue>> {
    let millis = args.get_or_undefined(0).to_u32(context);

    async move {
        let millis = millis?;
        let now = Instant::now();
        smol::Timer::after(Duration::from_millis(u64::from(millis))).await;
        let elapsed = now.elapsed().as_secs_f64();
        Ok(elapsed.into())
    }
}

/// Initialize the runtime process object
/// This object is available in the global scope as `TanxiumOptions.global_object_name`
pub fn runtime_object_init(tanxium: &mut Tanxium) -> Result<(), JsError> {
    if !tanxium.options.builtins.runtime {
        return Ok(());
    }

    let ts_supported = tanxium.options.typescript;
    let context = &mut tanxium.context;
    let process_version = ObjectInitializer::new(context)
        .property(
            js_str!("tanxium"),
            JsString::from(env!("CARGO_PKG_VERSION")),
            Attribute::all(),
        )
        .build();

    let app_script_features = ObjectInitializer::new(context)
        .property(
            js_str!("typescript"),
            JsValue::Boolean(ts_supported),
            Attribute::all(),
        )
        .build();

    let process = ObjectInitializer::new(context)
        .property(js_str!("features"), app_script_features, Attribute::all())
        .property(js_str!("versions"), process_version, Attribute::all())
        .function(NativeFunction::from_async_fn(sleep), js_string!("sleep"), 1)
        .build();

    context.register_global_property(
        JsString::from(tanxium.options.global_object_name.as_str()),
        process,
        Attribute::all(),
    )?;

    Ok(())
}
