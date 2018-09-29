use glium::Surface;
use glium::glutin;
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};
use glium::glutin::ElementState::Pressed;
use imgui::*;
use imgui;

use std::time::Instant;

use context::Context;
use conf::Conf;
use graphics;
use graphics::Color;
use glium::glutin::EventsLoop;
use event::EventHandler;
use error::Issue;


pub struct App {
    pub ctx : Context,
    events_loop: EventsLoop,
}

impl App {
    pub fn new(conf: Conf ) -> Self {
        let (ctx, events_loop) = Context::new(conf).expect("Failed to create a context");

        Self {
            ctx,
            events_loop,
        }
    }

    pub fn run<S>(&mut self, state: &mut S) -> Issue<()> 
    where S: EventHandler {
        let mut closed = false;
        let mut ctx = &mut self.ctx;
        let mut last_frame = Instant::now();
        while !closed {
            self.events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, ..} => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        glutin::WindowEvent::CursorMoved { position: pos, .. } => {
                            // Rescale position from glutin logical coordinates to our logical
                            // coordinates
                            ctx.mouse_context.pos = pos
                                .to_physical(ctx.get_hidpi_factor())
                                .to_logical(ctx.hidpi_factor)
                                .into();
                        },
                        glutin::WindowEvent::MouseInput { state, button, .. } => match button {
                            MouseButton::Left => ctx.mouse_context.pressed.0 = state == Pressed,
                            MouseButton::Right => ctx.mouse_context.pressed.1 = state == Pressed,
                            MouseButton::Middle => ctx.mouse_context.pressed.2 = state == Pressed,
                            _ => {}
                        },
                        _ => (),
                    },

                    _ => (),
                }

            });
            let now = Instant::now();
            let delta = now - last_frame;
            let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
            last_frame = now;

            ctx.mouse_context.update(&mut ctx.imgui);

            let mouse_cursor = ctx.imgui.mouse_cursor();
            let mut target = ctx.glium_context.draw();
            state.update(&mut ctx)?;
            target.clear_color(ctx.background_color[0], ctx.background_color[1], ctx.background_color[2], ctx.background_color[3]);
            state.draw(&mut target)?;
            {
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
            

                ui.window(im_str!("Debug"))
                    .position((10.0, 10.0), ImGuiCond::Appearing)
                    .size((100.0, 50.0), ImGuiCond::Appearing)
                    .build(|| {
                        ui.text(im_str!(
                            "fps: ({})", ui.framerate() as u32,
                            ));              
                });
            
                ctx.imgui_context.render(&mut target, ui).unwrap();
            }

            target.finish().unwrap();
        }

        Ok(())
    }
}

