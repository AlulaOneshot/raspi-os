use shgl::Color;

fn main() {
    let mut context = shgl::ShGLContext::new();
    match context.init() {
        Ok(_) => {
            while !context.should_close() {
                context.begin_drawing();
                context.begin_drawing_upper();
                context.clear_screen(Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0});
                context.draw_pixel(400, 240, Color {r: 1.0, g: 1.0, b: 1.0, a: 1.0});
                context.end_drawing_upper();
                context.begin_drawing_lower();
                context.clear_screen(Color {r: 1.0, g: 1.0, b: 1.0, a: 1.0});
                context.draw_pixel(400, 240, Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0});
                context.end_drawing_lower();
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize ShiotaGLContext: {}", e);
        }
    }
}