use std::rc::Rc;

use boa_engine::builtins::promise::PromiseState;
use boa_engine::context::ContextBuilder;
use boa_engine::module::ModuleLoader;
use boa_engine::property::Attribute;
use boa_engine::*;
use boa_runtime::Console;

use crate::globals;
use crate::runtime;
use crate::typescript;

pub struct ScriptExtension {
    /// The path to the script
    pub path: String,
    /// Whether the script should be transpiled using the TypeScript transpiler
    pub transpile: bool,
}

pub struct DefaultScriptExtension {
    /// The extension script
    pub script: &'static str,
    /// Whether the script should be transpiled using the TypeScript transpiler
    pub transpile: bool,
    /// Whether the script is enabled
    pub enabled: bool,
}

pub struct Tanxium {
    /// The current runtime context. This is required for executing JavaScript code.
    pub context: Context,
    /// The options used to create the runtime
    pub options: TanxiumOptions,
}

pub struct TanxiumBuiltinsExposure {
    /// Whether to expose the crypto api
    pub crypto: bool,
    /// Whether to expose the performance api
    pub performance: bool,
    /// Whether to expose the global runtime object
    pub runtime: bool,
    /// Whether to expose the console api
    pub console: bool,
    /// Whether to expose the timers api
    pub timers: bool,
    /// Whether to expose the base64 api
    pub base64: bool,
}

pub struct TanxiumOptions {
    /// The current working directory
    pub cwd: String,
    /// Whether to expose typescript transpilation functions to the runtime
    pub typescript: bool,
    /// Set of enabled builtins
    pub builtins: TanxiumBuiltinsExposure,
    /// The global object name used in the runtime. Defaults to `Tanxium`
    pub global_object_name: String,
}

impl Tanxium {
    /// Create a new Tanxium runtime
    pub fn new(options: TanxiumOptions) -> Result<Self, std::io::Error> {
        let job_queue = Rc::new(runtime::jobs_queue::TanxiumJobsQueue::new());
        let module_loader = Rc::new(runtime::module_loader::YasumuModuleLoader::new(
            options.cwd.clone(),
            options.typescript.clone(),
        ));
        let context = ContextBuilder::new()
            .job_queue(job_queue.clone())
            .module_loader(module_loader)
            .build()
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create context: {}", e),
                )
            })?;

        Ok(Tanxium { context, options })
    }

    /// Initializes the runtime with the builtins and default extensions
    pub fn initialize_runtime(&mut self) -> Result<(), std::io::Error> {
        self.init_runtime_apis().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize runtime: {}", e),
            )
        })?;
        self.load_default_extensions()?;

        Ok(())
    }

    /// Initialize the runtime with the builtins
    /// This function initializes the runtime with the builtins specified in the `TanxiumOptions` object
    pub fn init_runtime_apis(&mut self) -> Result<(), JsError> {
        let ctx = &mut self.context;

        ctx.register_global_property(
            js_str!("__IDENTIFIER__"),
            js_string!(self.options.global_object_name.clone()),
            Attribute::all(),
        )?;

        globals::base64::base64_init(self)?;
        globals::crypto::crypto_init(self)?;
        globals::performance::performance_init(self)?;
        globals::runtime_object::runtime_object_init(self)?;

        if self.options.builtins.console {
            let console = Console::init(&mut self.context);

            self.context.register_global_property(
                Console::NAME,
                console,
                boa_engine::property::Attribute::all(),
            )?;
        }

        Ok(())
    }

    /// The current module loader
    pub fn get_module_loader(&self) -> Rc<dyn ModuleLoader> {
        self.context.module_loader()
    }

    /// Load default extensions into the runtime. Default extensions are scripts that are loaded into the runtime before any other script.
    pub fn load_default_extensions(&mut self) -> Result<(), std::io::Error> {
        let exts = vec![DefaultScriptExtension {
            script: include_str!("./extensions/00_timers.ts"),
            transpile: true,
            enabled: self.options.builtins.timers,
        }];

        for ext in exts {
            let js_src = if ext.transpile {
                self.transpile(&ext.script)?
            } else {
                ext.script.to_string()
            };

            let src = Source::from_bytes(js_src.as_bytes());
            self.context
                .eval(src)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }

        Ok(())
    }

    /// Load extensions into the runtime. Extensions are scripts that are loaded into the runtime before any other script.
    /// This is useful for loading polyfills or other scripts that are required by the main script.
    /// The `ext` parameter is a vector of `ScriptExtension` objects. Each object contains the path to the script and a boolean indicating whether the script should be transpiled.
    /// If the script should be transpiled, the script will be transpiled using the TypeScript transpiler.
    pub fn load_extensions(&mut self, ext: Vec<ScriptExtension>) -> Result<(), std::io::Error> {
        for e in ext {
            let content = std::fs::read_to_string(e.path.as_str())?;

            let js_src = if e.transpile {
                self.transpile(content.as_str())?
            } else {
                content
            };

            let src = Source::from_bytes(js_src.as_bytes());
            self.context
                .eval(src)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }

        Ok(())
    }

    /// Execute a JavaScript code as a module. `tanxium.run_event_loop()` must be called in order to get the result of the promise.
    pub fn execute(&mut self, code: &str) -> JsResult<JsValue> {
        let src = Source::from_bytes(code.as_bytes());
        let module = Module::parse(src, None, &mut self.context)?;
        let promise = module.load_link_evaluate(&mut self.context);

        self.run_event_loop();

        match promise.state() {
            PromiseState::Pending => Err(JsError::from_opaque(JsValue::String(JsString::from(
                "Module failed to execute",
            )))),
            PromiseState::Fulfilled(value) => Ok(value),
            PromiseState::Rejected(err) => Err(JsError::from_opaque(err)),
        }
    }

    /// Evaluate a JavaScript code string in the runtime
    pub fn eval(&mut self, code: &str) -> JsResult<JsValue> {
        let src = Source::from_bytes(code.as_bytes());
        self.context.eval(src)
    }

    /// Transpile TypeScript code to JavaScript without type checking
    pub fn transpile(&mut self, code: &str) -> Result<String, std::io::Error> {
        let transpiled = typescript::transpile_typescript(code);

        match transpiled {
            Ok(js) => Ok(js),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        }
    }

    /// Run the event loop
    pub fn run_event_loop(&mut self) {
        self.context.run_jobs();
        self.context.clear_kept_objects();
    }
}
