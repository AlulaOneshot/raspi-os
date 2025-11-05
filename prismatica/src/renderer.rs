use crate::Color;

pub trait Renderer {
    fn init(&mut self) -> Result<(), String>;
    fn deinit(&mut self);
    fn handle_events(&mut self);
    fn should_close(&self) -> bool;
    fn begin_upper_screen(&mut self);
    fn end_upper_screen(&mut self);
    fn begin_lower_screen(&mut self);
    fn end_lower_screen(&mut self);
    fn clear_screen(&mut self, color: Color);
}