use crate::world::Chunk;
use crate::world::WorldDelegate;
use crate::world::ChunkBlockIndex;
use crate::world::Block;
use crate::world::ChunkIndex;
use noise::{NoiseFn, Seedable, MultiFractal};

type NoiseType = noise::Fbm;

pub struct ProceduralWorld {
    noise: NoiseType
}

impl ProceduralWorld {
    pub fn new(seed: u32) -> ProceduralWorld {
        ProceduralWorld {
            noise: NoiseType::new().set_seed(seed).set_frequency(0.05)
        }
    }
}

impl WorldDelegate for ProceduralWorld {
    fn create_chunk(&self, index: &ChunkIndex) -> Chunk {
        let mut chunk = Chunk::empty();

        // Create floor
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                // Sample the noise for the height
                let world_x = x + index.x as usize * Chunk::SIZE_X;
                let world_y = y + index.y as usize * Chunk::SIZE_Y;
                let height = (Chunk::SIZE_Z as f64 / 2.) +  self.noise.get([world_x as f64, world_y as f64]) * 10.;

                // Cap the height and cast to usize
                let height = height.max(0.).min(Chunk::SIZE_Z as f64) as usize;

                // Set the block to the given height
                for z in 0..=height as usize {
                    chunk.set_block(&ChunkBlockIndex::new(x, y, z), Block::DIRT);
                }
            }
        }

        chunk
    }
}
