use std::path::Path;

use prismatica::{Color, backends::opengl2d::{OpenGL2DRenderer, mesh::{Mesh, OpenGLVertex, Texture}, shader::OpenGL2DShader}, math::Matrix4, renderer::Renderer};

fn main() {
    let mut ctx = OpenGL2DRenderer::new();

    println!("Initializing OpenGL context...");
    match ctx.init() {
        Ok(()) => {
            println!("OpenGL context initialized successfully");
            let vertices = vec![
                OpenGLVertex {
                    position: [-0.5, -0.5, 0.0],
                    normal: [1.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                OpenGLVertex {
                    position: [-0.5, 0.5, 0.0],
                    normal: [1.0, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                OpenGLVertex {
                    position: [0.5, 0.5, 0.0],
                    normal: [1.0, 0.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
                OpenGLVertex {
                    position: [0.5, -0.5, 0.0],
                    normal: [1.0, 0.0, 0.0],
                    tex_coords: [1.0, 0.0],
                },
            ];

            let indices = vec![
                0, 1, 2, 2, 0, 3
            ];
            
            println!("Loading texture...");
            let texture = Texture::from_file(&Path::new("/Users/alula/Desktop/shiota-os/launcher/src/texture.jpeg"));
            println!("Creating mesh...");
            
            let triangle_mesh = Mesh::new(
                vertices,
                indices,
                vec![texture],
            );
            println!("Mesh created successfully");

            let vertex_shader_src = include_str!("shaders/triangle.vert");
            let fragment_shader_src = include_str!("shaders/triangle.frag");

            let shader = OpenGL2DShader::from_source(vertex_shader_src, fragment_shader_src).unwrap();

            // Create transforms that will make the triangle visible
            // Create transforms
            let upper_transform = Matrix4::identity();
            let lower_transform = Matrix4::identity();

            shader.bind();
            println!("Starting render loop");

            while !ctx.should_close() {
                ctx.handle_events();
                ctx.begin_upper_screen();
                println!("Rendering upper screen");
                // Use a bright color to make sure we can see it
                ctx.clear_screen(Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
                shader.bind(); // Ensure shader is bound before setting uniforms
                shader.set_uniform_mat4("transform", &upper_transform);
                ctx.draw_mesh(&triangle_mesh, &shader);
                println!("Drew mesh on upper screen");
                ctx.end_upper_screen();
                ctx.begin_lower_screen();
                ctx.clear_screen(Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
                shader.bind(); // Ensure shader is bound before setting uniforms
                shader.set_uniform_mat4("transform", &lower_transform);
                ctx.draw_mesh(&triangle_mesh, &shader);
                ctx.end_lower_screen();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenGL2DRenderer: {}", e);
        }
    }
}