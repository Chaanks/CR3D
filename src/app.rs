use glium::Surface;
use glium::glutin;
use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, KeyboardInput};
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
use overlay::Overlay;


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

    pub fn run(&mut self, state: &mut EventHandler) -> Issue<()> {
        let mut closed = false;
        let mut ctx = &mut self.ctx;
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
                        glutin::WindowEvent::KeyboardInput {input, ..} => match input.virtual_keycode {
                            Some(VirtualKeyCode) => state.key_down_event(ctx, VirtualKeyCode),
                            _ => {}
                        },
                        _ => (),
                    },

                    _ => (),
                }

            });
            let mut target = ctx.glium_context.draw();
            target.clear_color_and_depth((ctx.background_color[0], ctx.background_color[1], ctx.background_color[2], ctx.background_color[3]), 1.0);
            state.update(ctx)?;
            state.draw(ctx, &mut target)?;
            target.finish().unwrap();
        }

        Ok(())
    }


}

