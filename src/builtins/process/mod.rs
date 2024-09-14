use boa_engine::{
    js_str, js_string, object::ObjectInitializer, property::Attribute, Context, JsArgs, JsResult,
    JsString, JsValue, NativeFunction,
};
use futures_util::Future;
use std::time::{Duration, Instant};

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

pub fn runtime_init(context: &mut Context, ts_supported: bool) {
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
        .property(
            js_str!("version"),
            JsString::from(env!("CARGO_PKG_VERSION")),
            Attribute::all(),
        )
        .property(js_str!("features"), app_script_features, Attribute::all())
        .property(js_str!("versions"), process_version, Attribute::all())
        .function(NativeFunction::from_async_fn(sleep), js_string!("sleep"), 1)
        .build();

    context
        .register_global_property(js_str!("process"), process, Attribute::all())
        .unwrap();
}
