use std::{path::Path, sync::Mutex, time::Instant};

use prism::{Camera, Key, PrismRenderer, Vertex, glm::{self, Mat4, Vec2, Vec3, Vec4}};

fn main() {
    let mut ctx = PrismRenderer::new();

    match ctx.init() {
        Ok(()) => {
            let vertices = vec![
                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(0.5, -0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(0.5, 0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, 0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(0.0, 0.0, -1.0), tex_coords: Vec2::new(0.0, 0.0) },
                
                Vertex { position: Vec3::new(-0.5, -0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(0.5, -0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, 0.5), normal: Vec3::new(0.0, 0.0, 1.0), tex_coords: Vec2::new(0.0, 0.0) },

                Vertex { position: Vec3::new(-0.5, 0.5, 0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, -0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, 0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, 0.5), normal: Vec3::new(-1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },

                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(0.5, 0.5, -0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, -0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, -0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, 0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(1.0, 0.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },

                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, -0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, -0.5, 0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(0.5, -0.5, 0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, 0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, -0.5, -0.5), normal: Vec3::new(0.0, -1.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                
                Vertex { position: Vec3::new(-0.5, 0.5, -0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
                Vertex { position: Vec3::new(0.5, 0.5, -0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(1.0, 1.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(0.5, 0.5, 0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(1.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, 0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(0.0, 0.0) },
                Vertex { position: Vec3::new(-0.5, 0.5, -0.5), normal: Vec3::new(0.0, 1.0, 0.0), tex_coords: Vec2::new(0.0, 1.0) },
            ];

            let indices = vec![
                0, 1, 2, 3, 4, 5,
                6, 7, 8, 9, 10, 11,
                12, 13, 14, 15, 16, 17,
                18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29,
                30, 31, 32, 33, 34, 35,
            ];

            let texture = ctx.create_texture(&Path::new("/Users/alula/Desktop/shiota-os/launcher/src/texture.png"));

            let mut triangle_mesh = ctx.create_mesh(
                vertices,
                indices,
                vec![texture],
            );

            let vertex_shader_src = include_str!("shaders/vertex.vert");
            let fragment_shader_src = include_str!("shaders/fragment.frag");

            let mut shader = ctx.create_shader_from_source(vertex_shader_src, fragment_shader_src).unwrap();

            let mut camera = Camera::new(
                Vec3::new(0.0, 0.0, 8.0),
                Vec3::new(0.0, 1.0, 0.0),
                -90.0f32.to_radians(),
                0.0f32.to_radians(),
            );

            let projection_transform = glm::Mat4::perspective_rh(45.0f32.to_radians(), 800.0 / 480.0, 0.1, 100.0);

            while !ctx.should_close() {
                let color = Vec4::new(0.0, 0.0, 0.0, 1.0);
                if ctx.key_pressed(Key::W) {
                    camera.adjust_z(-10.0 * ctx.get_delta());
                }
                if ctx.key_pressed(Key::S) {
                    camera.adjust_z(10.0 * ctx.get_delta());
                }
                if ctx.key_pressed(Key::A) {
                    camera.adjust_x(-10.0 * ctx.get_delta());
                }
                if ctx.key_pressed(Key::D) {
                    camera.adjust_x(10.0 * ctx.get_delta());
                }
                if ctx.key_pressed(Key::Q) {
                    camera.adjust_y(10.0 * ctx.get_delta());
                }
                if ctx.key_pressed(Key::E) {
                    camera.adjust_y(-10.0 * ctx.get_delta());
                }
                triangle_mesh.set_rotation_x(ctx.get_time() as f32 * 25.0f32.to_radians());
                triangle_mesh.set_rotation_y(ctx.get_time() as f32 * 50.0f32.to_radians());
                triangle_mesh.set_rotation_z(ctx.get_time() as f32 * 50.0f32.to_radians());
                triangle_mesh.set_scale(Vec3::new(5.0, 5.0, 5.0));
                ctx.handle_events();
                ctx.begin_upper_screen();
                ctx.clear_screen(color);
                shader.set_uniform_mat4("view", camera.get_view_matrix());
                shader.set_uniform_mat4("projection", projection_transform);
                shader.set_uniform_vec3("viewPos", camera.get_position());
                shader.set_uniform_vec3("material.ambient", Vec3::new(0.725, 0.949, 1.0));
                shader.set_uniform_vec3("material.diffuse", Vec3::new(0.745, 0.949, 1.0));
                shader.set_uniform_vec3("material.specular", Vec3::new(0.5, 0.5, 0.5));
                shader.set_uniform_float("material.shininess", 32.0);
                shader.set_uniform_vec3("light.position", Vec3::new(1.2, 1.0, 8.0));
                shader.set_uniform_vec3("light.ambient", Vec3::new(0.2, 0.2, 0.2));
                shader.set_uniform_vec3("light.diffuse", Vec3::new(0.5, 0.5, 0.5));
                shader.set_uniform_vec3("light.specular", Vec3::new(1.0, 1.0, 1.0));
                ctx.draw_mesh(&triangle_mesh, &mut shader);
                ctx.end_upper_screen();
                ctx.begin_lower_screen();
                ctx.clear_screen(color);
                shader.set_uniform_mat4("view", camera.get_view_matrix());
                shader.set_uniform_mat4("projection", projection_transform);
                shader.set_uniform_vec3("viewPos", camera.get_position());
                ctx.draw_mesh(&triangle_mesh, &mut shader);
                ctx.end_lower_screen();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenGL2DRenderer: {}", e);
        }
    }
}