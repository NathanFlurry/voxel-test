#[macro_use] extern crate glium;

mod client;
mod utils;

fn main() {
    let mut app = utils::App::new("Voxel Test");
    let voxel_test = client::VoxelTest::new(&mut app);
    app.start(Box::new(voxel_test));
}
