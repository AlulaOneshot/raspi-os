use glm::Vec3;
use sdl2::render::Canvas;

#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Copy, Clone)]
pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
}

#[derive(Copy, Clone)]
pub struct Sampler2D(u32);

pub struct ShaderProgram(u32);

pub struct ShGLDisplay {
    width: u32, // Should always be 800, but we keep track in case we make future models with different resolutions
    height: u32, // Should always be 480, but ditto as above
    gl_context: sdl2::video::GLContext,
    canvas: Canvas<sdl2::video::Window>,
}

pub struct ShGLContext {
    sdl2_context: Option<sdl2::Sdl>,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    upper_display: Option<ShGLDisplay>,
    lower_display: Option<ShGLDisplay>,
    event_pump: Option<sdl2::EventPump>,
    should_close: bool,
    current_camera: Option<Camera>,
}

impl ShGLContext {
    pub fn new() -> Self {
        ShGLContext {
            sdl2_context: None,
            video_subsystem: None,
            upper_display: None,
            lower_display: None,
            event_pump: None,
            should_close: false,
            current_camera: None,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let sdl = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl.video().map_err(|e| e.to_string())?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
        gl_attr.set_context_version(3, 0);
        gl_attr.set_accelerated_visual(true);
        gl_attr.set_double_buffer(true);

        let display_count = video_subsystem.num_video_displays().map_err(|e| e.to_string())?;
        if display_count != 2 {
            return Err(format!(
                "Expected 2 displays, found {}. Ensure both displays are connected.",
                display_count
            ));
        }

        let upper_display_bounds = video_subsystem.display_bounds(0)?;

        let upper_window = video_subsystem
            .window("Upper Display", upper_display_bounds.width(), upper_display_bounds.height())
            .position(upper_display_bounds.x(), upper_display_bounds.y())
            .fullscreen_desktop()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let upper_gl_context = upper_window
            .gl_create_context()
            .map_err(|e| e.to_string())?;

        let upper_canvas = upper_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        gl_attr.set_share_with_current_context(true);

        let lower_display_bounds = video_subsystem.display_bounds(1)?;

        let lower_window = video_subsystem
            .window("Lower Display", lower_display_bounds.width(), lower_display_bounds.height())
            .position(lower_display_bounds.x(), lower_display_bounds.y())
            .fullscreen_desktop()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let lower_gl_context = lower_window
            .gl_create_context()
            .map_err(|e| e.to_string())?;

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        let lower_canvas = lower_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let mut tmp = ShGLDisplay {
            width: 0,
            height: 0,
            gl_context: upper_gl_context,
            canvas: upper_canvas,
        };

        tmp.width = tmp.canvas.window().drawable_size().0;
        tmp.height = tmp.canvas.window().drawable_size().1;
        self.upper_display = Some(tmp);

        let mut tmp = ShGLDisplay {
            width: 0,
            height: 0,
            gl_context: lower_gl_context,
            canvas: lower_canvas,
        };

        tmp.width = tmp.canvas.window().drawable_size().0;
        tmp.height = tmp.canvas.window().drawable_size().1;
        self.lower_display = Some(tmp);

        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        self.sdl2_context = Some(sdl);
        self.video_subsystem = Some(video_subsystem);
        self.event_pump = Some(event_pump);

        Ok(())
    }

    pub fn deinit(&mut self) {
        self.event_pump = None;
        self.lower_display = None;
        self.upper_display = None;
        self.video_subsystem = None;
        self.sdl2_context = None;
    }

    pub fn should_close(&mut self) -> bool {
        self.should_close
    }

    pub fn clear_screen(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn begin_drawing(&mut self) {
        if let Some(event_pump) = &mut self.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        self.should_close = true;
                    }
                    sdl2::event::Event::Window { timestamp: _, window_id: _, win_event } => {
                        match win_event {
                            sdl2::event::WindowEvent::Close => {
                                self.should_close = true;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn begin_drawing_upper(&mut self) {
        if let Some(upper_display) = &self.upper_display {
            upper_display.canvas.window().gl_make_current(&upper_display.gl_context).unwrap();
        }
    }

    pub fn end_drawing_upper(&mut self) {
        if let Some(upper_display) = &mut self.upper_display {
            upper_display.canvas.window().gl_swap_window();
        }
        if let Some(_) = &self.current_camera {
            self.current_camera = Some(Camera { position: Vec3::new(0.0, 0.0, 0.0), target: Vec3::new(0.0, 0.0, -1.0), up: Vec3::new(0.0, 1.0, 0.0) });
        }
    }

    pub fn begin_drawing_lower(&mut self) {
        if let Some(lower_display) = &self.lower_display {
            lower_display.canvas.window().gl_make_current(&lower_display.gl_context).unwrap();
        }
    }

    pub fn end_drawing_lower(&mut self) {
        if let Some(lower_display) = &mut self.lower_display {
            lower_display.canvas.window().gl_swap_window();
        }
        if let Some(_) = self.current_camera {
            self.current_camera = Some(Camera { position: Vec3::new(0.0, 0.0, 0.0), target: Vec3::new(0.0, 0.0, -1.0), up: Vec3::new(0.0, 1.0, 0.0) });
        }
    }

    pub fn apply_camera(&mut self, camera: Camera) {
        self.current_camera = Some(camera);
    }

    pub fn create_shader(&mut self, vertex_src: &str, fragment_src: &str) -> Result<ShaderProgram, String> {
        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let c_str_vert = std::ffi::CString::new(vertex_src.as_bytes()).map_err(|e| e.to_string())?;
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error = std::ffi::CString::from_vec_unchecked(buffer);;
                gl::GetShaderInfoLog(vertex_shader, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
                return Err(format!("Vertex shader compilation failed: {}", error.to_string_lossy()));
            }
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let c_str_frag = std::ffi::CString::new(fragment_src.as_bytes()).map_err(|e| e.to_string())?;
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error = std::ffi::CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(fragment_shader, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
                return Err(format!("Fragment shader compilation failed: {}", error.to_string_lossy()));
            }
        }

        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error = std::ffi::CString::from_vec_unchecked(buffer);
                gl::GetProgramInfoLog(shader_program, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
                return Err(format!("Shader program linking failed: {}", error.to_string_lossy()));
            }
        }

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(ShaderProgram(vertex_shader))
    }

    pub fn delete_shader(&mut self, shader: ShaderProgram) {
        unsafe {
            gl::DeleteProgram(shader.0);
        }
    }

    // Note: All uniform setters assume the shader program is already in use
    pub fn set_shader_uniform_f(&mut self, shader: &ShaderProgram, name: &str, value: f32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_shader_uniform_vec2(&mut self, shader: &ShaderProgram, name: &str, value: glm::Vec2) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform2f(location, value.x, value.y);
        }
    }

    pub fn set_shader_uniform_vec3(&mut self, shader: &ShaderProgram, name: &str, value: glm::Vec3) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_shader_uniform_vec4(&mut self, shader: &ShaderProgram, name: &str, value: glm::Vec4) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_shader_uniform_i(&mut self, shader: &ShaderProgram, name: &str, value: i32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_shader_uniform_ivec2(&mut self, shader: &ShaderProgram, name: &str, value: glm::IVec2) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform2i(location, value.x, value.y);
        }
    }

    pub fn set_shader_uniform_ivec3(&mut self, shader: &ShaderProgram, name: &str, value: glm::IVec3) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform3i(location, value.x, value.y, value.z);
        }
    }

    pub fn set_shader_uniform_ivec4(&mut self, shader: &ShaderProgram, name: &str, value: glm::IVec4) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform4i(location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_shader_uniform_f_array(&mut self, shader: &ShaderProgram, name: &str, values: &[f32]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform1fv(location, values.len() as i32, values.as_ptr());
        }
    }

    pub fn set_shader_uniform_vec2_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::Vec2]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform2fv(location, values.len() as i32, values.as_ptr() as *const f32);
        }
    }

    pub fn set_shader_uniform_vec3_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::Vec3]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform3fv(location, values.len() as i32, values.as_ptr() as *const f32);
        }
    }

    pub fn set_shader_uniform_vec4_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::Vec4]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform4fv(location, values.len() as i32, values.as_ptr() as *const f32);
        }
    }

    pub fn set_shader_uniform_i_array(&mut self, shader: &ShaderProgram, name: &str, values: &[i32]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform1iv(location, values.len() as i32, values.as_ptr());
        }
    }

    pub fn set_shader_uniform_ivec2_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::IVec2]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform2iv(location, values.len() as i32, values.as_ptr() as *const i32);
        }
    }

    pub fn set_shader_uniform_ivec3_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::IVec3]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform3iv(location, values.len() as i32, values.as_ptr() as *const i32);
        }
    }

    pub fn set_shader_uniform_ivec4_array(&mut self, shader: &ShaderProgram, name: &str, values: &[glm::IVec4]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::Uniform4iv(location, values.len() as i32, values.as_ptr() as *const i32);
        }
    }

    pub fn set_shader_uniform_mat4(&mut self, shader: &ShaderProgram, name: &str, value: &glm::Mat4) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(shader.0, c_name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_array().as_ptr() as *const f32);
        }
    }
}