extern crate kiss3d;
extern crate nalgebra as na;
extern crate noise;

mod block;
mod chunk;
mod world;
mod procedural_world;

use kiss3d::camera::FirstPerson;
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::Point3;
use crate::world::World;
use crate::world::WorldDelegate;
use crate::world::ChunkIndex;
use crate::chunk::Chunk;
use crate::block::Block;
use crate::chunk::ChunkBlockIndex;
use crate::world::WorldBlockIndex;
use kiss3d::resource::Mesh;
use std::rc::Rc;
use std::cell::RefCell;
use na::Vector3;
use na::Point2;
use kiss3d::window::State;
use crate::procedural_world::ProceduralWorld;
use kiss3d::camera::Camera;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::resource::Texture;
use kiss3d::resource::TextureManager;
use std::f64;
use std::u32;

struct AppState {
    first_person: FirstPerson
}

impl AppState {
    fn new(window: &mut Window) -> AppState {
        // Set BG
        window.set_background_color(0.62, 0.92, 0.99);

        // Create camera
        let eye = Point3::new(0., 10., -20.);
        let at = Point3::origin();
        let mut first_person = FirstPerson::new(eye, at);
        first_person.set_move_step(0.05);
        first_person.rebind_up_key(Some(Key::W));
        first_person.rebind_down_key(Some(Key::S));
        first_person.rebind_left_key(Some(Key::A));
        first_person.rebind_right_key(Some(Key::D));

        // Create world
        let delegate = ProceduralWorld::new(1234);
        let mut world = World::new(Box::new(delegate));

        // Set block
//        for x in 10..=20 {
//            for y in 10..=20 {
//                for z in 15..=20 {
//                    world.set_block(&WorldBlockIndex::new(x, y, z), Block::DIRT);
//                }
//            }
//        }

        // Set sphere
        fill_sphere(&mut world, Block::DIRT, Chunk::SIZE_X_U32 / 2, Chunk::SIZE_Y_U32 / 2, Chunk::SIZE_Z_U32 / 2, 5.);

        // Render to world
        let mut coords = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        // Get the chunk
        let mut chunk = world.get_or_create_chunk(&ChunkIndex::new(0, 0, 0));
        chunk.process_sides();
        chunk.render(&mut coords, &mut faces, &mut normals, &mut uvs);

        // Add texture
        let texture = TextureManager::get_global_manager(|tm| tm.add_image_from_memory(include_bytes!("../assets/brick_grey.png"), "brick_grey"));

        // Add the mesh
        let mut mesh = Rc::new(RefCell::new(Mesh::new(
            coords, faces, Some(normals), Some(uvs), false,
        )));
        let mut world_mesh = window.add_mesh(mesh, Vector3::new(1., 1., 1.));
        world_mesh.set_texture(texture);

        AppState { first_person }
    }
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        // Update the current camera.
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Release, _) => {
                    if key == Key::F {
                        println!("Pay respects")
                    }
                }
                _ => {}
            }
        }

        // Draw origin
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(1., 0., 0.), &Point3::new(1., 0., 0.));
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 1., 0.), &Point3::new(0., 1., 0.));
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 0., 1.), &Point3::new(0.25, 0.25, 1.));
    }

    fn cameras_and_effect(&mut self) -> (Option<&mut Camera>, Option<&mut PlanarCamera>, Option<&mut PostProcessingEffect>) {
        (Some(&mut self.first_person), None, None)
    }
}

fn fill_sphere(world: &mut World, block: Block, x_origin: u32, y_origin: u32, z_origin: u32, radius: f64) {
    let radius_ceil = radius.ceil() as u32;
    for x in x_origin-radius_ceil..=x_origin+radius_ceil {
        for y in y_origin-radius_ceil..=y_origin+radius_ceil {
            for z in z_origin-radius_ceil..=z_origin+radius_ceil {
                let dist = ((x as f64 - x_origin as f64).powi(2) + (y as f64 - y_origin as f64).powi(2) + (z as f64 - z_origin as f64).powi(2)).sqrt();
                if dist < radius + 1. {  // Adding 1 and using exclusive less-than makes a more natural sphere
                    world.set_block(&WorldBlockIndex::new(x, y, z), block);
                }
            }
        }
    }
}

fn main() {
    // Create window
    let mut window = Window::new("Test");
    window.set_light(Light::StickToCamera);

    // Create state
    let state = AppState::new(&mut window);

    // Start render loop
    window.render_loop(state);
}
