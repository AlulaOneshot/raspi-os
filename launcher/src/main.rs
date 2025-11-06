use std::path::Path;

use prism::{PrismRenderer, Vertex, glm::{self, Mat4, Vec2, Vec3, Vec4}};

fn main() {
    let mut ctx = PrismRenderer::new();

    match ctx.init() {
        Ok(()) => {
            let vertices = vec![
                Vertex { position: Vec3::new(-0.5, -0.5, 0.0), normal: Vec3::new(1.0, 1.0, 1.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, 0.0), normal: Vec3::new(1.0, 1.0, 1.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.0), normal: Vec3::new(1.0, 1.0, 1.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, 0.0), normal: Vec3::new(1.0, 1.0, 1.0), tex_coords: Vec2::new(1.0, 0.0) },
            ];

            let indices = vec![
                0, 1, 2, 2, 0, 3
            ];

            let texture = ctx.create_texture(&Path::new("/Users/alula/Desktop/shiota-os/launcher/src/texture.png"));

            let triangle_mesh = ctx.create_mesh(
                vertices,
                indices,
                vec![&texture],
            );

            let vertex_shader_src = include_str!("shaders/triangle.vert");
            let fragment_shader_src = include_str!("shaders/triangle.frag");

            let mut shader = ctx.create_shader_from_source(vertex_shader_src, fragment_shader_src).unwrap();

            let mut rotation = 90.0f32;

            let upper_transform = Mat4::IDENTITY * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5));
            let lower_transform = Mat4::IDENTITY * Mat4::from_scale(Vec3::new(1.5, 1.5, 1.5));
            
            while !ctx.should_close() {
                ctx.handle_events();
                ctx.begin_upper_screen();
                ctx.clear_screen(Vec4::new(1.0, 1.0, 1.0, 1.0));
                shader.set_uniform_mat4("transform", upper_transform);
                ctx.draw_mesh(&triangle_mesh, &shader);
                ctx.end_upper_screen();
                ctx.begin_lower_screen();
                ctx.clear_screen(Vec4::new(1.0, 1.0, 1.0, 1.0));
                shader.set_uniform_mat4("transform", lower_transform);
                ctx.draw_mesh(&triangle_mesh, &shader);
                ctx.end_lower_screen();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenGL2DRenderer: {}", e);
        }
    }
}