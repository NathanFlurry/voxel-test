use crate::world::Chunk;
use crate::world::WorldDelegate;
use crate::world::ChunkBlockIndex;
use crate::world::Block;
use crate::world::ChunkIndex;
use noise::{NoiseFn, Seedable, MultiFractal};
use std::time::Instant;
use crate::utils::AsFloatSeconds;

type NoiseType = noise::Fbm;

pub struct ProceduralWorld {
    height_noise: NoiseType,
    dirt_depth_noise: NoiseType
}

impl ProceduralWorld {
    pub fn new(seed: u32) -> ProceduralWorld {
        ProceduralWorld {
            height_noise: NoiseType::new().set_seed(seed).set_frequency(0.05),
            dirt_depth_noise: NoiseType::new().set_seed(seed+1).set_frequency(0.03)
        }
    }
}

impl WorldDelegate for ProceduralWorld {
    fn create_chunk(&self, index: &ChunkIndex) -> Chunk {
        let start_instant = Instant::now();

        let mut chunk = Chunk::empty();

        // Create floor
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                // Sample the noise for the height
                let world_x = x + index.x as usize * Chunk::SIZE_X;
                let world_y = y + index.y as usize * Chunk::SIZE_Y;
                let world_z = index.z as usize * Chunk::SIZE_Z;
                let noise_coords = [world_x as f64, world_y as f64];

                // Get the height of the terrain; saturating
                let height = (Chunk::SIZE_Z as f64 / 2.) +  self.height_noise.get(noise_coords) * 10.;
                let height = height as usize;

                // Determine the depth of the grass; saturating
                let grass_depth = (self.dirt_depth_noise.get(noise_coords) + 1.5) * 3.;
                let grass_depth = (grass_depth as usize).min(height);
                let grass_height = height - grass_depth;

                // Set the block to the given height
                for z in world_z..=height as usize {
                    let is_top = z == height;
                    let is_dirt = z >= grass_height;
                    let block_id = if is_dirt { if is_top { "dirt_grass" } else { "dirt" } } else { "stone" };
                    chunk.set_block(&ChunkBlockIndex::new(x, y, z - world_z), Block::from_id(block_id));
                }
            }
        }

        println!("Generated chunk {} {} {} - {:.2}", index.x, index.y, index.z, start_instant.elapsed().as_float_seconds());

        chunk
    }
}
