use crate::{Color, CurrentDisplay, ShGLContext};

impl ShGLContext {
    pub fn convert_coords_to_ndc_2d(&self, x: u32, y: u32) -> [f32; 2] {
        let w: u32;
        let h: u32;
        match self.current_display {
            CurrentDisplay::Lower => {
                w = self.lower_display.as_ref().unwrap().width;
                h = self.lower_display.as_ref().unwrap().height;
            }
            CurrentDisplay::Upper => {
                w = self.upper_display.as_ref().unwrap().width;
                h = self.upper_display.as_ref().unwrap().height;
            }
            CurrentDisplay::None => {
                panic!("No display selected for coordinate conversion");
            }
        }

        let ndc_x = ((x as f32)-((w as f32)/2.0))/((w as f32)/2.0);
        let ndc_y = -(((y as f32)-((h as f32)/2.0))/((h as f32)/2.0));
        [ndc_x, ndc_y]
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        let vertices: [f32; 2] = self.convert_coords_to_ndc_2d(x, y);

        let vbo = self.create_vbo();
        vbo.set_data(&vertices);
        vbo.bind();

        let vao = self.create_vao(1);
        vao.bind();

        let vertex_src = r#"#version 300 es
        layout(location = 0) in vec2 aPos;
        void main() {
            gl_Position = vec4(aPos, 0.0, 1.0);
            gl_PointSize = 1.0;
        }
        "#;

        let fragment_src = r#"#version 300 es
        precision mediump float;
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0, 0.0, 0.0, 1.0); // Set pixel color to red
        }
        "#;

        let shader_program = self.create_shader(vertex_src, fragment_src).unwrap();
        shader_program.bind();

        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::DrawArrays(gl::POINTS, 0, 1);
        }

        vao.unbind();
        vbo.unbind();
        shader_program.unbind();
    }
}