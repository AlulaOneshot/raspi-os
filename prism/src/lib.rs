use std::{ffi::{CStr, CString}, str::FromStr, time::Instant};

use glam::{Mat4, Vec3};
use glfw::{Context, PWindow};

pub use glam as glm;
pub use glfw::Key;

use crate::mesh::Mesh;
pub mod mesh;

pub struct PrismWindow {
    window: Option<PWindow>,
    events: Option<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>,
}

impl PrismWindow {
    pub fn create(glfw: &mut glfw::Glfw, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let (window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| "Failed to create GLFW window".to_string())?;
        
        Ok(PrismWindow {
            window: Some(window),
            events: Some(events),
        })
    }

    pub fn create_shared(&mut self, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let (window, events) = self.window.as_mut().unwrap().create_shared(width, height, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| "Failed to create shared GLFW window".to_string())?;

        Ok(PrismWindow {
            window: Some(window),
            events: Some(events),
        })
    }

    fn make_current(&mut self) {
        self.window.as_mut().unwrap().make_current();
    }
}

pub struct PrismRenderer {
    glfw: Option<glfw::Glfw>,
    should_close: bool,
    initialized: bool,
    upper_window: Option<PrismWindow>,
    lower_window: Option<PrismWindow>,
    delta_time: f32,
    delta_instant: Instant,
}

impl PrismRenderer {
    pub fn new() -> Self {
        Self {
            glfw: None,
            should_close: false,
            initialized: false,
            upper_window: None,
            lower_window: None,
            delta_time: 0.0,
            delta_instant: Instant::now(),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("PrismRenderer is already initialized".to_string());
        }

        let mut glfw = glfw::init(glfw::fail_on_errors).map_err(|e| e.to_string())?;
        
        // glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        // glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let mut upper_window = PrismWindow::create(&mut glfw, "Prism Upper Window", 800, 480)?;

        upper_window.make_current();

        gl_loader::init_gl();
        gl::load_with(|s| gl_loader::get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, 800, 480);
            gl::Enable(gl::DEPTH_TEST);
        }

        let mut lower_window = upper_window.create_shared("Prism Lower Window", 800, 480)?;

        lower_window.make_current();

        unsafe {
            gl::Viewport(0, 0, 800, 480);
            gl::Enable(gl::DEPTH_TEST);
        }
        
        glfw.make_context_current(None);

        self.glfw = Some(glfw);
        self.upper_window = Some(upper_window);
        self.lower_window = Some(lower_window);
        self.initialized = true;
        Ok(())
    }

    pub fn deinit(&mut self) {
        if !self.initialized {
            return;
        }

        gl_loader::end_gl();

        self.glfw = None;
        self.upper_window = None;
        self.lower_window = None;
        self.initialized = false;
    }

    pub fn create_texture(&mut self, path: &std::path::Path) -> Texture {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before creating textures");
        }
    
        match self.upper_window {
            Some(ref mut window) => {
                window.make_current();
            }
            None => panic!("Upper window is not initialized"),
        }

        // Load image using the image crate
        let img = image::open(path).expect("Failed to load texture").flipv();
        let data = img.to_rgba8();
        let (width, height) = (img.width(), img.height());

        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            match self.glfw {
                Some(ref mut glfw) => {
                    glfw.make_context_current(None);
                }
                None => panic!("GLFW is not initialized"),
            }

            Texture { id: texture }
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn handle_events(&mut self) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before handling events");
        }

        if let Some(glfw) = &mut self.glfw {
            glfw.poll_events();

            if let Some(upper_window) = &mut self.upper_window {
                if let Some(events) = &mut upper_window.events {
                    for (_, event) in glfw::flush_messages(events) {
                        match event {
                            glfw::WindowEvent::Close => {
                                self.should_close = true;
                            }
                            _ => {}
                        }
                    }
                }

                if let Some(window) = upper_window.window.as_mut() {
                    if window.should_close() {
                        self.should_close = true;
                    }
                }
            }

            if let Some(lower_window) = &mut self.lower_window {
                if let Some(events) = &mut lower_window.events {
                    for (_, event) in glfw::flush_messages(events) {
                        match event {
                            glfw::WindowEvent::Close => {
                                self.should_close = true;
                            },
                            _ => {}
                        }
                    }
                }

                if let Some(window) = lower_window.window.as_mut() {
                    if window.should_close() {
                        self.should_close = true;
                    }
                }
            }
        }
    }

    pub fn begin_upper_screen(&mut self) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before beginning upper screen");
        }

        self.delta_instant = Instant::now();

        if let Some(upper_window) = &mut self.upper_window {
            upper_window.make_current();
        }
    }

    pub fn end_upper_screen(&mut self) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before ending upper screen");
        }

        if let Some(upper_window) = &mut self.upper_window {
            if let Some(window) = &mut upper_window.window {
                window.swap_buffers();
            }
        }

        self.delta_time = self.delta_instant.elapsed().as_secs_f32();

        if let Some(glfw) = &mut self.glfw {
            glfw.make_context_current(None);
        }
    }

    pub fn begin_lower_screen(&mut self) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before beginning lower screen");
        }

        self.delta_instant = Instant::now();

        if let Some(lower_window) = &mut self.lower_window {
            lower_window.make_current();
        }
    }

    pub fn end_lower_screen(&mut self) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before ending lower screen");
        }

        if let Some(lower_window) = &mut self.lower_window {
            if let Some(window) = &mut lower_window.window {
                window.swap_buffers();
            }
        }

        self.delta_time = self.delta_instant.elapsed().as_secs_f32();

        if let Some(glfw) = &mut self.glfw {
            glfw.make_context_current(None);
        }
    }

    pub fn clear_screen(&mut self, color: glam::Vec4) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before clearing screen");
        }

        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn create_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before creating meshes");
        }

        match self.upper_window {
            Some(ref mut window) => {
                window.make_current();
            }
            None => panic!("Upper window is not initialized"),
        }
        
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            let converted_vertices: Vec<f32> = vertices.iter().flat_map(|v| {
                vec![
                    v.position.x, v.position.y, v.position.z,
                    v.normal.x, v.normal.y, v.normal.z,
                    v.tex_coords.x, v.tex_coords.y,
                ]
            }).collect();

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (converted_vertices.len() * std::mem::size_of::<f32>()) as isize,
                converted_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                (6 * std::mem::size_of::<f32>()) as *const std::ffi::c_void
            );

            gl::BindVertexArray(0);
        }

        match self.glfw {
            Some(ref mut glfw) => {
                glfw.make_context_current(None);
            }
            None => panic!("GLFW is not initialized"),
        }

        Mesh {
            vertices,
            indices,
            textures,
            model: Mat4::IDENTITY,
            vao,
            vbo,
            ebo,
            position: glam::Vec3::ZERO,
            rotation: glam::Vec3::ZERO,
            scale: glam::Vec3::ONE,
        }
    }

    pub fn create_shader_from_source(&mut self, vertex_src: &str, fragment_src: &str) -> Result<Shader, String> {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before creating shaders");
        }

        match self.upper_window {
            Some(ref mut window) => {
                window.make_current();
            }
            None => panic!("Upper window is not initialized"),
        }

        let mut vertex_shader: u32 = 0;
        let mut fragment_shader = 0;
        let mut shader = 0;

        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            let c_str_vert = std::ffi::CString::new(vertex_src.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);
            let mut success: i32 = 1;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut buffer: Vec<u8> = Vec::with_capacity(1024);

                gl::GetShaderInfoLog(vertex_shader, 1024, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                buffer.set_len(1024);
                let error_message = String::from_utf8_lossy(&buffer).into_owned();
                return Err(format!("Vertex shader compilation failed: {}", error_message));
            }

            let c_str_frag = std::ffi::CString::new(fragment_src.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);
            success = 1;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut buffer: Vec<u8> = Vec::with_capacity(1024);
                gl::GetShaderInfoLog(fragment_shader, 1024, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                buffer.set_len(1024);
                let error_message = String::from_utf8_lossy(&buffer).into_owned();
                return Err(format!("Fragment shader compilation failed: {}", error_message));
            }

            shader = gl::CreateProgram();
            gl::AttachShader(shader, vertex_shader);
            gl::AttachShader(shader, fragment_shader);
            gl::LinkProgram(shader);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        match self.glfw {
            Some(ref mut glfw) => {
                glfw.make_context_current(None);
            }
            None => panic!("GLFW is not initialized"),
        };

        Ok(Shader { id: shader })
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, shader: &mut Shader) {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before drawing meshes");
        }

        unsafe {
            gl::UseProgram(shader.id);

            for (i, texture) in mesh.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::Uniform1i(gl::GetUniformLocation(shader.id, format!("texture_{}", i).as_ptr() as *const i8), i as i32);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            shader.set_uniform_mat4("model", mesh.model);

            gl::BindVertexArray(mesh.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    pub fn get_time(&self) -> f64 {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before getting time");
        }

        match self.glfw {
            Some(ref glfw) => glfw.get_time(),
            None => panic!("GLFW is not initialized"),
        }
    }

    pub fn get_delta(&self) -> f32 {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before getting delta time");
        }

        self.delta_time
    }
    
    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        if !self.initialized {
            panic!("PrismRenderer must be initialized before checking key states");
        }

        let mut res = glfw::Action::Release;

        match self.upper_window {
            Some(ref window) => {
                if let Some(win) = &window.window {
                    if win.get_key(key) == glfw::Action::Press {
                        res = glfw::Action::Press;
                    }
                }
            }
            None => panic!("Upper window is not initialized"),
        }

        match self.lower_window {
            Some(ref window) => {
                if let Some(win) = &window.window {
                    if win.get_key(key) == glfw::Action::Press {
                        res = glfw::Action::Press;
                    }
                }
            }
            None => panic!("Lower window is not initialized"),
        }

        res == glfw::Action::Press
    }
}

impl Drop for PrismRenderer {
    fn drop(&mut self) {
        self.deinit();
    }
}

pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub tex_coords: glam::Vec2,
}

impl Vertex {
    pub fn new(position: glam::Vec3, normal: glam::Vec3, tex_coords: glam::Vec2) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
}

impl From<[f32; 8]> for Vertex {
    fn from(arr: [f32; 8]) -> Self {
        Self {
            position: glam::Vec3::new(arr[0], arr[1], arr[2]),
            normal: glam::Vec3::new(arr[3], arr[4], arr[5]),
            tex_coords: glam::Vec2::new(arr[6], arr[7]),
        }
    }
}

impl Into<[f32; 8]> for Vertex {
    fn into(self) -> [f32; 8] {
        [
            self.position.x,
            self.position.y,
            self.position.z,
            self.normal.x,
            self.normal.y,
            self.normal.z,
            self.tex_coords.x,
            self.tex_coords.y,
        ]
    }
}

pub struct Texture {
    pub id: u32
}

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn set_uniform_mat4(&mut self, name: &str, value: Mat4) {
        unsafe {
            gl::UseProgram(self.id);
            let name_c_str = CString::from_str(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_c_str.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ref().as_ptr())
        }
    }

    pub fn set_uniform_vec3(&mut self, name: &str, value: Vec3) {
        unsafe {
            gl::UseProgram(self.id);
            let name_c_str = CString::from_str(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_c_str.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_float(&mut self, name: &str, value: f32) {
        unsafe {
            gl::UseProgram(self.id);
            let name_c_str = CString::from_str(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_c_str.as_ptr());
            gl::Uniform1f(location, value);
        }
    }
}

pub struct Camera {
    position: glam::Vec3,
    front: glam::Vec3,
    up: glam::Vec3,
    right: glam::Vec3,
    world_up: glam::Vec3,
    /// in radians
    yaw: f32,
    /// in radians
    pitch: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(position: glam::Vec3, up: glam::Vec3, yaw_rad: f32, pitch_rad: f32) -> Self {        
        let mut x = Self {
            position,
            world_up: up,
            yaw: yaw_rad,
            pitch: pitch_rad,
            front: Vec3::ZERO,
            right: Vec3::ZERO,
            up: Vec3::ZERO,
            zoom: 45.0,
        };

        x.update_camera_vectors();
        x
    }

    fn update_camera_vectors(&mut self) {
        let mut front = Vec3::ZERO;
        front.x = self.yaw.cos() * self.pitch.cos();
        front.y = self.pitch.sin();
        front.z = self.yaw.sin() * self.pitch.cos();
        front = front.normalize();
        self.front = front;
        self.right = front.cross(self.world_up).normalize();
        self.up = self.right.cross(front).normalize();
    }
        
    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn adjust_x(&mut self, amount: f32) {
        self.position.x += amount;
        self.update_camera_vectors();
    }

    pub fn adjust_z(&mut self, amount: f32) {
        self.position.z += amount;
        self.update_camera_vectors();
    }
}