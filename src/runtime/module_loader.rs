use boa_engine::{
    job::NativeJob, module::ModuleLoader, JsError, JsNativeError, JsResult, JsValue, Module,
};
use boa_parser::Source;
use reqwest::blocking::Client;
use std::{path::Path, time::Duration};

use crate::typescript::transpile_typescript;

/// A module loader that can load modules from the file system or from remote sources.
pub struct YasumuModuleLoader {
    cwd: String,
    typescript: bool,
}

impl YasumuModuleLoader {
    pub fn new(cwd: String, typescript: bool) -> Self {
        Self { cwd, typescript }
    }

    pub fn should_transpile<'a>(&self, specifier: &'a str) -> bool {
        if !self.typescript {
            return false;
        }

        let ts_exts = vec![".ts", ".cts", ".mts", ".tsx"];
        ts_exts.iter().any(|ext| specifier.ends_with(ext))
    }
}

impl ModuleLoader for YasumuModuleLoader {
    fn load_imported_module(
        &self,
        _referrer: boa_engine::module::Referrer,
        specifier: boa_engine::JsString,
        finish_load: Box<
            dyn FnOnce(boa_engine::JsResult<boa_engine::Module>, &mut boa_engine::Context),
        >,
        context: &mut boa_engine::Context,
    ) {
        let is_ts_enabled = self.typescript.clone();
        let cwd = self.cwd.clone();
        let specifier = specifier.to_std_string_escaped();
        let specifier = if specifier.starts_with("file://") {
            specifier.replace("file://", "")
        } else {
            specifier
        };

        let is_file = Path::new(specifier.as_str()).exists();
        let is_remote_script =
            specifier.starts_with("http://") || specifier.starts_with("https://");

        if is_file {
            let is_ts = self.should_transpile(specifier.as_str());
            let job = Box::pin(async move {
                let source: Result<String, std::io::Error> = async {
                    let path = std::path::Path::new(specifier.as_str()).canonicalize()?;
                    let content = smol::fs::read_to_string(path).await;
                    Ok(content?)
                }
                .await;

                load_module_inner(source, is_ts, finish_load)
            });

            context.job_queue().enqueue_future_job(job, context);
        } else if is_remote_script {
            let job = Box::pin(async move {
                let mut is_ts = false;
                let script: Result<String, std::io::Error> = async {
                    println!("\x1b[92mFetching module {}\x1b[0m", specifier.clone());
                    let client = Client::new();
                    let res = client
                        .get(specifier.clone().as_str())
                        .timeout(Duration::from_secs(30))
                        .send()
                        .map_err(|e| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                        })?;

                    if is_ts_enabled {
                        is_ts = match res.headers().get("content-type") {
                            Some(content_type) => content_type == "application/typescript",
                            None => false,
                        };
                    }

                    let text = res.text().map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })?;

                    Ok(text)
                }
                .await;

                load_module_inner(script, is_ts, finish_load)
            });

            context.job_queue().enqueue_future_job(job, context);
        } else {
            let job = Box::pin(async move {
                let mut transpile = false;

                let content: Result<String, std::io::Error> = async {
                    let path = Path::new(cwd.as_str()).join(&specifier);
                    let possible_paths = vec![
                        "index.js",
                        "index.cjs",
                        "index.mjs",
                        "index.jsx",
                        "index.ts",
                        "index.cts",
                        "index.mts",
                        "index.tsx",
                    ];

                    let mut found = false;

                    for module_file in possible_paths {
                        let js_path = path.join(module_file);
                        if js_path.exists() {
                            found = true;
                            let ts_exts = vec![".ts", ".cts", ".mts", ".tsx"];
                            transpile = ts_exts.iter().any(|ext| specifier.ends_with(ext))
                        }
                    }

                    if !found {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("Cannot find module: {}", specifier),
                        ));
                    }

                    let content = smol::fs::read_to_string(path).await?;

                    Ok(content)
                }
                .await;

                load_module_inner(content, transpile, finish_load)
            });

            context.job_queue().enqueue_future_job(job, context);
        };
    }
}

/// Create a native job that loads a module from the file system.
fn load_module_inner(
    content: Result<String, std::io::Error>,
    transpile: bool,
    finish_load: Box<dyn FnOnce(JsResult<Module>, &mut boa_engine::Context)>,
) -> NativeJob {
    NativeJob::new(move |context| -> JsResult<JsValue> {
        let content = match content {
            Ok(content) => content,
            Err(err) => {
                finish_load(
                    Err(JsNativeError::typ().with_message(err.to_string()).into()),
                    context,
                );

                return Ok(JsValue::undefined());
            }
        };

        let source: Result<String, JsError> = if transpile {
            let value = transpile_typescript(content.as_str())
                .map_err(|e| JsNativeError::syntax().with_message(e.to_string()))?;
            Ok(value)
        } else {
            Ok(content)
        };

        match source {
            Ok(source) => {
                let source = Source::from_bytes(source.as_str());
                let module = Module::parse(source, None, context);
                finish_load(module, context);
                Ok(JsValue::undefined())
            }
            Err(err) => {
                finish_load(Err(err), context);
                Ok(JsValue::undefined())
            }
        }
    })
}
