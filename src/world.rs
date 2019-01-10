use crate::chunk::Chunk;
use std::collections::HashMap;
use crate::block::Block;
use crate::chunk::ChunkBlockIndex;

#[derive(Clone)]
pub struct WorldBlockIndex {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl WorldBlockIndex {
    #[inline]
    pub fn get_chunk_index(&self) -> ChunkIndex {
        ChunkIndex::new(self.x / Chunk::SIZE_X, self.y / Chunk::SIZE_Y, self.z / Chunk::SIZE_Z)
    }

    #[inline]
    pub fn get_chunk_block_index(&self) -> ChunkBlockIndex {
        let chunk_index = self.get_chunk_index();
        ChunkBlockIndex::new(
            self.x - chunk_index.x * Chunk::SIZE_X,
            self.y - chunk_index.y * Chunk::SIZE_Y,
            self.z - chunk_index.z * Chunk::SIZE_Z,
        )
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]  // TODO: Can I get rid of partial equal
pub struct ChunkIndex {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl ChunkIndex {
    #[inline]
    pub fn new(x: u32, y: u32, z: u32) -> ChunkIndex {
        ChunkIndex { x, y, z }
    }
}

pub trait WorldDelegate {
    fn create_chunk(&self, index: &ChunkIndex) -> Chunk;
}

pub struct World {
    chunks: HashMap<ChunkIndex, Chunk>,
    delegate: Box<WorldDelegate>
}

impl World {
    pub const WORLD_CENTER_INDEX: u32 = 2147483648;  // (2 ^ 32) / 2

    pub fn new(delegate: Box<WorldDelegate>) -> World {
        World {
            chunks: HashMap::new(),
            delegate
        }
    }

    pub fn get_or_create_chunk(&mut self, index: &ChunkIndex) -> &mut Chunk {
        // Create new chunk if needed
        if !self.chunks.contains_key(&index) {
            // Create the chunk
            let chunk = self.delegate.create_chunk(index);

            // Insert the chunk
            self.chunks.insert(index.clone(), chunk);
        }

        self.chunks.get_mut(&index).unwrap()
    }

    pub fn get_block_mut(&mut self, index: &WorldBlockIndex) -> &mut Block {
        let chunk = self.get_or_create_chunk(&index.get_chunk_index());
        chunk.get_block_mut(&index.get_chunk_block_index())
    }
}
