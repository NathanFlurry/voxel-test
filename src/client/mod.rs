mod cg;
mod program_register;
mod render;

use crate::utils;
use glium::{glutin, Surface};

pub struct VoxelTest {
    program_register: program_register::ProgramRegister,
    draw_params: glium::DrawParameters<'static>,
    camera: utils::CameraState
}

impl VoxelTest {
    pub fn new(app: &mut utils::App) -> VoxelTest {
        VoxelTest {
            program_register: program_register::ProgramRegister::new(&app.display),
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

impl utils::AppState for VoxelTest {
    fn update(&mut self, app: &mut utils::App, dt: f32) {

    }

    fn render(&mut self, app: &mut utils::App, dt: f32) {
        // MOVE ELSEWHERE: Create new triangle
        let vertex_buffer = glium::VertexBuffer::new(
            &app.display,
            &[
                cg::Vertex { position: [-50., -50., 0.], color: [0.0, 1.0, 0.0], normal: [0., 1., 0.], uv: [0., 0.] },
                cg::Vertex { position: [ 0.0,  50., 0.], color: [0.0, 0.0, 1.0], normal: [0., 1., 0.], uv: [0., 1.] },
                cg::Vertex { position: [ 50., -50., 0.], color: [1.0, 0.0, 0.0], normal: [0., 1., 0.], uv: [1., 0.] },
            ]
        ).unwrap();
        let index_buffer = glium::IndexBuffer::new(&app.display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

        // Update the camera
        self.camera.update(app, dt);

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
