use imgui_glium_renderer::Renderer;
use glium::{ glutin, Display, Surface };
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase, EventsLoop,
                    ContextBuilder, WindowBuilder, GlWindow };
use imgui::{ ImGui };

use conf::Conf;
use mouse::Mouse;
use event;
use graphics;
use error::Issue;


pub struct Context {
    pub imgui: ImGui,
    pub glium_context: Display,
    pub imgui_context: Renderer,
    pub mouse_context: Mouse,
    pub hidpi_factor: f64,
    pub background_color: [f32; 4],
}

impl Context {
    pub fn new(conf: Conf) -> Issue<(Self, EventsLoop)> {
        let mut events_loop = EventsLoop::new();
        let context = ContextBuilder::new()
            .with_vsync(conf.vsync);
        let window = WindowBuilder::new()
            .with_title(conf.title)
            .with_dimensions(glutin::dpi::LogicalSize::new(conf.width, conf.height));
        let glium_context = Display::new(window, context, &events_loop).expect("Failed to create display");

        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);

        let imgui_context = Renderer::init(&mut imgui, &glium_context).expect("Failed to create renderer");
        let hidpi_factor = glium_context.gl_window().get_hidpi_factor().round();

        let mouse_context = Mouse::default();
        let background_color = [0.0, 0.0, 0.0, 1.0];

        Ok((Self {
            imgui,
            glium_context,
            imgui_context,
            mouse_context,
            hidpi_factor,
            background_color,
        },  events_loop))

    }

    pub fn get_hidpi_factor(&self) -> f64 {
        self.glium_context.gl_window().get_hidpi_factor()

    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.background_color = [r, g, b, a];
    }
}