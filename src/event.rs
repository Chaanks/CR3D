use glium::Frame;

use error::Issue;
use context::Context;

pub trait EventHandler {
    fn update(&mut self, ctx: &mut Context ) -> Issue<()>;
    fn draw(&mut self, target: &mut Frame ) -> Issue<()>;

}
