use crate::world;
use std::collections::HashMap;
use vecmath::*;
use crate::client::cg;
use crate::utils;
use utils::RangeContains;
use std::u32;
use std::time::Instant;
use crate::utils::AsFloatSeconds;
use crate::utils::ChunkClamp;

pub struct ChunkMesh {
    pub transform: Matrix4<f32>,
    pub vertex_buffer: glium::VertexBuffer<cg::Vertex>,
}

pub struct WorldRenderer {
    view_distance: u32,
    vertical_view_distant: u32,
    visible_chunks: HashMap<world::ChunkIndex, ChunkMesh>
}

impl WorldRenderer {
    pub fn new(view_distance: u32) -> WorldRenderer {
        WorldRenderer {
            view_distance,
            vertical_view_distant: (view_distance / 2).max(1),  // Since chunks are twice as tall, make the vertical view range half as much
            visible_chunks: HashMap::new()
        }
    }

    pub fn get_visible_chunks(&self) -> &HashMap<world::ChunkIndex, ChunkMesh> {
        &self.visible_chunks
    }

    pub fn update(&mut self, app: &mut utils::App, world: &mut world::World, camera: &utils::CameraState) {
        // Get the current chunk; saturating
        let camera_pos = camera.get_position();
        let current_chunk = world::ChunkIndex::new(
            (camera_pos[0] / world::Chunk::SIZE_X_F32).chunk_clamp_x() as u32,
            (camera_pos[2] / world::Chunk::SIZE_Y_F32).chunk_clamp_y() as u32,  // Flip Y with Z
            (camera_pos[1] / world::Chunk::SIZE_Z_F32).chunk_clamp_z() as u32,  // Flip Z with Y
        );

        // Get view distance
        let x_range = current_chunk.x.saturating_sub(self.view_distance)..=current_chunk.x.saturating_add(self.view_distance);
        let y_range = current_chunk.y.saturating_sub(self.view_distance)..=current_chunk.y.saturating_add(self.view_distance);
        let z_range = current_chunk.z.saturating_sub(self.vertical_view_distant)..=current_chunk.z.saturating_add(self.vertical_view_distant);

        // Remove chunks out of the view range
        let mut chunks_to_remove = Vec::new();
        for (chunk_index, _) in self.visible_chunks.iter() {
            if !x_range.range_contains(&chunk_index.x) || !y_range.range_contains(&chunk_index.y) || !z_range.range_contains(&chunk_index.z) {
                chunks_to_remove.push(chunk_index.clone());
            }
        }
        for chunk_index in chunks_to_remove {
            self.visible_chunks.remove(&chunk_index);
            println!("Removed chunk {} {} {}", chunk_index.x, chunk_index.y, chunk_index.z);
        }

        // Add new chunks
        for chunk_x in *x_range.start()..=*x_range.end(){
            for chunk_y in *y_range.start()..=*y_range.end() {
                for chunk_z in *z_range.start()..=*z_range.end() {
                    // Get the chunk index
                    let chunk_index = world::ChunkIndex::new(chunk_x, chunk_y, chunk_z);

                    // Make sure doesn't already have chunk
                    if self.visible_chunks.contains_key(&chunk_index) { continue; }

                    let start_instant = Instant::now();

                    // Create the chunk
                    let chunk = world.get_or_create_chunk(&chunk_index);
                    chunk.process_sides();

                    // Get chunk vertices
                    let mut vertices = Vec::new();
                    chunk.render(&mut vertices);

                    // Create mesh
                    let transform = [
                        [1., 0., 0., 0.],
                        [0., 1., 0., 0.],
                        [0., 0., 1., 0.],
                        [
                            chunk_x as f32 * world::Chunk::SIZE_X_F32,
                            chunk_z as f32 * world::Chunk::SIZE_Z_F32,  // Flip Y with Z
                            chunk_y as f32 * world::Chunk::SIZE_Y_F32,  // Flip Z with Y
                            1.
                        ]
                    ];
                    let vertex_buffer = glium::VertexBuffer::new(&app.display, &vertices[..]).unwrap();

                    // Save the mesh
                    self.visible_chunks.insert(chunk_index, ChunkMesh { transform, vertex_buffer });

                    println!("Rendered chunk {} {} {} - {:.2}", chunk_x, chunk_y, chunk_z, start_instant.elapsed().as_float_seconds());
                }
            }
        }
    }
}
