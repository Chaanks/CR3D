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
mod camera;

use glium::glutin::{ KeyboardInput, VirtualKeyCode };
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
use camera::Camera;
use glm::Dimension;


pub struct State {
    square: graphics::Polygon,
    model: graphics::Model,
    overlay: Overlay,
    camera: Camera,
    cam_x: f32,
    cam_z: f32,
    factor: f32,
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
        let mut model = graphics::Model::new(&app.ctx.glium_context, "lego.obj".to_string());

        let overlay = Overlay::new();

        //square.scale(0.5, 0.5, 0.5);
        let scale = model.scale;
        model.scale(scale, scale, scale);

        let camera = Camera::new();

        let cam_x = 0.0;
        let cam_z = 0.0;
        let factor= 0.1;
        Self {
            square,
            overlay,
            model,
            camera,
            cam_x,
            cam_z,
            factor,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Issue<()> {
        self.camera.update();
        self.model.rotate_x(1.0);
        self.model.rotate_y(1.0);
        self.model.rotate_z(1.0);

        self.factor += 0.01;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, target: &mut Frame) -> Issue<()> {
        let cam = &mut self.camera;
        self.square.draw(target,cam);
        //self.model.draw(target);
        self.overlay.draw(ctx, target, &mut self.square);


        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: VirtualKeyCode) {
        if key == VirtualKeyCode::W {
            self.camera.camera_pos += 1.0  * self.camera.camera_front;
        }
        if key == VirtualKeyCode::S {
            self.camera.camera_pos -= 1.0  * self.camera.camera_front;
        }
        if key == VirtualKeyCode::A {

            let camera_pos = glm::vec3(0.0, 0.0, 3.0);
            let camera_pos2 = glm::vec3(0.0, 0.0, 3.0);
            let cross : glm::TVec3<f32> = glm::cross(&camera_pos2, &camera_pos);
            self.camera.camera_pos += 1.0  * glm::normalize(&glm::cross(&self.camera.camera_front, &self.camera.camera_up));
        }
        if key == VirtualKeyCode::D {
            self.square.translate(-0.1, 0.0, 0.0);
        }
    }
}




fn main() {

    let conf = Conf::new("square".into(), 800.0, 600.0, true);
    let mut app = App::new(conf);
    let mut state = State::new(&app);
    app.run(&mut state).unwrap();

}