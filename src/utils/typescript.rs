use deno_ast::{
    parse_module, EmitOptions, ImportsNotUsedAsValues, MediaType, ModuleSpecifier, ParseParams,
    SourceMapOption, TranspileOptions,
};

pub fn transpile_typescript(
    specifier: ModuleSpecifier,
    code: &str,
) -> Result<String, deno_core::error::AnyError> {
    let parsed = parse_module(ParseParams {
        specifier,
        text: code.into(),
        media_type: MediaType::TypeScript,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })?;

    let transpiled_source = parsed
        .transpile(
            &TranspileOptions {
                imports_not_used_as_values: ImportsNotUsedAsValues::Remove,
                ..Default::default()
            },
            &EmitOptions {
                source_map: SourceMapOption::Inline,
                ..Default::default()
            },
        )?
        .into_source();

    String::from_utf8(transpiled_source.source)
        .map_err(|e| deno_core::error::generic_error(e.to_string()))
}
