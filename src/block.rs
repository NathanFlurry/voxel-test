use na::Point3;
use na::Point2;
use na::Vector3;

pub type BlockSides = u8;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
    const SPRITESHEET_WIDTH: usize = 1024;
    const SPRITESHEET_HEIGHT: usize = 2048;
    const SPRITESHEET_TILE_SIZE: usize = 128;
    const SPRITESHEET_UV_TILE_SIZE_X: f32 = Block::SPRITESHEET_TILE_SIZE as f32 / Block::SPRITESHEET_WIDTH as f32;
    const SPRITESHEET_UV_TILE_SIZE_Y: f32 = Block::SPRITESHEET_TILE_SIZE as f32 / Block::SPRITESHEET_HEIGHT as f32;

    pub const AIR: Block = Block(0);
    pub const DIRT: Block = Block(1);
    pub const STONE: Block = Block(2);
    pub const GRAVEL: Block = Block(3);
    pub const STONE_BRICK: Block = Block(4);
    pub const BRICK: Block = Block(5);

    pub fn is_air(&self) -> bool { *self == Block::AIR }

    pub fn is_transparent(&self) -> bool { self.is_air() }

    pub fn is_invisible(&self) -> bool { self.is_air() }

    pub fn texture_pos(&self) -> (u8, u8) {
        match *self {
            Block::DIRT => (5, 1),
            Block::STONE => (2, 5),
            Block::GRAVEL => (4, 0),
            Block::STONE_BRICK => (0, 0),
            Block::BRICK => (2, 12),
            _ => (0, 0)  // TODO: Add unknown block
        }
    }
}

impl Block {
    // TODO: Convert these constants to Point3?
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

    pub fn render(&self, vertices: &mut Vec<Point3<f32>>, faces: &mut Vec<Point3<u16>>, normals: &mut Vec<Vector3<f32>>, uvs: &mut Vec<Point2<f32>>,  x: f32, y: f32, z: f32, sides: u8) {
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
                    // Get coords
                    let mut coords_data = Block::VERTICES[face_index[pos]];
                    let coords = Point3::new(coords_data[0] + x, coords_data[1] + z, coords_data[2] + y);  // Flip Y and Z
                    vertices.push(coords);

                    // Get normal
                    let normal_data = Block::NORMALS[side as usize];
                    let normal = Vector3::new(normal_data[0], normal_data[1], normal_data[2]);
                    normals.push(normal);

                    // Get UV coords
                    let uv_coord_data = uv_coords[pos];
                    let uv_coord = Point2::new(uv_coord_data[0], uv_coord_data[1]);
                    uvs.push(uv_coord);
                }
            }
        }

        // Assert that there are the correct number items for vert data
        assert_eq!(vertices.len(), normals.len());
        assert_eq!(vertices.len(), uvs.len());

        // Add the faces
        let vert_count = (vertices.len() - start_vert_count) as u16;
        assert_eq!(vert_count % 3, 0);
        let start_vert_count = start_vert_count as u16;
        for i in 0..(vert_count / 3) {
            let base = start_vert_count + i * 3;
            faces.push(Point3::new(base, base + 1, base + 2));
        }
    }
}
