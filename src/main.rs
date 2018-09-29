#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;


mod error;
mod event;
mod conf;
mod mouse;
mod graphics;
mod context;
mod app;

use glium::Frame;

use graphics::{ Vertex, Color };
use conf::Conf;
use context::Context;
use event::EventHandler;
use error::Issue;
use app::App;

struct State {
    triangle: graphics::Polygon,
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Issue<()> {
        ctx.set_background_color(0.0, 0.5, 0.5, 1.0);
        Ok(())
    }

    fn draw(&mut self, target: &mut Frame) -> Issue<()> {
        self.triangle.draw(target);
        Ok(())
    }
}


fn main() {

    let conf = Conf::new("triangle".into(), 600.0, 600.0, true);
    let mut app = App::new(conf);

    let triangle = [
        Vertex { position: [0.0, -0.5] },
        Vertex { position: [0.5, 0.5] },
        Vertex { position: [-0.5, 0.5] }
    ];

    let mut state = State { triangle: graphics::Polygon::new(&app.ctx.glium_context, triangle.to_vec(), Color::new(1.0, 0.0, 0.0)) };


    app.run(&mut state).unwrap();





}

