use std::{fs::File, rc::Rc, sync::Arc};

use deno_core::{FsModuleLoader, ModuleSpecifier};
use deno_runtime::{
    deno_fs::RealFs,
    deno_io::{Stdio, StdioPipe},
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{MainWorker, WorkerOptions, WorkerServiceOptions},
};

pub struct TanxiumOptions {
    /// The main module specifier.
    pub main_module: ModuleSpecifier,
    /// The stdout file
    pub stdout: Option<File>,
    /// The stderr file
    pub stderr: Option<File>,
    /// The stdin file
    pub stdin: Option<File>,
}

pub struct Tanxium {
    /// The options used to create the Tanxium instance.
    pub options: TanxiumOptions,
    /// The underlying Deno runtime.
    pub runtime: MainWorker,
}

impl Tanxium {
    pub fn new(options: TanxiumOptions) -> Self {
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

        let worker = MainWorker::bootstrap_from_options(
            main_module,
            WorkerServiceOptions {
                module_loader: Rc::new(FsModuleLoader),
                permissions: PermissionsContainer::allow_all(permission_desc_parser),
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
                extensions: vec![],
                stdio,
                ..Default::default()
            },
        );

        Tanxium {
            options: TanxiumOptions {
                main_module: options.main_module,
                stdout: None,
                stderr: None,
                stdin: None,
            },
            runtime: worker,
        }
    }
}
