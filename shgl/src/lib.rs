pub struct ShGLContext {

}

impl ShGLContext {
    pub fn new() -> Self {
        ShGLContext {

        }
    }

    pub fn init(&mut self) {
        let glfw = glfw::init(glfw::fail_on_error).unwrap();
    }
}