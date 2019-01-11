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
use crate::block::Block;
use crate::chunk::ChunkBlockIndex;
use crate::world::WorldBlockIndex;
use kiss3d::resource::Mesh;
use std::rc::Rc;
use std::cell::RefCell;
use na::Vector3;
use na::Point2;

// blocks:rs
// ```
// 0     1
// +-----+
// |   / |
// |  /  |
// | /   |
// +-----+
// 3     2
// ```
const FACES: [[usize; 4]; 6] = [
    [5, 4, 0, 1],
    [7, 6, 2, 3],
    [6, 5, 1, 2],
    [4, 7, 3, 0],
    [6, 7, 4, 5],
    [1, 0, 3, 2],
];

const VERTICES: [[f32; 3]; 8] = [
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 0., 1.],
    [0., 0., 1.],
    [0., 1., 0.],
    [1., 1., 0.],
    [1., 1., 1.],
    [0., 1., 1.],
];

const UVS: [[f32; 2]; 4] = [
    [0., 0.],
    [1., 0.],
    [1., 1.],
    [0., 1.],
];

const FACE_ORDER: [usize; 6] = [
    0, 3, 1, 1, 3, 2,
];

const NORMALS: [[f32; 3]; 6] = [
    [ 0.,  0., -1.],
    [ 0.,  0.,  1.],
    [ 1.,  0.,  0.],
    [-1.,  0.,  0.],
    [ 0.,  1.,  0.],
    [ 0., -1.,  0.],
];


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
        let mut chunk = Chunk::empty();

        // Create floor
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                chunk.set_block(&ChunkBlockIndex::new(x, y, if x*y%2==0 { 3 } else { 0 }), Block::DIRT);
//                chunk.set_block(&ChunkBlockIndex::new(x, y, 0), Block::DIRT);
            }
        }

        chunk
    }
}

fn generate_world() -> World {
    let delegate = ProceduralWorld::new();

    let mut world = World::new(Box::new(delegate));

    world
}

fn main() {
    // Create camera
    let eye = Point3::new(0., 10., -20.);
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
    let mut cube = window.add_cube(0.25, 0.25, 0.25);
    cube.set_color(0., 1., 0.);

    // Create world
    let mut world = generate_world();

    // Set block
    for x in 10..=20 {
        for y in 10..=20 {
            for z in 15..=20 {
                world.set_block(&WorldBlockIndex::new(x, y, z), Block::DIRT);
            }
        }
    }

    // Render to world
    enum BlockSide {}
    let mut coords = Vec::new();
    let mut faces = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    // Get the chunk
    let mut chunk = world.get_or_create_chunk(&ChunkIndex::new(0, 0, 0));
    chunk.process_sides();
    chunk.render(&mut coords, &mut faces, &mut normals, &mut uvs);

    // Add the mesh
    let mut mesh = Rc::new(RefCell::new(Mesh::new(
//        coords, faces, Some(normals), Some(uvs), false,
        coords, faces, Some(normals), None, false,
    )));
    let mut world_mesh = window.add_mesh(mesh, Vector3::new(1., 1., 1.));
    world_mesh.enable_backface_culling(true);

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

        // Draw origin
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(1., 0., 0.), &Point3::new(1., 0., 0.));
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 1., 0.), &Point3::new(0., 1., 0.));
        window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 0., 1.), &Point3::new(0.25, 0.25, 1.));


        // Render
        window.render_with_camera(&mut first_person);
    }
}
