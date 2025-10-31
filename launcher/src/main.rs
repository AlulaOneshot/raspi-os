use shgl::ShGLContext;

fn main() {
    let mut ctx = ShGLContext::new();
    match ctx.init() {
        Ok(()) => {
            // Initialization succeeded, proceed with the rest of the program.
        }
        Err(e) => {
            eprintln!("Failed to initialize ShGLContext: {}", e);
            std::process::exit(1);
        }
    }
}