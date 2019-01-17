mod cg;
mod procedural_world;
mod program_register;
mod render;
mod world_renderer;

use crate::utils;
use glium::{glutin, Surface};
use crate::world;
use std::io::Cursor;
use std::time::Instant;
use crate::utils::AsFloatSeconds;

pub struct VoxelTest {
    program_register: program_register::ProgramRegister,
    draw_params: glium::DrawParameters<'static>,
    camera: utils::CameraState,

    world: world::World,
    world_renderer: world_renderer::WorldRenderer,
    tile_texture: glium::texture::Texture2d
}

impl VoxelTest {
    pub fn new(app: &mut utils::App) -> VoxelTest {
        // Create world
        let delegate = procedural_world::ProceduralWorld::new(1234);
        let mut world = world::World::new(Box::new(delegate));

        // Get the tile texture
        let image_start = Instant::now();
        let image = image::load(Cursor::new(&include_bytes!("../../assets/img/spritesheet_tiles.png")[..]), image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(&app.display, image).unwrap();
        println!("Spritesheet loaded - {:.3}", image_start.elapsed().as_float_seconds());

        // Add sphere
        let radius = 7;
        world.fill_ellipsoid(
            world::Block::from_id("brick_stone"),
            &world::WorldBlockIndex::new(16 - radius, 16 - radius, 32 - radius),
            &world::WorldBlockIndex::new(16 + radius, 16 + radius, 32 + radius)
        );

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
            world_renderer: world_renderer::WorldRenderer::new(1),
            tile_texture: texture
        }
    }
}

impl utils::AppState for VoxelTest {
    fn update(&mut self, _app: &mut utils::App, _dt: f32) {

    }

    fn render(&mut self, app: &mut utils::App, dt: f32) {
        // Update the camera
        self.camera.update(app, dt);

        // Prepare the target
        let mut target: glium::Frame = app.display.draw();
        target.clear_color_and_depth((0.623, 0.929, 0.988, 1.), 1.);

        // Render the chunks
        self.world_renderer.update(app, &mut self.world, &mut self.camera);
        for (_, mesh) in self.world_renderer.get_visible_chunks().iter() {
            // Create uniforms
            let uniforms = uniform! {
                model_matrix: mesh.transform,
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

        // Finish rendering
        target.finish().unwrap();
    }

    fn process_event(&mut self, _app: &mut utils::App, event: glutin::Event) {
        // Update camera
        self.camera.process_input(&event);

        // Toggle debug data
        match event {
            glutin::Event::WindowEvent { ref event, .. } => match *event {
                glutin::WindowEvent::KeyboardInput { ref input, .. } => {
                    // Get key state
                    if input.state != glutin::ElementState::Pressed { return }
                    let key = match input.virtual_keycode {
                        Some(key) => key,
                        None => return,
                    };

                    // Toggle debug mode
                    match key {
                        glutin::VirtualKeyCode::Grave => self.toggle_debug_mode(),

                        _ => { },
                    };
                },

                _ => { }
            },

            _ => { }
        }
    }
}

impl VoxelTest {
    fn toggle_debug_mode(&mut self) {
        // Get the next polygon mode
        let (next_mode, next_cull) = match self.draw_params.polygon_mode {
            glium::PolygonMode::Point => (glium::PolygonMode::Fill, true),
            glium::PolygonMode::Line => (glium::PolygonMode::Point, false),
            glium::PolygonMode::Fill => (glium::PolygonMode::Line, false),
        };

        // Save the mode
        self.draw_params.polygon_mode = next_mode;
        self.draw_params.backface_culling = if next_cull { glium::BackfaceCullingMode::CullClockwise } else { glium::BackfaceCullingMode::CullingDisabled };
    }
}
