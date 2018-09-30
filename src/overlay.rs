use imgui;
use State;
use context;
use glium::{ glutin, Frame, Surface };
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};
use glium::glutin::ElementState::Pressed;
use imgui::*;
use std::time;


pub struct Overlay {
    last_frame: time::Instant,
} 

impl Overlay {
    pub fn new() -> Self{
        let last_frame = time::Instant::now();

        Self {
            last_frame,
        }
    }

    pub fn update(&mut self, ctx: &mut context::Context, target: &mut Frame) {
        let now = time::Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        ctx.mouse_context.update(&mut ctx.imgui);

        let mouse_cursor = ctx.imgui.mouse_cursor();
        let window = ctx.glium_context.gl_window();

        if ctx.imgui.mouse_draw_cursor() || mouse_cursor == ImGuiMouseCursor::None {
            // Hide OS cursor
            window.hide_cursor(true);
        } else {
            // Set OS cursor
            window.hide_cursor(false);
            window.set_cursor(match mouse_cursor {
                ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
                ImGuiMouseCursor::Arrow => glutin::MouseCursor::Arrow,
                ImGuiMouseCursor::TextInput => glutin::MouseCursor::Text,
                ImGuiMouseCursor::Move => glutin::MouseCursor::Move,
                ImGuiMouseCursor::ResizeNS => glutin::MouseCursor::NsResize,
                ImGuiMouseCursor::ResizeEW => glutin::MouseCursor::EwResize,
                ImGuiMouseCursor::ResizeNESW => glutin::MouseCursor::NeswResize,
                ImGuiMouseCursor::ResizeNWSE => glutin::MouseCursor::NwseResize,
            });

        // Rescale window size from glutin logical size to our logical size
        }



        let physical_size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        let logical_size = physical_size.to_logical(ctx.hidpi_factor);

        let frame_size = FrameSize {
            logical_size: logical_size.into(),
            hidpi_factor: ctx.hidpi_factor,
        };
        
        let ui = ctx.imgui.frame(frame_size, delta_s);

        Self::show_fps(&ui);

        ctx.imgui_context.render(target, ui).unwrap();

    }

    fn show_fps<'a>(ui: &imgui::Ui<'a>) {
        ui.window(im_str!("Debug"))
            .position((10.0, 10.0), imgui::ImGuiCond::Appearing)
            .size((100.0, 50.0), imgui::ImGuiCond::Appearing)
            .build(|| {
                ui.text(im_str!(
                    "fps: ({})", ui.framerate() as u32,
                ));              
    });
}

}