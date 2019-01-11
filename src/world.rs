use crate::chunk::Chunk;
use std::collections::HashMap;
use crate::block::Block;
use crate::chunk::ChunkBlockIndex;

#[derive(Debug, Clone)]
pub struct WorldBlockIndex {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl WorldBlockIndex {
    #[inline]
    pub fn new(x: u32, y: u32, z: u32) -> WorldBlockIndex {
        WorldBlockIndex { x, y, z }
    }

    #[inline]
    pub fn get_chunk_index(&self) -> ChunkIndex {
        ChunkIndex::new(self.x / Chunk::SIZE_X_U32, self.y / Chunk::SIZE_Y_U32, self.z / Chunk::SIZE_Z_U32)
    }

    #[inline]
    pub fn get_chunk_block_index(&self) -> ChunkBlockIndex {
        let chunk_index = self.get_chunk_index();
        ChunkBlockIndex::new(
            self.x as usize - chunk_index.x as usize * Chunk::SIZE_X,
            self.y as usize - chunk_index.y as usize * Chunk::SIZE_Y,
            self.z as usize - chunk_index.z as usize * Chunk::SIZE_Z
        )
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

    pub fn set_block(&mut self, index: &WorldBlockIndex, block: Block) {
        let chunk = self.get_or_create_chunk(&index.get_chunk_index());
        chunk.set_block(&index.get_chunk_block_index(), block);
    }
}
