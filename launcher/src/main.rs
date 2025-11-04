use std::path::Path;

use prismatica::{backends::opengl2d::OpenGL2DRenderer, renderer::Renderer};

fn main() {
    let mut ctx = OpenGL2DRenderer::new();

    match ctx.init() {
        Ok(()) => {
            while !ctx.should_close() {
                ctx.handle_events();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenGL2DRenderer: {}", e);
        }
    }
}