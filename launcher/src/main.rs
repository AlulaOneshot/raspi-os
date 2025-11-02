use shgl::{Color, ShGLContext, Vec3};

fn main() {
    let mut ctx = ShGLContext::new();
    match ctx.init() {
        Ok(()) => {
            while !ctx.should_close() {
                ctx.handle_events();
                ctx.begin_drawing_upper();
                ctx.clear_screen(Color::from_hex(0xFF0000FF));
                ctx.draw_triangle([Vec3::new(-1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(-1.0, -1.0, 0.0)], Color::from_hex(0x00FF00FF));
                ctx.end_drawing_upper();
                ctx.begin_drawing_lower();
                ctx.clear_screen(Color::from_hex(0x00FF00FF));
                ctx.draw_triangle([Vec3::new(-1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(-1.0, -1.0, 0.0)], Color::from_hex(0xFF0000FF));
                ctx.end_drawing_lower();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize ShGLContext: {}", e);
            std::process::exit(1);
        }
    }

    ctx.deinit();
}