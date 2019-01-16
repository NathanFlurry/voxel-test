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
        [5, 4, 0, 1],  // Close;   RTC, LTC, LBC, RBC
        [7, 6, 2, 3],  // Far;     LTF, RTF, RBF, LBF
        [6, 5, 1, 2],  // Right;   RTF, RTC, RBC, RBF
        [4, 7, 3, 0],  // Left;    LTC, LTF, LBF, LBC
        [6, 7, 4, 5],  // Top;     RTF, LTF, LTC, RTC
        [1, 0, 3, 2]   // Bottom;  RBC, LBC, LBF, RBF
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

    /// Determines the edges for each pair of vertices on a face. For sample, if the face points to
    /// vertices [a, b, c, d], the corresponding array [5, 7, 9, 11] says that the vertices d -> a
    /// are edge 5, vertices a -> b are edge 7, vertices b -> c are 9, and vertices c -> d are edge
    /// 11. Basically, if you want to find the two edges that touch a vertex, look at the vertex
    /// index and the next item. So vertex a touches edges 5 and 7.
    const FACE_EDGES: [[usize; 4]; 6] = [
        [ 0,  2,  1,  3],  // Close;   CT, CL, CB, CR
        [ 5,  6,  4,  7],  // Far;     FT, FR, FB, FL
        [ 4,  8,  0,  9],  // Right;   RT, CR, RB, FR
        [ 1, 10,  5, 11],  // Left;    LT, FL, LB, CL
        [ 8,  6, 10,  2],  // Top;     FT, LT, CT, RT
        [ 9,  3, 11,  7]   // Bottom;  CB, LB, FB, RB
    ];

    const NORMALS: [[f32; 3]; 6] = [
        [ 0.,  0., -1.],
        [ 0.,  0.,  1.],
        [ 1.,  0.,  0.],
        [-1.,  0.,  0.],
        [ 0.,  1.,  0.],
        [ 0., -1.,  0.],
    ];

//    const UVS: [[f32; 2]; 4] = [
//        [0., 0.],
//        [1., 0.],
//        [1., 1.],
//        [0., 1.],
//    ];

    pub fn render(&self, vertices: &mut Vec<cg::Vertex>, x: f32, y: f32, z: f32, sides: u8, edges: u32) {
        // If the block is empty, do nothing
        if sides == 0b000000 { return; }

        // Find UV coordinates
        let texture_pos = self.texture_pos();
        let uv_lower = [texture_pos.0 as f32 * Block::SPRITESHEET_UV_TILE_SIZE_X, 1. - texture_pos.1 as f32 * Block::SPRITESHEET_UV_TILE_SIZE_Y - Block::SPRITESHEET_UV_TILE_SIZE_Y];
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
                    let vertex_index = face_index[pos];
                    let mut position = Block::VERTICES[vertex_index];
                    position[0] += x;
                    position[1] += z;  // Swap Y with Z
                    position[2] += y;  // Swap Z with Y

                    // Get the color
                    let has_edge_a = edges & (1 << Block::FACE_EDGES[side][pos]) != 0;
                    let has_edge_b = edges & (1 << Block::FACE_EDGES[side][(pos + 1) % 4]) != 0;
                    let shade_corner = !has_edge_a || !has_edge_b;
                    let darkness = 0.5;
//                    let color = if shade_corner { [0., 1., 0.] } else { [1., 1., 1.] };
                    let color = if shade_corner { [darkness, darkness, darkness] } else { [1., 1., 1.] };

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
                    self.data()[x][y][z].render(vertices, x as f32, y as f32, z as f32, self.sides()[x][y][z], self.edges()[x][y][z]);
                }
            }
        }
    }
}
