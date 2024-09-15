use boa_engine::{
    js_str, js_string,
    object::{builtins::JsUint8Array, ObjectInitializer},
    property::Attribute,
    JsArgs, JsError, JsNativeError, JsString, JsValue, NativeFunction,
};
use nid::Nanoid;
use rand::RngCore;
use std::str::FromStr;
use ulid::Ulid;

use crate::tanxium::Tanxium;

/// Initialize the runtime crypto object
pub fn crypto_init(tanxium: &mut Tanxium) -> Result<(), JsError> {
    if !tanxium.options.builtins.crypto {
        return Ok(());
    }

    let context = &mut tanxium.context;
    let crypto = ObjectInitializer::new(context)
        .function(
            NativeFunction::from_fn_ptr(|_, args, context| {
                let length = args.get_or_undefined(0).as_number();

                if length.is_none() {
                    return Err(JsError::from(
                        JsNativeError::typ().with_message("randomBytes requires a number argument"),
                    ));
                }

                let mut data = vec![0u8; length.unwrap() as usize];

                rand::thread_rng().try_fill_bytes(&mut data).map_err(|e| {
                    JsError::from(
                        JsNativeError::typ()
                            .with_message(format!("Failed to generate random bytes: {}", e)),
                    )
                })?;

                let result = JsUint8Array::from_iter(data, context)?;

                Ok(JsValue::new(result))
            }),
            js_string!("randomBytes"),
            1,
        )
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
        .function(
            NativeFunction::from_fn_ptr(|_, _, _| {
                let result = Ulid::new().to_string();

                Ok(JsValue::String(
                    JsString::from_str(result.as_str()).unwrap(),
                ))
            }),
            js_string!("randomULID"),
            0,
        )
        .function(
            NativeFunction::from_fn_ptr(|_, _, _| {
                let result: Nanoid = Nanoid::new();

                Ok(JsValue::String(
                    JsString::from_str(result.as_str()).unwrap(),
                ))
            }),
            js_string!("randomNanoId"),
            0,
        )
        .build();

    context.register_global_property(js_str!("crypto"), crypto, Attribute::all())?;

    Ok(())
}
