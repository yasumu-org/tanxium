use std::{borrow::Cow, path::Path};

use crate::utils::typescript;
use deno_core::{
    anyhow::Context, error::generic_error, futures::FutureExt, resolve_import, FastString,
    ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSourceCode, ModuleType,
    RequestedModuleType,
};

pub const NPM_LOADER_CDN: &str = "https://cdn.jsdelivr.net/npm/";

pub const TRANSPILE_EXTENSIONS: [&str; 7] =
    [".ts", ".cts", ".mts", ".tsx", ".jsx", ".ctsx", ".mtsx"];
const TRANSPILE_REMOTE_EXTENSIONS: [&str; 1] = ["application/typescript"];
const REMOTE_MODULES: [&str; 3] = ["http://", "https://", "npm:"];

/// A module loader for Tanxium runtime.
///
/// This module loader supports loading module from local file system and remote URL.
pub struct TanxiumModuleLoader {
    cwd: String,
}

impl TanxiumModuleLoader {
    pub fn new(cwd: String) -> Self {
        Self { cwd }
    }

    pub fn is_remote_module(&self, specifier: &str) -> bool {
        REMOTE_MODULES
            .iter()
            .any(|prefix| specifier.starts_with(prefix))
    }

    pub fn should_transpile(&self, specifier: &str) -> bool {
        TRANSPILE_EXTENSIONS
            .iter()
            .any(|ext| specifier.ends_with(ext))
    }

    pub fn should_transpile_remote(&self, specifier: &str) -> bool {
        TRANSPILE_REMOTE_EXTENSIONS
            .iter()
            .any(|ext| specifier.eq_ignore_ascii_case(ext))
    }

    pub fn hash_module_specifier(&self, specifier: &str) -> String {
        let hash = md5::compute(specifier);
        format!("{:x}", hash)
    }

    pub fn get_module_cache_path(&self, specifier: &str) -> String {
        let hash = self.hash_module_specifier(specifier);
        format!("{}/.yasumu_modules/{}.js", self.cwd, hash)
    }
}

impl ModuleLoader for TanxiumModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let cache_path = self.get_module_cache_path(module_specifier.as_str());
        let cwd = self.cwd.clone();

        let fut = async move {
            let mod_str = module_specifier.as_str();
            let is_remote_module = REMOTE_MODULES.iter().any(|prefix| mod_str.starts_with(prefix));

            if is_remote_module {
                // Create the cache directory if it doesn't exist
                tokio::fs::create_dir_all(format!("{}/.yasumu_modules", cwd))
                .await
                .unwrap_or(());


                if Path::new(&cache_path).exists() {
                    let code = tokio::fs::read(cache_path).await.with_context(|| {
                        format!("Failed to load {}", module_specifier.as_str())
                    })?;

                    let module = ModuleSource::new(
                        ModuleType::JavaScript,
                        ModuleSourceCode::Bytes(code.into_boxed_slice().into()),
                        &module_specifier,
                        None,
                    );

                    return Ok(module);
                }

                let final_str = if mod_str.starts_with("npm:") {
                    format!("{}{}/+esm", NPM_LOADER_CDN, mod_str.replace("npm:", ""))
                } else {
                    mod_str.to_string()
                };

                let res = reqwest::get(&final_str).await.with_context(|| {
                    format!("Failed to load {}", module_specifier.as_str())
                })?;

                if res.status().is_success() {
                    let is_typescript = match res.headers().get("content-type") {
                        Some(content_type) => {
                            TRANSPILE_REMOTE_EXTENSIONS.iter().any(|ext| content_type.to_str().unwrap_or("").eq_ignore_ascii_case(ext))
                        },
                        None => false,
                    };

                    let code = res.text().await.with_context(|| {
                        format!("Failed to load {}", module_specifier.as_str())
                    })?;

                    let code = if is_typescript {
                        let transpiled = typescript::transpile_typescript(module_specifier.clone(), code.as_str())?;
                        transpiled
                    } else {
                        code
                    };

                    tokio::fs::write(cache_path, code.as_bytes()).await.unwrap_or(());

                    let code_bytes = code.into_bytes().into_boxed_slice().into();

                    let module = ModuleSource::new(ModuleType::JavaScript, ModuleSourceCode::Bytes(code_bytes), &module_specifier, None);

                    return Ok(module);
                }

                return Err(generic_error(format!("Failed to load remote module: {}", module_specifier.as_str())));
            }

            let path = module_specifier.to_file_path().map_err(|_| {
                generic_error(format!(
                "Provided module specifier \"{module_specifier}\" is not a file URL."
                ))
            })?;

            let ts_mod_type = Cow::Borrowed("typescript");

            let mut module_type = if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                // We only return JSON modules if extension was actually `.json`.
                // In other cases we defer to actual requested module type, so runtime
                // can decide what to do with it.
                if ext == "json" {
                    ModuleType::Json
                } else if TRANSPILE_EXTENSIONS.iter().any(|ext| ext.eq(ext)) {
                    ModuleType::Other(ts_mod_type.clone())
                } else {
                    match &requested_module_type {
                        RequestedModuleType::Other(ty) => ModuleType::Other(ty.clone()),
                        _ => ModuleType::JavaScript,
                    }
                }
            } else {
                ModuleType::JavaScript
            };

            // If we loaded a JSON file, but the "requested_module_type" (that is computed from
            // import attributes) is not JSON we need to fail.
            if module_type == ModuleType::Json
                && requested_module_type != RequestedModuleType::Json
            {
                return Err(generic_error("Attempted to load JSON module without specifying \"type\": \"json\" attribute in the import statement."));
            }

            let code = tokio::fs::read_to_string(path).await.with_context(|| {
                format!("Failed to load {}", module_specifier.as_str())
            })?;

            let code = if module_type == ModuleType::Other(ts_mod_type) {
                module_type = ModuleType::JavaScript;
                typescript::transpile_typescript(module_specifier.clone(), code.as_str())?
            } else {
                code
            };

            let module = ModuleSource::new(
                module_type,
                ModuleSourceCode::String(FastString::from(code)),
                &module_specifier,
                None,
            );

            Ok(module)
        }
        .boxed_local();

        ModuleLoadResponse::Async(fut)
    }
}
