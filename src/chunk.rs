use crate::block::Block;

#[derive(Debug)]
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

type ChunkSide = u8;

type ChunkData = [[[Block; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X];

type ChunkSides = [[[ChunkSide; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X];

pub struct Chunk {
    data: ChunkData,
    sides: ChunkSides
}

impl Chunk {
    pub const SIZE_X: usize = 32;
    pub const SIZE_Y: usize = 32;
    pub const SIZE_Z: usize = 64;
    pub const SIZE_X_U32: u32 = Chunk::SIZE_X as u32;
    pub const SIZE_Y_U32: u32 = Chunk::SIZE_Y as u32;
    pub const SIZE_Z_U32: u32 = Chunk::SIZE_Z as u32;

    pub fn empty() -> Chunk {
        Chunk {
            data: [[[Block::AIR; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X],
            sides: [[[0; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]
        }
    }

    pub fn get_block(&self, position: &ChunkBlockIndex) -> &Block {
        &self.data[position.x][position.y][position.z]
    }

    pub fn get_block_mut(&mut self, position: &ChunkBlockIndex) -> &mut Block {
        &mut self.data[position.x][position.y][position.z]
    }

    pub fn set_block(&mut self, position: &ChunkBlockIndex, block: Block) {
        self.data[position.x][position.y][position.z] = block;
    }
}

impl Chunk {
    const SIDE_DIRS: [[DeltaDir; 3]; 6] = [
        [ DeltaDir::P,  DeltaDir::Z,  DeltaDir::Z ],
        [ DeltaDir::N,  DeltaDir::Z,  DeltaDir::Z ],
        [ DeltaDir::Z,  DeltaDir::P,  DeltaDir::Z ],
        [ DeltaDir::Z,  DeltaDir::N,  DeltaDir::Z ],
        [ DeltaDir::Z,  DeltaDir::Z,  DeltaDir::P ],
        [ DeltaDir::Z,  DeltaDir::Z,  DeltaDir::N ]
    ];

    fn process_sides(&mut self) -> () {
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                for z in 0..Chunk::SIZE_Z {
                    self.process_sides_for_index(x, y, z);
                }
            }
        }
    }

    fn process_sides_for_index(&mut self, x: usize, y: usize, z: usize) {
        let mut sides = 0;

        // Check each side of the block
        for side in 0..6 {
            // Get the direction to check
            let dir = &Chunk::SIDE_DIRS[side];

            // TODO: Need to check the next chunk over if it's at an edge

            // Find the index to check and make sure it's in range
            let dx = if let Some(x) = dir[0].checked_add_usize(x) { x } else { continue; };
            let dy = if let Some(y) = dir[1].checked_add_usize(y) { y } else { continue; };
            let dz = if let Some(z) = dir[2].checked_add_usize(z) { z } else { continue; };

            // Check if the block can be seen from the given side
            if self.data[dx][dy][dz].is_transparent() {
                sides |= 1 << side;
            }
        }

        // Save the side data
        self.sides[x][y][z] = sides as u8;
    }
}

enum DeltaDir {
    Negative, Zero, Positive
}

impl DeltaDir {
    pub const N: DeltaDir = DeltaDir::Negative;
    pub const Z: DeltaDir = DeltaDir::Zero;
    pub const P: DeltaDir = DeltaDir::Positive;

    fn checked_add_usize(&self, base: usize) -> Option<usize> {
        match self {
            DeltaDir::Negative => base.checked_sub(1),
            DeltaDir::Zero => Some(base),
            DeltaDir::Positive => base.checked_add(1)
        }
    }
}
