use boa_engine::*;
use std::io::Write;
use tanxium::tanxium;

fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let is_typescript = file.ends_with(".ts");
    let code = std::fs::read_to_string(file).expect("Unable to read file");

    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Enable builtin APIs
    let builtins = tanxium::TanxiumBuiltinsExposure {
        crypto: true,
        performance: true,
        runtime: true,
        console: true,
    };

    // Initialize Tanxium options
    let options = tanxium::TanxiumOptions {
        cwd,
        typescript: true,
        builtins,
        global_object_name: "Tanxium".to_string(),
    };

    // Create a new Tanxium runtime
    let mut tanxium = tanxium::Tanxium::new(options).unwrap();

    // initialize tanxium's runtime APIs
    tanxium.init_runtime_apis().unwrap();
    tanxium.load_default_extensions().unwrap();

    // add custom native functions
    let ctx = &mut tanxium.context;
    ctx.register_global_builtin_callable(
        js_string!("prompt"),
        1,
        NativeFunction::from_fn_ptr(|_, args, context| {
            let src = args.get_or_undefined(0).to_string(context).unwrap();
            let message = src.to_std_string().unwrap();

            print!("{}: ", message);

            std::io::stdout().flush().unwrap();

            let mut input = String::new();

            match std::io::stdin().read_line(&mut input) {
                Ok(_) => Ok(JsValue::from(JsString::from(input.trim_end()))),
                Err(e) => Err(JsError::from_native(
                    JsNativeError::error().with_message(e.to_string()),
                )),
            }
        }),
    )
    .expect("Failed to register prompt function");

    // transpile if needed
    let code = if is_typescript {
        tanxium
            .transpile(code.as_str())
            .expect("Failed to transpile typescript")
    } else {
        code
    };

    // Execute the code
    tanxium.execute(code.as_str()).unwrap();
}
