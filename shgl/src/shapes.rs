use glm::{Vec3, Vec4, Vector4};

use crate::{Color, ShGLContext, Shader, Vao, Vbo};

impl<'a> ShGLContext<'a> {
    pub fn draw_triangle(&mut self, verticies: [Vec3; 3], color: Color) {
        let vbo: &Vbo = match &self.triangle_vbo {
            Some(vbo) => vbo,
            None => {
                eprintln!("Triangle VBO not initialized.");
                return;
            }
        };

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of::<[Vec3; 3]>() as isize,
                           verticies.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        }

        let shader: &Shader = match &self.current_shader {
            Some(shader) => *shader,
            None => {
                match &self.default_shader {
                    Some(default_shader) => default_shader,
                    None => {
                        eprintln!("No shader applied for drawing triangle.");
                        return;
                    }
                }
            }
        };

        shader.bind();
        shader.set_uniform_vec4("color", Vec4::new(color.r, color.g, color.b, color.a));

        vbo.bind();

        let vao: &Vao = match &self.triangle_vao {
            Some(vao) => vao,
            None => {
                eprintln!("Triangle VAO not initialized.");
                return;
            }
        };

        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(vao.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}