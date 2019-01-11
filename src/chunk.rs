use crate::block::Block;
use na::Point3;
use crate::block::BlockSides;
use na::Point2;
use na::Vector3;

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

type ChunkData = BlockDataArray<Block>;

type BlockSidesData = BlockDataArray<BlockSides>;

type BlockDataArray<T> = [[[T; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X];

pub struct Chunk {
    data: ChunkData,
    sides: BlockSidesData
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
            sides: [[[0b000000; Chunk::SIZE_Z]; Chunk::SIZE_Y]; Chunk::SIZE_X]
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

/*** SIDE PROCESSING ***/
impl Chunk {
    const SIDE_DIRS: [[DeltaDir; 3]; 6] = [
        [ DeltaDir::Z,  DeltaDir::N,  DeltaDir::Z ],  // Close
        [ DeltaDir::Z,  DeltaDir::P,  DeltaDir::Z ],  // Far
        [ DeltaDir::P,  DeltaDir::Z,  DeltaDir::Z ],  // Right
        [ DeltaDir::N,  DeltaDir::Z,  DeltaDir::Z ],  // Left
        [ DeltaDir::Z,  DeltaDir::Z,  DeltaDir::P ],  // Top
        [ DeltaDir::Z,  DeltaDir::Z,  DeltaDir::N ]   // Bottom
    ];

    pub fn process_sides(&mut self) -> () {
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

        // Register the sides if the block is not invisible
        if !self.data[x][y][z].is_invisible() {
            // Check each side of the block
            for side in 0..6 {
                // Get the direction to check
                let dir = &Chunk::SIDE_DIRS[side];

                // TODO: Need to check the next chunk over if it's at an edge

                // Find the index to check and make sure it's in range
                let dx = if let Some(x) = dir[0].checked_add_usize(x) { x } else { continue; };
                let dy = if let Some(y) = dir[1].checked_add_usize(y) { y } else { continue; };
                let dz = if let Some(z) = dir[2].checked_add_usize(z) { z } else { continue; };
                if dx < 0 || dx >= Chunk::SIZE_X || dy < 0 || dy >= Chunk::SIZE_Y || dz < 0 || dz >= Chunk::SIZE_Z { continue; }

                println!("dx dy dz {} {} {}", dx, dy, dz);

                // Check if the block can be seen from the given side
                if self.data[dx][dy][dz].is_transparent() {
                    sides |= 1 << side;
                }
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

/*** MESH GENERATION ***/
impl Chunk {
    // TODO: Add offset for the chunk
    pub fn render(&self, vertices: &mut Vec<Point3<f32>>, faces: &mut Vec<Point3<u16>>, normals: &mut Vec<Vector3<f32>>, uvs: &mut Vec<Point2<f32>>) {
        // Render each blocks
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                for z in 0..Chunk::SIZE_Z {
                    self.data[x][y][z].render(vertices, faces,  normals, uvs,x as f32, y as f32, z as f32, self.sides[x][y][z]);
                }
            }
        }
    }
}
