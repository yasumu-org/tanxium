use std::{rc::Rc, sync::Arc, time::Duration};

use crate::{
    exts::extensions::TanxiumExtension,
    module_loader::{TanxiumModuleLoader, TRANSPILE_EXTENSIONS},
};
use deno_runtime::{
    deno_core::{Extension, ModuleCodeString, ModuleSpecifier},
    deno_fs::RealFs,
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{MainWorker, WorkerOptions, WorkerServiceOptions},
    BootstrapOptions, WorkerExecutionMode,
};

pub struct TanxiumExtensionEntry {
    /// The module specifier of the extension.
    pub specifier: ModuleSpecifier,
    /// The code of the extension.
    pub code: &'static str,
}

pub struct TanxiumOptions {
    /// The current working directory.
    pub cwd: String,
    /// The main module specifier.
    pub main_module: ModuleSpecifier,
    /// Enable test mode.
    pub mode: WorkerExecutionMode,
    /// The extensions to include.
    pub extensions: Vec<Extension>,
}

pub struct Tanxium {
    /// The options used to create the Tanxium instance.
    pub options: TanxiumOptions,
    /// The underlying Deno runtime.
    pub runtime: MainWorker,
}

impl Tanxium {
    /// Create a new Tanxium instance.
    pub fn new(options: TanxiumOptions) -> Result<Self, deno_core::error::AnyError> {
        let main_module = options.main_module.clone();
        let fs = Arc::new(RealFs);
        let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

        let permissions = PermissionsContainer::allow_all(permission_desc_parser);

        let worker = MainWorker::bootstrap_from_options(
            main_module,
            WorkerServiceOptions {
                module_loader: Rc::new(TanxiumModuleLoader::new(options.cwd.clone())),
                permissions,
                blob_store: Default::default(),
                broadcast_channel: Default::default(),
                feature_checker: Default::default(),
                node_services: Default::default(),
                npm_process_state_provider: Default::default(),
                root_cert_store_provider: Default::default(),
                shared_array_buffer_store: Default::default(),
                compiled_wasm_module_store: Default::default(),
                v8_code_cache: Default::default(),
                fs,
            },
            WorkerOptions {
                extensions: vec![TanxiumExtension::init_ops()],
                skip_op_registration: false,
                bootstrap: BootstrapOptions {
                    user_agent: format!("Tanxium/{}", env!("CARGO_PKG_VERSION")),
                    mode: options.mode,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Ok(Tanxium {
            options: TanxiumOptions {
                extensions: vec![],
                ..options
            },
            runtime: worker,
        })
    }

    pub async fn load_runtime_api(
        &mut self,
        ext_entries: Option<Vec<TanxiumExtensionEntry>>,
    ) -> Result<(), deno_core::error::AnyError> {
        let modules = vec![
            TanxiumExtensionEntry {
                specifier: ModuleSpecifier::parse("ext:tanxium/core.ts")?,
                code: include_str!("./exts/js/tanxium.ts"),
            },
            TanxiumExtensionEntry {
                specifier: ModuleSpecifier::parse("ext:tanxium/testing.ts")?,
                code: include_str!("./exts/js/testing.ts"),
            },
        ];

        for module in modules {
            self.load_side_es_module_from_code(&module.specifier, module.code.to_string())
                .await?;
        }

        if ext_entries.is_some() {
            for entry in ext_entries.unwrap() {
                self.load_side_es_module_from_code(&entry.specifier, entry.code.to_string())
                    .await?;
            }
        }

        Ok(())
    }

    pub fn evaluate_script(
        &mut self,
        script_name: &'static str,
        source_code: String,
    ) -> Result<deno_core::v8::Global<deno_core::v8::Value>, deno_core::error::AnyError> {
        let specifier = ModuleSpecifier::parse(script_name)?;
        let maybe_transpiled = self.transpile_if_needed(specifier, source_code.as_str())?;
        let final_src_code = ModuleCodeString::from(maybe_transpiled);

        self.runtime.execute_script(script_name, final_src_code)
    }

    pub fn set_runtime_data(&mut self, data: String) -> Result<String, deno_core::error::AnyError> {
        let code_data = format!("Tanxium.setRuntimeData({});", data);
        let result = self.evaluate_script("file://tanxium-embedded/runtime_data.js", code_data)?;

        let scope = &mut self.runtime.js_runtime.handle_scope();
        let value = result.open(scope);
        let val = value.to_rust_string_lossy(scope);

        Ok(val)
    }

    pub fn get_runtime_data(&mut self) -> Result<String, deno_core::error::AnyError> {
        let result = self.runtime.execute_script(
            "file://tanxium-embedded/runtime_data.js",
            ModuleCodeString::from_static("Tanxium.getRuntimeDataString()"),
        )?;

        let scope = &mut self.runtime.js_runtime.handle_scope();
        let value = result.open(scope);
        let val = value.to_rust_string_lossy(scope);

        Ok(val)
    }

    pub async fn load_side_es_module(
        &mut self,
        specifier: &ModuleSpecifier,
    ) -> Result<(), deno_core::error::AnyError> {
        let result = self
            .runtime
            .js_runtime
            .load_side_es_module(specifier)
            .await?;

        self.runtime.js_runtime.mod_evaluate(result).await
    }

    pub async fn load_side_es_module_from_code(
        &mut self,
        specifier: &ModuleSpecifier,
        code: String,
    ) -> Result<(), deno_core::error::AnyError> {
        let result = self
            .runtime
            .js_runtime
            .load_side_es_module_from_code(
                specifier,
                self.transpile_if_needed(specifier.clone(), code.as_str())?,
            )
            .await?;

        self.runtime.js_runtime.mod_evaluate(result).await
    }

    pub async fn execute_main_module_code(
        &mut self,
        specifier: &ModuleSpecifier,
        code: String,
    ) -> Result<(), deno_core::error::AnyError> {
        let result = self
            .runtime
            .js_runtime
            .load_main_es_module_from_code(
                specifier,
                self.transpile_if_needed(specifier.clone(), code.as_str())?,
            )
            .await?;

        self.runtime.js_runtime.mod_evaluate(result).await
    }

    pub async fn execute_main_module(
        &mut self,
        module_specifier: &ModuleSpecifier,
    ) -> Result<(), deno_core::error::AnyError> {
        self.runtime.execute_main_module(module_specifier).await
    }

    pub async fn run_event_loop(
        &mut self,
        wait_for_inspector: bool,
    ) -> Result<(), deno_core::error::AnyError> {
        self.runtime.run_event_loop(wait_for_inspector).await
    }

    pub async fn run_up_to_duration(
        &mut self,
        duration: Duration,
    ) -> Result<(), deno_core::error::AnyError> {
        self.runtime.run_up_to_duration(duration).await
    }

    pub fn transpile_if_needed(
        &self,
        specifier: ModuleSpecifier,
        code: &str,
    ) -> Result<String, deno_core::error::AnyError> {
        if TRANSPILE_EXTENSIONS
            .iter()
            .any(|ext| specifier.path().ends_with(ext))
        {
            return crate::utils::typescript::transpile_typescript(specifier, code);
        }

        Ok(code.to_string())
    }
}

#[inline(always)]
/// Run the given future on the current thread.
pub fn run_current_thread<F, R>(future: F) -> R
where
    F: std::future::Future<Output = R> + 'static,
    R: Send + 'static,
{
    deno_runtime::tokio_util::create_and_run_current_thread(future)
}
