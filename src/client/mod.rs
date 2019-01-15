mod cg;
mod procedural_world;
mod program_register;
mod render;

use crate::utils;
use glium::{glutin, Surface};
use crate::world;
use std::io::Cursor;
use vecmath::*;

pub struct VoxelMesh {
    model: Matrix4<f32>,
    vertex_buffer: glium::VertexBuffer<cg::Vertex>,
    index_buffer: Option<glium::IndexBuffer<u16>>
}

impl VoxelMesh {
    pub fn new(model: Matrix4<f32>, vertex_buffer: glium::VertexBuffer<cg::Vertex>, index_buffer: Option<glium::IndexBuffer<u16>>) -> VoxelMesh {
        VoxelMesh { model, vertex_buffer, index_buffer }
    }
}

pub struct VoxelTest {
    program_register: program_register::ProgramRegister,
    draw_params: glium::DrawParameters<'static>,
    camera: utils::CameraState,

    world: world::World,
    meshes: Vec<VoxelMesh>,
    tile_texture: glium::texture::Texture2d
}

impl VoxelTest {
    pub fn new(app: &mut utils::App) -> VoxelTest {
        let mut meshes = Vec::new();

        // Create world
        let delegate = procedural_world::ProceduralWorld::new(1234);
        let mut world = world::World::new(Box::new(delegate));

        // Add sphere
        let radius = 7;
        world.fill_ellipsoid(
            world::Block::STONE_BRICK,
                 &world::WorldBlockIndex::new(16 - radius, 16 - radius, 32 - radius),
                 &world::WorldBlockIndex::new(16 + radius, 16 + radius, 32 + radius)
        );

        // Get the chunk and process the sides
        let mut chunk = world.get_or_create_chunk(&world::ChunkIndex::new(0, 0, 0));
        chunk.process_sides();

        // Get chunk vertices
        let mut vertices = Vec::new();
        chunk.render(&mut vertices);

        // Add test triangle
        let vertex_buffer = glium::VertexBuffer::new(
            &app.display,
            &[
                cg::Vertex { position: [-50., -50., 0.], color: [0.0, 1.0, 0.0], normal: [0., 1., 0.], uv: [0., 0.] },
                cg::Vertex { position: [ 0.0,  50., 0.], color: [0.0, 0.0, 1.0], normal: [0., 1., 0.], uv: [0., 1.] },
                cg::Vertex { position: [ 50., -50., 0.], color: [1.0, 0.0, 0.0], normal: [0., 1., 0.], uv: [1., 0.] },
            ]
        ).unwrap();
        let index_buffer = glium::IndexBuffer::new(&app.display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
        meshes.push(VoxelMesh::new(mat4_id(), vertex_buffer, None));

        let y = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Create mesh
        let vertex_buffer = glium::VertexBuffer::new(&app.display, &vertices[..]).unwrap();
        let index_buffer = glium::IndexBuffer::new(&app.display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
        meshes.push(VoxelMesh::new(mat4_id(), vertex_buffer, None));

        // Get the texture
        let image = image::load(Cursor::new(&include_bytes!("../../assets/img/spritesheet_tiles.png")[..]), image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(&app.display, image).unwrap();

        // Create app
        VoxelTest {
            program_register: program_register::ProgramRegister::new(&app.display),
            draw_params: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                .. Default::default()
            },
            camera: utils::CameraState::new([32., 64., 32.], [0., 0., 1.]),

            world,
            meshes,
            tile_texture: texture
        }
    }
}

impl utils::AppState for VoxelTest {
    fn update(&mut self, app: &mut utils::App, dt: f32) {

    }

    fn render(&mut self, app: &mut utils::App, dt: f32) {
        // Update the camera
        self.camera.update(app, dt);

        // Render the triangle
        let mut target: glium::Frame = app.display.draw();
        target.clear_color_and_depth((0.623, 0.929, 0.988, 1.), 1.);

        for mesh in self.meshes.iter() {
            // Create uniforms
            let uniforms = uniform! {
                model_matrix: mesh.model,
                view_matrix: self.camera.get_view(),
                projection_matrix: self.camera.get_perspective(),
                tex: &self.tile_texture
            };

            // Draw the mesh
            target.draw(
                &mesh.vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program_register.default_program,
                &uniforms,
                &self.draw_params
            ).unwrap();
        }

        target.finish().unwrap();
    }

    fn process_event(&mut self, event: glutin::Event) {
        // Update camera
        self.camera.process_input(&event)
    }
}

impl VoxelTest {

}
