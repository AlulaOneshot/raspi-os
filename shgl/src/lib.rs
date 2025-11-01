use glfw::{ClientApiHint, Context, Glfw, Monitor, OpenGlProfileHint, WindowHint, fail_on_errors};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Creates a Color from RGBA values in the range 0-255.
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r: (r as f32) / 255.0, g: (g as f32) / 255.0, b: (b as f32) / 255.0, a: (a as f32) / 255.0 }
    }

    /// Creates a Color from RGB values in the range 0-255.
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r: (r as f32) / 255.0, g: (g as f32) / 255.0, b: (b as f32) / 255.0, a: 1.0 }
    }

    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as u8;
        Color::from_rgba(r, g, b, a)
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    position: glm::Vec3,
    direction: glm::Vec3,
    up: glm::Vec3,
}

impl Camera {
    /// Updates the camera position by adding the new_position vector to the current position.
    pub fn change_position(&mut self, new_position: glm::Vec3) {
        self.position.x += new_position.x;
        self.position.y += new_position.y;
        self.position.z += new_position.z;
    }

    /// Sets the camera position to the specified new_position vector.
    pub fn set_position(&mut self, new_position: glm::Vec3) {
        self.position = new_position;
    }

    /// Updates the camera direction based on the provided yaw and pitch angles (in degrees).
    pub fn update_direction(&mut self, yaw: f32, pitch: f32) {
        self.direction.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        self.direction.y = pitch.to_radians().sin();
        self.direction.z = yaw.to_radians().sin() * pitch.to_radians().cos();
        let right = glm::normalize(glm::cross(self.direction, glm::vec3(0.0, 1.0, 0.0)));
        self.up = glm::normalize(glm::cross(right, self.direction));
    }
}

pub struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn delete(self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }

    /// Note that you must bind the shader before setting uniforms.
    pub fn set_uniform_vec4(&self, name: &str, value: glm::Vec4) {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }
}

pub struct ShGLWindow {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

pub struct ShGLContext {
    glfw_ctx: Option<Glfw>,
    upper_window: Option<ShGLWindow>,
    lower_window: Option<ShGLWindow>,
    should_close: bool,
    current_camera: Option<Camera>,
}

impl ShGLContext {
    pub fn new() -> Self {
        ShGLContext {
            glfw_ctx: None,
            upper_window: None,
            lower_window: None,
            should_close: false,
            current_camera: None
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let mut glfw = glfw::init(glfw::fail_on_errors).map_err(|e| e.to_string())?;


        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::ContextVersion(3, 0));  
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::OpenGlEs));
        glfw.window_hint(WindowHint::Resizable(false));

        let (mut upper_window, upper_events) = glfw
            .create_window(800, 480, "Upper Window", glfw::WindowMode::Windowed)
            .ok_or("Failed to create upper window")?;

        upper_window.make_current();

        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, 800, 480);
        }

        let (mut lower_window, lower_events) = upper_window.create_shared(800, 480, "Lower Window", glfw::WindowMode::Windowed).unwrap();

        lower_window.make_current();

        unsafe {
            gl::Viewport(0, 0, 800, 480);
        }

        self.upper_window = Some(ShGLWindow { window: upper_window, events: upper_events });
        self.lower_window = Some(ShGLWindow { window: lower_window, events: lower_events });

        Ok(())
    }

    pub fn deinit(&mut self) {
        self.upper_window = None;
        self.lower_window = None;
        self.glfw_ctx = None;
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn is_initialized(&self) -> bool {
        self.glfw_ctx.is_some() && self.upper_window.is_some() && self.lower_window.is_some()
    }

    pub const fn get_screen_size(&mut self) -> (u32, u32) {
        (800, 480)
    }

    pub fn clear_screen(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn handle_events(&mut self) {
        if let Some(glfw) = &mut self.glfw_ctx {
            glfw.poll_events();

            if let Some(upper_window) = &mut self.upper_window {
                for (_, event) in glfw::flush_messages(&upper_window.events) {
                    match event {
                        glfw::WindowEvent::Close => {
                            self.should_close = true;
                        }
                        _ => {}
                    }
                }
            }

            if let Some(lower_window) = &mut self.lower_window {
                for (_, event) in glfw::flush_messages(&lower_window.events) {
                    match event {
                        glfw::WindowEvent::Close => {
                            self.should_close = true;
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn begin_drawing_upper(&mut self) {
        if let Some(upper_window) = &mut self.upper_window {
            upper_window.window.make_current();
        }
    }

    pub fn end_drawing_upper(&mut self) {
        if let Some(upper_window) = &mut self.upper_window {
            upper_window.window.swap_buffers();
        }
    }

    pub fn begin_drawing_lower(&mut self) {
        if let Some(lower_window) = &mut self.lower_window {
            lower_window.window.make_current();
        }
    }

    pub fn end_drawing_lower(&mut self) {
        if let Some(lower_window) = &mut self.lower_window {
            lower_window.window.swap_buffers();
        }
    }

    pub fn apply_camera(&mut self, camera: Camera) {
        self.current_camera = Some(camera);
    }
}
