use glium::Frame;

use error::Issue;
use context::Context;
use overlay::Overlay;
use glium::glutin::{ KeyboardInput, VirtualKeyCode };

pub trait EventHandler {
    fn update(&mut self, ctx: &mut Context) -> Issue<()>;
    fn draw(&mut self, ctx: &mut Context, target: &mut Frame) -> Issue<()>;
    fn key_down_event(&mut self, ctx: &mut Context, key: VirtualKeyCode) {
        if key == VirtualKeyCode::W {
            println!("fdp2");
        }
    }
}
