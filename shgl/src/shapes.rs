use glm::{Vec2, Vec3, Vec4};

use crate::{Color, ShGLContext, Shader, Texture, Vao, Vbo};

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
    
    pub fn draw_triangle_textured(&mut self, verticies: [Vec3; 3], texture_coordinates: [Vec2; 3], texture: Texture) {
        let modified_verticies: [f32; 15] = [
            verticies[0].x, verticies[0].y, verticies[0].z, texture_coordinates[0].x, texture_coordinates[0].y,
            verticies[1].x, verticies[1].y, verticies[1].z, texture_coordinates[1].x, texture_coordinates[1].y,
            verticies[2].x, verticies[2].y, verticies[2].z, texture_coordinates[2].x, texture_coordinates[2].y,
        ];

        let vbo: &Vbo = match &self.triangle_textured_vbo {
            Some(vbo) => vbo,
            None => {
                eprintln!("Triangle Textured VBO not initialized.");
                return;
            }
        };

        let vao: &Vao = match &self.triangle_textured_vao {
            Some(vao) => vao,
            None => {
                eprintln!("Triangle Textured VAO not initialized.");
                return;
            }
        };

        let shader: &Shader = match &self.current_shader {
            Some(shader) => *shader,
            None => {
                match &self.default_textured_shader {
                    Some(default_shader) => default_shader,
                    None => {
                        eprintln!("No shader applied for drawing textured triangle.");
                        return;
                    }
                }
            }
        };

        vbo.bind();
        vao.bind();
        shader.bind();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of::<[f32; 15]>() as isize,
                           modified_verticies.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, 0 as *const std::ffi::c_void);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void);
            gl::EnableVertexAttribArray(1);
            
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}