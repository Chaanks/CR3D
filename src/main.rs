#[macro_use]
extern crate lazy_static;
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
use imgui::*;
use graphics::{ Vertex, Color };
use conf::Conf;
use context::Context;
use event::EventHandler;
use error::Issue;
use app::App;
use overlay::Overlay;
use std::time;


pub struct State {
    square: graphics::Polygon,
    model: graphics::Model,
    overlay: Overlay,
}

impl State {
    fn new(app: &App) -> Self{

        let square = [
        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 0.0]},
        Vertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5,  0.5, -0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [0.5,  0.5, -0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 0.0]},

        Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [ 0.0, 0.0]},
        Vertex { position: [0.5, -0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [ 0.0, 0.0]},

        Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [ 1.0, 0.0]},
        Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [ 1.0, 1.0]},
        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [ 0.0, 0.0]},
        Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [ 1.0, 0.0]},

        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5,  0.5, -0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0]},
        Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0]},
        Vertex { position: [0.5, -0.5,  0.5], tex_coords: [0.0, 0.0]},
        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 0.0]},

        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [0.5, -0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5, -0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [ 0.0, 0.0]},
        Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0]},

        Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        Vertex { position: [0.5,  0.5, -0.5], tex_coords: [1.0, 1.0]},
        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5,  0.5,  0.5], tex_coords: [1.0, 0.0]},
        Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [ 0.0, 0.0]},
        Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [ 0.0, 1.0]},
        ];

        let texture = graphics::Polygon::new_texture("tex.png".to_string(), &app.ctx.glium_context).unwrap();
        let mut square = graphics::Polygon::new(&app.ctx.glium_context, square.to_vec(), Color::new(1.0, 0.0, 0.0), texture);
        let mut model = graphics::Model::new(&app.ctx.glium_context, "deer.obj".to_string());

        let overlay = Overlay::new();

        //square.scale(0.5, 0.5, 0.5);
        let scale = model.scale;
        model.scale(scale, scale, scale);


        Self {
            square,
            overlay,
            model,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Issue<()> {
        self.square.rotate_x(0.0);
        self.square.rotate_y(0.0);
        self.square.rotate_z(0.0);
        //self.square.translate(0.0, 0.0, 0.0);

        self.model.rotate_x(1.0);
        self.model.rotate_y(1.0);
        self.model.rotate_z(1.0);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, target: &mut Frame) -> Issue<()> {
        
        self.square.draw(target);
        //self.model.draw(target);
        self.overlay.draw(ctx, target, &mut self.square);


        Ok(())
    }
}


fn main() {

    let conf = Conf::new("square".into(), 800.0, 600.0, true);
    let mut app = App::new(conf);
    let mut state = State::new(&app);
    app.run(&mut state).unwrap();

}