use glium::Frame;

use error::Issue;
use context::Context;

pub trait EventHandler {
    fn update(&mut self, ctx: &mut Context ) -> Issue<()>;
    fn draw(&mut self, ctx: &mut Context, target: &mut Frame ) -> Issue<()>;

}
