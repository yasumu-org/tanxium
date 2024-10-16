use tanxium::tanxium::{Tanxium, TanxiumOptions};

fn main() {
    let future = async {
        let cwd = std::env::current_dir().unwrap();
        let target_file = std::env::args().nth(1).expect("missing target file");
        let target_file = cwd.join(target_file);
        let main_module = deno_core::ModuleSpecifier::from_file_path(target_file).unwrap();

        let mut tanxium = Tanxium::new(TanxiumOptions {
            main_module: main_module.clone(),
            cwd: cwd.to_string_lossy().to_string(),
            extensions: vec![],
            test: true,
            stdout: None,
            stderr: None,
            stdin: None,
        })
        .unwrap();

        match tanxium.load_runtime_api(None).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        };

        match tanxium.execute_main_module(&main_module).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        };

        match tanxium.run_event_loop(false).await {
            Err(e) => eprintln!("{}", e.to_string()),
            _ => (),
        }
    };

    tanxium::tanxium::run_current_thread(future);
}
