use crate::world::block::Block;
use crate::world::block::BlockSides;
use crate::world::block::BlockEdges;
use crate::world::block::BlockCorners;

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
type BlockEdgeData = BlockDataArray<BlockEdges>;
type BlockCornerData = BlockDataArray<BlockCorners>;

const N: DeltaDir = DeltaDir::Negative;
const Z: DeltaDir = DeltaDir::Zero;
const P: DeltaDir = DeltaDir::Positive;

pub struct Chunk {
    data: Box<ChunkData>,
    sides: Box<BlockSidesData>,
    edges: Box<BlockEdgeData>,
    corners: Box<BlockCornerData>,
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
            data: Box::new([[[Block::AIR; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]),
            sides: Box::new([[[0b000000; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]),
            edges: Box::new([[[0b00000000000; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]),
            corners: Box::new([[[0b0000000; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X])
        }
    }
    #[allow(dead_code)] // TODO: Remove
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

    pub fn edges(&self) -> &BlockEdgeData {
        &self.edges
    }

    pub fn corners(&self) -> &BlockCornerData {
        &self.corners
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

    const EDGE_DIRS: [[DeltaDir; 3]; 12] = [
        [P, N, Z],  //  0: CR
        [N, N, Z],  //  1: CL
        [Z, N, P],  //  2: CT
        [Z, N, N],  //  3: CB
        [P, P, Z],  //  4: FR
        [N, P, Z],  //  5: FL
        [Z, P, P],  //  6: FT
        [Z, P, N],  //  7: FB
        [P, Z, P],  //  8: RT
        [P, Z, N],  //  9: RB
        [N, Z, P],  // 10: LT
        [N, Z, N],  // 11: LB
    ];

    const CORNER_DIRS: [[DeltaDir; 3]; 8] = [  // Y and Z are flipped
        [N, N, N],  // 0: LBC
        [P, N, N],  // 1: RBC
        [P, P, N],  // 2: RBF
        [N, P, N],  // 3: LBF
        [N, N, P],  // 4: LTC
        [P, N, P],  // 5: RTC
        [P, P, P],  // 6: RTF
        [N, P, P],  // 7: LTF
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
        let mut edges = 0b00000000000;
        let mut corners = 0b00000000;

        if !self.data[x][y][z].is_invisible() {
            // Check each side of the block
            for side in 0..6 {
                let dir = &Chunk::SIDE_DIRS[side];

                if let Some(block) = self.get_block_from_dir(x, y, z, dir) {
                    // Show the side if there is no visible block there
                    if block.is_transparent() {
                        sides |= 1 << side;
                    }
                } else {
                    // TODO: Need to check the next chunk over if it's at an edge
                    // HACK: Show the edge of the chunk by default so we don't have to check the
                    // block in the next chunk over
                    sides |= 1 << side;
                }
            }

            // Check each edge of the block
            for edge in 0..12 {
                let dir = &Chunk::EDGE_DIRS[edge];

                if let Some(block) = self.get_block_from_dir(x, y, z, dir) {
                    // Show the edge if there is no visible block there
                    if block.is_transparent() {
                        edges |= 1 << edge;
                    }
                } else {
                    // HACK: See above
                    edges |= 1 << edge;
                }
            }

            // Check each corner of the block
            for corner in 0..8 {
                let dir = &Chunk::CORNER_DIRS[corner];

                if let Some(block) = self.get_block_from_dir(x, y, z, dir) {
                    // Show the edge if there is no visible block there
                    if block.is_transparent() {
                        corners |= 1 << corner;
                    }
                } else {
                    // HACK: See above
                    corners |= 1 << corner;
                }
            }
        }

        // Save the side data
        self.sides[x][y][z] = sides as u8;
        self.edges[x][y][z] = edges as u32;
        self.corners[x][y][z] = corners as u8;
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
    fn checked_add_usize(&self, base: usize) -> Option<usize> {
        match self {
            DeltaDir::Negative => base.checked_sub(1),
            DeltaDir::Zero => Some(base),
            DeltaDir::Positive => base.checked_add(1)
        }
    }
}
