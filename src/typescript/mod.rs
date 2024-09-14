use boa_engine::{js_string, JsArgs, JsError, JsNativeError, JsString, JsValue, NativeFunction};
use deno_ast::{
    parse_module, EmitOptions, ImportsNotUsedAsValues, MediaType, ModuleSpecifier, ParseParams,
    SourceMapOption, TranspileOptions,
};

pub const YASUMU_WORKSPACE_SCRIPT_NAME: &str = "script.ts";
pub const YASUMU_WORKSPACE_SCRIPT_URL: &str = "file:///yasumu.workspace/script.ts";

/// Transpile TypeScript code to vanilla JavaScript
pub fn transpile_typescript(code: &str) -> Result<String, String> {
    let parsed = parse_module(ParseParams {
        specifier: ModuleSpecifier::parse(YASUMU_WORKSPACE_SCRIPT_URL).unwrap(),
        text: code.into(),
        media_type: MediaType::TypeScript,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })
    .unwrap();

    let transpiled_source = parsed
        .transpile(
            &TranspileOptions {
                imports_not_used_as_values: ImportsNotUsedAsValues::Remove,
                ..Default::default()
            },
            &EmitOptions {
                source_map: SourceMapOption::None,
                ..Default::default()
            },
        )
        .unwrap()
        .into_source();

    let source_text = String::from_utf8(transpiled_source.source).unwrap();

    Ok(source_text.into())
}

/// Initialize the TypeScript transpiler api in the runtime
pub fn typescript_init(context: &mut boa_engine::Context) -> Result<(), JsError> {
    context.register_global_builtin_callable(
        js_string!("transpileTypeScript"),
        1,
        NativeFunction::from_fn_ptr(|_, args, context| {
            let src = args.get_or_undefined(0).to_string(context).unwrap();
            let code = src.to_std_string().unwrap();
            let transpiled = transpile_typescript(code.as_str());

            match transpiled {
                Ok(js) => Ok(JsValue::from(JsString::from(js))),
                Err(e) => Err(JsError::from_native(JsNativeError::error().with_message(e))),
            }
        }),
    )?;

    Ok(())
}
