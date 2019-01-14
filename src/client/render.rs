use super::cg;
use crate::world::Block;
use crate::world::Chunk;

impl Block {
    const SPRITESHEET_WIDTH: usize = 1024;
    const SPRITESHEET_HEIGHT: usize = 2048;
    const SPRITESHEET_TILE_SIZE: usize = 128;
    const SPRITESHEET_UV_TILE_SIZE_X: f32 = Block::SPRITESHEET_TILE_SIZE as f32 / Block::SPRITESHEET_WIDTH as f32;
    const SPRITESHEET_UV_TILE_SIZE_Y: f32 = Block::SPRITESHEET_TILE_SIZE as f32 / Block::SPRITESHEET_HEIGHT as f32;

    const FACES: [[usize; 4]; 6] = [
        [5, 4, 0, 1],  // Close
        [7, 6, 2, 3],  // Far
        [6, 5, 1, 2],  // Right
        [4, 7, 3, 0],  // Left
        [6, 7, 4, 5],  // Top
        [1, 0, 3, 2]   // Bottom
    ];

    const VERTICES: [[f32; 3]; 8] = [
        [0., 0., 0.],  // 0: LBC
        [1., 0., 0.],  // 1: RBC
        [1., 0., 1.],  // 2: RBF
        [0., 0., 1.],  // 3: LBF
        [0., 1., 0.],  // 4: LTC
        [1., 1., 0.],  // 5: RTC
        [1., 1., 1.],  // 6: RTF
        [0., 1., 1.]   // 7: LTF
    ];

    const FACE_ORDER: [usize; 6] = [
        0, 3, 1, 1, 3, 2
    ];

    const NORMALS: [[f32; 3]; 6] = [
        [ 0.,  0., -1.],
        [ 0.,  0.,  1.],
        [ 1.,  0.,  0.],
        [-1.,  0.,  0.],
        [ 0.,  1.,  0.],
        [ 0., -1.,  0.],
    ];

    const UVS: [[f32; 2]; 4] = [
        [0., 0.],
        [1., 0.],
        [1., 1.],
        [0., 1.],
    ];

    pub fn render(&self, vertices: &mut Vec<cg::Vertex>, x: f32, y: f32, z: f32, sides: u8) {
        // If the block is empty, do nothing
        if sides == 0b000000 { return; }

        // Find UV coordinates
        let texture_pos = self.texture_pos();
        let uv_lower = [texture_pos.0 as f32 * Block::SPRITESHEET_UV_TILE_SIZE_X, texture_pos.1 as f32 * Block::SPRITESHEET_UV_TILE_SIZE_Y];
        let uv_upper = [uv_lower[0] + Block::SPRITESHEET_UV_TILE_SIZE_X, uv_lower[1] + Block::SPRITESHEET_UV_TILE_SIZE_Y];
        let uv_coords = [
            [uv_lower[0], uv_lower[1]],  // 0, 0
            [uv_upper[0], uv_lower[1]],  // 1, 0
            [uv_upper[0], uv_upper[1]],  // 1, 1
            [uv_lower[0], uv_upper[1]],  // 0, 1
        ];

        // Add the vertices
        let start_vert_count = vertices.len();
        for side in 0..6 {
            if sides & (1 << side) != 0b000000 {
                // Add the vert data
                let face_index = &Block::FACES[side];
                for &pos in &Block::FACE_ORDER {
                    // Get position
                    let mut position = Block::VERTICES[face_index[pos]];
                    position[0] += x;
                    position[1] += z;  // Swap Y with Z
                    position[2] += y;  // Swap Z with Y

                    // Get the color
                    let color = [1., 0., 0.];

                    // Get normal
                    let normal = Block::NORMALS[side as usize];

                    // Get UV coords
                    let uv = uv_coords[pos];

                    vertices.push(cg::Vertex { position, color, normal, uv });
                }
            }
        }
    }
}

impl Chunk {
    // TODO: Add offset for the chunk
    pub fn render(&self, vertices: &mut Vec<cg::Vertex>) {
        // Render each blocks
        for x in 0..Chunk::SIZE_X {
            for y in 0..Chunk::SIZE_Y {
                for z in 0..Chunk::SIZE_Z {
                    self.data()[x][y][z].render(vertices, x as f32, y as f32, z as f32, self.sides()[x][y][z]);
                }
            }
        }
    }
}
