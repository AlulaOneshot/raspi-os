use glfw::{ClientApiHint, Context, Glfw, Monitor, OpenGlProfileHint, WindowHint, fail_on_errors};

pub struct ShGLWindow {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

pub struct ShGLContext {
    glfw_ctx: Option<Glfw>,
    upper_window: Option<ShGLWindow>,
    lower_window: Option<ShGLWindow>,
}

impl ShGLContext {
    pub fn new() -> Self {
        ShGLContext {
            glfw_ctx: None,
            upper_window: None,
            lower_window: None,
        }
    }

    pub fn init(&mut self) {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::OpenGlEs));
        glfw.window_hint(WindowHint::ContextVersion(3, 0));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Resizable(false));
        
        // We're gonna safely extract the windows in fullscreen from glfw.with_connected_monitors.
        let mut upper_display = None;
        let mut upper_display_events = None;

        let mut lower_display = None;
        let mut lower_display_events = None;

        glfw.with_connected_monitors(|s, m| {
            if m.len() < 2 {
                panic!("At least two monitors are required for ShGL");
            }

            let mut tmp = s.create_window(800, 480, "Upper Display", glfw::WindowMode::FullScreen(m[0])).unwrap();

            tmp.0.make_current();
            tmp.0.set_key_polling(true);
            let mut size = tmp.0.get_framebuffer_size();

            unsafe {
                gl::load_with(|s| tmp.0.get_proc_address(s).unwrap() as *const _);
                gl::Viewport(0, 0, size.0, size.1);
            }

            upper_display = Some(tmp.0);
            upper_display_events = Some(tmp.1);

            tmp = upper_display.as_ref().unwrap().create_shared(800, 480, "Lower Display", glfw::WindowMode::FullScreen(m[1])).unwrap();
            
            tmp.0.make_current();
            tmp.0.set_key_polling(true);
            size = tmp.0.get_framebuffer_size();

            unsafe {
                gl::Viewport(0, 0, size.0, size.1);
            }

            s.make_context_current(None);

            lower_display = Some(tmp.0);
            lower_display_events = Some(tmp.1);
        });

        self.glfw_ctx = Some(glfw);
        self.upper_window = Some(ShGLWindow {
            window: upper_display.unwrap(),
            events: upper_display_events.unwrap(),
        });
        self.lower_window = Some(ShGLWindow {
            window: lower_display.unwrap(),
            events: lower_display_events.unwrap(),
        });

        
    }
}