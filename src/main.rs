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

    // Create world
    let mut world = generate_world();

    // Set block
    for x in 10..=15 {
        for y in 10..=15 {
            for z in 15..=17 {
                world.set_block(&WorldBlockIndex::new(x, y, z), Block::DIRT);
            }
        }
    }

    // Render to world
    enum BlockSide {}
    let mut coords = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();
    fn add_cube(
        coords: &mut Vec<Point3<f32>>,
        uvs: &mut Vec<Point2<f32>>,
        normals: &mut Vec<Vector3<f32>>,
        faces: &mut Vec<Point3<u16>>,
        x: u32, y: u32, z: u32,  // World block index
        top: bool, bottom: bool, left: bool, right: bool, front: bool, back: bool
    ) {
        let x0 = x as f32;
        let x1 = x0 + 1.;
        let y0 = z as f32;  // Flip Y with Z
        let y1 = y0 + 1.;
        let z0 = y as f32;  // Flip Z with Y
        let z1 = z0 + 1.;

        let _1 = 1.;
        let m1 = -_1;
        let _0 = 0.;

        coords.push(Point3::new(x0, y0, z1));
        coords.push(Point3::new(x0, y0, z0));
        coords.push(Point3::new(x1, y0, z0));
        coords.push(Point3::new(x1, y0, z1));
        coords.push(Point3::new(x0, y1, z1));
        coords.push(Point3::new(x0, y1, z0));
        coords.push(Point3::new(x1, y1, z0));
        coords.push(Point3::new(x1, y1, z1));

        uvs.push(Point2::new(_0, _1));
        uvs.push(Point2::new(_1, _1));
        uvs.push(Point2::new(_0, _0));
        uvs.push(Point2::new(_1, _0));

        normals.push(Vector3::new(m1, _0, _0));
        normals.push(Vector3::new(_0, _0, m1));
        normals.push(Vector3::new(_1, _0, _0));
        normals.push(Vector3::new(_0, _0, _1));
        normals.push(Vector3::new(_0, m1, _0));
        normals.push(Vector3::new(_0, _1, _0));

        let _0 = (coords.len() - 8) as u16;
        let _1 = (coords.len() - 7) as u16;
        let _2 = (coords.len() - 6) as u16;
        let _3 = (coords.len() - 5) as u16;
        let _4 = (coords.len() - 4) as u16;
        let _5 = (coords.len() - 3) as u16;
        let _6 = (coords.len() - 2) as u16;
        let _7 = (coords.len() - 1) as u16;

        faces.push(Point3::new(_4, _5, _0));
        faces.push(Point3::new(_5, _1, _0));
        faces.push(Point3::new(_5, _6, _1));
        faces.push(Point3::new(_6, _2, _1));
        faces.push(Point3::new(_6, _7, _3));
        faces.push(Point3::new(_2, _6, _3));
        faces.push(Point3::new(_7, _4, _0));
        faces.push(Point3::new(_3, _7, _0));
        faces.push(Point3::new(_0, _1, _2));
        faces.push(Point3::new(_3, _0, _2));
        faces.push(Point3::new(_7, _6, _5));
        faces.push(Point3::new(_4, _7, _5));
    }
    for x in 0..Chunk::SIZE_X_U32 * 1 {
        for y in 0..Chunk::SIZE_Y_U32 * 1 {
            for z in 0..Chunk::SIZE_Z_U32 * 1 {
                // Get block
                let block = world.get_block_mut(&WorldBlockIndex::new(x, y, z));
                if block.is_air() { continue }

                // Add cube
//                let mut block = window.add_cube(1., 1., 1.);
//                block.append_translation(&Translation3::new(x as f32, z as f32, y as f32));

                // Add mesh
                add_cube(
                    &mut coords,
                    &mut uvs,
                    &mut normals,
                    &mut faces,
                    x, y, z,
                    true, true, true, true, true, true
                );
            }
        }
    }

    // Add the mesh
    let mesh = Rc::new(RefCell::new(Mesh::new(
//        coords, faces, Some(normals), Some(uvs), false,
        coords, faces, None, None, false,
    )));
    let world_mesh = window.add_mesh(mesh, Vector3::new(1., 1., 1.));

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
