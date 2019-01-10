use crate::block::Block;

pub struct ChunkBlockIndex {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl ChunkBlockIndex {
    #[inline]
    pub fn new(x: usize, y: usize, z: usize) -> ChunkBlockIndex {
        ChunkBlockIndex { x, y, z }
    }
}

pub struct Chunk {
    data: [[[Block; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]
}

impl Chunk {
    pub const SIZE_X: usize = 32;
    pub const SIZE_Y: usize = 64;
    pub const SIZE_Z: usize = 32;
    pub const SIZE_X_U32: u32 = Chunk::SIZE_X as u32;
    pub const SIZE_Y_U32: u32 = Chunk::SIZE_Y as u32;
    pub const SIZE_Z_U32: u32 = Chunk::SIZE_Z as u32;

    pub fn get_block(&self, position: &ChunkBlockIndex) -> &Block {
        &self.data[position.x][position.y][position.z]
    }

    pub fn get_block_mut(&mut self, position: &ChunkBlockIndex) -> &mut Block {
        &mut self.data[position.x][position.y][position.z]
    }
}
