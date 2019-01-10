use crate::block::Block;

pub struct BlockIndex {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl BlockIndex {
    #[inline]
    pub fn new(x: usize, y: usize, z: usize) -> BlockIndex {
        BlockIndex { x, y, z }
    }

    #[inline]
    fn from_data_index(i: usize) -> BlockIndex {
        let z = i / (Chunk::SIZE_X * Chunk::SIZE_Y);
        let y = (i - z * Chunk::SIZE_X * Chunk::SIZE_Y) / Chunk::SIZE_X;
        let x = i - Chunk::SIZE_X * (y + Chunk::SIZE_Y * z);

        BlockIndex::new(x, y, z)
    }

    #[inline]
    fn get_data_index(&self) -> usize {
        self.x + Chunk::SIZE_X * (self.y + Chunk::SIZE_Y * self.z)
    }
}

pub struct Chunk {
    data: [Block; Chunk::DATA_SIZE]
}

impl Chunk {
    const SIZE_X: usize = 32;
    const SIZE_Y: usize = 64;
    const SIZE_Z: usize = 32;
    const DATA_SIZE: usize = Chunk::SIZE_X * Chunk::SIZE_Y * Chunk::SIZE_Z;  // 2^16

    fn get_block(&self, position: &BlockIndex) -> &Block {
        &self.data[position.get_data_index()]
    }

    fn get_block_mut(&mut self, position: &BlockIndex) -> &mut Block {
        &mut self.data[position.get_data_index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::chunk::BlockIndex;

    #[test]
    fn get_data_index() {
        // Manual data index counter
        let mut i = 0;

        // Iterate each possible x, y, and z value and check it against the calculated data index
        for z in 0..Chunk::SIZE_Z {
            for y in 0..Chunk::SIZE_Y {
                for x in 0..Chunk::SIZE_X {
                    let index = BlockIndex::new(x, y, z);
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
                    let index = BlockIndex::from_data_index(i);
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
