fn main() {
    let mut context = shgl::ShGLContext::new();
    match context.init() {
        Ok(_) => {
            loop {
                
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize ShiotaGLContext: {}", e);
        }
    }
}