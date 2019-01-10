extern crate kiss3d;
extern crate nalgebra as na;

mod world;
mod chunk;
mod block;

use kiss3d::camera::FirstPerson;
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::Point3;
use crate::world::World;
use crate::world::WorldDelegate;
use crate::world::ChunkIndex;
use crate::chunk::Chunk;

struct ProceduralWorld {
    // TODO: Add seed and noise generator based on the seed
}

impl ProceduralWorld {
    fn new() -> ProceduralWorld {
        ProceduralWorld { }
    }
}

impl WorldDelegate for ProceduralWorld {
    fn create_chunk(&self, index: &ChunkIndex) -> Chunk {
        // TODO: Generate world based on a seed
        unimplemented!()
    }
}

fn generate_world() -> World {
    let delegate = ProceduralWorld::new();

    let mut world = World::new(Box::new(delegate));

    world
}

fn main() {
    // Create camera
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);
    first_person.set_move_step(0.05);
    first_person.rebind_up_key(Some(Key::W));
    first_person.rebind_down_key(Some(Key::S));
    first_person.rebind_left_key(Some(Key::A));
    first_person.rebind_right_key(Some(Key::D));

    // Create window
    let mut window = Window::new("Test");
    window.set_light(Light::StickToCamera);

    // Add test cube
    let mut cube = window.add_cube(1., 1., 1.);
    cube.set_color(0., 1., 0.);

    while !window.should_close() {
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


        // Render
        window.render_with_camera(&mut first_person);
    }
}
