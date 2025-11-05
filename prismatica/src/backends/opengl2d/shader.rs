use crate::{math::Matrix4, shader::Shader};

pub struct OpenGL2DShader {
    id: u32,
}

impl OpenGL2DShader {
    pub fn from_source(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        let vertex_shader = unsafe {gl::CreateShader(gl::VERTEX_SHADER)};
        let c_vertex_src = std::ffi::CString::new(vertex_src).map_err(|e| e.to_string())?;
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &c_vertex_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);
        }

        let mut success = 0;
        unsafe { gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success); }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;    
            unsafe {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr() as *mut i8,
                );
                v.set_len(log_len as usize);
            }
            return Err("Vertex Shader Compilation Failed: ".to_string() + &String::from_utf8_lossy(&v));
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let c_fragment_src = std::ffi::CString::new(fragment_src).map_err(|e| e.to_string())?;
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &c_fragment_src.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);
        }

        success = 0;
        unsafe { gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success); }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;    
            unsafe {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr() as *mut i8,
                );
                v.set_len(log_len as usize);
            }
            return Err("Fragment Shader Compilation Failed: ".to_string() + &String::from_utf8_lossy(&v));
        }

        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }
        success = 0;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = Vec::with_capacity(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
                buffer.set_len(len as usize);
            }

            println!("Shader Program Linking Error Log: {}", String::from_utf8_lossy(&buffer));
            return Err(String::from_utf8_lossy(&buffer).to_string());
        }

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(OpenGL2DShader { id: program })
    }

    /// Make the shader active
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        self.bind();
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value as i32,
            );
        }
        self.unbind();
    }

    pub fn set_uniform_int(&self, name: &str, value: i32) {
        self.bind();
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
        self.unbind();
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        self.bind();
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
        self.unbind();
    }
    
    pub fn set_uniform_mat4(&self, name: &str, mat: &Matrix4) {
        self.bind();
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                gl::FALSE,
                mat.to_array().as_ptr() as *const f32,
            );
        }
        self.unbind();
    }
}

impl Shader for OpenGL2DShader {
    
}