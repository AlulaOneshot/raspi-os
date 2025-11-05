use glfw::Context;

use crate::{backends::opengl2d::{shader::OpenGL2DShader, window::OpenGL2DWindow}, renderer::Renderer};

pub mod window;
pub mod mesh;
pub mod shader;

pub struct OpenGL2DRenderer {
    glfw: Option<glfw::Glfw>,
    upper_window: Option<OpenGL2DWindow>,
    lower_window: Option<OpenGL2DWindow>,
    initialized: bool,
    should_close: bool,
}

impl OpenGL2DRenderer {
    pub fn new() -> Self {
        OpenGL2DRenderer {
            glfw: None,
            upper_window: None,
            lower_window: None,
            initialized: false,
            should_close: false,
        }
    }
}

impl Renderer for OpenGL2DRenderer {
    fn init(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("OpenGL2DRenderer is already initialized".to_string());
        }

        let mut glfw = glfw::init(glfw::fail_on_errors).map_err(|e| e.to_string())?;

        // glfw.window_hint(glfw::WindowHint::ContextVersion(3, 0));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        // glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let mut upper_window = OpenGL2DWindow::create(&mut glfw, "OpenGL2D Upper Window", 800, 480)?;

        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s) as *const _);

        let lower_window = upper_window.create_shared("OpenGL2D Lower Window", 800, 480)?;

        upper_window.window.as_mut().unwrap().make_current();

        self.glfw = Some(glfw);
        self.upper_window = Some(upper_window);
        self.lower_window = Some(lower_window);
        self.initialized = true;

        Ok(())
    }

    fn deinit(&mut self) {
        self.upper_window = None;
        self.lower_window = None;
        self.glfw = None;
        self.initialized = false;
        gl_loader::end_gl();
    }

    fn handle_events(&mut self) {
        if let Some(glfw) = &mut self.glfw {
            glfw.poll_events();
        }

        if let Some(upper_window) = &mut self.upper_window {
            match upper_window.should_close() {
                Ok(true) => {
                    self.should_close = true;
                }
                Ok(false) => {}
                Err(_) => {}
            }

            
            match upper_window.get_events() {
                Ok(events) => {
                    for (_, event) in events {
                        match event {
                            glfw::WindowEvent::Close => {
                                self.should_close = true;
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {}
            }
        }

        if let Some(lower_window) = &mut self.lower_window {
            match lower_window.should_close() {
                Ok(true) => {
                    self.should_close = true;
                }
                Ok(false) => {}
                Err(_) => {}
            }

            match lower_window.get_events() {
                Ok(events) => {
                    for (_, event) in events {
                        match event {
                            glfw::WindowEvent::Close => {
                                self.should_close = true;
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn begin_upper_screen(&mut self) {
        if let Some(upper_window) = &mut self.upper_window {
            if let Some(window) = &mut upper_window.window {
                window.make_current();
            }
        }
    }

    fn begin_lower_screen(&mut self) {
        if let Some(lower_window) = &mut self.lower_window {
            if let Some(window) = &mut lower_window.window {
                window.make_current();
            }
        }
    }

    fn end_upper_screen(&mut self) {
        if let Some(upper_window) = &mut self.upper_window {
            if let Some(window) = &mut upper_window.window {
                window.swap_buffers();
            }
        }
    }

    fn end_lower_screen(&mut self) {
        if let Some(lower_window) = &mut self.lower_window {
            if let Some(window) = &mut lower_window.window {
                window.swap_buffers();
            }
        }
    }

    fn clear_screen(&mut self, color: crate::Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl OpenGL2DRenderer {
    pub fn draw_mesh(&mut self, mesh: &mesh::Mesh, shader: &OpenGL2DShader) {
        unsafe {
            shader.bind();

            let mut i = 0;
            for texture in &mesh.textures {
                gl::ActiveTexture(gl::TEXTURE0 + i);
                shader.set_uniform_int(format!("texture_{}", i).as_str(), i as i32);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
                i += 1;
            }
            gl::ActiveTexture(gl::TEXTURE0);

            gl::BindVertexArray(mesh.vao);

            gl::DrawElements(
                gl::TRIANGLES,
                mesh.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(0);
        }
    }
}