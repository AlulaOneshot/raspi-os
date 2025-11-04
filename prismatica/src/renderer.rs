use crate::Color;

pub trait Renderer {
    fn init(&mut self) -> Result<(), String>;
    fn deinit(&mut self);
    fn handle_events(&mut self);
    fn should_close(&self) -> bool;
}