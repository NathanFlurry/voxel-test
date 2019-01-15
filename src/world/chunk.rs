use crate::world::block::Block;
use crate::world::block::BlockSides;
use crate::world::block::BlockEdgeAO;
use crate::utils;

#[derive(Debug)]
pub struct ChunkBlockIndex {  // TODO: Remove all these unneeded structures
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

type BlockDataArray<T> = [[[T; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X];
type ChunkData = BlockDataArray<Block>;
type BlockSidesData = BlockDataArray<BlockSides>;
type BlockCornerAOData = BlockDataArray<BlockEdgeAO>;

const N: DeltaDir = DeltaDir::Negative;
const Z: DeltaDir = DeltaDir::Zero;
const P: DeltaDir = DeltaDir::Positive;

pub struct Chunk {
    data: ChunkData,
    sides: BlockSidesData,
    edge_ao: BlockCornerAOData
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
            sides: [[[0b000000; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X],
            edge_ao: [[[[0b0000; 6]; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X],
        }
    }

    pub fn get_block(&self, position: &ChunkBlockIndex) -> &Block {
        &self.data[position.x][position.y][position.z]
    }

    pub fn set_block(&mut self, position: &ChunkBlockIndex, block: Block) {
        self.data[position.x][position.y][position.z] = block;
    }
}

impl Chunk {
    pub fn data(&self) -> &ChunkData {
        &self.data
    }

    pub fn sides(&self) -> &BlockSidesData {
        &self.sides
    }

    pub fn corner_ao(&self) -> &BlockCornerAOData {
        &self.edge_ao
    }
}

/*** SIDE PROCESSING ***/
impl Chunk {
    const SIDE_DIRS: [[DeltaDir; 3]; 6] = [
        [Z, N, Z],  // Close
        [Z, P, Z],  // Far
        [P, Z, Z],  // Right
        [N, Z, Z],  // Left
        [Z, Z, P],  // Top
        [Z, Z, N],   // Bottom
    ];

//    const CORNER_DIRS: [[DeltaDir; 3]; 8] = [ // X and Y are flipped to reflect corresponding coords
//        [DeltaDir::N, DeltaDir::N,  DeltaDir::N],  // 0: LBC
//        [DeltaDir::P, DeltaDir::N,  DeltaDir::N],  // 1: RBC
//        [DeltaDir::P, DeltaDir::P,  DeltaDir::N],  // 2: RBF
//        [DeltaDir::N, DeltaDir::P,  DeltaDir::N],  // 3: LBF
//        [DeltaDir::N, DeltaDir::N,  DeltaDir::P],  // 4: LTC
//        [DeltaDir::P, DeltaDir::N,  DeltaDir::P],  // 5: RTC
//        [DeltaDir::P, DeltaDir::P,  DeltaDir::P],  // 6: RTF
//        [DeltaDir::N, DeltaDir::P,  DeltaDir::P]   // 7: LTF
//    ];

    const EDGE_OFFSETS: [[DetlaDir; 2]; 4] = [
        [N, N],  // BL
        [N, P],  // TL
        [P, P],  // TR
        [P, N],  // BR
    ];

    pub fn process_sides(&mut self) -> () {  // TODO: Rename this to `clean_sides` and make `process_sides_for_index` get called every time a block changes
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                for z in 0..Chunk::SIZE_Z {
                    self.process_sides_for_index(x, y, z);
                }
            }
        }
    }

    fn process_sides_for_index(&mut self, x: usize, y: usize, z: usize) {
        let mut sides = 0b000000;
        let mut edge_ao = [0b0000; 6];

        // Register the sides if the block is not invisible
        if !self.data[x][y][z].is_invisible() {
            // Check each side of the block
            for side in 0..6 {
                let dir = &Chunk::SIDE_DIRS[side];

                if let Some(block) = self.get_block_from_dir(x, y, z, dir) {
                    // Show the side if there is no visible block there
                    if block.is_transparent() {
                        // Flag the side as visible
                        sides |= 1 << side;

                        // Check the edges
                        for edge_index in 0..4 {
                            let edge_offset = &Chunk::EDGE_OFFSETS[edge_index];

                            let edge_dir = DeltaDir::merge_dirs(dir, edge_offset);

                            // Flag the edge as dark
                            if let Some(block) = self.get_block_from_dir(x, y, z, &edge_dir) {
                                if !block.is_transparent() {
                                    edge_ao[side] |= 1 << edge_index;
                                }
                            }
                        }
                    }
                } else {
                    // TODO: Need to check the next chunk over if it's at an edge
                    // HACK: Show the edge of the chunk by default so we don't have to check the
                    // block in the next chunk over
                    sides |= 1 << side;
                }
            }
        }

        // Save the side data
        self.sides[x][y][z] = sides as u8;
        self.edge_ao[x][y][z] = corner_ao as u8;
    }

    fn get_block_from_dir(&self, x: usize, y: usize, z: usize, dir: &[DeltaDir; 3]) -> Option<Block> {
        // Get the new X, Y, and Z position
        let dx = if let Some(x) = dir[0].checked_add_usize(x) { x } else { return None; };
        let dy = if let Some(y) = dir[1].checked_add_usize(y) { y } else { return None; };
        let dz = if let Some(z) = dir[2].checked_add_usize(z) { z } else { return None; };

        // Check that the block is not outside of the chunk
        if dx >= Chunk::SIZE_X || dy >= Chunk::SIZE_Y || dz >= Chunk::SIZE_Z { return None; }

        // Return the block
        Some(self.data[dx][dy][dz])
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
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

    fn matches_side(&self, index: usize, max: usize) -> bool {
        return (*self == DeltaDir::Negative && index == 0) || (*self == DeltaDir::Positive && index == max - 1);
    }

    fn flip(&self) -> DeltaDir {
        match self {
            DeltaDir::Negative => DeltaDir::Positive,
            DeltaDir::Zero => DeltaDir::Zero,
            DeltaDir::Positive => DeltaDir::Negative
        }
    }

    fn merge_dirs(base: &[DeltaDir; 3], merge: &[DeltaDir; 2]) -> [DeltaDir; 3] {
        if base[0] != DeltaDir::Zero {
            [base[0], merge[0], merge[1]]
        } else if base[1] != DeltaDir::Zero {
            [merge[0], base[1], merge[1]]
        } else if base[2] != DeltaDir::Zero {
            [merge[0], merge[1], base[1]]
        } else {
            panic!("Base is all zero");
        }
    }
}
