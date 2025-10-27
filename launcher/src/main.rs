fn main() {
    let mut context = shgl::ShiotaGLContext::new();
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