use glium;
use glium::{ Surface, uniforms, Frame, Display, Texture2d, vertex::VertexBufferAny };
use context::Context;
use error::Issue;
use image;
use std::io::Cursor;
use std::path::Path;
use std::f32;
use tobj;
use glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);


#[derive(Copy, Clone)]
pub struct Color {
    pub rgb: [f32; 3],
}

impl Color {
    pub fn new(r: f32, g:f32, b: f32) -> Self {
        Self {
            rgb: [r, g, b],
        }
    }
}


pub struct Polygon {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub indices: glium::index::NoIndices,
    pub program: glium::Program,
    pub color: Color,
    pub data: Vec<Vertex>,
    pub texture: Texture2d,
    pub position: glm::Mat4x4,
}

impl Polygon {
    pub fn new(display: &glium::Display,  data: Vec<Vertex>, color: Color, texture: Texture2d) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(display, &data).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 150
            #extension GL_KHR_vulkan_glsl : enable

            in vec3 position;
            in vec2 tex_coords;
            out vec3 color;
            out vec2 v_tex_coords;

            uniform vec3 tcolor;
            uniform mat4 model;

            void main() {
                color = tcolor;
                v_tex_coords = tex_coords;
                gl_Position = model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 color;
            in vec2 v_tex_coords;
            out vec4 f_color;

            uniform sampler2D tex;

            void main() {
                f_color = texture(tex, v_tex_coords);
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to create program");

        let position = glm::identity();


        Self {
            vertex_buffer,
            indices,
            program,
            color,
            data,
            texture,
            position,
        }
    }

    pub fn draw(&mut self, target: &mut Frame) {

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = 45.0;
        let near = 0.1;
        let far = 100.0;

        let projection = glm::perspective(fov.to_radians(), aspect_ratio, near, far);
        let view = glm::look_at_rh(&glm::vec3(0.0,0.0,5.0), &glm::vec3(0.0,0.0,0.0), &glm::vec3(0.0,1.0,0.0));
        let mvp = projection * view * self.position;

        let mvp_ref: &[[f32; 4]; 4] = mvp.as_ref();

        target.draw(&self.vertex_buffer, &self.indices, &self.program,
                    &uniform! {tcolor : self.color.rgb, model: *mvp_ref}, &params).expect("Failed to draw");
    }

    pub fn new_texture(path: String, display: &Display) -> Issue<Texture2d> {
        //let image = image::load(Cursor::new(&include_bytes!("ss")[..]), image::PNG).unwrap().to_rgba();
        let image = image::open(&Path::new(&path)).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();

        Ok(texture)
    }

    pub fn scale(&mut self, x: f32, y:f32, z:f32) {
        let scale_matrix = glm::scale(&self.position, &glm::vec3(x,y,z));
        self.position = scale_matrix;
    }

    pub fn translate(&mut self, x: f32, y:f32, z:f32) {
        let scale_matrix = glm::translate(&self.position, &glm::vec3(x,y,z));
        self.position = scale_matrix;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_x(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_y(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_z(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }
}

pub struct Model {
    pub vertex_buffer: VertexBufferAny,
    pub indices: glium::index::NoIndices,
    pub program: glium::Program,
    pub scale: f32,
    pub light: [f32; 3],
    pub position: glm::Mat4x4,
}

impl Model {
    pub fn new(display: &glium::Display, path: String) -> Self {
        let (mut vertex_buffer, mut scale) = Model::load_model(display, path);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;
            out vec3 v_normal;

            uniform mat4 model;

            void main() {
                v_normal = normal;
                gl_Position = model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec3 v_normal;
            out vec4 f_color;

            uniform vec3 u_light;

            void main() {
                    float lum = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    f_color = vec4(color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to create program");


        // the direction of the light
        let light = [-1.0, -1.0, 1.0f32];
        
        let position = glm::identity();

        Self {
            vertex_buffer,
            indices,
            program,
            scale,
            light,
            position,
        }

    }

    pub fn draw(&mut self, target: &mut Frame) {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = 45.0;
        let near = 0.1;
        let far = 100.0;

        let projection = glm::perspective(fov.to_radians(), aspect_ratio, near, far);
        let view = glm::look_at_rh(&glm::vec3(0.0,0.0,3.0), &glm::vec3(0.0,0.0,0.0), &glm::vec3(0.0,1.0,0.0));
        let mvp = projection * view * self.position;

        let mvp_ref: &[[f32; 4]; 4] = mvp.as_ref();

        target.draw(&self.vertex_buffer, &self.indices, &self.program,
                    &uniform! {model: *mvp_ref, u_light: self.light}, &params).expect("Failed to draw");
    }

    fn load_model(display: &glium::Display, path: String) -> (VertexBufferAny, f32) {
        #[derive(Copy, Clone)]
        struct Vertex3 {
            position: [f32; 3],
            normal: [f32; 3],
        }

        implement_vertex!(Vertex3, position, normal);

        let mut min_pos = [f32::INFINITY; 3];
        let mut max_pos = [f32::NEG_INFINITY; 3];

        let obj = tobj::load_obj(&Path::new(&path));
        let (models, materials) = obj.unwrap();
        let mut vertex_data = Vec::new();

        for model in &models {
            println!("Uploading model: {}", model.name);
            let mesh = &model.mesh;
            for idx in &mesh.indices {
                let i = *idx as usize;
                let pos = [
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                ];
                
                let normal = if !mesh.normals.is_empty() {
                    [
                        mesh.normals[3 * i],
                        mesh.normals[3 * i + 1],
                        mesh.normals[3 * i + 2],
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                };
                
                vertex_data.push(Vertex3 {
                    position: pos,
                    normal: normal,
                });

                for i in 0..3 {
                    min_pos[i] = f32::min(min_pos[i], pos[i]);
                    max_pos[i] = f32::max(max_pos[i], pos[i]);
                }                
            }

        }

        // Compute scale factor to fit the model with a [-1, 1] bounding box
        let diagonal_len = 1.41;
        let current_len = f32::powf(max_pos[0] - min_pos[0], 2.0)
            + f32::powf(max_pos[1] - min_pos[1], 2.0)
            + f32::powf(max_pos[2] - min_pos[2], 2.0);
        let scale = f32::sqrt(diagonal_len / current_len);
        println!("Model scaled by {} to fit", scale);

        (glium::vertex::VertexBuffer::new(display, &vertex_data)
            .unwrap()
            .into_vertex_buffer_any(),
        scale)
    }

    pub fn scale(&mut self, x: f32, y:f32, z:f32) {
        let scale_matrix = glm::scale(&self.position, &glm::vec3(x,y,z));
        self.position = scale_matrix;
    }

    pub fn translate(&mut self, x: f32, y:f32, z:f32) {
        let scale_matrix = glm::translate(&self.position, &glm::vec3(x,y,z));
        self.position = scale_matrix;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_x(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_y(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let rotate_matrix = glm::rotate_z(&self.position, angle.to_radians());
        self.position = rotate_matrix;
    }
}