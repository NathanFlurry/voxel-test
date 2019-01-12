use crate::chunk::Chunk;
use crate::world::WorldDelegate;
use crate::chunk::ChunkBlockIndex;
use crate::block::Block;
use crate::world::ChunkIndex;
use noise::{NoiseFn, Seedable, MultiFractal};

type NoiseType = noise::Fbm;

pub struct ProceduralWorld {
    noise: NoiseType
}

impl ProceduralWorld {
    pub fn new(seed: u32) -> ProceduralWorld {
        ProceduralWorld {
            noise: NoiseType::new().set_seed(seed).set_octaves(2)
        }
    }
}

impl WorldDelegate for ProceduralWorld {
    fn create_chunk(&self, index: &ChunkIndex) -> Chunk {
        let mut chunk = Chunk::empty();

        // Create floor
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                let scale = 0.05;
                let height = (Chunk::SIZE_Z as f64 / 2.) +  self.noise.get([x as f64 * scale, y as f64 * scale]) * 10.;
                let height = height.max(0.).min(Chunk::SIZE_Z as f64) as usize;

                for z in 0..=height as usize {
                    chunk.set_block(&ChunkBlockIndex::new(x, y, z), Block::DIRT);
                }

//                chunk.set_block(&ChunkBlockIndex::new(x, y, if x*y%2==0 { 3 } else { 0 }), Block::DIRT);

//                chunk.set_block(&ChunkBlockIndex::new(x, y, 0), Block::DIRT);
            }
        }

        chunk
    }
}
