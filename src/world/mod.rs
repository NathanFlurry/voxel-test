mod block;
mod chunk;

pub use chunk::*;
pub use block::*;
use std::collections::HashMap;

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
    #[allow(dead_code)]  // TODO: Remove
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

impl World {
    pub fn fill_ellipsoid(&mut self, block: Block, lower: &WorldBlockIndex, upper: &WorldBlockIndex) {
        // Get radii of the ellipsoid; we add 1 to the radius, since drawing an ellipsoid directly
        // to the edge results in only one block touching the edge
        let rx = (upper.x as f64 - lower.x as f64) / 2. + 1.;
        let ry = (upper.y as f64 - lower.y as f64) / 2. + 1.;
        let rz = (upper.z as f64 - lower.z as f64) / 2. + 1.;

        // Get the center of the ellipsoid
        let cx = (lower.x as f64 + upper.x as f64) / 2.;
        let cy = (lower.y as f64 + upper.y as f64) / 2.;
        let cz = (lower.z as f64 + upper.z as f64) / 2.;

        // Set the blocks
        for x in lower.x..=upper.x {
            for y in lower.y..=upper.y {
                for z in lower.z..=upper.z {
                    // Get ellipsoid distance from the center
                    let dist =
                        ((x as f64 - cx as f64) / rx).powi(2) +
                        ((y as f64 - cy as f64) / ry).powi(2) +
                        ((z as f64 - cz as f64) / rz).powi(2);

                    // Check if distance is within ellipsoid
                    if dist <= 1. {
                        self.set_block(&WorldBlockIndex::new(x, y, z), block);
                    }
                }
            }
        }
    }
}
