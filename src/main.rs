use deno_runtime::WorkerExecutionMode;
use tanxium::tanxium::{run_current_thread, Tanxium, TanxiumOptions};

fn main() {
    let future = async {
        let cwd = std::env::current_dir().unwrap();
        let target_file = std::env::args().nth(1).expect("missing target file");
        let target_file = cwd.join(target_file);
        let main_module = deno_core::ModuleSpecifier::from_file_path(target_file).unwrap();

        // Create a new Tanxium runtime instance
        let mut tanxium = Tanxium::new(TanxiumOptions {
            main_module: main_module.clone(),
            cwd: cwd.to_string_lossy().to_string(),
            extensions: vec![],
            mode: WorkerExecutionMode::None,
        })
        .unwrap();

        // load runtime apis
        match tanxium.load_runtime_api(None).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        };

        // run main module
        match tanxium.execute_main_module(&main_module).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        };

        // run event loop
        match tanxium.run_event_loop(false).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        }
    };

    run_current_thread(future);
}
