use std::{fs::File, rc::Rc, sync::Arc, time::Duration};

use crate::{exts::extensions::TanxiumExtension, module_loader::TanxiumModuleLoader};
use deno_core::{Extension, ModuleSpecifier};
use deno_runtime::{
    deno_fs::RealFs,
    deno_io::{Stdio, StdioPipe},
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{MainWorker, WorkerOptions, WorkerServiceOptions},
    BootstrapOptions,
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
    pub test: bool,
    /// The stdout file
    pub stdout: Option<File>,
    /// The stderr file
    pub stderr: Option<File>,
    /// The stdin file
    pub stdin: Option<File>,
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

        let stdio = Stdio {
            stderr: match options.stderr {
                Some(file) => StdioPipe::file(file),
                None => StdioPipe::inherit(),
            },
            stdin: match options.stdin {
                Some(file) => StdioPipe::file(file),
                None => StdioPipe::inherit(),
            },
            stdout: match options.stdout {
                Some(file) => StdioPipe::file(file),
                None => StdioPipe::inherit(),
            },
        };

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
                stdio,
                bootstrap: BootstrapOptions {
                    user_agent: format!("Tanxium/{}", env!("CARGO_PKG_VERSION")),
                    enable_testing_features: options.test,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Ok(Tanxium {
            options: TanxiumOptions {
                cwd: options.cwd,
                main_module: options.main_module,
                extensions: vec![],
                test: options.test,
                stdout: None,
                stderr: None,
                stdin: None,
            },
            runtime: worker,
        })
    }

    pub async fn load_runtime_api(
        &mut self,
        ext_entries: Option<Vec<TanxiumExtensionEntry>>,
    ) -> Result<(), deno_core::error::AnyError> {
        let js_runtime = &mut self.runtime.js_runtime;

        let mut modules = vec![TanxiumExtensionEntry {
            specifier: ModuleSpecifier::parse("ext:tanxium/core")?,
            code: include_str!("./exts/js/tanxium.js"),
        }];

        if self.options.test {
            modules.push(TanxiumExtensionEntry {
                specifier: ModuleSpecifier::parse("ext:tanxium/testing")?,
                code: include_str!("./exts/js/testing.js"),
            });
        }

        for module in modules {
            let module = js_runtime
                .load_side_es_module_from_code(&module.specifier, module.code)
                .await?;
            js_runtime.mod_evaluate(module).await?;
        }

        if ext_entries.is_some() {
            for entry in ext_entries.unwrap() {
                let module = js_runtime
                    .load_side_es_module_from_code(&entry.specifier, entry.code)
                    .await?;
                js_runtime.mod_evaluate(module).await?;
            }
        }

        Ok(())
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
