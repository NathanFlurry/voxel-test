#[macro_use] extern crate glium;

mod app;
mod utils;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;

struct ProgramRegister {
    default_program: glium::Program
}

impl ProgramRegister {
    pub fn new(display: &glium::Display) -> ProgramRegister {
        ProgramRegister {
            default_program: program!(display,
                140 => {
                    vertex: "
                        #version 140
                        uniform mat4 matrix;
                        in vec3 position;
                        in vec3 color;
                        in vec2 uv;
                        out vec3 vColor;
                        void main() {
                            gl_Position = vec4(position, 1.0) * matrix;
                            vColor = color;
                        }
                    ",

                    fragment: "
                        #version 140
                        in vec3 vColor;
                        out vec4 f_color;
                        void main() {
                            f_color = vec4(vColor, 1.0);
                        }
                    "
                }
            ).unwrap()
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    uv: [f32; 2]
}

implement_vertex!(Vertex, position, color);

struct VoxelTest {
    program_register: ProgramRegister
}

impl VoxelTest {
    fn new(app: &mut app::App) -> VoxelTest {
        VoxelTest {
            program_register: ProgramRegister::new(&app.display)
        }
    }
}

impl app::AppState for VoxelTest {
    fn update(&mut self, app: &mut app::App) {

    }

    fn render(&mut self, app: &mut app::App) {
        // MOVE ELSEWHERE: Create new triangle
        let vertex_buffer = glium::VertexBuffer::new(
            &app.display,
            &[
                Vertex { position: [-0.5, -0.5, 0. ], color: [ 0.0, 1.0, 0.0 ], uv: [0., 0.,] },
                Vertex { position: [ 0.0,  0.5, 0. ], color: [ 0.0, 0.0, 1.0 ], uv: [0., 1.] },
                Vertex { position: [ 0.5, -0.5, 0. ], color: [ 1.0, 0.0, 0.0 ], uv: [1., 0.] },
            ]
        ).unwrap();
        let index_buffer = glium::IndexBuffer::new(&app.display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // MOVE ELSEWHERE: Setup draw params
//        let params = glium::DrawParameters {
//            depth: glium::Depth {
//                test: glium::DepthTest::IfLess,
//                write: true,
//                .. Default::default()
//            },
//            .. Default::default()
//        };
        let params = Default::default();

        // Render the triangle
        let mut target = app.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(  // TODO: Add easy to use method for this
            &vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &self.program_register.default_program,
            &uniforms,
            &params
        ).unwrap();
        target.finish().unwrap();
    }

    fn process_event(&mut self, event: glutin::Event) {
        println!("event {:?}", event);
    }
}

impl VoxelTest {

}

fn main() {
    let mut app = app::App::new();
    let voxel_test = VoxelTest::new(&mut app);
    app.start(Box::new(voxel_test));
}
