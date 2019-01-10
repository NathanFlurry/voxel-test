use crate::block::Block;

pub struct ChunkBlockIndex {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl ChunkBlockIndex {
    #[inline]
    pub fn new(x: u32, y: u32, z: u32) -> ChunkBlockIndex {
        ChunkBlockIndex { x, y, z }
    }

    #[inline]
    fn from_data_index(i: usize) -> ChunkBlockIndex {
        let i = i as u32;
        let z = i / (Chunk::SIZE_X * Chunk::SIZE_Y);
        let y = (i - z * Chunk::SIZE_X * Chunk::SIZE_Y) / Chunk::SIZE_X;
        let x = i - Chunk::SIZE_X * (y + Chunk::SIZE_Y * z);

        ChunkBlockIndex::new(x, y, z)
    }

    #[inline]
    fn get_data_index(&self) -> usize {
        self.x as usize + Chunk::SIZE_X_USIZE * (self.y as usize + Chunk::SIZE_Y_USIZE * self.z as usize)
    }
}

pub struct Chunk {
    data: [Block; Chunk::DATA_SIZE]
}

impl Chunk {
    pub const SIZE_X: u32 = 32;
    pub const SIZE_Y: u32 = 64;
    pub const SIZE_Z: u32 = 32;
    pub const SIZE_X_USIZE: usize = Chunk::SIZE_X as usize;
    pub const SIZE_Y_USIZE: usize = Chunk::SIZE_Y as usize;
    pub const SIZE_Z_USIZE: usize = Chunk::SIZE_Z as usize;
    const DATA_SIZE: usize = Chunk::SIZE_X_USIZE * Chunk::SIZE_Y_USIZE * Chunk::SIZE_Z_USIZE;  // 2^16

    pub fn get_block(&self, position: &ChunkBlockIndex) -> &Block {
        // TODO: Is it faster to look up data in a 3d array or do the data index thing?
        &self.data[position.get_data_index()]
    }

    pub fn get_block_mut(&mut self, position: &ChunkBlockIndex) -> &mut Block {
        &mut self.data[position.get_data_index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::chunk::ChunkBlockIndex;

    #[test]
    fn get_data_index() {
        // Manual data index counter
        let mut i = 0;

        // Iterate each possible x, y, and z value and check it against the calculated data index
        for z in 0..Chunk::SIZE_Z {
            for y in 0..Chunk::SIZE_Y {
                for x in 0..Chunk::SIZE_X {
                    let index = ChunkBlockIndex::new(x, y, z);
                    assert_eq!(index.get_data_index(), i);
                    i += 1;
                }
            }
        }
    }

    #[test]
    fn from_data_index() {
        // Manual data index counter
        let mut i = 0;

        // Iterate each possible x, y, and z value and check it against the calculated data index
        for z in 0..Chunk::SIZE_Z {
            for y in 0..Chunk::SIZE_Y {
                for x in 0..Chunk::SIZE_X {
                    let index = ChunkBlockIndex::from_data_index(i);
                    println!("i {}", i);
                    assert_eq!(index.x, x);
                    assert_eq!(index.y, y);
                    assert_eq!(index.z, z);
                    i += 1;
                }
            }
        }
    }
}
