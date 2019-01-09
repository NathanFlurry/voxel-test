use crate::block::Block;

pub struct ChunkIndex {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl ChunkIndex {
    #[inline]
    pub fn new(x: usize, y: usize, z: usize) -> ChunkIndex {
        ChunkIndex { x, y, z }
    }

//    #[inline]
//    fn from_data_index(usize) -> ChunkPosition {
//
//    }

    #[inline]
    fn to_data_index(self) -> usize {
        self.x + self.y * Chunk::HORIZONTAL_SIZE + self.z * Chunk::HORIZONTAL_SIZE * Chunk::VERTICAL_SIZE
    }
}

pub struct Chunk {
    data: [Block; Chunk::DATA_SIZE]
}

impl Chunk {
    const HORIZONTAL_SIZE: usize = 32;
    const VERTICAL_SIZE: usize = 64;
    const DATA_SIZE: usize = Chunk::HORIZONTAL_SIZE * Chunk::VERTICAL_SIZE;  // u16

    fn get_block(&self, position: ChunkIndex) -> &Block {
        &self.data[position.to_data_index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::chunk::ChunkIndex;

    #[test]
    fn data_index() {
        // Manual data index counter
        let mut i = 0;

        // Iterate each possible x, y, and z value and check it against the calculated data index
        for z in 0..Chunk::HORIZONTAL_SIZE {
            for y in 0..Chunk::VERTICAL_SIZE {
                for x in 0..Chunk::HORIZONTAL_SIZE {
                    let index = ChunkIndex::new(x, y, z);
                    assert_eq!(index.to_data_index(), i);
                    i += 1;
                }
            }
        }
    }
}
