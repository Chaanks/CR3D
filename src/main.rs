#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate image;
extern crate tobj;
extern crate nalgebra_glm as glm;

mod error;
mod event;
mod conf;
mod mouse;
mod overlay;
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
use overlay::Overlay;


pub struct State {
    triangle: graphics::Polygon,
    model: graphics::Model,
    overlay: Overlay,
}

impl State {
    fn new(app: &App) -> Self{

        let triangle = [
            Vertex { position: [0.0, -0.5], tex_coords: [0.0, 0.0] },
            Vertex { position: [0.5, 0.5], tex_coords: [0.0, 1.0] },
            Vertex { position: [-0.5, 0.5], tex_coords: [1.0, 0.0] }
        ];

        let texture = graphics::Polygon::new_texture("tex.png".to_string(), &app.ctx.glium_context).unwrap();
        let triangle = graphics::Polygon::new(&app.ctx.glium_context, triangle.to_vec(), Color::new(1.0, 0.0, 0.0), texture);
        let model = graphics::Model::new(&app.ctx.glium_context, "deer.obj".to_string());

        let overlay = Overlay::new();

        Self {
            triangle,
            model,
            overlay,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Issue<()> {
        //ctx.set_background_color(0.0, 0.0, 0.0, 1.0);
        self.model.light[0] += 0.009;
        self.model.light[1] += 0.004;
        self.model.light[2] += 0.002;

        if self.model.light[0] > 1.0 { self.model.light[0] = -1.0 };
        if self.model.light[1] > 1.0 { self.model.light[1] = -1.0 };
        if self.model.light[2] > 1.0 { self.model.light[2] = -1.0 };

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, target: &mut Frame) -> Issue<()> {
        //self.triangle.draw(target);
        self.model.draw(target);
        self.overlay.update(ctx, target);
        Ok(())
    }
}


fn main() {

    let conf = Conf::new("triangle".into(), 800.0, 800.0, true);
    let mut app = App::new(conf);
    let mut state = State::new(&app);
    app.run(&mut state).unwrap();





}

