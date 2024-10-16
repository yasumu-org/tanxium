use tanxium::tanxium::{Tanxium, TanxiumOptions};

#[tokio::main]
async fn main() {
    let cwd = std::env::current_dir().unwrap();
    let target_file = std::env::args().nth(1).expect("missing target file");
    let target_file = cwd.join(target_file);
    let main_module = deno_core::ModuleSpecifier::from_file_path(target_file).unwrap();

    let mut tanxium = Tanxium::new(TanxiumOptions {
        main_module: main_module.clone(),
        stderr: None,
        stdin: None,
        stdout: None,
        // stdout: Some(std::fs::File::create("stdout.log").unwrap()),
        // stderr: Some(std::fs::File::create("stderr.log").unwrap()),
        // stdin: Some(std::fs::File::open("stdin.log").unwrap()),
    });
    let runtime = &mut tanxium.runtime;
    runtime.execute_main_module(&main_module).await.unwrap();
    runtime.run_event_loop(false).await.unwrap();
}
