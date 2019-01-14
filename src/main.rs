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

                        uniform mat4 persp_matrix;
                        uniform mat4 view_matrix;

                        in vec3 position;
                        in vec3 normal;
                        in vec3 color;

                        // TODO: Add uv

                        out vec3 v_position;
                        out vec3 v_normal;
                        out vec3 v_color;

                        void main() {
                            v_position = position;
                            v_normal = normal;
                            v_color = color;
                            gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                        }
                    ",

                    fragment: "
                        #version 140

                        in vec3 v_normal;
                        in vec3 v_color;

                        out vec4 f_color;

                        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                        void main() {
                            float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                            vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                            color *= v_color;
                            f_color = vec4(color, 1.0);
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
    normal: [f32; 3],
    uv: [f32; 2]
}

implement_vertex!(Vertex, position, color, normal, uv);

struct VoxelTest {
    program_register: ProgramRegister,
    draw_params: glium::DrawParameters<'static>,
    camera: utils::CameraState
}

impl VoxelTest {
    fn new(app: &mut app::App) -> VoxelTest {
        VoxelTest {
            program_register: ProgramRegister::new(&app.display),
            draw_params: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            },
            camera: utils::CameraState::new()
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
                Vertex { position: [-50., -50., 0.], color: [0.0, 1.0, 0.0], normal: [0., 1., 0.], uv: [0., 0.] },
                Vertex { position: [ 0.0,  50., 0.], color: [0.0, 0.0, 1.0], normal: [0., 1., 0.], uv: [0., 1.] },
                Vertex { position: [ 50., -50., 0.], color: [1.0, 0.0, 0.0], normal: [0., 1., 0.], uv: [1., 0.] },
            ]
        ).unwrap();
        let index_buffer = glium::IndexBuffer::new(&app.display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

        // Update the camera
        self.camera.update(app);

        // Create uniforms
        let uniforms = uniform! {
            persp_matrix: self.camera.get_perspective(),
            view_matrix: self.camera.get_view(),
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // Render the triangle
        let mut target = app.display.draw();
        target.clear_color_and_depth((0., 0., 1., 1.), 1.);
        target.draw(  // TODO: Add easy to use method for this
            &vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &self.program_register.default_program,
            &uniforms,
            &self.draw_params
        ).unwrap();
        target.finish().unwrap();
    }

    fn process_event(&mut self, event: glutin::Event) {
        // Update camera
        self.camera.process_input(&event)
    }
}

impl VoxelTest {

}

fn main() {
    let mut app = app::App::new();
    let voxel_test = VoxelTest::new(&mut app);
    app.start(Box::new(voxel_test));
}
