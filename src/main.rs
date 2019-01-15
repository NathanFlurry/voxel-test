#[macro_use] extern crate glium;
extern crate image;
extern crate vecmath;

mod client;
mod utils;
mod world;

fn main() {
    let mut app = utils::App::new("Voxel Test");
    let voxel_test = client::VoxelTest::new(&mut app);
    app.start(Box::new(voxel_test));
}
