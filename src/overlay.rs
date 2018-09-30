use imgui;
use State;
use context;
use glium::{ glutin, Frame, Surface };
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};
use glium::glutin::ElementState::Pressed;
use imgui::*;
use std::time;
use graphics::Polygon;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct PolygonState {
    x: bool,
    y: bool,
    z: bool,

    x_speed: f32,
    y_speed: f32,
    z_speed: f32, 
}

impl PolygonState {
    fn update(&mut self, polygon: &mut Polygon) {
        if self.x {
            polygon.rotate_x(self.x_speed);
        }
        if self.y {
            polygon.rotate_y(self.y_speed);
        }
        if self.z {
            polygon.rotate_z(self.z_speed);
        }
    }
}



pub struct Overlay {
    pub last_frame: time::Instant,
    pub polygon_state: PolygonState,
} 

impl Overlay {
    pub fn new() -> Self{
        let last_frame = time::Instant::now();
        let polygon_state = PolygonState::default();
        Self {
            last_frame,
            polygon_state,
        }
    }

    pub fn draw(&mut self, ctx: &mut context::Context, target: &mut Frame, square: &mut Polygon) {
        let now = time::Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;
        //let fps = (1.0 / self.delta_s) as i32;
        
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
        Self::color_picker(&ui, &mut ctx.background_color);
        Self::polygon_edit(&ui, &mut self.polygon_state);
        ctx.imgui_context.render(target, ui).unwrap();

        self.polygon_state.update(square);
    }

    fn show_fps<'a>(ui: &imgui::Ui<'a>) {
        ui.window(im_str!("Debug"))
            .position((10.0, 10.0), ImGuiCond::FirstUseEver)
            .size((100.0, 50.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!(
                    "fps: ({})", ui.framerate() as u32,
                ));              
            });
    }

    fn color_picker<'a>(ui: &imgui::Ui<'a>, bg: &mut [f32; 4]) {
        let color = EditableColor::Float4(bg);
        let color_edit = imgui::ColorEdit::new(&ui, im_str!(" "), color);
        ui.window(im_str!("Background"))
            .position((250.0, 10.0), ImGuiCond::FirstUseEver)
            .size((300.0, 50.0), ImGuiCond::FirstUseEver)
            .build(|| {
                color_edit.build();

            });

    }

    fn polygon_edit<'a>(ui: &imgui::Ui<'a>, state: &mut PolygonState) {
        ui.window(im_str!("Rotation"))
            .position((700.0, 5.0), ImGuiCond::Always)
            .size((100.0, 180.0), ImGuiCond::Always)
            .build(|| {
                //ui.checkbox(im_str!("Rotate X"), &mut state);
                ui.checkbox(im_str!("x axis"), &mut state.x);
                ui.slider_float(im_str!(" "), &mut state.x_speed, 0.0, 10.0)
                    .build();
                ui.separator();
                ui.checkbox(im_str!("y axis"), &mut state.y);
                ui.slider_float(im_str!("  "), &mut state.y_speed, 0.0, 10.0)
                    .build();
                ui.separator();
                ui.checkbox(im_str!("z axis"), &mut state.z);
                ui.slider_float(im_str!("   "), &mut state.z_speed, 0.0, 10.0)
                    .build();
                ui.separator();
            }
            );

    }
}
