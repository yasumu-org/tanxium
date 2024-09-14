use boa_engine::{
    js_str, js_string, object::ObjectInitializer, property::Attribute, JsError, JsString, JsValue,
    NativeFunction,
};
use std::str::FromStr;

use crate::tanxium::Tanxium;

/// Initialize the runtime crypto object
pub fn crypto_init(tanxium: &mut Tanxium) -> Result<(), JsError> {
    if !tanxium.options.builtins.crypto {
        return Ok(());
    }

    let context = &mut tanxium.context;
    let crypto = ObjectInitializer::new(context)
        .function(
            NativeFunction::from_fn_ptr(|_, _, _| {
                let result = uuid::Uuid::new_v4().to_string();

                Ok(JsValue::String(
                    JsString::from_str(result.as_str()).unwrap(),
                ))
            }),
            js_string!("randomUUID"),
            0,
        )
        .build();

    context.register_global_property(js_str!("crypto"), crypto, Attribute::all())?;

    Ok(())
}
