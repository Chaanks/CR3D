use glium;
use glium::{ Surface, uniforms, Frame };
use context::Context;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);


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
}

impl Polygon {
    pub fn new(display: &glium::Display,  data: Vec<Vertex>, color: Color) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(display, &data).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140
            #extension GL_KHR_vulkan_glsl : enable

            in vec2 position;
            uniform vec3 tcolor;
            out vec3 color;


            void main() {
                vec2 pos = position;
                color = tcolor;
                gl_Position = vec4(pos, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 color;
            out vec4 f_color;

            void main() {
                f_color = vec4(color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to create program");

        Self {
            vertex_buffer,
            indices,
            program,
            color,
            data,
        }
    }

    pub fn draw(&mut self, target: &mut Frame) {
        target.draw(&self.vertex_buffer, &self.indices, &self.program,
                    &uniform! {tcolor : self.color.rgb}, &Default::default()).expect("Failed to draw");
    }

}