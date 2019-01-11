use crate::chunk::Chunk;
use crate::world::WorldDelegate;
use crate::chunk::ChunkBlockIndex;
use crate::block::Block;
use crate::world::ChunkIndex;

pub struct ProceduralWorld {
    // TODO: Add seed and noise generator based on the seed
}

impl ProceduralWorld {
    pub fn new() -> ProceduralWorld {
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
