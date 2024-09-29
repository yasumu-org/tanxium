use crate::tanxium::Tanxium;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};
use boa_engine::{
    js_str, js_string, object::ObjectInitializer, property::Attribute, JsArgs, JsError,
    JsNativeError, JsString, JsValue, NativeFunction,
};
use std::str::FromStr;

/// Initialize the runtime Base64 object
pub fn base64_init(tanxium: &mut Tanxium) -> Result<(), JsError> {
    if !tanxium.options.builtins.base64 {
        return Ok(());
    }

    let context = &mut tanxium.context;

    let base64_encode = NativeFunction::from_fn_ptr(|_, args, context| {
        let data = args.get_or_undefined(0).to_string(context)?;
        let result = STANDARD.encode(data.to_std_string_escaped().as_bytes());
        let val = JsString::from_str(result.as_str())
            .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

        Ok(JsValue::String(val))
    });

    let base64_decode = NativeFunction::from_fn_ptr(|_, args, context| {
        let data = args.get_or_undefined(0).to_string(context)?;
        let result = STANDARD
            .decode(data.to_std_string_escaped().as_bytes())
            .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

        let stringified = String::from_utf8(result)
            .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

        let val = JsString::from_str(stringified.as_str())
            .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

        Ok(JsValue::String(val))
    });

    let base64 = ObjectInitializer::new(context)
        .function(base64_encode.clone(), js_string!("encode"), 1)
        .function(base64_decode.clone(), js_string!("decode"), 1)
        .function(
            NativeFunction::from_fn_ptr(|_, args, context| {
                let data = args.get_or_undefined(0).to_string(context)?;
                let result = URL_SAFE.encode(data.to_std_string_escaped().as_bytes());
                let val = JsString::from_str(result.as_str())
                    .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

                Ok(JsValue::String(val))
            }),
            js_string!("encodeURL"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(|_, args, context| {
                let data = args.get_or_undefined(0).to_string(context)?;
                let result = URL_SAFE
                    .decode(data.to_std_string_escaped().as_bytes())
                    .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

                let stringified = String::from_utf8(result)
                    .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

                let val = JsString::from_str(stringified.as_str())
                    .map_err(|e| JsError::from(JsNativeError::typ().with_message(e.to_string())))?;

                Ok(JsValue::String(val))
            }),
            js_string!("decodeURL"),
            1,
        )
        .build();

    context.register_global_property(js_str!("Base64"), base64, Attribute::all())?;
    context.register_global_builtin_callable(js_string!("btoa"), 1, base64_encode)?;
    context.register_global_builtin_callable(js_string!("atob"), 1, base64_decode)?;

    Ok(())
}
