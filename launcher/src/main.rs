use std::path::Path;

use prism::{Mesh, PrismRenderer, Vertex, math::{Matrix4, Vector2, Vector3, Vector4}};

fn main() {
    let mut ctx = PrismRenderer::new();

    match ctx.init() {
        Ok(()) => {
            let vertices = vec![
                Vertex { position: Vector3::new(-0.5, -0.5, 0.0), normal: Vector3::new(1.0, 1.0, 1.0), tex_coords: Vector2::new(0.0, 0.0) },
                Vertex { position: Vector3::new(-0.5, 0.5, 0.0), normal: Vector3::new(1.0, 1.0, 1.0), tex_coords: Vector2::new(0.0, 1.0) },
                Vertex { position: Vector3::new(0.5, 0.5, 0.0), normal: Vector3::new(1.0, 1.0, 1.0), tex_coords: Vector2::new(1.0, 1.0) },
                Vertex { position: Vector3::new(0.5, -0.5, 0.0), normal: Vector3::new(1.0, 1.0, 1.0), tex_coords: Vector2::new(1.0, 0.0) },
            ];

            let indices = vec![
                0, 1, 2, 2, 0, 3
            ];

            let texture = ctx.create_texture(&Path::new("/Users/alula/Desktop/shiota-os/launcher/src/texture.jpeg"));

            let triangle_mesh = ctx.create_mesh(
                vertices,
                indices,
                vec![&texture],
            );

            let vertex_shader_src = include_str!("shaders/triangle.vert");
            let fragment_shader_src = include_str!("shaders/triangle.frag");

            let shader = ctx.create_shader_from_source(vertex_shader_src, fragment_shader_src).unwrap();

            let upper_transform = Matrix4::identity();
            let lower_transform = Matrix4::identity();

            while !ctx.should_close() {
                ctx.handle_events();
                ctx.begin_upper_screen();
                ctx.clear_screen(Vector4::one());
                ctx.draw_mesh(&triangle_mesh, &shader);
                ctx.end_upper_screen();
                ctx.begin_lower_screen();
                ctx.clear_screen(Vector4::one());
                ctx.draw_mesh(&triangle_mesh, &shader);
                ctx.end_lower_screen();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenGL2DRenderer: {}", e);
        }
    }
}