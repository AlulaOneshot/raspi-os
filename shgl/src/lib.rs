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

    pub fn create_vbo(&mut self) -> Vbo {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
        }
        Vbo(vbo_id)
    }

    pub fn create_vao(&mut self, attribute_count: u32) -> Vao {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }
        Vao(vao_id, attribute_count)
    }
}