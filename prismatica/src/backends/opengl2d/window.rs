use glfw::{FlushedMessages, WindowEvent};

use crate::window::Window;

pub struct OpenGL2DWindow {
    pub(crate) window: Option<glfw::PWindow>,
    pub(crate) events: Option<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>,
}

impl OpenGL2DWindow {
    pub fn create(glfw: &mut glfw::Glfw, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let window = match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
            Some(win) => win,
            None => return Err("Failed to create OpenGL2DWindow".to_string()),
        };
        Ok(OpenGL2DWindow {
            window: Some(window.0),
            events: Some(window.1)
        })
    }

    pub fn create_shared(&mut self, title: &str, width: u32, height: u32) -> Result<OpenGL2DWindow, String> {
        let parent_window = match &self.window {
            Some(win) => win,
            None => return Err("Parent window is not initialized".to_string()),
        };

        let new_window = match parent_window.create_shared(
            width,
            height,
            title,
            glfw::WindowMode::Windowed,
        ) {
            Some(win) => win,
            None => return Err("Failed to create shared OpenGL2DWindow".to_string()),
        };

        Ok(OpenGL2DWindow {
            window: Some(new_window.0),
            events: Some(new_window.1),
        })
    }

    pub fn get_events(&mut self) -> Result<FlushedMessages<'_, (f64, WindowEvent)>, String> {
        let events = match &mut self.events {
            Some(ev) => ev,
            None => return Err("Failed to get events".to_string()),
        };

        Ok(glfw::flush_messages(events))
    }

    pub fn should_close(&self) -> Result<bool, String> {
        let window = match &self.window {
            Some(win) => win,
            None => return Err("Window is not initialized".to_string()),
        };

        Ok(window.should_close())
    }
}

impl Window for OpenGL2DWindow {
    fn deinit(&mut self) {
        self.window = None;
        self.events = None;
    }
}